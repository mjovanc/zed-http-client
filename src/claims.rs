use chrono::{TimeZone, Utc};
use serde_json::Value;

pub fn analyze_claims(payload: &Value, current_time: i64) -> Vec<String> {
    let mut messages = Vec::new();

    // Standard claims
    if let Some(iss) = payload.get("iss").and_then(|v| v.as_str()) {
        messages.push(format!("Issuer (iss): {}", iss));
    }
    if let Some(sub) = payload.get("sub").and_then(|v| v.as_str()) {
        messages.push(format!("Subject (sub): {}", sub));
    }
    if let Some(aud) = payload.get("aud") {
        let aud_str = match aud {
            Value::String(s) => s.clone(),
            Value::Array(arr) => arr
                .iter()
                .filter_map(|v| v.as_str())
                .collect::<Vec<_>>()
                .join(", "),
            _ => String::new(),
        };
        if !aud_str.is_empty() {
            messages.push(format!("Audience (aud): {}", aud_str));
        }
    }
    if let Some(jti) = payload.get("jti").and_then(|v| v.as_str()) {
        messages.push(format!("JWT ID (jti): {}", jti));
    }

    // Time-based claims
    let iat = payload.get("iat").and_then(|v| v.as_i64());
    let nbf = payload.get("nbf").and_then(|v| v.as_i64());
    let exp = payload.get("exp").and_then(|v| v.as_i64());

    if let Some(iat_val) = iat {
        if let Some(dt) = Utc.timestamp_opt(iat_val, 0).single() {
            messages.push(format!(
                "Issued At (iat): {} ({})",
                dt.to_rfc3339(),
                relative_time(current_time, iat_val)
            ));
        }
    }

    if let Some(nbf_val) = nbf {
        if let Some(dt) = Utc.timestamp_opt(nbf_val, 0).single() {
            messages.push(format!(
                "Not Before (nbf): {} ({})",
                dt.to_rfc3339(),
                relative_time(current_time, nbf_val)
            ));
        }
        if nbf_val > current_time {
            messages.push("Warning: Token not yet valid (nbf in future)".to_string());
        }
    }

    if let Some(exp_val) = exp {
        if let Some(dt) = Utc.timestamp_opt(exp_val, 0).single() {
            messages.push(format!(
                "Expires (exp): {} ({})",
                dt.to_rfc3339(),
                relative_time(current_time, exp_val)
            ));
        }
        if exp_val < current_time {
            messages.push("Warning: Token expired".to_string());
        }
    }

    messages
}

fn relative_time(now: i64, ts: i64) -> String {
    let diff = now - ts;
    let abs_diff = diff.abs();
    let (count, unit) = if abs_diff >= 86_400 {
        (abs_diff / 86_400, "day")
    } else if abs_diff >= 3_600 {
        (abs_diff / 3_600, "hour")
    } else if abs_diff >= 60 {
        (abs_diff / 60, "minute")
    } else {
        (abs_diff, "second")
    };
    let plural = if count != 1 { "s" } else { "" };
    if diff > 0 {
        format!("{} {}{} ago", count, unit, plural)
    } else {
        format!("in {} {}{}", count, unit, plural)
    }
}
