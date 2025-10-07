use zed_extension_api::{Result, SlashCommandOutput, SlashCommandOutputSection};

pub fn build_output(
    messages: Vec<String>,
    header_json: String,
    payload_json: String,
    sig_str: String,
    verification: Option<bool>,
) -> Result<SlashCommandOutput, String> {
    let mut text = String::new();
    let mut sections = Vec::new();
    let mut offset: usize = 0;

    if !messages.is_empty() {
        text.push_str("## Claims Analysis\n");
        for msg in &messages {
            if msg.starts_with("Warning:") {
                text.push_str(&format!("**{}**\n", msg));
            } else {
                text.push_str(&format!("- {}\n", msg));
            }
        }
        text.push('\n');
        let len = text.len();
        sections.push(SlashCommandOutputSection {
            label: "Claims Analysis".to_string(),
            range: (0..len).into(),
        });
        offset = len;
    }

    text.push_str("## Header\n```json\n");
    text.push_str(&header_json);
    text.push_str("\n```\n\n");
    let header_end = text.len();
    sections.push(SlashCommandOutputSection {
        label: "Header".to_string(),
        range: (offset..header_end).into(),
    });
    offset = header_end;

    text.push_str("## Payload\n```json\n");
    text.push_str(&payload_json);
    text.push_str("\n```\n\n");
    let payload_end = text.len();
    sections.push(SlashCommandOutputSection {
        label: "Payload".to_string(),
        range: (offset..payload_end).into(),
    });
    offset = payload_end;

    text.push_str("## Signature\n```\n");
    text.push_str(&sig_str);
    text.push_str("\n```\n\n");
    let sig_end = text.len();
    sections.push(SlashCommandOutputSection {
        label: "Signature (base64url)".to_string(),
        range: (offset..sig_end).into(),
    });
    offset = sig_end;

    if let Some(is_valid) = verification {
        text.push_str("## Verification\n");
        if is_valid {
            text.push_str("Signature is valid.\n");
        } else {
            text.push_str("**Warning:** Signature is invalid.\n");
        }
        let ver_end = text.len();
        sections.push(SlashCommandOutputSection {
            label: "Verification".to_string(),
            range: (offset..ver_end).into(),
        });
    }

    Ok(SlashCommandOutput { text, sections })
}
