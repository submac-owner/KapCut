// SPDX-License-Identifier: AGPL-3.0-or-later
//! KapCut Studio — OpenCode full-UI side panel.
//!
//! Opens the OpenCode web UI in a native WebView window that can be docked
//! beside the Concat timeline. The panel hosts the *full* OpenCode surface
//! (sessions, model picker, diffs, tools, permissions, MCP list), not a chat
//! input. The editor and the panel share the project through the KapCut MCP
//! server in window mode.

use std::path::Path;
use tao::event_loop::{ControlFlow, EventLoop};
use tao::window::WindowBuilder;
use wry::WebViewBuilder;

/// Default local OpenCode web UI address (the desktop service).
pub const DEFAULT_OPENCODE_URL: &str = "http://127.0.0.1:4096";

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
}
