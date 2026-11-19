---
name: video-editing-craft
description: Core video editing craft - pacing, cut grammar, rhythm, hooks, retention and story order. Use whenever planning or executing an edit, before choosing effects or presets.
---

# Video Editing Craft

Editing is decisions about time, not effects. Apply these before reaching for tools.

## The five dials of any edit
1. **Pace** — seconds between picture changes. Retention edits 0.6–3 s; documentary 4–10 s; cinematic 8–20 s.
2. **Text** — none / lower thirds / keyword callouts / every-word captions.
3. **Graphics & b-roll** — share of runtime showing something other than the speaker.
4. **Sound** — music bed only, or risers/whooshes/hits on every change.
5. **Look** — natural grade vs stylised; one angle vs many; punch-ins or none.

A style is a setting of these five dials. Name the dials in your plan so the edit is checkable.

## Cut grammar
- **Jump cut** — remove pauses/retakes; keep motion continuity.
- **J-cut** — audio arrives before video; **L-cut** — audio lingers after video. Use for conversation and podcasts.
- **Match cut** — similar shapes across cuts; **cut on action** — hide the cut inside movement.
- **Cut on the beat** — sync changes to music transients (`studio_beat_sync` with BPM).
- **Micro-move** — every 3–5 s add a subtle scale/offset key so no frame is static.

## Hook engineering (first 1.5 s)
Open on motion, not a static frame. Options: scale-in payoff text, wipe reveal, element crossing frame, or starting mid-animation. Choose one deliberately. Structure: **hook → payoff → CTA**.

## Rhythm and retention
- Vary shot length; identical durations flatten attention.
- Put the strongest visual in the first second, then a pattern interrupt every 8–15 s.
- One graphic shouting means the rest of the frame stays quiet.
- Animate to point or explain, never to decorate.

## Applying with KapCut MCP
- Plan the cut list in seconds first; use `timeline_read` to learn clip/track ids.
- Place with `clip_add`, `clips_add_sequence`, `clips_split`, `clips_move`, `trimClip`.
- Add motion with `keyframe_set` / `motion_apply_preset` / `studio_beat_sync`.
- Verify with `preview_frame` at several times before `export_start`.
