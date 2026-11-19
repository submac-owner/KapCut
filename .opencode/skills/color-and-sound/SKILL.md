---
name: color-and-sound
description: Color grading workflows and sound design for video - grades, LUTs, wheels, curves, loudness, music sync, foley and risers. Use for finishing passes or when a user asks for a 'look' or better audio.
---

# Color & Sound

## Grading workflow (Resolve-style in KapCut)
1. **Balance** — correct exposure/white balance first (`adjust_*`, `color_wheels_set`).
2. **Look** — apply a grade package: `concat.teal-orange`, `concat.noir`, `concat.vintage`, `concat.vibrance` via `effect_layer_add`.
3. **Shape** — HSL, curves, vignette (`color_hsl_set`, `color_curves_set`, `adjust_set`).
4. **Match** — copy grade across clips (`adjust_apply_all`).
5. One grade for the whole piece; consistency reads as production value.

## Loudness
- Voice ~ -16 LUFS for podcasts, -14 for online video; music bed 12–18 dB under voice.
- MCP: `audio_volume`, `audio_normalize`, `audio_fade_in/out`, `audio_enhance`.

## Music sync
- Derive BPM; place cuts and graphic pulses on transients with `studio_beat_sync` (bpm, property, amount).
- Use risers before drops and hits on pattern interrupts: `sfx_add` at exact timeline seconds.

## Sound design layers
- **Bed** — continuous music, low.
- **Foley** — small real-world sounds under actions.
- **Whoosh/click** — on cuts and transitions.
- **Hit/impact** — on reveals and emphasis.
- Silence is a tool: drop the bed for one beat before a reveal.

## MCP checklist
`audio_add` → `sfx_add` → `audio_fade_*` → `audio_normalize` → `preview_frame` at beat points → `export_start`.
