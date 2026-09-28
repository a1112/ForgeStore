use crate::catalogue::{Artifact, CatalogError, MAX_FORGEPKG_BYTES};
use crate::queue::Action;
use serde_json::{json, Map, Value};
use std::collections::BTreeMap;
use std::path::Path;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BackendError {
    #[error("invalid reviewed backend input: {0}")]
    Invalid(&'static str),
    #[error("catalogue artifact: {0}")]
    Catalog(#[from] CatalogError),
    #[error("JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("backend I/O: {0}")]
    Io(#[from] std::io::Error),
    #[error("backend service rejected operation: {code}: {message}")]
    Service { code: String, message: String },
    #[error("backend process failed: {0}")]
    Process(String),
}

pub fn decode_compatforge_reply(
    bytes: &[u8],
    request_id: &str,
    operation: &str,
) -> Result<Value, BackendError> {
    if bytes.len() > 1024 * 1024 {
        return Err(BackendError::Invalid("CompatForge reply exceeds limit"));
    }
    let reply: Value = serde_json::from_slice(bytes)?;
    if reply.get("schemaVersion") != Some(&json!("1"))
        || reply.get("requestId") != Some(&json!(request_id))
        || reply.get("operation") != Some(&json!(operation))
    {
        return Err(BackendError::Invalid("CompatForge reply identity mismatch"));
    }
    reply
        .get("result")
        .cloned()
        .ok_or(BackendError::Invalid("CompatForge reply lacks result"))
}

pub fn compatforge_poll_status<'a>(
    result: &'a Value,
    job_id: &str,
    app_id: &str,
) -> Result<&'a str, BackendError> {
    let job = result
        .get("job")
        .ok_or(BackendError::Invalid("missing CompatForge poll job"))?;
    if job.get("schemaVersion") != Some(&json!("1"))
        || job.get("id") != Some(&json!(job_id))
        || job.get("applicationId") != Some(&json!(app_id))
    {
        return Err(BackendError::Invalid(
            "CompatForge poll job identity mismatch",
        ));
    }
    let status = job
        .get("status")
        .and_then(Value::as_str)
        .ok_or(BackendError::Invalid("missing CompatForge poll status"))?;
    if !matches!(
        status,
        "preparing" | "running" | "cancelling" | "succeeded" | "failed" | "cancelled"
    ) {
        return Err(BackendError::Invalid("unknown CompatForge poll status"));
    }
    Ok(status)
}

pub fn compatforge_selected_install(summary: &Value, app_id: &str) -> Option<(String, bool)> {
    if summary.get("installed") != Some(&json!(true))
        || summary.get("application")?.get("id")?.as_str()? != app_id
    {
        return None;
    }
    let state = summary.get("generations")?;
    if state.get("applicationId")?.as_str()? != app_id {
        return None;
    }
    let selected = state.get("selectedGeneration")?.as_str()?;
    let generations = state.get("generations")?.as_array()?;
    let current = generations.iter().find(|generation| {
        generation.get("id").and_then(Value::as_str) == Some(selected)
            && generation.get("status").and_then(Value::as_str) == Some("ready")
    })?;
    if current.get("definition")?.get("id")?.as_str()? != app_id {
        return None;
    }
    let version = current
        .get("definition")?
        .get("version")?
        .as_str()?
        .to_string();
    let can_rollback = generations.iter().any(|generation| {
        generation.get("id").and_then(Value::as_str) != Some(selected)
            && generation.get("status").and_then(Value::as_str) == Some("ready")
            && generation
                .get("definition")
                .and_then(|value| value.get("id"))
                .and_then(Value::as_str)
                == Some(app_id)
    });
    Some((version, can_rollback))
}

pub struct CompatForgeClient {
    executable: PathBuf,
    request_dir: PathBuf,
}

impl CompatForgeClient {
    pub fn system(request_dir: &Path) -> Self {
        Self {
            executable: PathBuf::from("/usr/bin/compatforge-cli"),
            request_dir: request_dir.to_path_buf(),
        }
    }

    #[cfg(test)]
    pub fn for_test(executable: &Path, request_dir: &Path) -> Self {
        Self {
            executable: executable.to_path_buf(),
            request_dir: request_dir.to_path_buf(),
        }
    }

    pub fn available(&self) -> bool {
        self.executable.is_file()
    }

    pub async fn call(&self, request: Value) -> Result<Value, BackendError> {
        use tokio::io::AsyncWriteExt;
        let operation = request
            .get("operation")
            .and_then(Value::as_str)
            .ok_or(BackendError::Invalid("missing operation"))?
            .to_string();
        let request_id = request
            .get("requestId")
            .and_then(Value::as_str)
            .ok_or(BackendError::Invalid("missing request ID"))?
            .to_string();
        let encoded = serde_json::to_vec(&request)?;
        if encoded.len() > 1024 * 1024 {
            return Err(BackendError::Invalid("CompatForge request exceeds limit"));
        }
        tokio::fs::create_dir_all(&self.request_dir).await?;
        let path = self
            .request_dir
            .join(format!("request-{}.json", uuid::Uuid::new_v4()));
        let outcome = async {
            let mut file = tokio::fs::OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(&path)
                .await?;
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                file.set_permissions(std::fs::Permissions::from_mode(0o600))
                    .await?;
            }
            file.write_all(&encoded).await?;
            file.sync_all().await?;
            drop(file);
            let reply = tokio::time::timeout(
                std::time::Duration::from_secs(30),
                tokio::process::Command::new(&self.executable)
                    .arg("service-call")
                    .arg(&path)
                    .kill_on_drop(true)
                    .output(),
            )
            .await
            .map_err(|_| BackendError::Process("CompatForge request timed out".into()))??;
            if !reply.status.success() {
                return Err(BackendError::Process(
                    String::from_utf8_lossy(&reply.stderr[..reply.stderr.len().min(4096)])
                        .to_string(),
                ));
            }
            decode_compatforge_reply(&reply.stdout, &request_id, &operation)
        }
        .await;
        let _ = tokio::fs::remove_file(path).await;
        outcome
    }

    pub async fn operation(&self, operation: &str, payload: Value) -> Result<Value, BackendError> {
        if !matches!(
            operation,
            "applications.list"
                | "applications.get"
                | "applications.generations"
                | "applications.rollback"
                | "applications.uninstall"
                | "jobs.get"
                | "jobs.poll"
                | "jobs.cancel"
        ) {
            return Err(BackendError::Invalid("operation is not in store allowlist"));
        }
        self.call(
            json!({"schemaVersion":"1","requestId":format!("store-{}",uuid::Uuid::new_v4()),
            "operation":operation,"payload":payload}),
        )
        .await
    }
}

pub struct CompatForgeRequest;

impl CompatForgeRequest {
    pub fn install(
        app_id: &str,
        version: &str,
        artifact: &Artifact,
        path: &str,
        reviewed_definition: &Value,
    ) -> Result<Value, BackendError> {
        artifact.validate(crate::catalogue::MAX_ARTIFACT_BYTES)?;
        let app = reviewed_definition
            .get("application")
            .ok_or(BackendError::Invalid("missing application"))?;
        let installer = app
            .get("installer")
            .ok_or(BackendError::Invalid("missing reviewed installer"))?;
        let file_name = artifact
            .target
            .rsplit('/')
            .next()
            .ok_or(BackendError::Invalid("invalid target"))?;
        if app.get("id").and_then(Value::as_str) != Some(app_id)
            || app.get("version").and_then(Value::as_str) != Some(version)
            || installer.get("fileName").and_then(Value::as_str) != Some(file_name)
            || installer.get("sha256").and_then(Value::as_str) != Some(artifact.sha256.as_str())
            || !path.starts_with('/')
            || path.len() > 4096
            || path.contains("..")
            || path.contains('\0')
        {
            return Err(BackendError::Invalid(
                "catalogue differs from reviewed recipe or cache path",
            ));
        }
        Ok(
            json!({"schemaVersion":"1","requestId":format!("store-{}",uuid::Uuid::new_v4()),
            "operation":"jobs.submit","payload":{"schemaVersion":"1","applicationId":app_id,
            "kind":"install","executablePath":path}}),
        )
    }
}

#[derive(Debug, Clone)]
pub struct FlatpakInvocation {
    pub args: Vec<String>,
}

impl FlatpakInvocation {
    pub fn new(remote: &str, reference: &str, action: Action) -> Result<Self, BackendError> {
        let valid_remote = remote == "forge-store-fixture";
        let valid_ref = reference.starts_with("app/")
            && reference.split('/').count() == 4
            && reference.len() <= 256
            && !reference.contains("..")
            && reference
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"-._/".contains(&b));
        if !valid_remote || !valid_ref {
            return Err(BackendError::Invalid(
                "unreviewed Flatpak remote or reference",
            ));
        }
        let args: Vec<&str> = match action {
            Action::Install => vec![
                "--user",
                "install",
                "--noninteractive",
                "--assumeyes",
                remote,
                reference,
            ],
            Action::Update => vec![
                "--user",
                "update",
                "--noninteractive",
                "--assumeyes",
                reference,
            ],
            Action::Uninstall => vec![
                "--user",
                "uninstall",
                "--noninteractive",
                "--assumeyes",
                reference,
            ],
            Action::Rollback => {
                return Err(BackendError::Invalid("Flatpak rollback is unavailable"))
            }
        };
        Ok(Self {
            args: args.into_iter().map(str::to_string).collect(),
        })
    }
}

pub fn flatpak_record_matches(record: &Value, reference: &str, remote: &str) -> bool {
    let parts = reference.split('/').collect::<Vec<_>>();
    parts.len() == 4
        && parts[0] == "app"
        && record.get("application_id").and_then(Value::as_str) == Some(parts[1])
        && record.get("arch").and_then(Value::as_str) == Some(parts[2])
        && record.get("branch").and_then(Value::as_str) == Some(parts[3])
        && record.get("origin").and_then(Value::as_str) == Some(remote)
}

pub fn flatpak_action_completed(action: Action, before: Option<&str>, after: Option<&str>) -> bool {
    match action {
        Action::Install => before.is_none() && after.is_some(),
        Action::Update => before.is_some() && after.is_some() && before != after,
        Action::Uninstall => before.is_some() && after.is_none(),
        Action::Rollback => false,
    }
}

#[derive(Debug, Clone)]
pub struct PackageRequest {
    fields: BTreeMap<String, Value>,
}

pub struct PackageClient {
    #[cfg_attr(not(unix), allow(dead_code))]
    socket: PathBuf,
}

impl Default for PackageClient {
    fn default() -> Self {
        Self::new()
    }
}

impl PackageClient {
    pub fn new() -> Self {
        Self {
            socket: PathBuf::from("/run/forge-package/forge-package.sock"),
        }
    }

    #[cfg(all(test, unix))]
    fn for_test(path: &std::path::Path) -> Self {
        Self {
            socket: path.to_path_buf(),
        }
    }

    #[cfg(unix)]
    pub async fn call(&self, request: PackageRequest) -> Result<Value, BackendError> {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        let limit = if request.fields.get("operation") == Some(&json!("list")) {
            512 * 1024
        } else {
            4096
        };
        let id = request
            .fields
            .get("requestId")
            .and_then(Value::as_str)
            .ok_or(BackendError::Invalid("missing request ID"))?
            .to_string();
        let line = request.to_canonical_line()?;
        let exchange = async {
            let mut socket = tokio::net::UnixStream::connect(&self.socket).await?;
            socket.write_all(&line).await?;
            socket.shutdown().await?;
            let mut response = Vec::new();
            socket
                .take((limit + 1) as u64)
                .read_to_end(&mut response)
                .await?;
            if response.len() > limit
                || response.last() != Some(&b'\n')
                || response[..response.len() - 1].contains(&b'\n')
            {
                return Err(BackendError::Invalid(
                    "package response exceeds bounds or is not one line",
                ));
            }
            let decoded: Value = serde_json::from_slice(&response[..response.len() - 1])?;
            if decoded.get("schemaVersion") != Some(&json!(1))
                || decoded.get("requestId") != Some(&json!(id))
            {
                return Err(BackendError::Invalid("package response identity mismatch"));
            }
            match decoded.get("ok").and_then(Value::as_bool) {
                Some(true) if decoded.get("error").is_none() => decoded
                    .get("result")
                    .cloned()
                    .ok_or(BackendError::Invalid("missing package result")),
                Some(false) if decoded.get("result").is_none() => {
                    let error = decoded
                        .get("error")
                        .ok_or(BackendError::Invalid("missing package error"))?;
                    let code = error
                        .get("code")
                        .and_then(Value::as_str)
                        .unwrap_or("internal");
                    let message = error
                        .get("message")
                        .and_then(Value::as_str)
                        .unwrap_or("service rejected operation");
                    Err(BackendError::Service {
                        code: code.into(),
                        message: message.into(),
                    })
                }
                _ => Err(BackendError::Invalid("invalid package response status")),
            }
        };
        tokio::time::timeout(std::time::Duration::from_secs(300), exchange)
            .await
            .map_err(|_| {
                BackendError::Io(std::io::Error::new(
                    std::io::ErrorKind::TimedOut,
                    "package service timed out",
                ))
            })?
    }

    #[cfg(not(unix))]
    pub async fn call(&self, _request: PackageRequest) -> Result<Value, BackendError> {
        Err(BackendError::Invalid(
            "system package service requires Unix",
        ))
    }
}

impl PackageRequest {
    fn new(request_id: &str, operation: &str) -> Result<Self, BackendError> {
        if !safe_token(request_id, 64) {
            return Err(BackendError::Invalid("invalid request ID"));
        }
        let mut fields = BTreeMap::new();
        fields.insert("schemaVersion".into(), json!(1));
        fields.insert("requestId".into(), json!(request_id));
        fields.insert("operation".into(), json!(operation));
        Ok(Self { fields })
    }

    pub fn install(
        request_id: &str,
        path: &str,
        artifact: &Artifact,
    ) -> Result<Self, BackendError> {
        artifact.validate(MAX_FORGEPKG_BYTES)?;
        let expected = format!("/staging/{}.forgepkg", artifact.sha256);
        if !path.starts_with('/')
            || !path.ends_with(&expected)
            || path.len() > 2048
            || path.contains("..")
        {
            return Err(BackendError::Invalid(
                "package path is outside the digest staging form",
            ));
        }
        let mut request = Self::new(request_id, "install")?;
        request.fields.insert("path".into(), json!(path));
        request
            .fields
            .insert("sha256".into(), json!(artifact.sha256));
        request.fields.insert("size".into(), json!(artifact.size));
        Ok(request)
    }

    pub fn status(request_id: &str, id: &str) -> Result<Self, BackendError> {
        let mut request = Self::new(request_id, "status")?;
        request.set_id(id)?;
        Ok(request)
    }

    pub fn list(request_id: &str) -> Result<Self, BackendError> {
        Self::new(request_id, "list")
    }

    pub fn activate(request_id: &str, id: &str, digest: &str) -> Result<Self, BackendError> {
        let mut request = Self::new(request_id, "activate")?;
        request.set_id(id)?;
        if !digest.bytes().all(|b| b.is_ascii_hexdigit())
            || digest.len() != 64
            || digest != digest.to_ascii_lowercase()
        {
            return Err(BackendError::Invalid("invalid package digest"));
        }
        request.fields.insert("digest".into(), json!(digest));
        Ok(request)
    }

    pub fn rollback(request_id: &str, id: &str) -> Result<Self, BackendError> {
        let mut request = Self::new(request_id, "rollback")?;
        request.set_id(id)?;
        Ok(request)
    }

    fn set_id(&mut self, id: &str) -> Result<(), BackendError> {
        if id.len() > 255
            || id.split('.').count() < 2
            || id.split('.').any(|part| {
                part.is_empty()
                    || !part.starts_with(|c: char| c.is_ascii_lowercase())
                    || !part
                        .bytes()
                        .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
            })
        {
            return Err(BackendError::Invalid("invalid package ID"));
        }
        self.fields.insert("id".into(), json!(id));
        Ok(())
    }

    pub fn to_canonical_line(&self) -> Result<Vec<u8>, BackendError> {
        let value = Value::Object(Map::from_iter(
            self.fields
                .iter()
                .map(|(key, value)| (key.clone(), value.clone())),
        ));
        let mut output = Vec::new();
        canonical_value(&value, &mut output)?;
        output.push(b'\n');
        if output.len() > 4096 {
            return Err(BackendError::Invalid("package request exceeds 4096 bytes"));
        }
        Ok(output)
    }
}

fn safe_token(value: &str, max: usize) -> bool {
    !value.is_empty()
        && value.len() <= max
        && value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"_.-".contains(&b))
}

pub fn fixture_remote_is_trusted(output: &str) -> bool {
    output.lines().any(|line| {
        let mut columns = line.split_whitespace();
        columns.next() == Some("forge-store-fixture")
            && matches!(
                columns.next(),
                Some(
                    "file:///usr/share/forge-store/flatpak/repo-v1"
                        | "file:///usr/share/forge-store/flatpak/repo-v2"
                )
            )
            && columns.next().is_none()
    })
}

fn canonical_value(value: &Value, output: &mut Vec<u8>) -> Result<(), BackendError> {
    match value {
        Value::Null => output.extend_from_slice(b"null"),
        Value::Bool(value) => output.extend_from_slice(if *value { b"true" } else { b"false" }),
        Value::Number(number) => output.extend_from_slice(number.to_string().as_bytes()),
        Value::String(string) => canonical_string(string, output),
        Value::Array(items) => {
            output.push(b'[');
            for (index, item) in items.iter().enumerate() {
                if index > 0 {
                    output.push(b',');
                }
                canonical_value(item, output)?;
            }
            output.push(b']');
        }
        Value::Object(map) => {
            output.push(b'{');
            let mut keys = map.keys().collect::<Vec<_>>();
            keys.sort();
            for (index, key) in keys.into_iter().enumerate() {
                if index > 0 {
                    output.push(b',');
                }
                canonical_string(key, output);
                output.push(b':');
                canonical_value(&map[key], output)?;
            }
            output.push(b'}');
        }
    }
    Ok(())
}

fn canonical_string(value: &str, output: &mut Vec<u8>) {
    output.push(b'"');
    for ch in value.chars() {
        match ch {
            '"' => output.extend_from_slice(br#"\""#),
            '\\' => output.extend_from_slice(br"\\"),
            '\u{0008}' => output.extend_from_slice(br"\b"),
            '\u{000C}' => output.extend_from_slice(br"\f"),
            '\n' => output.extend_from_slice(br"\n"),
            '\r' => output.extend_from_slice(br"\r"),
            '\t' => output.extend_from_slice(br"\t"),
            ch if ch.is_ascii() && !ch.is_control() => output.push(ch as u8),
            ch => {
                for unit in ch.encode_utf16(&mut [0; 2]).iter() {
                    output.extend_from_slice(format!("\\u{unit:04x}").as_bytes());
                }
            }
        }
    }
    output.push(b'"');
}

#[cfg(all(test, unix))]
mod transport_tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    #[tokio::test]
    async fn package_client_checks_request_identity_and_service_result() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("package.sock");
        let listener = tokio::net::UnixListener::bind(&path).unwrap();
        let peer = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut input = Vec::new();
            socket.read_to_end(&mut input).await.unwrap();
            assert_eq!(
                serde_json::from_slice::<Value>(&input).unwrap()["operation"],
                "list"
            );
            socket.write_all(b"{\"schemaVersion\":1,\"requestId\":\"test-1\",\"ok\":true,\"result\":{\"applications\":[]}}\n").await.unwrap();
        });
        let client = PackageClient::for_test(&path);
        let result = client
            .call(PackageRequest::list("test-1").unwrap())
            .await
            .unwrap();
        assert_eq!(result["applications"], json!([]));
        peer.await.unwrap();
    }

    #[tokio::test]
    async fn package_list_accepts_multiple_bounded_records() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("package.sock");
        let listener = tokio::net::UnixListener::bind(&path).unwrap();
        let peer = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut input = Vec::new();
            socket.read_to_end(&mut input).await.unwrap();
            let applications = (0..100)
                .map(|index| {
                    json!({"appId":format!("org.forgeos.app{index}"),
                "active":"a".repeat(64),"previous":null})
                })
                .collect::<Vec<_>>();
            let mut reply = serde_json::to_vec(&json!({"schemaVersion":1,"requestId":"large-list",
                "ok":true,"result":{"applications":applications}}))
            .unwrap();
            assert!(reply.len() > 4096);
            reply.push(b'\n');
            socket.write_all(&reply).await.unwrap();
        });
        let result = PackageClient::for_test(&path)
            .call(PackageRequest::list("large-list").unwrap())
            .await
            .unwrap();
        assert_eq!(result["applications"].as_array().unwrap().len(), 100);
        peer.await.unwrap();
    }
}
