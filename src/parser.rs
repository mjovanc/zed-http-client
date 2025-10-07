use base64::engine::{Engine, general_purpose::URL_SAFE_NO_PAD};
use serde_json::Value;
use std::time::{SystemTime, UNIX_EPOCH};
use zed_extension_api::Result;

pub fn parse_token(
    token: &str,
) -> Result<(Vec<String>, Value, Value, String, Vec<u8>, String, i64), String> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return Err(
            "invalid JWT format: must have exactly three parts separated by '.'".to_string(),
        );
    }

    let header_bytes = URL_SAFE_NO_PAD
        .decode(parts[0])
        .map_err(|e| e.to_string())?;
    let header_str = String::from_utf8(header_bytes).map_err(|e| e.to_string())?;
    let header: Value = serde_json::from_str(&header_str).map_err(|e| e.to_string())?;

    let payload_bytes = URL_SAFE_NO_PAD
        .decode(parts[1])
        .map_err(|e| e.to_string())?;
    let payload_str = String::from_utf8(payload_bytes).map_err(|e| e.to_string())?;
    let payload: Value = serde_json::from_str(&payload_str).map_err(|e| e.to_string())?;

    let sig_str = parts[2].to_string();
    let sig_bytes = URL_SAFE_NO_PAD
        .decode(&sig_str)
        .map_err(|e| e.to_string())?;

    let signing_input = format!("{}.{}", parts[0], parts[1]);

    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_secs() as i64;

    Ok((
        parts.into_iter().map(|s| s.to_string()).collect(),
        header,
        payload,
        sig_str,
        sig_bytes,
        signing_input,
        current_time,
    ))
}
