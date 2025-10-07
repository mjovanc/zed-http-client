use zed_extension_api::{self as zed, Result};

pub struct ZedDbclient {
    // Placeholder for future state, e.g., DB connections
}

impl zed::Extension for ZedDbclient {
    fn new() -> Self {
        Self {}
    }
}

// Simulate a query result as a Markdown table
fn format_query_results() -> String {
    // Hardcoded example: replace with `postgres` crate query later
    let headers = vec!["id", "name", "email"];
    let rows = vec![
        vec!["1", "Alice", "alice@example.com"],
        vec!["2", "Bob", "bob@example.com"],
    ];

    let mut table = String::from("| ");
    table.push_str(&headers.join(" | "));
    table.push_str(" |\n| ");
    table.push_str(
        &headers
            .iter()
            .map(|_| "---")
            .collect::<Vec<_>>()
            .join(" | "),
    );
    table.push_str(" |\n");
    for row in rows {
        table.push_str("| ");
        table.push_str(&row.join(" | "));
        table.push_str(" |\n");
    }
    table
}

pub extern "C" fn zed_extension_command_run_query() -> zed::Result<()> {
    let content = format_query_results();
    zed::current_extension_worktree()?
        .create_entry("query_results.md", zed::CreateEntry::File)?
        .set_text(&content)?;
    Ok(())
}

zed::register_extension!(ZedDbclient);
