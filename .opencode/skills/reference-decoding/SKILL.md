---
name: reference-decoding
description: Decode a reference video into an editable style profile - shot cadence, cut pattern, palette, pacing and loudness - then apply that pattern to the current project. Use when the user provides a reference video or says 'edit it like this'.
---

# Reference Video Decoding

When the user gives a reference video and says "edit my project like that", decode patterns instead of guessing.

## Step 1 — decode
Run the local CPU decoder (no GPU, no upload):
```powershell
python C:\Users\Admin\Desktop\Capcut\KapCut\kapcut\studio\reference_decode.py "<reference.mp4>"
```
It returns a JSON profile with:
- `meta` — fps, resolution, aspect, duration, audio presence
- `style_profile.pace` — fast/medium/slow from median shot length
- `style_profile.cut_times` / `shot_lengths` — the actual cut rhythm
- `style_profile.mean_rgb` — dominant color signature
- `style_profile.loudness_db` — music/voice loudness

## Step 2 — translate into the current project
Pass the profile to `studio_edit_style` (or `editing_style_engine.compile_plan`) together with the user's description:
```
studio_edit_style({
  path: "<current project>",
  description: "<user's request>",
  profile: <decoded profile>,
  title: "...",
  duration: ...
})
```
The engine derives all five dials (pace, text, graphics, sound, look) — the reference supplies shot cadence and palette, the user's words supply the rest.

## Step 3 — verify
- `timeline_read` to confirm shot count and cadence.
- `preview_frame` at 3–5 timestamps to compare against the reference.
- `export_start` / `export_wait` for the final video.

## Notes
- Scene detection threshold default 0.25; raise it for subtler cuts, lower it for soft transitions.
- If the reference has no detected cuts, the tool falls back to `pace` from its duration and description.
- Never copy the reference's copyrighted footage or assets — reproduce its *pattern* only.
