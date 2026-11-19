// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Jareer Abdullah and Concat contributors

//! The streamable-HTTP MCP endpoint: `POST /mcp` with a bearer token.
//!
//! usage: concat-mcp-http [auto|window|headless]   (or CONCAT_MCP_MODE)
//! address: KAPCUT_MCP_HTTP_ADDRESS (default 127.0.0.1:7421)
//! auth:    KAPCUT_AGENT_TOKEN, KAPCUT_PROJECT_ROOT, KAPCUT_TOOL_FAMILIES

use std::process::ExitCode;

fn main() -> ExitCode {
    let word = std::env::args()
        .nth(1)
        .or_else(|| std::env::var("CONCAT_MCP_MODE").ok())
        .unwrap_or_default();
    let Some(mode) = concat_mcp::Mode::parse(&word) else {
        eprintln!("usage: concat-mcp-http [auto|window|headless]");
        return ExitCode::from(2);
    };
    let address = std::env::var("KAPCUT_MCP_HTTP_ADDRESS")
        .ok()
        .filter(|a| !a.is_empty())
        .unwrap_or_else(|| concat_mcp::http::DEFAULT_HTTP_ADDRESS.to_owned());
    match concat_mcp::run_http(&address, mode) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("concat mcp http: {error}");
            ExitCode::FAILURE
        }
    }
}
