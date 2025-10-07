use zed_extension_api::{
    self as zed, Result, SlashCommand, SlashCommandArgumentCompletion, SlashCommandOutput,
};

pub struct JwtExtension;

impl zed::Extension for JwtExtension {
    fn new() -> Self {
        JwtExtension
    }

    fn run_slash_command(
        &self,
        command: SlashCommand,
        args: Vec<String>,
        _worktree: Option<&zed::Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        if command.name != "jwt" {
            return Err("unknown command".to_string());
        }

        if args.is_empty() {
            return Err("usage: /jwt <subcommand> <token> [key for verify]".to_string());
        }

        let subcommand = &args[0];
        let token = args.get(1).ok_or("missing token".to_string())?.clone();

        match subcommand.as_str() {
            "decode" => {
                let (_parts, header, payload, sig_str, _sig_bytes, _signing_input, current_time) =
                    crate::parser::parse_token(&token)?;
                let messages = crate::claims::analyze_claims(&payload, current_time);
                let header_json = serde_json::to_string_pretty(&header).unwrap();
                let payload_json = serde_json::to_string_pretty(&payload).unwrap();
                crate::builder::build_output(messages, header_json, payload_json, sig_str, None)
            }
            "verify" => {
                let key = args
                    .get(2)
                    .ok_or("missing key for verify".to_string())?
                    .clone();
                let (_parts, header, payload, sig_str, sig_bytes, signing_input, current_time) =
                    crate::parser::parse_token(&token)?;
                let messages = crate::claims::analyze_claims(&payload, current_time);
                let is_valid = crate::verifier::verify(&header, &signing_input, &sig_bytes, &key)?;
                let header_json = serde_json::to_string_pretty(&header).unwrap();
                let payload_json = serde_json::to_string_pretty(&payload).unwrap();
                crate::builder::build_output(
                    messages,
                    header_json,
                    payload_json,
                    sig_str,
                    Some(is_valid),
                )
            }
            _ => Err(format!("unknown subcommand: {}", subcommand)),
        }
    }

    fn complete_slash_command_argument(
        &self,
        command: SlashCommand,
        args: Vec<String>,
    ) -> Result<Vec<SlashCommandArgumentCompletion>, String> {
        if command.name != "jwt" {
            return Err("unknown command".to_string());
        }

        match args.len() {
            0 => Ok(vec![
                SlashCommandArgumentCompletion {
                    label: "decode".to_string(),
                    new_text: "decode ".to_string(),
                    run_command: false,
                },
                SlashCommandArgumentCompletion {
                    label: "verify".to_string(),
                    new_text: "verify ".to_string(),
                    run_command: false,
                },
            ]),
            _ => Ok(vec![]),
        }
    }
}
