# KapCut Ultra Studio — architecture and status

## Vision
One agentic video studio combining **CapCut, DaVinci Resolve, After Effects,
Premiere Pro and Blender**, usable by the local agent through the OpenCode
panel and by remote agents through MCP. The agent can create and edit any video
a human can describe. Voice/narration is a first-class discipline (see
`docs/KAPCUT_VOICE_PLAN.md`).

## Hard constraint: CPU only
This host has **8 GB RAM and no GPU**. All new features compile to CPU data
(Concat keyframe/command batches or FFmpeg CPU filter graphs). Heavy builds and
final renders run on GitHub Actions CPU runners. Local work is authoring,
debugging and inspection only.

## Capability map
| Capability | Inspired by | Our approach | Status |
|---|---|---|---|
| Timeline cutting, tracks, captions, stickers | CapCut | Concat core commands | done |
| Effects, filters, transitions, masks, audio | CapCut | material bridge (132 packages) | done |
| Motion: presets, keys, easing, loop | AE / CapCut | 12 presets, CPU keyframe compiler | done |
| Expressions → keyframes | AE | 8 sampled expressions | done |
| Shapes / paths | AE | 12 shape presets, trim-path draw-on | done (data layer) |
| Color grading, wheels, curves, LUTs | Resolve | `concat.adjust` + LUT packages | done (preset layer) |
| Multi-cam, loudness, delivery | Premiere | Concat core + CI render workers | partial |
| Studio composition, design system | AE | templates + design tokens + compiler | done |
| Per-word text animators, beat sync | AE | sampled keyframe compilers | done |
| Editing-style engine, reference decoder | AE / Resolve | five-dial style + reference profile | done |
| Overlay frames & animations | CapCut | 67 frames + 76 in/out/loop animations | done |
| Curve / graph editing | AE | 12 easing curves + bezier sampler → keys | done (data layer) |
| 3D camera / lights / particles / composite | Blender / AE | 10 rigs, 5 lights, 8 particle fields, 8 blend modes | done (preset layer) |
| Tracking / camera solve / rotoscoping | AE / Resolve | 4 trackers, 4 roto shapes, 3 smoothing levels | done (data layer) |
| Transcript-first voice + narration | Premiere / Resolve | 40 MCP tools, Somali + English | done (V1) |
| OpenCode full-UI side panel | — | native WebView crate + title-bar button | source done |
| Live reload on MCP save | — | project watcher | source done |
| Remote MCP transport | — | token-protected JSON-RPC at `7420` + streamable HTTP `/mcp` at `7421` | done in source |
| Visual graph/curve editor UI | AE | `BezierEditor` + `InterpolationPanel` + `Curves.ease` solver | done in source |
| Peak meter / waveform UI | Premiere / Resolve | `vu-meter.slint`, `LevelMeter`, waveform rendering | done in source |

## Tool surface
**213 MCP tools across 25 families**, compiled and verified by Windows CI.
Families: agent, animations, audio, captions, clip, color, curves, effects,
export, expressions, masks, media, motion, overlays, preview, project, shapes,
studio, style, text, three_d, timeline, tracking, transitions, voice.

## Phases
- **A–D (done)** — motion, expressions, shapes, advanced presets.
- **E (done in source)** — templates, render-worker workflow, and both remote
  transports: token-protected JSON-RPC at `7420` and streamable HTTP `/mcp`
  at `7421` with bearer auth, project-root sandbox and tool-family allow-list.
- **F (done)** — studio composition engine, design system, 4 templates.
- **G (done)** — editing-style engine, reference-video decoder, overlay
  libraries (67 frames, 76 animations), Studio Look button.
- **H (done)** — curve engine, 3D camera/light/particle/composite rigs,
  tracking/rotoscoping presets (18 tools).
- **Voice V1 (done, verified)** — transcript-first voice engine, 40 tools.

## Holds (per user directive)
- **No Windows installer / exe** until the mixed AE + CapCut + Premiere +
  Resolve capability set is approved. The existing draft release stays as-is.
- The OpenCode panel must host OpenCode's **full UI**, not a chat prototype
  (`docs/KAPCUT_OPENCODE_PANEL.md`).

## Verification status
- Source-only features are **not live** in the installed Concat binary.
- Nothing is called "Done" until GitHub CI reports success and this document
  reflects it. No installer is built without explicit approval.

## Rule for every change
1. Data/preset (JSON) → generator → Rust module → catalog → CI.
2. Validate locally (`generate_catalog.py`, `validate_catalog.py`, `test_*.py`)
   before pushing.
3. Push; let Windows CI compile, lint and test. Never build on this host.
