---
name: short-form-video
description: TikTok, Reels and YouTube Shorts production - vertical safe zones, 1.5s hooks, sound-off captions, retention structure, platform UI avoidance. Use for any 9:16 or under-60s video.
---

# Short-Form Video (TikTok / Reels / Shorts)

## Frame
- 1080×1920. Keep critical text in the center band; avoid bottom quarter (captions/UI) and right rail (engagement).
- Test legibility at phone size: large sans-serif, high contrast, subtle backing shape or shadow.

## Structure
- **Hook (0–1.5 s)** → **payoff** → **CTA**. Open on motion or a text statement of the payoff.
- Optimal length 31–60 s. One idea per video.

## Captions (sound-off viewers)
- Word-by-word with emphasis on high-salience words (numbers, twist, command).
- Use `studio_animate_text(type_on/highlight_pop)` and `title_style_words` (fill/font/size per range).
- Never animate every word with the same bounce; motion should follow voice rhythm.

## Common aesthetics
- **Brainrot stack** — 2–3 vertical layers, cuts every 0.6–0.8 s, multi-color captions. Use `clips_add_sequence` + overlays + `studio_beat_sync`.
- **Vox / video essay** — one sans font, charts/maps every 8–10 s, calm voice, low music. Use `studio_compose(data-story)`, `title_add`.
- **Minimal documentarian** — clean thin captions bottom-center, cinematic frames, one grade.
- **Green-screen explain** — presenter on one third, visuals swapped every 4–6 s.

## MCP workflow
`project_create` (9:16 via `project_set_video`) → `media_import` → `clips_add_sequence` → `title_add`/`studio_animate_text` → `sfx_add` on key beats → `preview_frame` → `export_start`.
