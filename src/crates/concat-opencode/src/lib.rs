// SPDX-License-Identifier: AGPL-3.0-or-later
//! KapCut Studio — OpenCode full-UI side panel.
//!
//! Opens the OpenCode web UI in a native WebView window that can be docked
//! beside the Concat timeline. The panel hosts the *full* OpenCode surface
//! (sessions, model picker, diffs, tools, permissions, MCP list), not a chat
//! input. The editor and the panel share the project through the KapCut MCP
//! server in window mode.

use std::path::Path;
use wry::application::event_loop::{ControlFlow, EventLoop};
use wry::application::window::WindowBuilder;
use wry::webview::WebViewBuilder;

/// Default local OpenCode web UI address (the desktop service).
pub const DEFAULT_OPENCODE_URL: &str = "http://127.0.0.1:4096";

/// Open the full OpenCode UI in a WebView panel.
///
/// `url` is the OpenCode web UI to load; `project` is the folder the agent
/// should act on (exposed to the page through the bridge initialization).
pub fn open_panel(url: &str, project: Option<&Path>) -> Result<(), String> {
    let event_loop = EventLoop::new().map_err(|e| e.to_string())?;
    let window = WindowBuilder::new()
        .with_title("OpenCode — KapCut Studio")
        .with_inner_size(wry::application::dpi::LogicalSize::new(520, 900))
        .build(&event_loop)
        .map_err(|e| e.to_string())?;

    let init = match project {
        Some(p) => {
            let json = serde_json::to_string(p.to_string_lossy().as_ref()).unwrap_or_default();
            format!("window.kapcutStudio = {{ project: {} }};", json)
        }
        None => "window.kapcutStudio = {};".to_string(),
    };

    let _webview = WebViewBuilder::new(window)
        .map_err(|e| e.to_string())?
        .with_url(url)
        .map_err(|e| e.to_string())?
        .with_initialization_script(&init)
        .build()
        .map_err(|e| e.to_string())?;

    event_loop.run(move |_event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn default_url_is_loopback() {
        assert!(DEFAULT_OPENCODE_URL.starts_with("http://127.0.0.1"));
    }
}
