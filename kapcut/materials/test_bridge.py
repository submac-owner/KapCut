#!/usr/bin/env python3
"""Tests for the CapCut -> Concat material bridge."""
import json, pathlib, sys
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
import bridge

reg = {p["id"]: {x["key"] for x in p["params"]} for p in bridge.REGISTRY["packages"]}

def check(cond, msg):
    if not cond:
        raise AssertionError(msg)

# adjust mapping
c = bridge.map_adjust("contrast", -0.9147, "clip1")
check(c and c["op"] == "setEffectKey", "contrast -> setEffectKey")
check(c["key"] == "contrast" and c["value"] == -91.47, f"contrast scale: {c}")
check("contrast" in reg["concat.adjust"], "concat.adjust has contrast")

c = bridge.map_adjust("temperature", 0.5, "clip1")
check(c["key"] == "temperature" and c["value"] == 8750.0, f"kelvin: {c}")

c = bridge.map_adjust("black_white", 0.8, "clip1")
check(c["key"] == "saturation" and c["value"] == -100.0, f"invert: {c}")

check(bridge.map_adjust("not_a_real_effect", 1, "x") is None, "unknown adjust rejected")

# effect mapping
cmds = bridge.map_effect("Chroma Stretch", 0, 20.2,
                         [{"name": "effects_adjust_speed", "value": 0.5}], "Chroma Stretch")
check(len(cmds) >= 1, f"effect commands: {cmds}")
check(cmds[0]["effectId"] == "concat.chroma-key", cmds[0])
# effects_adjust_speed is skipped because concat.chroma-key has no `speed` param
check(all(c["key"] in reg["concat.chroma-key"] for c in cmds if "key" in c), cmds)

check(bridge.map_effect("Totally Unknown FX", 0, 1) == [], "unknown effect -> no commands")

# full draft translation
draft_path = pathlib.Path(r"C:\Users\Admin\Desktop\Capcut\notes\capcut-draft-1118-v2.json")
if draft_path.exists():
    d = json.loads(draft_path.read_text(encoding="utf-8"))
    out = bridge.translate_draft(d)
    check(out["op"] == "batch", "batch")
    check(len(out["commands"]) >= 4, "real draft produced commands")

print("bridge tests: OK")
