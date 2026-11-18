# KapCut implementation — slice 1

Goal: Concat fork that behaves like CapCut, exposes 100+ MCP tools, and has an
embedded chat panel driving OpenCode. Local machine = debug/analysis only;
**all builds and renders run on GitHub Actions**.

## What exists now

- Base fork: `jub0t/Concat` @ `acc6367b` (AGPL-3.0), Rust workspace under `src/crates`.
- **Tool catalog**: `kapcut/catalog.json` — 120 new CapCut-style tools in 16
  families, derived from real CapCut draft schemas (`../notes/capcut-schemas.md`).
- **Generated Rust**: `src/crates/concat-mcp/src/capcut_tools.rs`, wired into
  `tools::catalogue()` in `src/crates/concat-mcp/src/tools.rs`.
- **Generator + validator**: `kapcut/generate_catalog.py`, `kapcut/validate_catalog.py`.
- **Material bridge**: `kapcut/materials/` — registry of Concat's **132 real
  effect packages**, 58 CapCut→Concat effect mappings and 14 adjust mappings,
  plus Python reference implementation + tests and generated Rust
  (`capcut_bridge.rs`). It converts CapCut draft materials into real
  `edit.apply` commands (`setEffectKey` on `concat.adjust`, `addLayerClip` with
  real package IDs). Verified against the real draft: 7 commands translated.
- **Bridge tool**: `kapcut_translate` (Action::CapCutTranslate) is in the catalog
  and wired into the MCP server, so an agent can translate a CapCut material
  and apply it.
- **Chat panel prototype**: `kapcut/chat-panel/` — static UI + Node bridge that
  talks to OpenCode's HTTP API (`opencode api`).
- Concat's existing `.github/workflows/ci.yml` and `release.yml` handle CI/builds.

## Tool families (120 tools)

animations, audio, captions, clip, color, effects, export, masks, media,
overlays, preview, project, style, text, timeline, transitions.

Each tool maps to a real Concat API method or `edit.apply` command op. The
catalog is the single source of truth: edit it, regenerate, run the validator,
push, and let CI compile.

```powershell
cd C:\Users\Admin\Desktop\Capcut\KapCut
python kapcut\generate_catalog.py
python kapcut\validate_catalog.py
git add -A && git commit -m "kapcut: regenerate tool catalog"
git push
```

## Design decisions

- **Reuse, don't rewrite.** CapCut structures map onto existing Concat commands:
  `materials.texts` → `addTextClip`/`styleTextRange`; `common_mask` → `setClipCutout`;
  `material_animations` → layer/keyframe commands; `effects`/`hsl`/`color_*` →
  `addLayerClip`/`setEffectKey`; `transitions` → `addTransitionClip`;
  `audios`/`audio_fades` → sound layers + `setClipVoice`/`updateClip`.
- **No duplicate tools.** Names already shipped by Concat are filtered out by
  the generator.
- **Progressive disclosure.** 120 flat tools is a lot for an agent. Next slice
  adds `discover_tools`/`describe_tool` plus family allow-lists via
  `REA_MCP_TOOL_FAMILIES`-style filtering (planned for KapCut MCP).
- **Remote MCP (planned).** Streamable HTTP transport with per-agent tokens,
  tool allow-lists, and sandboxed project paths, so an outside agent can drive
  a project without host filesystem access.
- **Chat panel** uses `@opencode/client`/`opencode api` HTTP endpoints:
  `POST /api/session`, `POST /api/session/{id}/prompt`, `GET /api/session/{id}/message`.

## Run the chat panel prototype (local)

```powershell
node kapcut\chat-panel\server.mjs
# open http://localhost:7421
```

It reuses the shared OpenCode service (no new auth). Click **+ New session**,
then ask for edits. Tool calls appear in the log.

## Remaining work

1. CI verification of the generated Rust (runs triggered on the pushed commit).
2. Extend the bridge: exact preset definitions for CapCut transitions,
   animations, text styles and masks (the current mappings are semantic
   approximations, e.g. "Chroma Stretch" -> `concat.chroma-key`).
3. Implement `discover_tools`/`describe_tool` and tool-family filtering in `concat-mcp`.
3. CapCut-style preset packages: effects, filters, transitions, animations,
   text/font styles, masks, audio and color presets built from open shaders/fonts.
4. Remote MCP HTTP transport + auth + outside-agent docs.
5. Embed the chat panel into the Concat window (Rust UI change, CI build).
6. Render workers on GitHub Actions for production exports.

## References

- `../notes/capcut-recon.md`, `../notes/capcut-schemas.md`
- `../notes/capcut-draft-1118-v2.json`
- `../evidence/capcut-reference/` (static snapshot)
