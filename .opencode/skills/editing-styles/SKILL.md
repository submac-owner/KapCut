---
name: editing-styles
description: Recognisable video editing styles (retention/Hormozi, Ali Abdaal, MrBeast, documentary, video essay, cinematic, vlog/podcast, clean explainer) and how to reproduce them. Use when a user names a style or asks for a 'look'.
---

# Editing Styles

Creator names are shorthand, not specs — build the style from its dials.

## Fast retention / Hormozi
- 0.6–2 s cuts, punch-in zooms, bold word-by-word captions centered, high-contrast colors, SFX on key words.
- MCP: `studio_animate_text(type_on/burst)` for captions, `studio_beat_sync` for cuts, `clip_transform` punch-ins, `sfx_add` on emphasis.

## Ali Abdaal / clean education
- Talking head + b-roll every 8–15 s, text callouts, bright tidy grade, light motion.
- MCP: `clips_add_sequence` for b-roll, `title_add` callouts, `color_*` gentle grade, restrained `motion_apply_preset(fade_in_up)`.

## MrBeast / entertainment
- Constant visual change, hook in first seconds, stakes on screen, heavy sound design.
- MCP: rapid `clips_split`/`clips_move`, `text_*` overlays for stakes, layered `sfx_add`, `transition_add` between beats.

## Documentary / video essay
- Narration over archive, maps, timelines, motion graphics, low music bed, moderate pace.
- MCP: `media_import` for archive, ` studio_compose(data-story)` style layers, `title_add` lower thirds, `color_*` restrained.

## Cinematic
- Graded footage, music-led cuts, speed ramps, slow motion, few words.
- MCP: `clip_speed`, `color_*` LUT packages, `transition_add`, minimal text.

## Vlog / podcast
- Natural pacing, jump cuts or multicam switching, minimal graphics.
- MCP: `clips_split` for jumps, `audio_*` tools for clean voice, sparse titles.

## Clean tech / UI explainer
- Screen recordings in tilted frames, cursor zooms, callouts, rounded UI cards.
- MCP: `clip_transform` for tilt/crop, `keyframe_set` for cursor zooms, `shape_*` for cards.

## Style recipe format
When asked for a style, write: `pace=…, text=…, graphics=…, sound=…, look=…`, then map each dial to MCP tools. This keeps the result reproducible and lets you invent hybrids.
