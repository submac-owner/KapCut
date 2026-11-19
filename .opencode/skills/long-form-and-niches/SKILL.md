---
name: long-form-and-niches
description: Long-form video formats and niche-specific editing - video essays, tutorials, podcasts, product ads, explainers, music videos, real estate, travel. Use when the user names a content type or audience.
---

# Long-Form & Niches

## Video essay / explainer
- Structure: thesis → evidence → counterpoint → conclusion. Narration-led.
- Graphics every 8–10 s: maps, charts, headlines. One typeface, low music.
- MCP: `media_import` archive, `studio_compose` data layers, `title_add`, `text_*` callouts.

## Tutorial / software explainer
- Screen recording + cursor zooms + numbered steps + callouts.
- Keep steps on screen long enough to read twice. Use `keyframe_set` on scale/offset for zooms, `shape_*` cards.

## Podcast / interview
- Multicam switching on speaker turns, J/L cuts, minimal graphics, waveform or avatar optionally.
- MCP: `audio_*` for voice, `clips_split` for turns, `title_add` lower thirds.

## Product ad / commercial
- Hook with product in motion, benefit text, 3 feature beats, CTA end card.
- MCP: `studio_compose(product/lockup)` layers, `color_*` polished grade, `sfx_add` on beats, `export_start`.

## Music video
- Cut to rhythm, lyric captions, color/lighting changes per section, performance + abstract B-roll.
- MCP: `studio_beat_sync`, `studio_animate_text`, `effect_layer_add(concat.glitch-warp/neon)`.

## Travel / real estate
- Wide establishing shots, smooth moves, one grade, minimal text, music-led pacing.
- MCP: `clip_speed` ramps, `color_*` grade, sparse `title_add`.

## Niche rule
Every niche has a **visual grammar** (what shots repeat), a **rhythm** (cut cadence) and a **palette**. Define all three before editing; then map each to MCP tools. This is how you invent a style for a niche nobody named yet.
