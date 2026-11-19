// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Jareer Abdullah and Concat contributors

//! KapCut Studio — the OpenCode full-UI hand-off.
//!
//! The studio button opens OpenCode's full web UI (sessions, model picker,
//! diffs, tools, permissions, MCP list) in the system browser. An earlier
//! version tried to host it in an in-process WebView on a worker thread;
//! WebView2 requires a main-thread STA message loop and crashed the editor
//! with an access violation on Windows. The browser hosts the identical UI
//! reliably and shares the project through the KapCut MCP server.

use std::fs;
use std::path::{Path, PathBuf};

/// Fallback local OpenCode web UI address (the desktop service).
pub const DEFAULT_OPENCODE_URL: &str = "http://127.0.0.1:4096";

/// Where OpenCode desktop writes the URL of its local UI service. Each of
/// these is checked, newest first, for a `http://127.0.0.1:<port>` line.
fn discovery_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    if let Ok(appdata) = std::env::var("APPDATA") {
        dirs.push(
            PathBuf::from(appdata)
                .join("ai.opencode.desktop")
                .join("logs"),
        );
    }
    if let Ok(home) = std::env::var("USERPROFILE") {
        dirs.push(PathBuf::from(&home).join(".opencode"));
        dirs.push(PathBuf::from(&home).join(".config").join("opencode"));
    }
    dirs
}

/// Pull the first local service URL out of a log line.
fn find_local_url(text: &str) -> Option<String> {
    for host in ["http://127.0.0.1:", "http://localhost:"] {
        let mut rest = text;
        while let Some(i) = rest.find(host) {
            let after = &rest[i + host.len()..];
            let port: String = after.chars().take_while(|c| c.is_ascii_digit()).collect();
            if !port.is_empty() {
                return Some(format!("{host}{port}"));
            }
            if after.is_empty() {
                break;
            }
            rest = &after[1..];
        }
    }
    None
}

/// The OpenCode UI address: the explicit override first, then the newest
/// address the running desktop service logged, then the historical default.
pub fn discover_url() -> String {
    if let Ok(url) = std::env::var("KAPCUT_OPENCODE_URL") {
        if !url.is_empty() {
            return url;
        }
    }
    for dir in discovery_dirs() {
        let Ok(entries) = fs::read_dir(dir) else {
            continue;
        };
        let mut files: Vec<_> = entries.filter_map(Result::ok).collect();
        files.sort_by_key(|e| e.metadata().and_then(|m| m.modified()).ok());
        files.reverse();
        for entry in files {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            if let Ok(text) = fs::read_to_string(&path) {
                if let Some(url) = find_local_url(&text) {
                    return url;
                }
            }
        }
    }
    DEFAULT_OPENCODE_URL.to_string()
}

/// Open the full OpenCode UI. `project` is the folder the editor has open;
/// it is passed as a URL fragment so an OpenCode session started from here
/// can use it as its working directory.
pub fn open_panel(url: &str, project: Option<&Path>) -> Result<(), String> {
    let target = match project {
        Some(path) => format!("{url}/#/?folder={}", path.to_string_lossy()),
        None => url.to_string(),
    };
    // On Windows `cmd /c start "" <url>` opens the default browser without
    // waiting. No COM or UI thread is created in the editor process, so this
    // cannot crash it.
    let mut command = std::process::Command::new("cmd");
    command.args(["/C", "start", "", &target]);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x08000000); // CREATE_NO_WINDOW
    }
    command.spawn().map_err(|error| error.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn default_url_is_loopback() {
        assert!(DEFAULT_OPENCODE_URL.starts_with("http://127.0.0.1"));
    }
    #[test]
    fn finds_local_url_in_a_log_line() {
        let log = "info: starting service\n  url: 'http://127.0.0.1:28894',\n";
        assert_eq!(
            find_local_url(log).as_deref(),
            Some("http://127.0.0.1:28894")
        );
        assert_eq!(find_local_url("nothing here").as_deref(), None);
    }
}
