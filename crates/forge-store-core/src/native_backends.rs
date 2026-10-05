//! Fixed, authenticated OS socket boundary. This client never owns root package policy.
use crate::backends::BackendError;
use crate::catalogue::{Artifact, Delivery, MAX_NATIVE_DEB_BYTES};
use serde::Deserialize;
use serde_json::{json, Value};

#[cfg(unix)]
pub const NATIVE_SOCKET: &str = "/run/forge-native-package/forge-native-package.sock";

#[derive(Clone, Debug)]
pub struct NativeRequest(Value);

fn token(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 64
        && value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"_-".contains(&b))
}

fn package_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 64
        && !value.starts_with(['-', '.'])
        && !value.ends_with(['-', '.'])
        && value
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b".-".contains(&b))
}

impl NativeRequest {
    fn new(request_id: &str, operation: &str) -> Result<Self, BackendError> {
        if !token(request_id) {
            return Err(BackendError::Invalid("invalid native request ID"));
        }
        Ok(Self(
            json!({"schemaVersion":1,"requestId":request_id,"operation":operation}),
        ))
    }

    pub fn probe(request_id: &str) -> Result<Self, BackendError> {
        Self::new(request_id, "probe")
    }

    pub fn status(request_id: &str, id: &str) -> Result<Self, BackendError> {
        if !package_id(id) {
            return Err(BackendError::Invalid("invalid native package ID"));
        }
        let mut req = Self::new(request_id, "status")?;
        req.0["id"] = json!(id);
        Ok(req)
    }

    pub fn mutation(
        request_id: &str,
        operation: &str,
        id: &str,
        artifact: Option<&Artifact>,
    ) -> Result<Self, BackendError> {
        if !matches!(operation, "install" | "update" | "uninstall") || !package_id(id) {
            return Err(BackendError::Invalid("invalid native operation or ID"));
        }
        let mut req = Self::new(request_id, operation)?;
        req.0["id"] = json!(id);
        if let Some(artifact) = artifact {
            if operation == "uninstall" {
                return Err(BackendError::Invalid("uninstall cannot stage artifacts"));
            }
            artifact.validate_remote(MAX_NATIVE_DEB_BYTES)?;
            if !crate::catalogue::valid_digest(&artifact.sha256) {
                return Err(BackendError::Invalid("noncanonical native digest"));
            }
            req.0["sha256"] = json!(artifact.sha256);
            req.0["size"] = json!(artifact.size);
        }
        Ok(req)
    }

    pub fn to_line(&self) -> Result<Vec<u8>, BackendError> {
        let mut bytes = serde_json::to_vec(&self.0)?;
        bytes.push(b'\n');
        if bytes.len() > 4096 {
            return Err(BackendError::Invalid("native request exceeds 4096 bytes"));
        }
        Ok(bytes)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Reply {
    schema_version: u32,
    request_id: String,
    ok: bool,
    result: Option<Value>,
    error: Option<ServiceError>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ServiceError {
    code: String,
    message: String,
}

pub fn decode_native_reply(bytes: &[u8], request_id: &str) -> Result<Value, BackendError> {
    if bytes.len() > 4096
        || bytes.last() != Some(&b'\n')
        || bytes[..bytes.len().saturating_sub(1)].contains(&b'\n')
    {
        return Err(BackendError::Invalid(
            "native reply exceeds bounds or framing",
        ));
    }
    let reply: Reply = serde_json::from_slice(&bytes[..bytes.len() - 1])?;
    let raw: Value = serde_json::from_slice(&bytes[..bytes.len() - 1])?;
    let result_field = if reply.ok { "result" } else { "error" };
    if !fields(&raw, &["schemaVersion", "requestId", "ok", result_field]) {
        return Err(BackendError::Invalid(
            "native response envelope is not closed",
        ));
    }
    if reply.schema_version != 1 || reply.request_id != request_id {
        return Err(BackendError::Invalid("native reply identity mismatch"));
    }
    match (reply.ok, reply.result, reply.error) {
        (true, Some(result), None) => Ok(result),
        (false, None, Some(error))
            if token(&error.code)
                && error.message.len() <= 512
                && !error.message.chars().any(char::is_control) =>
        {
            Err(BackendError::Service {
                code: error.code,
                message: error.message,
            })
        }
        _ => Err(BackendError::Invalid("native reply has invalid status")),
    }
}

fn fields(value: &Value, names: &[&str]) -> bool {
    value
        .as_object()
        .is_some_and(|o| o.len() == names.len() && names.iter().all(|name| o.contains_key(*name)))
}

pub fn native_probe(result: &Value) -> Result<Vec<(String, bool, String)>, BackendError> {
    if !fields(result, &["backends"]) {
        return Err(BackendError::Invalid("invalid native probe"));
    }
    let rows = result["backends"]
        .as_array()
        .ok_or(BackendError::Invalid("invalid native probe list"))?;
    if rows.len() != 2 {
        return Err(BackendError::Invalid(
            "native probe must describe both backends",
        ));
    }
    let mut output = Vec::new();
    for row in rows {
        if !fields(row, &["backend", "available", "detail"]) {
            return Err(BackendError::Invalid("invalid native probe fields"));
        }
        let name = row["backend"]
            .as_str()
            .ok_or(BackendError::Invalid("invalid probe backend"))?;
        let available = row["available"]
            .as_bool()
            .ok_or(BackendError::Invalid("invalid probe availability"))?;
        let detail = row["detail"]
            .as_str()
            .ok_or(BackendError::Invalid("invalid probe detail"))?;
        if !matches!(name, "ubuntu-deb" | "snap")
            || output.iter().any(|(n, _, _)| n == name)
            || detail.len() > 512
            || detail.chars().any(char::is_control)
        {
            return Err(BackendError::Invalid("invalid native probe identity"));
        }
        output.push((name.into(), available, detail.into()));
    }
    Ok(output)
}

pub fn native_status_matches(
    result: &Value,
    delivery: &Delivery,
    installed: bool,
) -> Result<(), BackendError> {
    if !fields(
        result,
        &[
            "id",
            "backend",
            "installed",
            "version",
            "revision",
            "matchesPolicy",
            "identity",
        ],
    ) || result["installed"] != json!(installed)
        || !result["matchesPolicy"].is_boolean()
    {
        return Err(BackendError::Invalid("invalid native status fields"));
    }
    let identity = &result["identity"];
    let matched = match delivery {
        Delivery::UbuntuDeb {
            reviewed_package_id,
            package,
            version,
            architecture,
            distribution,
            release,
            ..
        } => {
            result["id"] == json!(reviewed_package_id)
                && result["backend"] == json!("ubuntu-deb")
                && fields(
                    identity,
                    &["package", "architecture", "distribution", "release"],
                )
                && identity
                    == &json!({"package":package,"architecture":architecture,"distribution":distribution,"release":release})
                && result["revision"].is_null()
                && if installed {
                    result["version"] == json!(version)
                } else {
                    result["version"].is_null()
                }
        }
        Delivery::Snap {
            reviewed_package_id,
            name,
            snap_id,
            revision,
            confinement,
            ..
        } => {
            result["id"] == json!(reviewed_package_id)
                && result["backend"] == json!("snap")
                && fields(identity, &["name", "snapId", "confinement"])
                && identity == &json!({"name":name,"snapId":snap_id,"confinement":confinement})
                && if installed {
                    result["revision"] == json!(revision)
                        && result["version"].as_str().is_some_and(|v| {
                            !v.is_empty() && v.len() <= 128 && !v.chars().any(char::is_control)
                        })
                } else {
                    result["revision"].is_null() && result["version"].is_null()
                }
        }
        _ => false,
    };
    if !matched || installed && result["matchesPolicy"] != json!(true) {
        return Err(BackendError::Invalid(
            "native installed identity or pin differs from signed catalogue",
        ));
    }
    Ok(())
}

/// Check an actual or absent identity before mutation while allowing an older
/// installed version for updates. Final success still requires the exact pin.
pub fn native_identity_matches(result: &Value, delivery: &Delivery) -> Result<(), BackendError> {
    if !result["installed"].is_boolean()
        || !(result["version"].is_null()
            || result["version"]
                .as_str()
                .is_some_and(|s| s.len() <= 128 && !s.chars().any(char::is_control)))
        || !(result["revision"].is_null()
            || result["revision"]
                .as_u64()
                .is_some_and(|n| n > 0 && n <= i32::MAX as u64))
    {
        return Err(BackendError::Invalid("invalid native preflight state"));
    }
    let mut target = result.clone();
    target["installed"] = json!(false);
    target["version"] = Value::Null;
    target["revision"] = Value::Null;
    native_status_matches(&target, delivery, false)
}

pub struct NativeClient;
impl NativeClient {
    #[cfg(unix)]
    pub async fn call(&self, request: NativeRequest) -> Result<Value, BackendError> {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        let request_id = request.0["requestId"]
            .as_str()
            .ok_or(BackendError::Invalid("missing native request ID"))?
            .to_owned();
        let line = request.to_line()?;
        let exchange = async {
            let mut socket = tokio::net::UnixStream::connect(NATIVE_SOCKET).await?;
            if socket.peer_cred()?.uid() != 0 {
                return Err(BackendError::Invalid(
                    "native package service is not root-owned",
                ));
            }
            socket.write_all(&line).await?;
            socket.shutdown().await?;
            let mut reply = Vec::new();
            socket.take(4097).read_to_end(&mut reply).await?;
            decode_native_reply(&reply, &request_id)
        };
        tokio::time::timeout(std::time::Duration::from_secs(1800), exchange)
            .await
            .map_err(|_| {
                BackendError::Io(std::io::Error::new(
                    std::io::ErrorKind::TimedOut,
                    "native outcome unknown; replay same request ID",
                ))
            })?
    }

    #[cfg(not(unix))]
    pub async fn call(&self, _request: NativeRequest) -> Result<Value, BackendError> {
        Err(BackendError::Invalid(
            "native packages require Ubuntu Unix service",
        ))
    }
}
