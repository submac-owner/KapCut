#!/usr/bin/env python3
import pathlib, sys
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
import animate_text as at

as_ = at.list_animators()
assert len(as_) >= 6
r = at.animate_text("MOTION DESIGN STUDIO", "burst", start=1, duration=6)
words = [c for c in r["commands"] if c["op"] == "addTextClip"]
assert len(words) == 3, len(words)
keys = [c for c in r["commands"] if c["op"] == "setClipKey"]
assert len(keys) == 9, len(keys)
assert all("ease" in c for c in keys)
print(f"text animator tests: OK ({len(as_)} animators)")
