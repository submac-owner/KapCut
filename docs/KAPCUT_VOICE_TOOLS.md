# KapCut voice tools — agent usage guide (V1)

The transcript is the single source of truth. The agent reads all speech as
text and never needs to hear the audio.

## Canonical artifact

After import, a project has:

```
<project>/voice/transcript.json     words + segments
<project>/voice/transcript.srt
<project>/voice/transcript.vtt
<project>/voice/voice_context.md    full text + topics + filler/pause/breath markers
```

## Workflow 1 — Somali narration (user supplied transcript)

1. Ask the user for a timestamped transcript (SRT, VTT or JSON).
2. `voice_transcript_import` with:
   - `path`: project folder
   - `text`: the transcript text
   - `format`: `srt` | `vtt` | `json` (or omit to auto-detect)
   - `language`: `so`
3. `voice_transcript_get` — read the whole transcript as text.
4. `voice_transcript_context` — get the brief with filler/pause/breath markers.
5. Edit from the text:
   - `voice_cut_fillers`, `voice_cut_pauses`, `voice_cut_breaths`
   - `voice_edit_delete_phrase` with `query` + `clipIds` (ids from `timeline_read`)
   - `voice_ripple_delete` to close the gap
   - `voice_trim_to_phrases` to keep only chosen phrases
6. `voice_mark_markers` with `kind` = `fillers` | `pauses` | `breaths`.
7. `voice_enhance` / `voice_normalize` for the cleanup chain.

## Workflow 2 — English narration (Gemini first, Whisper fallback)

1. `voice_stt_status` — check whether a Gemini key is present in env.
2. `voice_transcribe_auto` — returns the hand-off plan: Gemini first (top
   priority), local Whisper fallback. Never put an API key into a tool call.
3. Run the chosen engine outside the MCP call and pass its text to
   `voice_transcript_import` with `language: en`.
4. Continue exactly as Workflow 1.

## Workflow 3 — Mixed Somali + English

1. Import the Somali transcript (`language: so`).
2. Import the English transcript into a second project folder
   (`language: en`).
3. `voice_transcript_merge` with `path` (Somali project) and `otherPath`
   (English project) — produces one `language: mixed` transcript.
4. `voice_transcript_segments` shows each segment's language tag.

## Tool reference

### Transcript core
| Tool | Key arguments |
|---|---|
| `voice_transcript_import` | `path`, `text`, `format`, `language` |
| `voice_transcript_get` | `path` |
| `voice_transcript_segments` | `path` |
| `voice_transcript_search` | `path`, `query` |
| `voice_transcript_word` | `path`, `query` |
| `voice_transcript_context` | `path` |
| `voice_transcript_set_language` | `path`, `language`, `query` (optional) |
| `voice_transcript_merge` | `path`, `otherPath` |
| `voice_transcript_export` | `path`, `vtt` (bool) |
| `voice_transcript_captions` | `path` → cues for `captions_add` |

### English STT
| Tool | Purpose |
|---|---|
| `voice_stt_status` | Which engines are usable |
| `voice_transcribe_gemini` | Gemini 3.5 Transcribe hand-off (primary) |
| `voice_transcribe_whisper` | Local CPU Whisper hand-off |
| `voice_transcribe_auto` | Gemini → Whisper fallback hand-off |
| `voice_stt_set_provider` | Persist provider choice |

### Editing
| Tool | Key arguments |
|---|---|
| `voice_edit_delete_phrase` | `path`, `query`, `clipIds`, `ripple` (optional) |
| `voice_edit_replace_phrase` | `path`, `query` → time range for a replacement |
| `voice_cut_fillers` | `path`, `clipIds`, `language` (optional) |
| `voice_cut_pauses` | `path`, `clipIds`, `threshold` (seconds) |
| `voice_cut_breaths` | `path`, `clipIds` |
| `voice_select_phrase` | `path`, `query` |
| `voice_split_at_word` | `path`, `query`, `clipIds` |
| `voice_jcut_lcut` | `path`, `moves` (clipId/start/trackId) |
| `voice_ripple_delete` | `path`, `query`, `clipIds` |
| `voice_trim_to_phrases` | `path`, `phrases` (list of `{query}`), `clipIds` |
| `voice_mark_markers` | `path`, `kind`, `threshold` (optional) |
| `voice_dialogue_crossfade` | `path`, `clipId`, `duration` |

### Cleanup / mix / analysis
| Tool | Key arguments |
|---|---|
| `voice_denoise` | `path`, `clipId`, `amount` |
| `voice_enhance` | `path`, `clipId` (standard cleanup chain) |
| `voice_normalize` | `path`, `clipId`, `lufs` |
| `voice_compress` | `path`, `clipId`, `preset` |
| `voice_equalize` | `path`, `clipId`, `preset` (incl. `somali_voice`) |
| `voice_duck_music` | `path`, `clipId`, `level` |
| `voice_remove_silence` | `path`, `clipId` |
| `voice_analyze_loudness` | reports configured LUFS targets |

## Notes
- Cut tools call `timeline_read` first to get clip ids, then pass `clipIds`.
- Audio filter ids are Concat catalogue ids; an unknown id is skipped at render.
- V1 STT tools return a hand-off plan, not a fake transcript. The engine runs
  outside the MCP call; the result is imported as the canonical transcript.
- Source-only until a Windows build is installed; no installer is built yet.
