# KapCut — plan

## Goal

A personal, non-commercial fork of **Concat** optimized to work like **CapCut**, with:

1. An embedded **chat panel that drives OpenCode**.
2. An **MCP server exposing 100+ tools** covering studio-level video production (projects, media, timeline, effects, filters, transitions, styles, captions, fonts, audio, export, render jobs).
3. The same MCP reachable **remotely** so an agent on another machine can drive the project.
4. Local machine = debug/analysis only. All builds/compiles and heavy renders happen on **GitHub Actions / CI** (this host has 8 GB RAM and no GPU).

## Current state

- Base: [`jub0t/Concat`](https://github.com/jub0t/Concat) @ `acc6367b`, AGPL-3.0-or-later, cloned into this folder.
- Concat is Rust (`src/crates`), needs Rust 1.93+, FFmpeg 7+ dev libs, cmake, C++ toolchain — **do not build locally**.
- Concat already ships `concat --mcp`: stdio MCP, ~40 tools (`project_*`, `clip_*`, `title_*`, `captions_add`, `effects_list`, `effect_layer_add`, `export_*`, `preview_frame`, `edit_apply`, ...), plus JSON-RPC and (source builds) gRPC. `edit_apply` accepts any edit command or batch, which is the lever for growing the tool catalog.
- Concat already has 100 effects/filters/transitions, keyframes, auto-captions, TTS, background removal, multi-track, titles, proxies, H.264/HEVC/AV1 export.

## CapCut recon

- Installed version **9.7.1.4203** at `C:\Users\Admin\AppData\Local\CapCut\Apps\`.
- Native **Qt / QML / CEF (Chromium Embedded)** application (directories `Qt`, `QtQml`, `QtQuick`, `cef`, `Resources`, `plugins`, `x64/x86`, `smart_cut.ofx.bundle`, `agent_runtime_addon.node`). REA's generic desktop-family probe found no standard candidate — expected for a Qt app.
- No runtime behavior inspected yet. Next: Frida instrumentation (works on this host), plus targeted analysis of project format and resource catalogs with REA.

## Constraints / decisions

- **No local compilation.** GitHub Actions for builds; local only runs/debugs/analyzes.
- **Tool catalog design:** namespaced families (e.g. `project_*`, `timeline_*`, `media_*`, `effects_*`, `styles_*`, `captions_*`, `audio_*`, `export_*`, `render_*`, `agent_*`) plus `discover_tools` / `describe_tool` for progressive disclosure, so 100+ tools don't blow the agent's context.
- **Remote MCP:** Streamable HTTP with per-agent tokens, tool allowlists, sandboxed paths. Remote agent drives the *project*, not the host filesystem.
- **Chat panel:** use `@opencode/client` / OpenCode HTTP API to drive the local OpenCode service from the app UI.
- **Assets:** reproduce CapCut-like looks with our own presets/shaders/LUTs and open fonts. Do not copy proprietary CapCut code or asset files; REA findings inform behavior and format only.

## Workstreams (proposed order)

1. **Recon** — CapCut feature/UI inventory, project format, effect/transition/styling catalogs. Store findings in `../evidence/` and notes in `../notes/`.
2. **Vertical slice** — one flow end-to-end: chat panel -> OpenCode -> Concat MCP -> edit -> preview -> export job. Register a minimal tool set first.
3. **CapCut-style layer** — presets, timeline UX, captions/fonts, transition and style catalogs mapped onto Concat's existing `edit_apply` commands.
4. **Scale to 100+ tools** — generated registry, namespaced families, discovery tools.
5. **Remote MCP** — HTTP transport, auth, allowlists, docs for connecting an outside agent.
6. **CI** — GitHub Actions build, test, release; render workers return artifacts.

## Open questions

- Which CapCut behaviors matter most first (UI layout, templates/styles, captions, effects, export presets)?
- Which tool families should exist in v1 of the 100+ catalog?
- Remote MCP auth model: static per-agent tokens or signed short-lived tokens?
- Should the chat panel use the shared OpenCode service or a dedicated profile?
