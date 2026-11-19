# KapCut Studio — OpenCode full-UI side panel

## Requirement
The studio must host **OpenCode's full UI** in a dockable side panel — sessions,
agent/model picker, diffs, changes, tools, permissions, MCP list — not a bare
chat input. Local agent and remote agents drive the same project through MCP.

## Current state
`kapcut/chat-panel/` is only a minimal HTML prototype (input + message list).
It is **not** the full OpenCode UI and will be replaced.

## Target architecture
1. **Native side panel in Concat (Rust + Slint)**
   - New `concat-opencode` crate embeds a WebView (`wry` + `tao` on Windows,
     WebView2) docked right of the timeline.
   - Loads the OpenCode web UI from the local OpenCode service
     (`http://127.0.0.1:<port>`), discovered from the OpenCode desktop
     installation or started by the studio.
   - Panel is resizable, can be undocked into its own window, and shares the
     project path with the editor (session `cwd` = project folder).
2. **Bridge between panel and editor**
   - The WebView exposes a small JS bridge (`window.kapcutStudio`) for:
     `openProject(path)`, `currentTimeline()`, `requestPreview(time)`,
     `applyCommand(cmd)` — all routed to Concat's MCP server in window mode.
   - OpenCode already has MCP access to KapCut (189+ tools); the panel just
     gives the user the full OpenCode surface while those tools run.
3. **Auth / safety**
   - Local WebView only; no remote code execution from project content.
   - MCP token stays in the local server environment, never in the panel JS.

## Why this needs a CI build
Slint has no WebView widget; adding `wry` and a new crate changes the Rust
workspace. That must be compiled and tested by GitHub Actions, then shipped in
a future installer. Per the current directive, **no installer is built** until
studio parity (AE/CapCut/Premiere/Resolve mix) is reached.

## Shipped behaviour
The title-bar wand button opens the full OpenCode UI in the system browser
with the current project path in the URL fragment. An in-process WebView on a
dedicated thread was tried first; WebView2 on Windows requires a main-thread
STA message loop and crashed the editor with `0xC0000005`. Docking the WebView
inside the editor stays a future engine change; the browser route shows the
identical full UI (sessions, model picker, diffs, tools, MCP list) reliably.
