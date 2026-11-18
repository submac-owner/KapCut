#!/usr/bin/env python3
import json, pathlib, sys
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
import compile as motion

def check(c, m):
    if not c: raise AssertionError(m)

ps = motion.list_presets()
check(len(ps) >= 10, f"preset count {len(ps)}")
check(any(p["kind"] == "loop" for p in ps) and any(p["kind"] == "in" for p in ps) and any(p["kind"] == "out" for p in ps), "kinds")

b = motion.compile_preset("slice_rebuild", "clip1", 10.0)
check(b["op"] == "batch" and len(b["commands"]) >= 6, b)
check(all(c["op"] == "setClipKey" and 0 <= c["at"] <= 1 for c in b["commands"]), b)
check(all(c["ease"] in motion.EASES for c in b["commands"]), b)

loop = motion.compile_preset("popping_candy", "clip1", 20.0, loops=3)
scale_keys = [c for c in loop["commands"] if c["property"] == "scale"]
check(len(scale_keys) == 9, f"loop keys: {len(scale_keys)}")

neon = motion.compile_preset("neon_pulse", "clip1", 10.0)
check(any(c["op"] == "addLayerClip" and c["effectId"] == "concat.neon" for c in neon["commands"]), neon)

graph = motion.compile_ffmpeg("ken_burns", start=5)
check("zoompan" in graph and "st=" not in graph, graph)
graph2 = motion.compile_ffmpeg("slice_rebuild", start=2)
check("st=2" in graph2, graph2)

print(f"motion compiler tests: OK ({len(ps)} presets)")
