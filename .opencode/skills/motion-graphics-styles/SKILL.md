---
name: motion-graphics-styles
description: Motion graphics design languages - kinetic typography, Swiss/Bauhaus, glitch, neon, minimal, mixed-media collage, data storytelling, logo sting. Use when creating titles, lower thirds, explainers or any motion graphic.
---

# Motion Graphics Styles

A motion-graphic style is a vocabulary of **shape, type, color, timing and transition**. Invent new looks by recombining these; don't just replay a template.

## Kinetic typography
- Words are the animation. Per-word scale-in, color pop, tracking, baseline shifts.
- MCP: `studio_animate_text` (type_on, wave, burst, slider, flip_up, highlight_pop), `title_style_words`.

## Swiss / Bauhaus
- Grid, geometric primitives, flat primary colors, strict alignment, minimal motion, hard cuts.
- MCP: `shape_add`/`edit_apply` with `kind: square|circle|triangle`, `studio_design` grid tokens, `keyframe_set` with linear eases.

## Glitch / cyber
- RGB splits, scanlines, datamosh feel, jitter, chromatic aberration, stutter.
- MCP: `effect_layer_add(concat.glitch-warp, concat.chromatic-aberration)`, `motion_apply_preset(glitch_in)`, `studio_beat_sync` for stutters.

## Neon / glow
- Dark background, saturated strokes, bloom, concentric shapes, halo.
- MCP: `effect_layer_add(concat.neon, concat.glow, concat.halation)`, shapes with `stroke`/`shadow` style.

## Minimal / documentary
- One typeface, low-contrast palette, subtle fade/float, generous negative space.
- MCP: `studio_design` palette/type tokens, `motion_apply_preset(fade_in_up)`, restrained effects.

## Mixed-media collage
- Cut-out photos, stickers, scribbles, stop-motion, posterize, layers with depth.
- MCP: multiple `media_import` layers, `overlay_*`, `shape_*` strokes, `effect_layer_add(concat.posterize)`.

## Data storytelling
- Animated charts, counters, comparisons, trim-path lines, map routes.
- MCP: `shape_*` bars/paths, `studio_beat_sync` counters, `keyframe_set` on scale/offset, `expression` presets.

## Logo sting
- Reveal → hold → resolve. One signature motion, one accent color, no clutter.
- MCP: `studio_compose(logo_lockup)`, `effect_layer_add(concat.halation)`.

## Inventing a new style
Pick: palette (studio_design), type pair (display + body font), 2 shapes, 1 easing curve, 1 rhythm (beat sync or stagger), 1 effect. Combine via `edit_apply` batches. Save the result as a new entry in `kapcut/studio/templates.json`.
