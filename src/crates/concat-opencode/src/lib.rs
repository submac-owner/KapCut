// SPDX-License-Identifier: AGPL-3.0-or-later
//! KapCut Studio — OpenCode full-UI side panel.
//!
//! Opens the OpenCode web UI in a native WebView window that can be docked
//! beside the Concat timeline. The panel hosts the *full* OpenCode surface
//! (sessions, model picker, diffs, tools, permissions, MCP list), not a chat
//! input. The editor and the panel share the project through the KapCut MCP
//! server in window mode.

use std::fs;
use std::path::{Path, PathBuf};
use tao::event_loop::{ControlFlow, EventLoop};
use tao::window::WindowBuilder;
use wry::WebViewBuilder;

/// Fallback local OpenCode web UI address (the desktop service).
pub const DEFAULT_OPENCODE_URL: &str = "http://127.0.0.1:4096";

/// Where OpenCode desktop writes the URL of its local UI service. Each of
/// these is checked, newest first, for a `http://127.0.0.1:<port>` line.
fn discovery_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    if let Ok(appdata) = std::env::var("APPDATA") {
        dirs.push(PathBuf::from(appdata).join("ai.opencode.desktop").join("logs"));
    }
    if let Ok(home) = std::env::var("USERPROFILE") {
        dirs.push(PathBuf::from(home).join(".opencode"));
        dirs.push(PathBuf::from(home).join(".config").join("opencode"));
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

/// Open the full OpenCode UI in a WebView panel and run its event loop.
pub fn open_panel(url: &str, project: Option<&Path>) -> Result<(), String> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("OpenCode — KapCut Studio")
        .with_inner_size(tao::dpi::LogicalSize::new(520.0, 900.0))
        .build(&event_loop)
        .map_err(|e| e.to_string())?;

    let init = match project {
        Some(p) => {
            let json = serde_json::to_string(p.to_string_lossy().as_ref()).unwrap_or_default();
            format!("window.kapcutStudio = {{ project: {} }};", json)
        }
        None => "window.kapcutStudio = {};".to_string(),
    };

    let _webview = WebViewBuilder::new()
        .with_url(url)
        .with_initialization_script(&init)
        .build(&window)
        .map_err(|e| e.to_string())?;

    event_loop.run(move |_event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
    });
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
        assert_eq!(find_local_url(log).as_deref(), Some("http://127.0.0.1:28894"));
        assert_eq!(find_local_url("nothing here").as_deref(), None);
    }
}
