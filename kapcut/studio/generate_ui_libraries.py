#!/usr/bin/env python3
"""Emit ui_libraries.rs: id/name pairs for the Studio Library GUI picker.

Source of truth: the same JSON libraries the MCP tools use, so the GUI and the
agent offer identical presets. Data only; no build needed here.
"""
import json, pathlib

HERE = pathlib.Path(__file__).resolve().parent
OUT = pathlib.Path(__file__).resolve().parents[2] / "src/crates/concat/src/ui_libraries.rs"

def load(name, key):
    return json.loads((HERE / name).read_text(encoding="utf-8"))[key]

frames = load("overlay_frames.json", "frames")
anims = load("overlay_animations.json", "animations")
curves = load("curves.json", "presets")
rigs = load("three_d.json", "camera_rigs")
lights = load("three_d.json", "light_rigs")
particles = load("three_d.json", "particle_fields")
composite = load("three_d.json", "composite_modes")
trackers = load("tracking.json", "trackers")
roto = load("tracking.json", "roto_shapes")

def pairs(items):
    out = []
    for item in items:
        i = item.get("id", "").replace('"', '\\"')
        n = item.get("name", item.get("id", "")).replace('"', '\\"')
        out.append(f'    ("{i}", "{n}"),')
    return "\n".join(out)

lines = [
    "// GENERATED FILE - do not edit. Run: python3 kapcut/studio/generate_ui_libraries.py",
    "//! Preset names and ids for the Studio Library dialog. The same data drives",
    "//! the MCP tools, so what the GUI shows is what the agent can apply.",
    "",
]
groups = [
    ("FRAMES", frames), ("ANIMATIONS", anims), ("CURVES", curves),
    ("CAMERA_RIGS", rigs), ("LIGHT_RIGS", lights), ("PARTICLES", particles),
    ("COMPOSITE", composite), ("TRACKERS", trackers), ("ROTO", roto),
]
for name, items in groups:
    lines += [f"pub static {name}: &[(&str, &str)] = &[", pairs(items), "];", ""]
lines += ["pub fn count(name: &str) -> usize {", "    match name {"]
for name, items in groups:
    lines.append(f'        "{name.lower()}" => {name}.len(),')
lines += ["        _ => 0,", "    }", "}", ""]

OUT.write_text("\n".join(lines), encoding="utf-8")
print(f"wrote {OUT} ({sum(len(i) for _, i in groups)} presets)")
