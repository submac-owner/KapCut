#!/usr/bin/env python3
"""Local CPU-only checks for the transcript-first voice engine data + wiring."""
import json, pathlib, sys

HERE = pathlib.Path(__file__).resolve().parent
ROOT = HERE.parents[1]

def load(p):
    return json.loads(p.read_text(encoding="utf-8"))

def main():
    presets = load(HERE / "presets.json")
    catalog = load(ROOT / "kapcut" / "catalog.json")
    voice = [t for t in catalog["tools"] if t["family"] == "voice"]

    # canonical model contract
    assert presets["version"] == 1
    assert "so" in presets["fillers"] and "en" in presets["fillers"], "Somali + English filler lists"
    assert presets["default_provider"] == "gemini", "Gemini is top priority for English"
    assert presets["lufs_targets"]["podcast"] == -16.0
    assert "somali_voice" in presets["eq_presets"], "Somali voice EQ curve"

    # 40 tools, right backends, no duplicates
    names = [t["name"] for t in voice]
    assert len(voice) == 40, len(voice)
    assert len(set(names)) == 40, "duplicate voice tool names"
    for t in voice:
        assert t["target"] == "capcut_voice", t["name"]
        assert t["target_kind"] in ("voice_query", "voice_import", "voice_edit"), t["name"]

    # transcript-first essentials present
    for name in (
        "voice_transcript_import", "voice_transcript_get", "voice_transcript_search",
        "voice_transcript_context", "voice_cut_fillers", "voice_cut_pauses",
        "voice_transcribe_auto", "voice_stt_status", "voice_mark_markers",
    ):
        assert name in names, name

    # generated Rust module is present and balanced
    rs = ROOT / "src" / "crates" / "concat-mcp" / "src" / "tools" / "capcut_voice.rs"
    text = rs.read_text(encoding="utf-8")
    assert "pub fn parse_transcript" in text
    assert "pub fn cut_ranges" in text
    assert "pub fn edit_command" in text
    # char literals like '{' and format strings contain braces that are not
    # Rust block delimiters; strip those before counting.
    import re
    stripped = re.sub(r'r##".*?"##', "", text, flags=re.S)
    stripped = re.sub(r'"(?:\\.|[^"\\])*"', "", stripped)
    stripped = re.sub(r"'(?:\\.|[^'\\])'", "", stripped)
    stripped = re.sub(r"//.*", "", stripped)
    assert stripped.count("{") == stripped.count("}"), "unbalanced braces"

    print(f"voice tests: OK ({len(voice)} tools, {len(catalog['tools'])} total, "
          f"{len(presets['fillers']['so'])} Somali + {len(presets['fillers']['en'])} English fillers)")

if __name__ == "__main__":
    main()
