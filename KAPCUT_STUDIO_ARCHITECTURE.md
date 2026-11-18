# KapCut Ultra Studio — architecture

## Vision

One agentic video studio combining the capabilities of **CapCut, DaVinci
Resolve, After Effects, Premiere Pro and Blender** — usable by the local agent
through the chat panel and by remote agents through MCP. The agent can create
and edit any video a human can describe.

## Hard constraint: CPU only

This host has **8 GB RAM and no GPU**. Therefore:

- All new Phase A features are **CPU-native**: motion presets compile to
  Concat keyframe commands (CPU data), and the parallel render path is an
  **FFmpeg CPU filter graph** (`zoompan`, `fade`, `rotate`, `eq`, `drawtext`,
  `overlay`) — no WGSL/GPU shaders in our new code.
- Concat's own compositor has a software-adapter fallback
  (`CONCAT_GPU=software`, WARP/lavapipe/SwiftShader); we treat it as a
  preview-only path, not a render path.
- Heavy builds and final renders run on **GitHub Actions CPU runners**, never
  on this machine. Local work is authoring, debugging and inspection only.
- RAM budget: one heavy operation at a time; close sessions and export
  Evidence between targets. 8 GB cannot hold a 4K timeline + a JVM + Chrome
  simultaneously.

## Capability map (target -> how we get there -> status)

| Capability | Inspired by | Our approach | Status |
|---|---|---|---|
| Timeline cutting, tracks, captions, stickers | CapCut | Concat core commands; 120+ MCP tools | Phase 0 done |
| Effects, filters, transitions, masks, audio | CapCut | material bridge -> 132 real Concat packages | bridge done |
| Motion graphics: presets, keys, easing, loop | After Effects / CapCut | `motion_*` tools + preset compiler (12 presets, CPU graphs) | Phase A done |
| Graph editor, expressions, shape paths | After Effects | Phase B/C: JS expression runtime + bezier path layers + curve UI | planned |
| Color grading, wheels, curves, LUTs | Resolve | `concat.adjust`, `concat.color-wheels`, `concat.curves`, LUT packages | mapped |
| Multi-cam, proxies,响 loudness, delivery | Premiere | Concat core + CI render workers | partial |
| 3D camera, lights, particles, compositing | Blender | Phase D: engine crates + CPU particle raster | planned |
| Tracking / camera solve / roto | AE / Resolve | Phase D: point tracker + rotoscoping | planned |
| Agentic control, remote MCP, chat panel | — | OpenCode chat bridge + stdio/HTTP MCP | prototype |

## Phase A (this slice)

- `kapcut/motion/presets.json` — 12 motion presets (in/out/loop), including the
  three seen in the real CapCut draft: **Slice Rebuild**, **Glare Shrink**,
  **Popping Candy**.
- `kapcut/motion/compile.py` — compiles a preset into real `setClipKey` /
  `addLayerClip` batches and a CPU FFmpeg graph; tests pass.
- 13 `motion_*` MCP tools (134 total tools, 18 families).
- Generated Rust `capcut_motion.rs`, wired via `Action::CapCutMotion` and
  `Action::CapCutMotionQuery`.

## Phases B–E

- **B — Expressions**: embed a JS runtime so properties can be procedurally
  driven (`wiggle`, `loopOut`, layer links). CPU-only.
- **C — Shape paths + graph editor**: bezier shape layers, trim paths, repeaters;
  curve editor UI (CI-built).
- **D — 3D/particles/tracking/roto**: new engine crates, CPU rasterization.
- **E — Studio glue**: template packs, CI render farm, remote MCP with tokens,
  embedded chat panel, plugin/package SDK.

## Rule for every change

1. Data/preset first (JSON) -> generator -> Rust module -> catalog -> CI.
2. Validate locally with Python (`test_bridge.py`, `test_compile.py`,
   `validate_catalog.py`) before pushing.
3. Push; let GitHub CI compile and lint. Never compile on this host.
