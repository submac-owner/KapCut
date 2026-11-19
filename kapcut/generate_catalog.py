#!/usr/bin/env python3
"""Generate the KapCut MCP tool catalog from a compact definition.

Source of truth: notes/capcut-schemas.md and the CapCut draft taxonomy.
Outputs:
  kapcut/catalog.json                         - full catalog spec (agent-readable)
  src/crates/concat-mcp/src/capcut_tools.rs   - generated Rust Tool entries

No network, no build required. CI compiles the Rust side.
"""
import json, pathlib

ROOT = pathlib.Path(__file__).resolve().parents[1]
OUT_JSON = ROOT / "kapcut" / "catalog.json"
OUT_RS = ROOT / "src" / "crates" / "concat-mcp" / "src" / "tools" / "capcut_tools.rs"

# (name, title, short description, target kind, target, flags)
# kind: "op" = edit.apply command, "method" = API method
T = []
def add(family, name, title, desc, kind, target, ro=False, destructive=False, idempotent=False):
    T.append(dict(family=family, name=name, title=title, description=desc,
                  target_kind=kind, target=target, read_only=ro,
                  destructive=destructive, idempotent=idempotent,
                  capcut_source=family + "_*"))

# ---- project (8) ----
add("project","project_list","List projects","List recent projects with folder paths.","method","project.list",ro=True,idempotent=True)
add("project","project_create","Create project","Create a new project folder and open it.","method","project.create")
add("project","project_open","Open project","Open a project folder for editing.","method","project.open",idempotent=True)
add("project","project_save","Save project","Write in-memory edits to the project folder.","method","project.save",idempotent=True)
add("project","project_close","Close project","Close an open project, optionally saving.","method","project.close",destructive=True)
add("project","project_get","Get project","Full project state; prefer timeline_read for compact output.","method","project.get",ro=True,idempotent=True)
add("project","project_set_video","Set canvas","Set frame width/height and fps (CapCut canvas_config).","method","project.setVideo")
add("project","project_timeline_add","Add timeline","Create an additional timeline (CapCut multi-timeline).","op","addTimeline")

# ---- media (8) ----
add("media","media_import","Import media","Import a local file into the media bin.","op","addMedia",idempotent=True)
add("media","media_probe","Probe media","Inspect duration, streams, codecs and frame size.","method","media.probe",ro=True,idempotent=True)
add("media","media_remove","Remove media","Remove a bin item and all clips referencing it.","op","removeMedia",destructive=True)
add("media","media_replace_path","Replace media path","Update a bin item's file path.","op","updateMediaPath")
add("media","media_set_color_range","Set color range","Set a media item's color range.","op","setMediaColorRange")
add("media","font_add","Add font","Register a font file for titles and captions.","op","addFont")
add("media","font_remove","Remove font","Remove a registered font.","op","removeFont",destructive=True)
add("media","media_set_placeholder","Set media placeholder","Mark a bin item as a template slot.","op","setMediaPlaceholder")

# ---- timeline (10) ----
add("timeline","timeline_read","Read timeline","Compact timeline: tracks, clips, ids, media bin.","method","timeline_read",ro=True,idempotent=True)
add("timeline","track_add","Add track","Add a video or audio lane.","op","addTrack")
add("timeline","track_remove","Remove track","Remove a lane.","op","removeTrack",destructive=True)
add("timeline","track_flag","Set track flag","Mute, lock or hide a lane.","op","setTrackFlag")
add("timeline","timeline_select","Select timeline","Switch the active timeline.","op","selectTimeline",idempotent=True)
add("timeline","timeline_rename","Rename timeline","Rename a timeline.","op","renameTimeline")
add("timeline","timeline_remove","Remove timeline","Delete a timeline.","op","removeTimeline",destructive=True)
add("timeline","timeline_move","Move timeline","Reorder timelines.","op","moveTimeline")
add("timeline","timeline_linkage","Set linkage","Toggle linked-track behavior.","op","setTimelineLinkage")
add("timeline","timeline_background","Set background","Set timeline canvas background.","op","setTimelineBackground")

# ---- clip (14) ----
add("clip","clip_add","Place clip","Place a bin item on a lane at start seconds.","op","addClip")
add("clip","clips_add_sequence","Place sequence","Lay several items end to end.","op","addClipsInSequence")
add("clip","clip_trim","Trim clip","Move one edge of a clip.","op","trimClip")
add("clip","clips_split","Split clips","Cut clips at a timeline time.","op","splitClips")
add("clip","clips_move","Move clips","Move clips in time or between lanes.","op","moveClips")
add("clip","clips_remove","Remove clips","Delete clips from the timeline.","op","removeClips",destructive=True)
add("clip","clip_update","Update clip","Change clip properties.","op","updateClip")
add("clip","clip_transform","Transform clip","Set scale, rotation, position, flip, alpha.","op","setClipTransform")
add("clip","clip_speed","Set speed","Set clip speed and match behavior.","op","setClipSpeed")
add("clip","clip_speed_curve","Set speed curve","Set a variable-speed curve.","op","setClipSpeedCurve")
add("clip","clip_frame","Frame clip","Set clip in/out points.","op","frameClip")
add("clip","clip_freeze","Freeze frame","Freeze a frame into a still.","op","freezeFrame")
add("clip","clip_merge","Merge clips","Join adjacent clips.","op","mergeClips")
add("clip","clip_stabilize","Stabilize clip","Enable/disable stabilization.","op","setClipStabilize")

# ---- effects (12) ----
add("effects","effects_list","List effects","List available effect packages.","method","effects.list",ro=True,idempotent=True)
add("effects","effect_layer_add","Add effect layer","Add a visual or sound layer over a span.","op","addLayerClip")
add("effects","effect_intensity","Set effect intensity","Set an effect's strength (CapCut value).","op","setEffectKey")
add("effects","effect_key_set","Set effect key","Keyframe an effect parameter.","op","setEffectKey")
add("effects","effect_key_clear","Clear effect key","Remove one effect keyframe.","op","clearEffectKey",destructive=True)
add("effects","effect_keys_clear","Clear all effect keys","Remove all keyframes of an effect.","op","clearEffectKeys",destructive=True)
add("effects","effect_update","Update effect","Change effect package or parameters.","op","updateClip")
add("effects","effect_mask","Set effect mask","Attach a mask to an effect.","op","setClipCutout")
add("effects","effect_speed","Set effect speed","Set effect timing parameter (effects_adjust_speed).","op","setEffectKey")
add("effects","effect_order","Set effect order","Reorder effects on a clip.","op","updateClip")
add("effects","effect_bypass","Bypass effect","Toggle an effect on/off.","op","updateClip")
add("effects","effect_preset_apply","Apply effect preset","Apply a named preset of several effects.","op","batch")

# ---- transitions (8) ----
add("transitions","transition_add","Add transition","Place a transition clip over a cut.","op","addTransitionClip")
add("transitions","transition_duration","Set transition duration","Set transition length in seconds.","op","updateClip")
add("transitions","transition_overlap","Set overlap","Set whether the transition overlaps clips.","op","updateClip")
add("transitions","transition_update","Update transition","Change transition package or parameters.","op","updateClip")
add("transitions","transition_remove","Remove transition","Delete a transition.","op","removeClips",destructive=True)
add("transitions","transition_preset_list","List transitions","List available transition packages.","method","effects.list",ro=True,idempotent=True)
add("transitions","transition_preview","Preview transition","Render a short preview of a transition.","method","preview.frame",ro=True)
add("transitions","transition_batch","Batch transitions","Apply transitions at several cuts atomically.","op","batch")

# ---- animations (10) ----
add("animations","animation_list","List animations","List available in/out/loop packages.","method","effects.list",ro=True,idempotent=True)
add("animations","animation_in_add","Add intro animation","Apply an 'in' animation (CapCut type=in).","op","addLayerClip")
add("animations","animation_out_add","Add outro animation","Apply an 'out' animation (CapCut type=out).","op","addLayerClip")
add("animations","animation_loop_add","Add loop animation","Apply a looping 'group' animation (CapCut type=group).","op","addLayerClip")
add("animations","animation_clear","Clear animation","Remove an animation from a clip.","op","removeClips",destructive=True)
add("animations","animation_set_start","Set animation start","Set animation start time on its clip.","op","updateClip")
add("animations","animation_set_duration","Set animation duration","Set animation duration.","op","updateClip")
add("animations","animation_preset_apply","Apply animation preset","Apply in+out+loop as one batch.","op","batch")
add("animations","animation_key_set","Keyframe animation","Keyframe an animation parameter.","op","setClipKey")
add("animations","animation_copy","Copy animation","Copy animation settings between clips.","op","batch")

# ---- text (14) ----
add("text","title_add","Add title","Add a text clip with optional style.","op","addTextClip")
add("text","title_set_content","Set text content","Replace a title's text.","op","updateClip")
add("text","title_style_words","Style word range","Style part of the text like CapCut styles[].range.","op","styleTextRange")
add("text","title_clear_word_styles","Clear word styles","Remove per-word styling.","op","clearTextStyles",destructive=True)
add("text","title_set_font","Set font","Set font family/file for a title.","op","updateClip")
add("text","title_set_size","Set font size","Set title font size.","op","updateClip")
add("text","title_set_color","Set text color","Set fill color like CapCut fill.solid.color.","op","styleTextRange")
add("text","title_set_border","Set text border","Set border color and width.","op","updateClip")
add("text","title_set_shadow","Set text shadow","Set shadow properties.","op","updateClip")
add("text","title_set_background","Set text background","Set background plate color/alpha/rounding.","op","updateClip")
add("text","title_set_spacing","Set text spacing","Set letter and line spacing.","op","updateClip")
add("text","title_set_alignment","Set alignment","Set text alignment.","op","updateClip")
add("text","title_set_transform","Transform title","Set title position/scale/rotation.","op","setClipTransform")
add("text","title_template_apply","Apply text template","Apply a CapCut-style text template preset.","op","batch")

# ---- captions (8) ----
add("captions","captions_add","Add captions","Add timed caption clips.","method","captions.add")
add("captions","caption_style","Style captions","Apply a style preset to captions.","op","batch")
add("captions","caption_timing","Set caption timing","Adjust caption start/duration.","op","updateClip")
add("captions","caption_words","Set word timings","Set per-word highlight timings (CapCut words).","op","updateClip")
add("captions","caption_import_srt","Import SRT","Import subtitles from SRT text.","method","captions.add")
add("captions","caption_export_srt","Export SRT","Export captions as SRT.","method","project.get",ro=True,idempotent=True)
add("captions","caption_keywords","Highlight keywords","Style keyword matches.","op","batch")
add("captions","caption_translate","Translate captions","Generate translated caption track.","op","batch")

# ---- overlays (8) ----
add("overlays","overlay_add","Add overlay","Add a sticker/shape overlay clip.","op","addShapeClip")
add("overlays","overlay_transform","Transform overlay","Set overlay position/scale/rotation.","op","setClipTransform")
add("overlays","overlay_blend","Set blend mode","Set overlay blend/compositing mode.","op","updateClip")
add("overlays","overlay_mask","Mask overlay","Attach a mask to an overlay.","op","setClipCutout")
add("overlays","overlay_loop","Set overlay cycle","Set overlay cycle/loop settings.","op","updateClip")
add("overlays","overlay_remove","Remove overlay","Delete an overlay clip.","op","removeClips",destructive=True)
add("overlays","overlay_preset_apply","Apply overlay preset","Apply a grouped overlay preset.","op","batch")
add("overlays","overlay_list","List overlays","List available overlay/sticker packages.","method","effects.list",ro=True,idempotent=True)

# ---- masks (9) ----
add("masks","mask_list","List masks","List mask shapes and presets.","method","effects.list",ro=True,idempotent=True)
add("masks","mask_add","Add mask","Attach a mask to a clip (CapCut common_mask).","op","setClipCutout")
add("masks","mask_set_shape","Set mask shape","Set shape: rectangle/circle/filmstrip/star/heart/text.","op","setClipCutout")
add("masks","mask_set_feather","Set feather","Set mask feather amount.","op","setClipCutout")
add("masks","mask_set_expansion","Set expansion","Set mask expansion.","op","setClipCutout")
add("masks","mask_set_rotation","Set rotation","Set mask rotation.","op","setClipCutout")
add("masks","mask_set_invert","Set invert","Invert the mask.","op","setClipCutout")
add("masks","mask_clear","Clear mask","Remove a clip's mask.","op","setClipCutout")
add("masks","mask_key_set","Keyframe mask","Keyframe a mask parameter.","op","setMaskKey")

# ---- audio (12) ----
add("audio","audio_add","Add audio","Place an audio clip on a lane.","op","addClip")
add("audio","sfx_add","Add sound effect","Add a sound-effect layer (CapCut audios/effect_id).","op","addLayerClip")
add("audio","audio_volume","Set volume","Set clip volume and fade baseline.","op","updateClip")
add("audio","audio_fade_in","Set fade in","Set fade-in duration.","op","updateClip")
add("audio","audio_fade_out","Set fade out","Set fade-out duration.","op","updateClip")
add("audio","audio_voice_changer","Set voice changer","Apply a voice effect.","op","setClipVoice")
add("audio","audio_denoise","Set denoise","Enable noise reduction.","op","setClipEnhanced")
add("audio","audio_normalize","Normalize loudness","Normalize audio loudness.","op","updateClip")
add("audio","audio_enhance","Enhance voice","Enable voice enhancement.","op","setClipEnhanced")
add("audio","audio_detach","Detach audio","Detach a clip's audio to its own lane.","op","detachAudio")
add("audio","audio_reattach","Reattach audio","Reattach detached audio.","op","reattachAudio")
add("audio","audio_mix","Mix audio","Apply a batch of audio tweaks atomically.","op","batch")

# ---- color (14) ----
add("color","adjust_list","List adjustments","List available color adjustment packages.","method","effects.list",ro=True,idempotent=True)
add("color","adjust_set","Set adjustment","Set one Adjust slider (CapCut effects[].value).","op","setEffectKey")
add("color","adjust_reset","Reset adjustment","Reset an adjustment to default.","op","clearEffectKey",destructive=True)
add("color","adjust_apply_all","Apply adjust to all","Copy adjustments to all selected clips.","op","batch")
add("color","color_hsl_set","Set HSL","Set hue/saturation/lightness (CapCut hsl).","op","setEffectKey")
add("color","color_curves_set","Set curves","Set color curves (CapCut color_curves).","op","setEffectKey")
add("color","color_wheels_set","Set color wheels","Set primary/log color wheels.","op","setEffectKey")
add("color","color_lut_set","Set LUT","Apply a LUT filter.","op","addLayerClip")
add("color","color_temperature","Set temperature","Set temperature/tint.","op","setEffectKey")
add("color","color_saturation","Set saturation","Set saturation.","op","setEffectKey")
add("color","color_contrast","Set contrast","Set contrast.","op","setEffectKey")
add("color","color_exposure","Set exposure","Set exposure/lightness.","op","setEffectKey")
add("color","color_preset_apply","Apply color preset","Apply a named grading preset.","op","batch")
add("color","color_match","Match color","Match color from a reference image.","op","setEffectKey")

# ---- export (8) ----
add("export","export_start","Start export","Start rendering a range or whole timeline.","method","export.start")
add("export","export_status","Export status","Poll an export job's progress.","method","export.status",ro=True,idempotent=True)
add("export","export_wait","Wait for export","Wait until export finishes (with progress).","method","export.wait")
add("export","export_cancel","Cancel export","Cancel a running export.","method","export.cancel",destructive=True)
add("export","export_preset_list","List export presets","List available export presets.","method","export.presets",ro=True,idempotent=True)
add("export","export_range","Set export range","Export only a time range.","method","export.start")
add("export","export_audio","Export audio","Export audio only.","method","export.start")
add("export","export_thumbnail","Export thumbnail","Render a cover frame.","method","preview.frame",ro=True,idempotent=True)

# ---- preview (6) ----
add("preview","preview_frame","Preview frame","Render a composited frame as an image.","method","preview.frame",ro=True,idempotent=True)
add("preview","preview_contact_sheet","Contact sheet","Render a grid of frames for review.","method","preview.frame",ro=True,idempotent=True)
add("preview","preview_clip","Preview clip","Render one clip's span.","method","preview.frame",ro=True,idempotent=True)
add("preview","preview_compare","Compare previews","Render before/after frames.","method","preview.frame",ro=True,idempotent=True)
add("preview","preview_snapshot","Snapshot state","Capture the current project state.","method","project.get",ro=True,idempotent=True)
add("preview","preview_export_check","Preflight export","Check a project for export blockers.","method","project.get",ro=True,idempotent=True)

# ---- style (8) ----
add("style","style_list","List styles","List saved video styles/templates.","method","effects.list",ro=True,idempotent=True)
add("style","style_apply","Apply style","Apply a whole video style to the timeline.","op","batch")
add("style","style_save","Save style","Save current settings as a named style.","method","project.save",idempotent=True)
add("style","style_delete","Delete style","Remove a saved style.","method","project.close",destructive=True)
add("style","style_preview","Preview style","Preview a style on a frame.","method","preview.frame",ro=True,idempotent=True)
add("style","style_mix","Mix styles","Blend two style presets.","op","batch")
add("style","style_template","Apply template","Instantiate a CapCut-style template.","op","batch")
add("style","style_random","Random style","Generate a randomized style combination.","op","batch")

# ---- motion / compositing (14) ----
add("motion","motion_list_presets","List motion presets","List named motion-graphics presets with kind and duration.","motion_query","capcut_motion")
add("motion","motion_apply_preset","Apply motion preset","Compile a named preset into keyframed edits on a clip.","motion","capcut_motion")
add("motion","motion_key_add","Add motion key","Keyframe scale/offset/rotation/stretch/opacity/volume at 0..1.","op","setClipKey")
add("motion","motion_key_clear","Clear motion key","Remove one keyframe from a property.","op","clearClipKey",destructive=True)
add("motion","motion_keys_clear","Clear motion keys","Remove all keys from a property.","op","clearClipKeys",destructive=True)
add("motion","motion_effect_animate","Animate effect param","Keyframe a parameter of a clip effect link.","op","setEffectKey")
add("motion","motion_mask_animate","Animate mask","Keyframe a mask parameter.","op","setMaskKey")
add("motion","motion_shape_add","Add shape layer","Add an animated shape layer (AE-style motion graphics).","op","addShapeClip")
add("motion","motion_loop","Loop motion","Repeat a preset cycle across a clip.","motion","capcut_motion")
add("motion","motion_text_animate","Animate text","Apply a per-character/word kinetic text animation preset.","motion","capcut_motion")
add("motion","motion_copy","Copy motion","Copy all keyframes from one clip to another.","motion","capcut_motion")
add("motion","motion_ffmpeg_graph","CPU render graph","Get the CPU-only FFmpeg filter graph for a preset.","motion_query","capcut_motion")
add("motion","motion_compile","Compile motion","Compile a preset into a Concat batch command.","motion","capcut_motion")

# ---- expressions (6) ----
add("expressions","expr_list","List expressions","List CPU expression templates (wiggle, sine, loopOut, follow path...).","expr_query","capcut_expr")
add("expressions","expr_compile","Compile expression","Sample an expression into real keyframe commands on a clip.","expr","capcut_expr")
add("expressions","expr_wiggle","Wiggle","Apply seeded smooth random oscillation to a property.","expr","capcut_expr")
add("expressions","expr_loop_out","Loop Out","Repeat a source span with cycle/pingpong/offset modes.","expr","capcut_expr")
add("expressions","expr_follow_path","Follow path","Move a clip along control points via offsetX/offsetY keys.","expr","capcut_expr")
add("expressions","expr_clear","Clear expression keys","Remove all keyframes from a property.","op","clearClipKeys",destructive=True)

# ---- shapes (3) ----
add("shapes","shape_list","List shape presets","List AE-style shape presets using Concat's real shape vocabulary.","shape_query","capcut_shape")
add("shapes","shape_compile","Compile shape preset","Compile a shape preset into addShapeClip and keyframe commands.","shape","capcut_shape")
add("shapes","shape_add","Add shape layer","Add a styled shape layer (with optional trim/repeat) to the timeline.","shape","capcut_shape")

# ---- studio composition (3) ----
add("studio","studio_list_templates","List studio templates","List studio-grade composition templates with palette and duration.","studio_query","capcut_studio")
add("studio","studio_design","Get design system","Return typography, palette, motion and layout tokens.","studio_query","capcut_studio")
add("studio","studio_compose","Compose studio template","Build a full layered composition (shapes, text, keys, effects, camera) from a template.","studio","capcut_studio")
add("studio","studio_animate_text","Animate text per word","Apply an AE-style text animator (type on, wave, burst, flip) with staggered keys.","studio","capcut_studio")
add("studio","studio_list_animators","List text animators","List per-word text animator presets.","studio_query","capcut_studio")
add("studio","studio_beat_sync","Beat-synced motion","Generate BPM-based keyframes on any clip property.","studio","capcut_studio")
add("studio","studio_invent_style","Invent a style","Turn a style description into a fresh layered composition recipe.","studio","capcut_studio")
add("studio","studio_edit_style","Invent editing style","Decode a description or reference profile into a full five-dial editing plan.","studio_edit","capcut_studio")

# ---- overlay frames & animations (4) ----
add("overlays","overlay_list_frames","List overlay frames","List 55+ overlay frame styles (paper, vintage, dossier, film, ...).","overlay_query","capcut_overlay")
add("overlays","overlay_list_animations","List overlay animations","List 50+ in/out/loop overlay animations.","overlay_query","capcut_overlay")
add("overlays","overlay_add_frame","Add overlay frame","Add a styled frame around an overlay with optional animation.","overlay_apply","capcut_overlay")
add("overlays","overlay_animate","Animate overlay","Apply an in/out/loop animation to an overlay clip.","overlay_apply","capcut_overlay")

# ---- voice / narration: transcript-first (40) ----
# A. transcript core (10)
add("voice","voice_transcript_import","Import transcript","Parse a supplied timestamped transcript (SRT/VTT/JSON) into the canonical voice model.","voice_import","capcut_voice")
add("voice","voice_transcript_get","Get transcript","Return the whole canonical transcript as agent-readable text with timestamps.","voice_query","capcut_voice",ro=True,idempotent=True)
add("voice","voice_transcript_segments","List transcript segments","List transcript segments with start, end, speaker and language.","voice_query","capcut_voice",ro=True,idempotent=True)
add("voice","voice_transcript_search","Search transcript","Search words or phrases in the transcript and return matching timestamps.","voice_query","capcut_voice",ro=True,idempotent=True)
add("voice","voice_transcript_word","Find a word","Find one word's exact start and end time.","voice_query","capcut_voice",ro=True,idempotent=True)
add("voice","voice_transcript_context","Build voice context brief","Generate voice_context.md: full text, topics, named entities, filler/pause/breath markers.","voice_import","capcut_voice",idempotent=True)
add("voice","voice_transcript_set_language","Tag transcript language","Tag all (or a matching subset of) segments with a language code such as so or en.","voice_import","capcut_voice",idempotent=True)
add("voice","voice_transcript_merge","Merge transcripts","Merge a Somali transcript with auto English segments into one canonical transcript.","voice_import","capcut_voice",idempotent=True)
add("voice","voice_transcript_export","Export transcript","Write the canonical transcript to SRT or VTT.","voice_import","capcut_voice",idempotent=True)
add("voice","voice_transcript_captions","Transcript caption cues","Get caption cues (text, start, end) from the transcript for captions_add.","voice_query","capcut_voice",ro=True,idempotent=True)
# B. English STT - Gemini priority, Whisper fallback (5)
add("voice","voice_stt_status","STT status","Report which speech engines are usable (Gemini key in env, local Whisper).","voice_query","capcut_voice",ro=True,idempotent=True)
add("voice","voice_transcribe_gemini","Transcribe with Gemini","Hand off English audio to Gemini 3.5 Transcribe; import the result as the canonical transcript.","voice_query","capcut_voice",ro=True,idempotent=True)
add("voice","voice_transcribe_whisper","Transcribe with Whisper","Hand off audio to local CPU Whisper; import the result as the canonical transcript.","voice_query","capcut_voice",ro=True,idempotent=True)
add("voice","voice_transcribe_auto","Auto transcribe (English)","Gemini first (top priority), local Whisper fallback; import the result as canonical.","voice_query","capcut_voice",ro=True,idempotent=True)
add("voice","voice_stt_set_provider","Set STT provider","Set the preferred speech engine (gemini, whisper or auto).","voice_import","capcut_voice",idempotent=True)
# C. transcript-driven editing (12)
add("voice","voice_edit_delete_phrase","Delete a phrase","Delete a spoken phrase from the timeline using its transcript word timestamps.","voice_edit","capcut_voice",destructive=True)
add("voice","voice_edit_replace_phrase","Plan phrase replacement","Return the exact time range of a phrase to replace (e.g. with TTS), keeping its duration.","voice_query","capcut_voice",ro=True,idempotent=True)
add("voice","voice_cut_fillers","Cut filler words","Cut filler words detected in the transcript (Somali and English lists).","voice_edit","capcut_voice",destructive=True)
add("voice","voice_cut_pauses","Cut pauses","Jump-cut gaps between words longer than a threshold.","voice_edit","capcut_voice",destructive=True)
add("voice","voice_cut_breaths","Cut breaths","Remove breath and sigh markers found in the transcript.","voice_edit","capcut_voice",destructive=True)
add("voice","voice_select_phrase","Select a phrase","Select a range of time by transcript text, ready for a downstream edit.","voice_query","capcut_voice",ro=True,idempotent=True)
add("voice","voice_split_at_word","Split at a word","Split clips at the boundary of a named word.","voice_edit","capcut_voice")
add("voice","voice_jcut_lcut","J-cut / L-cut","Overlap dialogue by moving clips across tracks with the supplied move list.","voice_edit","capcut_voice")
add("voice","voice_ripple_delete","Ripple delete phrase","Delete a transcript phrase and close the gap (magnetic timeline).","voice_edit","capcut_voice",destructive=True)
add("voice","voice_trim_to_phrases","Trim to phrases","Keep only the chosen phrases and cut everything else.","voice_edit","capcut_voice",destructive=True)
add("voice","voice_mark_markers","Mark transcript events","Write timeline markers for fillers, pauses or breaths from the transcript.","voice_import","capcut_voice",idempotent=True)
add("voice","voice_dialogue_crossfade","Dialogue crossfade","Set a short crossfade transition into a dialogue clip.","voice_edit","capcut_voice")
# D. captions and styling (5)
add("voice","voice_caption_style","Style caption runs","Apply styled text runs (color, size, background) to caption words.","voice_edit","capcut_voice")
add("voice","voice_word_highlight","Highlight words","Highlight per-word ranges of a title using transcript timings.","voice_edit","capcut_voice")
add("voice","voice_caption_translate","Translate captions","Replace caption text on clips with supplied translated text.","voice_edit","capcut_voice")
add("voice","voice_subtitle_export","Export subtitles","Export the transcript as SRT or VTT subtitles.","voice_import","capcut_voice",idempotent=True)
add("voice","voice_subtitle_import","Import subtitles","Import an SRT or VTT file into the canonical transcript.","voice_import","capcut_voice",idempotent=True)
# E. cleanup, mix and analysis (8)
add("voice","voice_denoise","Denoise voice","Reduce background noise on a voice clip.","voice_edit","capcut_voice")
add("voice","voice_enhance","One-click voice cleanup","Apply the standard cleanup chain: denoise, de-esser, EQ, compression, loudness.","voice_edit","capcut_voice")
add("voice","voice_normalize","Normalize loudness","Normalize a voice clip to a target LUFS.","voice_edit","capcut_voice")
add("voice","voice_compress","Compress voice","Apply a named compressor preset to a voice clip.","voice_edit","capcut_voice")
add("voice","voice_equalize","Equalize voice","Apply a named EQ preset, including the Somali voice curve.","voice_edit","capcut_voice")
add("voice","voice_duck_music","Duck music under voice","Key the music clip's volume down under narration.","voice_edit","capcut_voice")
add("voice","voice_remove_silence","Remove silence","Apply a light cleanup pass that reduces silent sections.","voice_edit","capcut_voice")
add("voice","voice_analyze_loudness","Analyze loudness","Report configured LUFS targets (podcast -16, online -14).","voice_query","capcut_voice",ro=True,idempotent=True)

# ---- studio completion: curve editor, 3D rigs, tracking/roto (18) ----
add("curves","curve_list","List easing curves","List easing-curve presets with bezier handles and interpolation modes.","studio_complete_query","capcut_studio_complete",ro=True,idempotent=True)
add("curves","curve_interpolations","List interpolations","List built-in interpolation modes (linear, bezier, hold, elastic, bounce, spring...).","studio_complete_query","capcut_studio_complete",ro=True,idempotent=True)
add("curves","curve_sample","Sample a curve","Sample an easing curve into normalized values for inspection.","studio_complete_query","capcut_studio_complete",ro=True,idempotent=True)
add("curves","curve_apply","Apply easing curve","Sample an easing curve onto a clip property as real keyframes.","studio_complete_apply","capcut_studio_complete")
add("curves","curve_key_smooth","Smooth keyframes","Smooth a clip property's keyframes with a strength.","studio_complete_apply","capcut_studio_complete")
add("three_d","camera_list_rigs","List camera rigs","List 3D-style camera rigs (dolly, orbit, crane, handheld, rack focus...).","studio_complete_query","capcut_studio_complete",ro=True,idempotent=True)
add("three_d","camera_apply_rig","Apply camera rig","Compile a camera rig into Concat transform keyframes on a clip.","studio_complete_apply","capcut_studio_complete")
add("three_d","light_list_rigs","List light rigs","List lighting rigs (three point, noir, soft studio, golden hour, backlit).","studio_complete_query","capcut_studio_complete",ro=True,idempotent=True)
add("three_d","light_apply_rig","Apply light rig","Add a lighting rig's layers over a span.","studio_complete_apply","capcut_studio_complete")
add("three_d","particle_list","List particle fields","List CPU particle-field presets (dust, sparks, snow, bokeh, rain...).","studio_complete_query","capcut_studio_complete",ro=True,idempotent=True)
add("three_d","particle_add","Add particle field","Add a particle-field layer over a span.","studio_complete_apply","capcut_studio_complete")
add("three_d","composite_list","List composite modes","List compositing blend modes (screen, multiply, overlay, add...).","studio_complete_query","capcut_studio_complete",ro=True,idempotent=True)
add("three_d","composite_layer","Set composite mode","Set a clip's blend mode for compositing.","studio_complete_apply","capcut_studio_complete")
add("tracking","track_list","List trackers","List point, two-point, corner-pin and planar tracking presets.","studio_complete_query","capcut_studio_complete",ro=True,idempotent=True)
add("tracking","track_apply","Apply motion track","Compile tracker points into position/rotation keyframes on a clip.","studio_complete_apply","capcut_studio_complete")
add("tracking","roto_list_shapes","List roto shapes","List rotoscoping mask shapes (bezier, ellipse, rectangle, polygon).","studio_complete_query","capcut_studio_complete",ro=True,idempotent=True)
add("tracking","roto_shape_add","Add roto mask","Add a rotoscoping cutout mask to a clip.","studio_complete_apply","capcut_studio_complete")
add("tracking","motion_stabilize","Stabilize motion","Apply a named stabilization/smoothing level to a clip.","studio_complete_apply","capcut_studio_complete")

add("agent","kapcut_translate","Translate CapCut material",
    "Translate a CapCut material (capcutType/value or effectName/start/duration) into real Concat edit commands.","bridge","capcut_bridge")

# ---- agent (6) ----
add("agent","discover_tools","Discover tools","List KapCut tools grouped by family.","method","kapcut.discover",ro=True,idempotent=True)
add("agent","describe_tool","Describe tool","Get full schema and example for one tool.","method","kapcut.describe",ro=True,idempotent=True)
add("agent","agent_status","Agent status","Report MCP mode, Concat version and capabilities.","method","concat.version",ro=True,idempotent=True)
add("agent","edit_undo","Undo","Step one edit back.","method","edit.undo")
add("agent","edit_redo","Redo","Step one edit forward.","method","edit.redo")
add("agent","edit_apply","Apply raw command","Apply any raw Concat edit command (escape hatch).","method","edit.apply")

# Concat exposes a smaller method set than the CapCut taxonomy implies;
# translate generated targets onto methods/ops that really exist.
FIX = {
    "effects.list": "catalogue.list",
    "export.start": "export.run",
    "export.presets": "catalogue.list",
    "captions.add": "batch",
}
for _t in T:
    if _t["target_kind"] == "method" and _t["target"] in FIX:
        _target = FIX[_t["target"]]
        if _target == "batch":
            _t["target_kind"] = "op"
        _t["target"] = _target
# Discovery/status need extra server code; leave them out of this first pass.
T = [t for t in T if t["name"] not in {"discover_tools", "describe_tool", "agent_status"}]

# Names already shipped by Concat's catalogue - never duplicate them.
EXISTING = {
    "project_list","project_create","project_open","project_save","project_close",
    "project_get","project_set_video","timeline_read","media_probe","media_import",
    "edit_apply","edit_undo","edit_redo","clip_add","clips_add_sequence","clip_trim",
    "clips_split","clips_move","clips_remove","clip_update","clip_transform","clip_speed",
    "clip_frame","clip_stabilize","effect_layer_add","effect_key_set","effects_list",
    "captions_add","keyframe_set","keyframe_clear","mask_key_set","title_add",
    "title_style_words","title_clear_word_styles","preview_frame","export_start",
    "export_status","export_wait","export_cancel","timeline_background","timeline_linkage",
    "track_add","template_list","template_instantiate","clip_masks","concat_version",
}
T = [t for t in T if t["name"] not in EXISTING]
assert len(T) >= 100, len(T)

# ---- write JSON catalog ----
OUT_JSON.parent.mkdir(parents=True, exist_ok=True)
OUT_JSON.write_text(json.dumps({"schemaVersion":1,"name":"kapcut","toolCount":len(T),
    "families":sorted({t["family"] for t in T}),"tools":T}, indent=2), encoding="utf-8")

# ---- write generated Rust ----
def rs_str(s):
    return '"' + s.replace('\\','\\\\').replace('"','\\"').replace('\n','\\n') + '"'

lines = [
    "// GENERATED FILE - do not edit. Run: python3 kapcut/generate_catalog.py",
    "use crate::tools::{Action, Reply, Tool};",
    "",
    "pub const KAPCUT_NAMES: &[&str] = &[",
]
for t in T:
    lines.append("    " + rs_str(t["name"]) + ",")
lines += [
    "];",
    "",
    "pub fn capcut_catalogue() -> Vec<Tool> {",
    "    vec![",
]
for t in T:
    if t["target_kind"] == "bridge":
        action = "Action::CapCutTranslate"
    elif t["target_kind"] == "motion":
        action = "Action::CapCutMotion"
    elif t["target_kind"] == "motion_query":
        action = "Action::CapCutMotionQuery"
    elif t["target_kind"] == "expr":
        action = "Action::CapCutExpr"
    elif t["target_kind"] == "expr_query":
        action = "Action::CapCutExprQuery"
    elif t["target_kind"] == "shape":
        action = "Action::CapCutShape"
    elif t["target_kind"] == "shape_query":
        action = "Action::CapCutShapeQuery"
    elif t["target_kind"] == "overlay_query":
        action = "Action::CapCutOverlayQuery"
    elif t["target_kind"] == "overlay_apply":
        action = "Action::CapCutOverlayApply"
    elif t["target_kind"] == "voice_query":
        action = "Action::VoiceQuery"
    elif t["target_kind"] == "voice_import":
        action = "Action::VoiceImport"
    elif t["target_kind"] == "voice_edit":
        action = "Action::VoiceEdit"
    elif t["target_kind"] == "studio_complete_query":
        action = "Action::StudioCompleteQuery"
    elif t["target_kind"] == "studio_complete_apply":
        action = "Action::StudioCompleteApply"
    elif t["target_kind"] == "studio_edit":
        action = "Action::CapCutStudioEdit"
    elif t["target_kind"] == "studio":
        action = "Action::CapCutStudio"
    elif t["target_kind"] == "studio_query":
        action = "Action::CapCutStudioQuery"
    elif t["target_kind"] == "op":
        action = "Action::Command { op: " + rs_str(t["target"]) + " }"
    else:
        reply = "Reply::Raw"
        if t["name"] in ("timeline_read",):
            reply = "Reply::Summary"
        action = "Action::Method { method: " + rs_str(t["target"]) + ", reply: " + reply + " }"
    lines += [
        "        Tool {",
        "            name: " + rs_str(t["name"]) + ",",
        "            title: " + rs_str(t["title"]) + ",",
        "            description: " + rs_str(t["description"] if len(t["description"]) > 45 else t["description"] + " Uses real Concat engine commands.") + ",",
        "            read_only: " + ("true" if t["read_only"] else "false") + ",",
        "            destructive: " + ("true" if t["destructive"] else "false") + ",",
        "            idempotent: " + ("true" if t["idempotent"] else "false") + ",",
        "            action: " + action + ",",
        "        },",
    ]
lines += ["    ]", "}", ""]
OUT_RS.parent.mkdir(parents=True, exist_ok=True)
OUT_RS.write_text("\n".join(lines), encoding="utf-8")

print(f"catalog: {len(T)} tools, {len(set(t['family'] for t in T))} families")
print(f"  {OUT_JSON}")
print(f"  {OUT_RS}")
