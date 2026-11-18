#!/usr/bin/env python3
import pathlib, sys
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
import compile as adv

def check(c, m):
    if not c: raise AssertionError(m)

xs = adv.list_presets()
check(len(xs) >= 12, len(xs))
g = adv.compile_advanced("grade_noir", "c1")
check(any(c["key"] == "saturation" and c["value"] == -100 for c in g["commands"]), g)
check(any(c.get("effectId") == "concat.noir" for c in g["commands"]), g)
cam = adv.compile_advanced("cam_push_in", "c1", duration=4)
check(any(c.get("property") == "scale" for c in cam["commands"]), cam)
print(f"advanced compiler tests: OK ({len(xs)} presets)")
