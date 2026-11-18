#!/usr/bin/env python3
"""Build the Concat package registry and the CapCut -> Concat material bridge.

Reads Concat's real effect.toml packages; validates every mapping against them.
Outputs concat-packages.json and bridge.json in this folder.
"""
import json, pathlib, tomllib

ROOT = pathlib.Path(__file__).resolve().parents[2]
PKGS = ROOT / "src" / "crates" / "concat-effects" / "packages"
OUT = pathlib.Path(__file__).resolve().parent

registry = []
for toml in sorted(PKGS.glob("*/effect.toml")):
    d = tomllib.loads(toml.read_text(encoding="utf-8"))
    e = d.get("effect", {})
    params = []
    for p in d.get("param", []):
        params.append({
            "key": p.get("key"), "group": p.get("group"), "label": p.get("label"),
            "type": p.get("type", "number"), "min": p.get("min"),
            "max": p.get("max"), "default": p.get("default"), "unit": p.get("unit"),
            "animate": p.get("animate", False),
        })
    registry.append({
        "id": e.get("id"), "name": e.get("name"), "kind": e.get("kind"),
        "category": e.get("category"), "description": e.get("description"),
        "params": params,
    })

available = {p["id"]: p for p in registry}

# CapCut Adjust types (materials.effects[].type) -> concat.adjust parameter.
# CapCut values are normalized floats, usually -1..1 or 0..1; scale to the
# Concat parameter's range. "kelvin" means CapCut's temperature maps to 2000..11000.
ADJUST_MAP = {
    "contrast":        ("concat.adjust", "contrast", 100),
    "saturation":      ("concat.adjust", "saturation", 100),
    "sharpen":         ("concat.adjust", "sharpen", 100),
    "highlight":       ("concat.adjust", "highlights", 100),
    "shadow":          ("concat.adjust", "shadows", 100),
    "brightness":      ("concat.adjust", "brightness", 100),
    "temperature":     ("concat.adjust", "temperature", "kelvin"),
    "tone":            ("concat.adjust", "temperature", "kelvin"),
    "hsl":             ("concat.adjust", "saturation", 100),
    "vignetting":      ("concat.adjust", "vignette", 100),
    "fade":            ("concat.adjust", "fade", 100),
    "clear":           ("concat.adjust", "sharpen", 100),
    "light_sensation": ("concat.adjust", "exposure", 3),
    "black_white":     ("concat.adjust", "saturation", -100),
}

# CapCut effect names (case-insensitive substring) -> Concat package id.
EFFECT_MAP = {
    "chroma": "concat.chroma-key",
    "green screen": "concat.chroma-key",
    "green_screen": "concat.chroma-key",
    "luma key": "concat.luma-key",
    "remove black": "concat.remove-black",
    "glow": "concat.glow",
    "blur": "concat.gaussian-blur",
    "box blur": "concat.gaussian-blur",
    "zoom blur": "concat.zoom-blur",
    "motion blur": "concat.motion-blur",
    "vhs": "concat.vhs",
    "sepia": "concat.vintage",
    "neon": "concat.neon",
    "film grain": "concat.film-grain",
    "shake": "concat.shake",
    "beat shake": "concat.beat-shake",
    "swirl": "concat.swirl",
    "fisheye": "concat.fisheye",
    "echo": "concat.echo",
    "dissolve": "concat.dissolve",
    "bloom": "concat.bloom-pulse",
    "dust": "concat.falling-dust",
    "after hours": "concat.after-hours",
    "aquamarine": "concat.aquamarine",
    "autumn": "concat.autumn",
    "blush": "concat.blush",
    "impact": "concat.impact",
    "color wheels": "concat.color-wheels",
    "contrast": "concat.contrast",
    "sharpen": "concat.sharpen",
    "vignette": "concat.vignette",
    "saturation": "concat.saturation",
    "exposure": "concat.exposure",
    "denoise": "concat.denoise",
    "noise reduction": "concat.denoise",
    "robot": "concat.robot",
    "chipmunk": "concat.chipmunk",
    "telephone": "concat.telephone",
    "megaphone": "concat.megaphone",
    "enhance voice": "concat.enhance-voice",
    "loudness": "concat.loudness",
    "clock wipe": "concat.clock-wipe",
    "linear wipe": "concat.linear-wipe",
    "shape wipe": "concat.shape-wipe",
    "rgb split wipe": "concat.rgb-split-wipe",
    "iris": "concat.iris",
    "slide": "concat.slide",
    "push": "concat.push",
    "cube": "concat.cube",
    "fade black": "concat.fade-black",
    "fade white": "concat.fade-white",
    "glitch": "concat.glitch-warp",
    "pixelate": "concat.pixelate",
    "halftone": "concat.halftone",
    "posterize": "concat.posterize",
    "chromatic aberration": "concat.chromatic-aberration",
    "halation": "concat.halation",
    "light leak": "concat.light-leak-flash",
}

# CapCut effect adjust keys -> Concat parameter name (when package supports it).
EFFECT_PARAM_MAP = {
    "effects_adjust_speed": "speed",
    "effects_adjust_intensity": "intensity",
}

errors = []
for key, (pkg, param, _) in ADJUST_MAP.items():
    if pkg not in available:
        errors.append(f"adjust {key}: missing package {pkg}")
    elif param not in {p["key"] for p in available[pkg]["params"]}:
        errors.append(f"adjust {key}: missing param {pkg}.{param}")
for key, pkg in EFFECT_MAP.items():
    if pkg not in available:
        errors.append(f"effect {key}: missing package {pkg}")

(OUT / "concat-packages.json").write_text(
    json.dumps({"count": len(registry), "packages": registry}, indent=2), encoding="utf-8")
(OUT / "bridge.json").write_text(json.dumps({
    "schemaVersion": 1,
    "adjustMap": ADJUST_MAP,
    "effectMap": EFFECT_MAP,
    "effectParamMap": EFFECT_PARAM_MAP,
}, indent=2), encoding="utf-8")

print(f"packages={len(registry)} adjust_rules={len(ADJUST_MAP)} effect_rules={len(EFFECT_MAP)}")
if errors:
    print("FAIL")
    for e in errors: print("  -", e)
    raise SystemExit(1)
print("OK")
