# KapCut Voice / Narration Studio — plan (transcript-first)

## Goal
Make voice editing a first-class discipline: 40 MCP tools for the agent and a
manual voice panel for when the user edits alone. CPU-only (8 GB, no GPU).
The voice is where most time is burned, so it is the priority.

## Core principle — the transcript is the single source of truth
The agent must know **everything** about the speech **as text**, without hearing
audio. Every input route converges on one canonical artifact per project:

```
project/voice/transcript.json   # words + segments
  word  = { text, start, end, confidence, speaker, language }
  segment = { text, start, end, speaker, language, summary }
project/voice/transcript.srt    # captions
project/voice/transcript.vtt    # captions (web)
project/voice/voice_context.md  # auto-generated brief injected into agent context
```

`voice_context.md` = full text with timestamps, key topics, named entities,
language tags, and silence/filler/breath markers. Once it exists, **all editing
decisions are made from text** — no STT engine is consulted during editing.

## Language routes (input → canonical transcript)
Both routes produce the identical canonical transcript, so downstream editing
never cares how the text was obtained.

### Somali — user-supplied transcript (the dependable route)
- User provides a timestamped transcript: SRT / VTT / plain JSON.
- `voice_transcript_import` parses it into the canonical model.
- Rationale: no STT engine reliably supports Somali. Gemini 3.5 Transcribe does
  not list Somali; so the human transcript is authoritative. No API key needed.

### English — Gemini (top priority) + local Whisper (fallback)
- `voice_transcribe_auto`: try **Gemini 3.5 Transcribe** first (word-level
  timestamps + diarization + smart disfluency removal), fall back to **local
  Whisper** (CPU, free, private) when no key/network is available.
- `voice_stt_status` reports which engines are usable; `voice_stt_set_provider`
  sets preference. Gemini is the default priority.
- API keys are read from env/config — **never written into code or committed**.
- Both engines output the same canonical transcript; English segments are
  tagged `language: "en"`.

### Mixed Somali + English (code-switching)
- `voice_transcript_merge` combines a user Somali transcript with auto English
  segments into one timeline, preserving timestamps and language tags.

## 40 MCP tools

### A. Transcript core — source of truth (10)
1 `voice_transcript_import` — parse SRT/VTT/JSON → canonical transcript
2 `voice_transcript_get` — full transcript as agent-readable text + timestamps
3 `voice_transcript_segments` — segments, speakers, language tags
4 `voice_transcript_search` — search words/phrases → timestamps
5 `voice_transcript_word` — a single word's timing
6 `voice_transcript_context` — generate `voice_context.md` brief
7 `voice_transcript_set_language` — tag segments so/en
8 `voice_transcript_merge` — merge Somali + English transcripts
9 `voice_transcript_export` — canonical → SRT/VTT/JSON
10 `voice_transcript_captions` — build caption clips from segments

### B. English STT — Gemini priority, Whisper fallback (5)
11 `voice_stt_status` — which engines are available
12 `voice_transcribe_gemini` — Gemini 3.5 Transcribe (primary)
13 `voice_transcribe_whisper` — local Whisper CPU (fallback)
14 `voice_transcribe_auto` — Gemini first, Whisper fallback
15 `voice_stt_set_provider` — set engine preference

### C. Transcript-driven editing — THE FOCUS (12)
16 `voice_edit_delete_phrase` — delete a phrase via its transcript timestamps
17 `voice_edit_replace_phrase` — replace a spoken phrase (keep timing)
18 `voice_cut_fillers` — cut filler words (so/en lists) from the timeline
19 `voice_cut_pauses` — jump-cut on transcript gaps over a threshold
20 `voice_cut_breaths` — remove breaths marked in the transcript
21 `voice_select_phrase` — select a time range by text
22 `voice_split_at_word` — split clips at a word boundary
23 `voice_jcut_lcut` — dialogue overlap from transcript
24 `voice_ripple_delete` — ripple-delete a phrase, closing the gap
25 `voice_trim_to_phrases` — keep only chosen phrases
26 `voice_mark_markers` — timeline markers from transcript events

### D. Captions & styling (5)
27 `voice_caption_style` — font/color/size/background
28 `voice_word_highlight` — per-word highlight timing
29 `voice_caption_translate` — so ↔ en caption translation
30 `voice_subtitle_export` — SRT/VTT out
31 `voice_subtitle_import` — SRT/VTT in

### E. Cleanup, mix & analysis (8)
32 `voice_denoise` — denoise
33 `voice_enhance` — one-click chain: denoise → de-esser → normalize
34 `voice_normalize` — LUFS target (podcast -16, online -14)
35 `voice_compress` — compressor
36 `voice_equalize` — EQ presets
37 `voice_duck_music` — sidechain music under voice
38 `voice_remove_silence` — FFmpeg silencedetect pass
39 `voice_analyze_loudness` — report LUFS / peaks

## Manual in-app features (voice panel)
- Waveform + transcript side by side; click a word → seek; edit text → captions update.
- One-click cleanup chain button (denoise → de-esser → normalize).
- Timeline markers for silences, fillers, breaths.
- Loudness meter with target (podcast -16, online -14).
- Ducking toggle for the music track.
- STT controls: Gemini key status, Whisper fallback, "Auto" button (English).

## Implementation phases
| Phase | What | Verification |
|---|---|---|
| V1 | Canonical transcript parser (SRT/VTT/JSON) + voice preset library + FFmpeg CPU filter compiler + all 40 tools mapped to Concat commands | Local tests + Windows CI |
| V2 | Slint voice panel: transcript editor, markers, loudness meter, cleanup button, STT controls | Windows CI |
| V3 | Engine DSP: real-time waveform/LUFS, ducking/restoration nodes | Windows CI |

## CPU discipline & security
- Whisper CPU backend and Chatterbox CPU inference already in `concat-speech`.
- V1 processing uses FFmpeg filters (`arnndn`, `acompressor`, `loudnorm`,
  `silencedetect`, `highpass`, `deesser`, `aecho`) — no GPU.
- Gemini is an optional cloud call (English only, key from env/config). Somali
  never needs a network or key. Keys are never committed.

## Verification standard
Nothing is called "Done" until verified. Source-only features are never claimed
as live in the installed binary. No Windows installer/exe is built until studio
parity is explicitly approved.
