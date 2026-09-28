use crate::queue::Action;
use serde_json::{json, Value};
use thiserror::Error;

pub const MAX_LINE_BYTES: usize = 4096;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("invalid store request")]
    Invalid,
    #[error("invalid JSON: {0}")]
    Json(#[from] serde_json::Error),
}

#[derive(Debug)]
pub enum StoreRequest {
    Snapshot {
        request_id: String,
    },
    Enqueue {
        request_id: String,
        app_id: String,
        action: Action,
    },
    Cancel {
        request_id: String,
        job_id: String,
    },
    Retry {
        request_id: String,
        job_id: String,
    },
}

impl StoreRequest {
    pub fn request_id(&self) -> &str {
        match self {
            Self::Snapshot { request_id }
            | Self::Enqueue { request_id, .. }
            | Self::Cancel { request_id, .. }
            | Self::Retry { request_id, .. } => request_id,
        }
    }
}

pub fn parse_request(bytes: &[u8]) -> Result<StoreRequest, ApiError> {
    if bytes.len() > MAX_LINE_BYTES
        || bytes.last() != Some(&b'\n')
        || bytes[..bytes.len() - 1].contains(&b'\n')
    {
        return Err(ApiError::Invalid);
    }
    let value: Value = serde_json::from_slice(&bytes[..bytes.len() - 1])?;
    let fields = value.as_object().ok_or(ApiError::Invalid)?;
    if fields.get("schemaVersion") != Some(&json!(1)) {
        return Err(ApiError::Invalid);
    }
    let request_id = field(fields, "requestId", 64)?.to_string();
    if !safe_token(&request_id) {
        return Err(ApiError::Invalid);
    }
    let operation = field(fields, "operation", 32)?;
    if !keys(
        fields,
        &["schemaVersion", "requestId", "operation", "payload"],
    ) {
        return Err(ApiError::Invalid);
    }
    let payload = fields
        .get("payload")
        .and_then(Value::as_object)
        .ok_or(ApiError::Invalid)?;
    match operation {
        "snapshot" if payload.is_empty() => Ok(StoreRequest::Snapshot { request_id }),
        "enqueue" if keys(payload, &["appId", "action"]) => {
            let app_id = field(payload, "appId", 64)?.to_string();
            if !valid_app_id(&app_id) {
                return Err(ApiError::Invalid);
            }
            let action = match field(payload, "action", 16)? {
                "install" => Action::Install,
                "update" => Action::Update,
                "uninstall" => Action::Uninstall,
                "rollback" => Action::Rollback,
                _ => return Err(ApiError::Invalid),
            };
            Ok(StoreRequest::Enqueue {
                request_id,
                app_id,
                action,
            })
        }
        "cancel" if keys(payload, &["jobId"]) => Ok(StoreRequest::Cancel {
            request_id,
            job_id: checked_job_id(payload)?,
        }),
        "retry" if keys(payload, &["jobId"]) => Ok(StoreRequest::Retry {
            request_id,
            job_id: checked_job_id(payload)?,
        }),
        _ => Err(ApiError::Invalid),
    }
}

fn field<'a>(
    fields: &'a serde_json::Map<String, Value>,
    name: &str,
    max: usize,
) -> Result<&'a str, ApiError> {
    let value = fields
        .get(name)
        .and_then(Value::as_str)
        .ok_or(ApiError::Invalid)?;
    if value.is_empty() || value.len() > max || value.chars().any(char::is_control) {
        return Err(ApiError::Invalid);
    }
    Ok(value)
}
fn keys(fields: &serde_json::Map<String, Value>, names: &[&str]) -> bool {
    fields.len() == names.len() && names.iter().all(|name| fields.contains_key(*name))
}
fn safe_token(value: &str) -> bool {
    value
        .bytes()
        .all(|b| b.is_ascii_alphanumeric() || b"_.-".contains(&b))
}
fn valid_app_id(value: &str) -> bool {
    value
        .bytes()
        .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b".-".contains(&b))
}
fn checked_job_id(fields: &serde_json::Map<String, Value>) -> Result<String, ApiError> {
    let value = field(fields, "jobId", 64)?;
    if !safe_token(value) {
        return Err(ApiError::Invalid);
    }
    Ok(value.to_string())
}

pub fn response(request_id: &str, result: Result<Value, (&str, &str)>) -> Value {
    match result {
        Ok(result) => json!({"schemaVersion":1,"requestId":request_id,"ok":true,"result":result}),
        Err((code, message)) => json!({"schemaVersion":1,"requestId":request_id,"ok":false,
            "error":{"code":code,"message":message}}),
    }
}
