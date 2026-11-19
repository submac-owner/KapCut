// GENERATED FILE - do not edit. Run: python3 kapcut/voice/generate_voice_rs.py
//! Transcript-first voice engine. The canonical transcript is the single
//! source of truth: every edit decision is read from text, never from audio.
use serde_json::{json, Value};
use std::fs;
use std::path::PathBuf;

const PRESETS_JSON: &str = r##"{"version":1,"fillers":{"en":["um","uh","erm","er","ah","mm","hmm","like","you know","basically","actually","sort of","kind of"],"so":["hmm","hm","mm","ah","ee","oo","waa hagaag","marka"]},"breaths":["*breath*","[breath]","(breath)","*inhale*","[inhale]","(sigh)"],"pause_threshold_seconds":0.65,"lufs_targets":{"podcast":-16.0,"online_video":-14.0,"broadcast":-23.0,"streaming":-18.0},"cleanup_chains":{"voice_standard":[{"id":"denoise","params":{"amount":0.5}},{"id":"deesser","params":{"amount":0.4}},{"id":"equalizer","params":{"lowCut":80.0,"presence":2.0}},{"id":"compressor","params":{"threshold":-20.0,"ratio":3.0}},{"id":"loudnorm","params":{"target":-16.0}}],"voice_podcast":[{"id":"denoise","params":{"amount":0.6}},{"id":"deesser","params":{"amount":0.5}},{"id":"compressor","params":{"threshold":-18.0,"ratio":4.0}},{"id":"loudnorm","params":{"target":-16.0}}],"voice_light":[{"id":"denoise","params":{"amount":0.3}},{"id":"loudnorm","params":{"target":-14.0}}],"voice_restore":[{"id":"denoise","params":{"amount":0.8}},{"id":"equalizer","params":{"lowCut":60.0,"presence":3.0}},{"id":"deesser","params":{"amount":0.6}},{"id":"compressor","params":{"threshold":-22.0,"ratio":2.5}},{"id":"loudnorm","params":{"target":-18.0}}]},"eq_presets":{"warm":{"lowCut":80.0,"low":1.5,"mid":-1.0,"presence":2.5,"air":1.0},"bright":{"lowCut":100.0,"low":-1.0,"mid":0.0,"presence":3.5,"air":2.0},"podcast":{"lowCut":70.0,"low":2.0,"mid":-1.5,"presence":2.0,"air":0.5},"telephone":{"lowCut":300.0,"highCut":3400.0,"mid":1.0},"somali_voice":{"lowCut":75.0,"low":1.0,"mid":-0.5,"presence":3.0,"air":1.5}},"compressor_presets":{"gentle":{"threshold":-24.0,"ratio":2.0,"attack":20.0,"release":150.0},"podcast":{"threshold":-18.0,"ratio":4.0,"attack":10.0,"release":120.0},"aggressive":{"threshold":-14.0,"ratio":8.0,"attack":5.0,"release":90.0}},"reverb_presets":{"room":{"size":0.3,"decay":1.2,"mix":0.15},"studio":{"size":0.15,"decay":0.7,"mix":0.08},"hall":{"size":0.75,"decay":3.0,"mix":0.3}},"stt_providers":["gemini","whisper","auto"],"default_provider":"gemini","default_language":"so"}"##;

fn presets() -> Value { serde_json::from_str(PRESETS_JSON).unwrap_or(json!({})) }
fn s(v: &Value, k: &str) -> String { v.get(k).and_then(Value::as_str).unwrap_or("").to_string() }
fn n(v: &Value, k: &str, d: f64) -> f64 { v.get(k).and_then(Value::as_f64).unwrap_or(d) }
fn b(v: &Value, k: &str) -> bool { v.get(k).and_then(Value::as_bool).unwrap_or(false) }
fn arr(v: &Value, k: &str) -> Vec<Value> { v.get(k).and_then(Value::as_array).cloned().unwrap_or_default() }
fn vdir(path: &str) -> PathBuf { PathBuf::from(path).join("voice") }
fn file(path: &str, name: &str) -> PathBuf { vdir(path).join(name) }

fn write(path: &PathBuf, text: &str) -> Result<(), String> {
    if let Some(p) = path.parent() { fs::create_dir_all(p).map_err(|e| e.to_string())?; }
    fs::write(path, text).map_err(|e| e.to_string())
}

// ---------- canonical transcript ----------
fn empty_transcript() -> Value {
    json!({ "version": 1, "language": "", "words": [], "segments": [] })
}
pub fn read_transcript(path: &str) -> Value {
    fs::read_to_string(file(path, "transcript.json")).ok()
        .and_then(|t| serde_json::from_str(&t).ok()).unwrap_or_else(empty_transcript)
}
fn canonical(segments: Vec<Value>, language: &str) -> Value {
    let mut words = Vec::new();
    for seg in &segments {
        let text = s(seg, "text");
        let a = n(seg, "start", 0.0);
        let z = n(seg, "end", a);
        let toks: Vec<&str> = text.split_whitespace().collect();
        if toks.is_empty() { continue; }
        let step = (z - a) / toks.len() as f64;
        for (i, w) in toks.iter().enumerate() {
            words.push(json!({
                "text": w,
                "start": a + step * (i as f64),
                "end": a + step * ((i + 1) as f64),
                "confidence": n(seg, "confidence", 1.0),
                "speaker": s(seg, "speaker"),
                "language": s(seg, "language")
            }));
        }
    }
    json!({ "version": 1, "language": language, "words": words, "segments": segments })
}

// time "HH:MM:SS,mmm" / "HH:MM:SS.mmm" / "MM:SS.mmm" -> seconds
fn time(s_in: &str) -> f64 {
    let t = s_in.trim().replace(',', ".");
    let p: Vec<&str> = t.split(':').collect();
    match p.len() {
        3 => p[0].parse::<f64>().unwrap_or(0.0) * 3600.0
           + p[1].parse::<f64>().unwrap_or(0.0) * 60.0
           + p[2].parse::<f64>().unwrap_or(0.0),
        2 => p[0].parse::<f64>().unwrap_or(0.0) * 60.0
           + p[1].parse::<f64>().unwrap_or(0.0),
        _ => t.parse::<f64>().unwrap_or(0.0),
    }
}
fn stamp(seconds: f64) -> String {
    let ms = (seconds * 1000.0).round() as i64;
    let h = ms / 3_600_000; let m = (ms / 60_000) % 60;
    let sec = (ms / 1000) % 60; let milli = ms % 1000;
    format!("{:02}:{:02}:{:02},{:03}", h, m, sec, milli)
}
fn parse_srt(text: &str, language: &str) -> Value {
    let mut segs = Vec::new();
    for block in text.split("\n\n") {
        let lines: Vec<&str> = block.lines().filter(|l| !l.trim().is_empty()).collect();
        let ti = match lines.iter().position(|l| l.contains("-->")) { Some(i) => i, None => continue };
        let ends: Vec<&str> = lines[ti].split("-->").collect();
        if ends.len() < 2 { continue; }
        let a = time(ends[0]);
        let z = time(ends[1].split_whitespace().next().unwrap_or(""));
        let body = lines[ti + 1..].join(" ");
        if body.is_empty() { continue; }
        segs.push(json!({ "text": body, "start": a, "end": z,
                          "speaker": "", "language": language, "confidence": 1.0, "summary": "" }));
    }
    canonical(segs, language)
}
fn parse_vtt(text: &str, language: &str) -> Value {
    parse_srt(&text.replace("WEBVTT", ""), language)
}
fn parse_json_input(text: &str, language: &str) -> Value {
    let v: Value = serde_json::from_str(text).unwrap_or(json!({}));
    let lang = if s(&v, "language").is_empty() { language.to_string() } else { s(&v, "language") };
    if let Some(segs) = v.get("segments").and_then(Value::as_array) {
        let mut out = Vec::new();
        for sg in segs {
            let mut item = sg.clone();
            item["text"] = json!(s(sg, "text"));
            item["start"] = json!(n(sg, "start", 0.0));
            item["end"] = json!(n(sg, "end", 0.0));
            item["language"] = json!(if s(sg, "language").is_empty() { lang.clone() } else { s(sg, "language") });
            if let Some(ws) = sg.get("words").and_then(Value::as_array) {
                let mut words = Vec::new();
                for w in ws {
                    let txt = if s(w, "text").is_empty() { s(w, "word") } else { s(w, "text") };
                    words.push(json!({ "text": txt.trim(),
                        "start": n(w, "start", 0.0), "end": n(w, "end", 0.0),
                        "confidence": n(w, "confidence", 1.0),
                        "speaker": s(w, "speaker"), "language": s(&item, "language") }));
                }
                item["words"] = json!(words);
            }
            out.push(item);
        }
        let has_words = out.iter().any(|x| x.get("words").is_some());
        if has_words {
            let mut words = Vec::new();
            for sg in &out {
                if let Some(ws) = sg.get("words").and_then(Value::as_array) {
                    for w in ws { words.push(w.clone()); }
                }
            }
            if !words.is_empty() {
                return json!({ "version": 1, "language": lang, "words": words, "segments": out });
            }
        }
        return canonical(out, &lang);
    }
    if let Some(ws) = v.get("words").and_then(Value::as_array) {
        return json!({ "version": 1, "language": lang, "words": ws.clone(), "segments": [] });
    }
    empty_transcript()
}
pub fn parse_transcript(text: &str, format: &str, language: &str) -> Value {
    match format {
        "srt" => parse_srt(text, language),
        "vtt" | "webvtt" => parse_vtt(text, language),
        "json" => parse_json_input(text, language),
        _ => if text.trim_start().starts_with('{') { parse_json_input(text, language) }
             else if text.contains("-->") { parse_vtt(text, language) }
             else { parse_srt(text, language) },
    }
}

// ---------- text derived from the transcript ----------
fn words_array(t: &Value) -> Vec<Value> {
    let ws = arr(t, "words");
    if !ws.is_empty() { return ws; }
    let mut out = Vec::new();
    for sg in arr(t, "segments") {
        let text = s(&sg, "text"); let a = n(&sg, "start", 0.0); let z = n(&sg, "end", a);
        let toks: Vec<&str> = text.split_whitespace().collect();
        if toks.is_empty() { continue; }
        let step = (z - a) / toks.len() as f64;
        for (i, w) in toks.iter().enumerate() {
            out.push(json!({ "text": w, "start": a + step * i as f64,
                "end": a + step * (i + 1) as f64, "speaker": s(&sg, "speaker"),
                "language": s(&sg, "language"), "confidence": 1.0 }));
        }
    }
    out
}
fn filler_list(lang: &str) -> Vec<String> {
    let p = presets();
    let f = p.get("fillers").and_then(Value::as_object);
    f.and_then(|m| m.get(lang)).and_then(Value::as_array)
        .map(|a| a.iter().filter_map(Value::as_str).map(|x| x.to_string()).collect())
        .unwrap_or_default()
}
fn is_filler(word: &str, list: &[String]) -> bool {
    let w = word.trim_matches(|c: char| !c.is_alphanumeric() && c != '\'').to_lowercase();
    list.iter().any(|f| f.to_lowercase() == w)
}
fn is_breath(word: &str) -> bool {
    let p = presets();
    arr(&p, "breaths").iter().filter_map(Value::as_str).any(|x| word.to_lowercase().contains(x))
}

fn filler_ranges(t: &Value, lang: &str) -> Vec<Value> {
    let list = filler_list(lang);
    let mut out = Vec::new();
    for w in words_array(t) {
        if is_filler(&s(&w, "text"), &list) {
            out.push(json!({ "start": n(&w, "start", 0.0), "end": n(&w, "end", 0.0), "reason": "filler" }));
        }
    }
    out
}
fn breath_ranges(t: &Value) -> Vec<Value> {
    words_array(t).iter().filter(|w| is_breath(&s(w, "text")))
        .map(|w| json!({ "start": n(w, "start", 0.0), "end": n(w, "end", 0.0), "reason": "breath" }))
        .collect()
}
fn pause_ranges(t: &Value, threshold: f64) -> Vec<Value> {
    let ws = words_array(t);
    let mut out = Vec::new();
    for pair in ws.windows(2) {
        let gap = n(&pair[1], "start", 0.0) - n(&pair[0], "end", 0.0);
        if gap >= threshold {
            out.push(json!({ "start": n(&pair[0], "end", 0.0),
                             "end": n(&pair[1], "start", 0.0), "reason": "pause" }));
        }
    }
    out
}
fn phrase_range(t: &Value, query: &str) -> Option<Value> {
    let ws = words_array(t);
    let q = query.to_lowercase();
    let first = ws.iter().position(|w| s(w, "text").to_lowercase().contains(&q))?;
    let last = ws.iter().rposition(|w| s(w, "text").to_lowercase().contains(&q))?;
    Some(json!({ "start": n(&ws[first], "start", 0.0),
                 "end": n(&ws[last.max(first)], "end", 0.0), "reason": "phrase",
                 "text": query }))
}

fn context_brief(t: &Value) -> String {
    let mut md = String::new();
    md.push_str("# Voice context\n\n");
    md.push_str(&format!("Language: {}\n", s(t, "language")));
    let ws = words_array(t);
    let dur = ws.last().map(|w| n(w, "end", 0.0)).unwrap_or(0.0);
    md.push_str(&format!("Duration: {:.2}s\nWords: {}\n\n", dur, ws.len()));
    md.push_str("## Full transcript with timestamps\n\n");
    for sg in arr(t, "segments") {
        md.push_str(&format!("[{} --> {}] {}\n", stamp(n(&sg, "start", 0.0)), stamp(n(&sg, "end", 0.0)), s(&sg, "text")));
    }
    if arr(t, "segments").is_empty() {
        for w in &ws {
            md.push_str(&format!("[{}] {}\n", stamp(n(w, "start", 0.0)), s(w, "text")));
        }
    }
    let lang = if s(t, "language").is_empty() { "en".to_string() } else { s(t, "language") };
    let fl = filler_ranges(t, &lang);
    md.push_str(&format!("\nFiller words flagged: {}\n", fl.len()));
    md.push_str(&format!("Breath markers flagged: {}\n", breath_ranges(t).len()));
    md.push_str(&format!("Pauses over threshold: {}\n", pause_ranges(t, n(&presets(), "pause_threshold_seconds", 0.65)).len()));
    md
}
fn subtitle(t: &Value, vtt: bool) -> String {
    let mut out = String::new();
    if vtt { out.push_str("WEBVTT\n\n"); }
    for (i, sg) in arr(t, "segments").iter().enumerate() {
        let a = n(sg, "start", 0.0); let z = n(sg, "end", 0.0);
        let (fa, fz) = if vtt { (stamp(a).replace(',', "."), stamp(z).replace(',', ".")) }
                       else { (stamp(a), stamp(z)) };
        out.push_str(&format!("{}\n{} --> {}\n{}\n\n", i + 1, fa, fz, s(sg, "text")));
    }
    out
}

// ---------- queries (read only) ----------
pub fn query(name: &str, args: &Value) -> Value {
    let t = read_transcript(&s(args, "path"));
    match name {
        "voice_transcript_get" => t,
        "voice_transcript_segments" => json!({ "language": s(&t, "language"), "segments": arr(&t, "segments"), "count": arr(&t, "segments").len() }),
        "voice_transcript_word" => {
            let q = s(args, "query");
            let found = words_array(&t).into_iter().find(|w| s(w, "text").to_lowercase() == q.to_lowercase());
            json!({ "word": found })
        }
        "voice_transcript_search" => {
            let q = s(args, "query").to_lowercase();
            let mut hits = Vec::new();
            for (i, w) in words_array(&t).iter().enumerate() {
                if s(w, "text").to_lowercase().contains(&q) {
                    hits.push(json!({ "index": i, "text": s(w, "text"), "start": n(w, "start", 0.0), "end": n(w, "end", 0.0) }));
                }
            }
            json!({ "query": q, "count": hits.len(), "matches": hits })
        }
        "voice_transcript_context" => json!({ "path": "voice/voice_context.md", "markdown": context_brief(&t) }),
        "voice_transcript_captions" => {
            let cues: Vec<Value> = arr(&t, "segments").iter().map(|sg| json!({
                "text": s(sg, "text"), "start": n(sg, "start", 0.0), "end": n(sg, "end", 0.0)
            })).collect();
            json!({ "cues": cues, "count": cues.len(), "note": "Feed these cues to captions_add." })
        }
        "voice_select_phrase" | "voice_edit_replace_phrase" => {
            let r = phrase_range(&t, &s(args, "query"));
            json!({ "name": name, "range": r, "note": "Use the returned time range for a downstream edit." })
        }
        "voice_transcribe_gemini" | "voice_transcribe_whisper" | "voice_transcribe_auto" => {
            let provider = match name {
                "voice_transcribe_gemini" => "gemini".to_string(),
                "voice_transcribe_whisper" => "whisper".to_string(),
                _ => s(args, "provider"),
            };
            if provider == "auto" || name == "voice_transcribe_auto" {
                json!({ "provider": "auto", "order": ["gemini", "whisper"],
                    "status": " Gemini first (top priority), local Whisper fallback.",
                    "next": "voice_transcript_import",
                    "note": "English only. The produced text must be imported as the canonical transcript. No key is stored in code." })
            } else if provider == "gemini" {
                json!({ "provider": "gemini",
                    "status": if std::env::var("GEMINI_API_KEY").is_ok() || std::env::var("GOOGLE_API_KEY").is_ok() { "key present in env" } else { "no key in env" },
                    "next": "voice_transcript_import",
                    "note": "Call Gemini 3.5 Transcribe with word timestamps + diarization, then import the result. Never paste the key into this tool." })
            } else {
                json!({ "provider": "whisper", "status": "local CPU backend available",
                    "next": "voice_transcript_import",
                    "note": "Run the local Whisper CPU backend, then import the resulting transcript." })
            }
        }
        "voice_detect_fillers" => {
            let lang = if s(&t, "language").is_empty() { s(args, "language") } else { s(&t, "language") };
            let r = filler_ranges(&t, &lang); json!({ "count": r.len(), "ranges": r })
        }
        "voice_detect_pauses" => {
            let th = n(args, "threshold", n(&presets(), "pause_threshold_seconds", 0.65));
            let r = pause_ranges(&t, th); json!({ "count": r.len(), "ranges": r })
        }
        "voice_analyze_loudness" => json!({ "note": "LUFS analysis requires the Concat engine loudness pass; this tool reports configured targets.", "targets": presets().get("lufs_targets").cloned().unwrap_or(json!({})) }),
        "voice_stt_status" => {
            let key = std::env::var("GEMINI_API_KEY").is_ok() || std::env::var("GOOGLE_API_KEY").is_ok();
            json!({ "gemini_key_present": key, "whisper_local": true,
                    "providers": presets().get("stt_providers").cloned().unwrap_or(json!([])),
                    "default_provider": s(&presets(), "default_provider"),
                    "note": "Gemini is top priority for English; Somali uses the supplied transcript." })
        }
        "voice_list_presets" => json!({
            "cleanup_chains": arr(&presets(), "cleanup_chains"),
            "eq_presets": presets().get("eq_presets").cloned().unwrap_or(json!({})),
            "lufs_targets": presets().get("lufs_targets").cloned().unwrap_or(json!({}))
        }),
        _ => json!({ "note": "unknown voice query", "name": name }),
    }
}

// ---------- import / file tools ----------
pub fn import(name: &str, args: &Value) -> Value {
    let path = s(args, "path");
    let t = read_transcript(&path);
    match name {
        "voice_transcript_import" => {
            let format = s(args, "format");
            let language = s(args, "language");
            let text = s(args, "text");
            let canon = parse_transcript(&text, &format, &language);
            let _ = write(&file(&path, "transcript.json"), &serde_json::to_string_pretty(&canon).unwrap_or_default());
            let _ = write(&file(&path, "transcript.srt"), &subtitle(&canon, false));
            let _ = write(&file(&path, "transcript.vtt"), &subtitle(&canon, true));
            let brief = context_brief(&canon);
            let _ = write(&file(&path, "voice_context.md"), &brief);
            json!({ "ok": true, "words": arr(&canon, "words").len(),
                    "segments": arr(&canon, "segments").len(), "language": s(&canon, "language"),
                    "files": ["voice/transcript.json", "voice/transcript.srt", "voice/transcript.vtt", "voice/voice_context.md"] })
        }
        "voice_subtitle_import" => {
            let src = s(args, "file");
            let fpath = if PathBuf::from(&src).is_absolute() { PathBuf::from(&src) } else { file(&path, &src) };
            let text = fs::read_to_string(&fpath).unwrap_or_default();
            let fmt = if src.to_lowercase().ends_with(".vtt") || text.contains("WEBVTT") { "vtt" }
                      else if src.to_lowercase().ends_with(".srt") { "srt" } else { "auto" };
            let canon = parse_transcript(&text, fmt, &s(args, "language"));
            let _ = write(&file(&path, "transcript.json"), &serde_json::to_string_pretty(&canon).unwrap_or_default());
            json!({ "ok": true, "format": fmt, "words": arr(&canon, "words").len() })
        }
        "voice_transcript_export" | "voice_subtitle_export" => {
            let vtt = b(args, "vtt");
            let txt = subtitle(&t, vtt);
            let name_f = if vtt { "export.vtt" } else { "export.srt" };
            let _ = write(&file(&path, name_f), &txt);
            json!({ "ok": true, "path": format!("voice/{}", name_f) })
        }
        "voice_transcript_context" => {
            let md = context_brief(&t);
            let _ = write(&file(&path, "voice_context.md"), &md);
            json!({ "ok": true, "path": "voice/voice_context.md", "markdown": md })
        }
        "voice_transcript_set_language" => {
            let language = s(args, "language");
            let query_txt = s(args, "query");
            let mut out = t.clone();
            let segs: Vec<Value> = arr(&t, "segments").into_iter().map(|mut sg| {
                if query_txt.is_empty() || s(&sg, "text").to_lowercase().contains(&query_txt.to_lowercase()) {
                    sg["language"] = json!(language);
                }
                sg
            }).collect();
            out["segments"] = json!(segs);
            out["language"] = json!(language);
            let _ = write(&file(&path, "transcript.json"), &serde_json::to_string_pretty(&out).unwrap_or_default());
            json!({ "ok": true, "language": language })
        }
        "voice_transcript_merge" => {
            let other_path = s(args, "otherPath");
            let other = fs::read_to_string(file(&other_path, "transcript.json")).ok()
                .and_then(|x| serde_json::from_str::<Value>(&x).ok()).unwrap_or_else(empty_transcript);
            let mut words = words_array(&t); words.extend(words_array(&other));
            words.sort_by(|a, z| n(a, "start", 0.0).partial_cmp(&n(z, "start", 0.0)).unwrap_or(std::cmp::Ordering::Equal));
            let mut segs = arr(&t, "segments"); segs.extend(arr(&other, "segments"));
            segs.sort_by(|a, z| n(a, "start", 0.0).partial_cmp(&n(z, "start", 0.0)).unwrap_or(std::cmp::Ordering::Equal));
            let merged = json!({ "version": 1, "language": "mixed", "words": words, "segments": segs });
            let _ = write(&file(&path, "transcript.json"), &serde_json::to_string_pretty(&merged).unwrap_or_default());
            json!({ "ok": true, "language": "mixed", "words": words.len(), "segments": segs.len() })
        }
        "voice_stt_set_provider" => {
            let provider = s(args, "provider");
            let cfg = json!({ "provider": provider, "note": "Gemini is top priority for English; Whisper is the CPU fallback." });
            let _ = write(&file(&path, "stt_config.json"), &serde_json::to_string_pretty(&cfg).unwrap_or_default());
            cfg
        }
        "voice_mark_markers" => {
            let kind = s(args, "kind");
            let ranges = match kind.as_str() {
                "fillers" => { let lang = if s(&t, "language").is_empty() { "en".to_string() } else { s(&t, "language") }; filler_ranges(&t, &lang) },
                "pauses" => pause_ranges(&t, n(args, "threshold", 0.65)),
                "breaths" => breath_ranges(&t),
                _ => Vec::new(),
            };
            let _ = write(&file(&path, "markers.json"), &serde_json::to_string_pretty(&json!({ "kind": kind, "markers": ranges })).unwrap_or_default());
            json!({ "ok": true, "kind": kind, "count": ranges.len(), "markers": ranges })
        }
        _ => json!({ "note": "unknown voice import", "name": name }),
    }
}

// ---------- cut planning ----------
pub fn cut_ranges(name: &str, args: &Value) -> Value {
    let t = read_transcript(&s(args, "path"));
    let lang = if s(&t, "language").is_empty() { s(args, "language") } else { s(&t, "language") };
    let mut ranges: Vec<Value> = match name {
        "voice_cut_fillers" => filler_ranges(&t, &lang),
        "voice_cut_pauses" => pause_ranges(&t, n(args, "threshold", n(&presets(), "pause_threshold_seconds", 0.65))),
        "voice_cut_breaths" => breath_ranges(&t),
        "voice_edit_delete_phrase" | "voice_ripple_delete" => {
            match phrase_range(&t, &s(args, "query")) { Some(r) => vec![r], None => vec![] }
        }
        "voice_trim_to_phrases" => {
            let keep = arr(args, "phrases");
            let mut cuts = Vec::new();
            let ws = words_array(&t);
            if let (Some(first), Some(last)) = (ws.first(), ws.last()) {
                let mut cursor = n(first, "start", 0.0);
                for phrase in &keep {
                    if let Some(r) = phrase_range(&t, &s(phrase, "query")) {
                        let a = n(&r, "start", cursor); let z = n(&r, "end", a);
                        if a > cursor { cuts.push(json!({ "start": cursor, "end": a, "reason": "trim" })); }
                        cursor = z;
                    }
                }
                let end = n(last, "end", cursor);
                if end > cursor { cuts.push(json!({ "start": cursor, "end": end, "reason": "trim" })); }
            }
            cuts
        }
        _ => vec![json!({ "start": n(args, "start", 0.0), "end": n(args, "end", 0.0), "reason": "manual" })],
    };
    ranges.sort_by(|a, z| n(a, "start", 0.0).partial_cmp(&n(z, "start", 0.0)).unwrap_or(std::cmp::Ordering::Equal));
    let ripple = name == "voice_ripple_delete" || b(args, "ripple");
    json!({ "name": name, "count": ranges.len(), "ranges": ranges, "ripple": ripple })
}

fn filter(id: &str, params: &Value) -> Value {
    json!({ "id": id, "params": params, "enabled": true })
}
// ---------- non-cut edit commands ----------
pub fn edit_command(name: &str, args: &Value) -> Value {
    let clip = s(args, "clipId");
    let preset_name = s(args, "preset");
    match name {
        "voice_denoise" => json!({ "op": "updateClip", "clipId": clip, "patch": { "filters": [filter("denoise", &json!({ "amount": n(args, "amount", 0.5) }))] } }),
        "voice_compress" => {
            let cp = presets().get("compressor_presets").and_then(Value::as_object)
                .and_then(|m| m.get(preset_name.as_str())).cloned().unwrap_or(json!({}));
            json!({ "op": "updateClip", "clipId": clip, "patch": { "filters": [filter("compressor", &cp)] } })
        }
        "voice_equalize" => {
            let eq = presets().get("eq_presets").and_then(Value::as_object)
                .and_then(|m| m.get(preset_name.as_str())).cloned().unwrap_or(json!({}));
            json!({ "op": "updateClip", "clipId": clip, "patch": { "filters": [filter("equalizer", &eq)] } })
        }
        "voice_reverb_preset" => {
            let rv = presets().get("reverb_presets").and_then(Value::as_object)
                .and_then(|m| m.get(preset_name.as_str())).cloned().unwrap_or(json!({}));
            json!({ "op": "updateClip", "clipId": clip, "patch": { "filters": [filter("reverb", &rv)] } })
        }
        "voice_phone_filter" => {
            let eq = presets().get("eq_presets").and_then(|m| m.get("telephone")).cloned().unwrap_or(json!({}));
            json!({ "op": "updateClip", "clipId": clip, "patch": { "filters": [filter("equalizer", &eq)] } })
        }
        "voice_normalize" => {
            let targets = presets().get("lufs_targets").cloned().unwrap_or(json!({}));
            let target = n(args, "lufs", n(&targets, "podcast", -16.0));
            json!({ "op": "updateClip", "clipId": clip, "patch": { "filters": [filter("loudnorm", &json!({ "target": target }))] } })
        }
        "voice_enhance" | "voice_remove_silence" => {
            let key = if name == "voice_enhance" { "voice_standard" } else { "voice_light" };
            let chain = presets().get("cleanup_chains").and_then(|m| m.get(key)).and_then(Value::as_array)
                .cloned().unwrap_or_default();
            let filters: Vec<Value> = chain.iter().map(|f| filter(&s(f, "id"), f.get("params").unwrap_or(&json!({})))).collect();
            json!({ "op": "updateClip", "clipId": clip, "patch": { "filters": filters } })
        }
        "voice_duck_music" => {
            let level = n(args, "level", 0.35);
            // Keyed gain on the music clip: duck where voice is present.
            json!({ "op": "batch", "commands": [
                json!({ "op": "setClipKey", "clipId": clip, "property": "volume", "at": 0.05, "value": level, "ease": "in" }),
                json!({ "op": "setClipKey", "clipId": clip, "property": "volume", "at": 0.95, "value": level, "ease": "out" })
            ] })
        }
        "voice_volume_automation" => {
            let mut cmds = Vec::new();
            for k in arr(args, "points") {
                cmds.push(json!({ "op": "setClipKey", "clipId": clip, "property": "volume",
                    "at": n(&k, "at", 0.0).clamp(0.0, 1.0), "value": n(&k, "value", 1.0), "ease": s(&k, "ease") }));
            }
            json!({ "op": "batch", "commands": cmds })
        }
        "voice_jcut_lcut" => {
            let mut cmds = Vec::new();
            for m in arr(args, "moves") {
                cmds.push(json!({ "op": "moveClips", "moves": [{
                    "clipId": s(&m, "clipId"), "start": n(&m, "start", 0.0), "trackId": s(&m, "trackId")
                }] }));
            }
            json!({ "op": "batch", "commands": cmds })
        }
        "voice_caption_style" | "voice_word_highlight" => {
            let mut cmds = Vec::new();
            for h in arr(args, "runs") {
                cmds.push(json!({ "op": "styleTextRange", "clipId": clip,
                    "start": n(&h, "start", 0.0) as usize, "end": n(&h, "end", 0.0) as usize,
                    "patch": h.get("patch").cloned().unwrap_or(json!({})) }));
            }
            json!({ "op": "batch", "commands": cmds })
        }
        "voice_dialogue_crossfade" => {
            let patch = json!({ "transition_in": { "kind": "crossfade", "duration": n(args, "duration", 0.08) } });
            json!({ "op": "updateClip", "clipId": clip, "patch": patch })
        }
        "voice_match_loudness" | "voice_translate_captions" | "voice_caption_translate" | "voice_captions_from_transcript" => {
            let mut cmds = Vec::new();
            for c in arr(args, "clips") {
                cmds.push(json!({ "op": "updateClip", "clipId": s(&c, "clipId"), "patch": c.get("patch").cloned().unwrap_or(json!({})) }));
            }
            json!({ "op": "batch", "commands": cmds })
        }
        _ => json!({ "op": "batch", "commands": [] }),
    }
}

pub fn is_cutting_tool(name: &str) -> bool {
    matches!(name, "voice_cut_fillers" | "voice_cut_pauses" | "voice_cut_breaths"
        | "voice_edit_delete_phrase" | "voice_ripple_delete" | "voice_trim_to_phrases"
        | "voice_split_at_word")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_srt_and_vtt_and_json() {
        let srt = "1\n00:00:01,000 --> 00:00:03,000\nHello world\n\n2\n00:00:03,500 --> 00:00:05,000\nSecond line\n";
        let t = parse_transcript(srt, "srt", "en");
        assert_eq!(arr(&t, "segments").len(), 2);
        assert_eq!(arr(&t, "words").len(), 4);
        let vtt = "WEBVTT\n\n00:00:01.000 --> 00:00:02.000\nHi there\n";
        assert_eq!(arr(&parse_transcript(vtt, "vtt", "en"), "words").len(), 2);
        let js = "{\"language\":\"so\",\"segments\":[{\"text\":\"walaal\",\"start\":0,\"end\":1}]}";
        let j = parse_transcript(js, "json", "");
        assert_eq!(s(&j, "language"), "so");
    }
    #[test]
    fn detects_fillers_pauses_and_phrases() {
        let js = r#"{"language":"en","words":[
            {"text":"Hello","start":0,"end":0.5},
            {"text":"um","start":0.5,"end":0.8},
            {"text":"world","start":2.0,"end":2.5}]}"#;
        let t = parse_transcript(js, "json", "");
        assert_eq!(filler_ranges(&t, "en").len(), 1);
        assert_eq!(pause_ranges(&t, 0.65).len(), 1);
        assert!(phrase_range(&t, "world").is_some());
        assert!(!context_brief(&t).is_empty());
    }
    #[test]
    fn compiles_edit_commands() {
        let a = json!({ "clipId": "c1", "preset": "podcast" });
        let c = edit_command("voice_compress", &a);
        assert_eq!(s(&c, "op"), "updateClip");
        assert_eq!(s(&c["patch"]["filters"][0], "id"), "compressor");
    }
}
