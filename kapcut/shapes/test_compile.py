#!/usr/bin/env python3
import pathlib, sys
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
import compile as shapes

def check(c, m):
    if not c: raise AssertionError(m)

xs = shapes.list_shapes()
check(len(xs) >= 10, f"count {len(xs)}")

ring = shapes.compile_shape("neon_ring", 2.0, 4.0)
check(len(ring["commands"]) >= 3, ring)
check(ring["commands"][0]["op"] == "addShapeClip", ring)
check(ring["commands"][0]["style"]["kind"] == "Circle", ring)
check(any(c.get("property") == "rotation" for c in ring["commands"]), ring)

arc = shapes.compile_shape("progress_arc", 0, 3.0, steps=4)
clips = [c for c in arc["commands"] if c["op"] == "addShapeClip"]
check(len(clips) == 4, arc)
check(clips[-1]["style"]["trimEnd"] > clips[0]["style"]["trimEnd"], arc)

dots = shapes.compile_shape("orbit_dots", 0, 5.0)
check(len([c for c in dots["commands"] if c["op"] == "addShapeClip"]) == 8, dots)

print(f"shape compiler tests: OK ({len(xs)} shapes)")
