#!/usr/bin/env python3
import pathlib, sys
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
import overlay_compile as oc

f = oc.list_frames(); a = oc.list_animations()
assert len(f) >= 50, len(f)
assert len(a) >= 50, len(a)
phases = {x["phase"] for x in a}
assert phases == {"in", "out", "loop"}, phases
r = oc.add_frame("frame_paper_original", 1.0, 5.0, "anim_fade_in")
assert len(r["commands"]) >= 4, len(r["commands"])
assert any(c["op"] == "addShapeClip" for c in r["commands"])
assert any(c["op"] == "setClipKey" for c in r["commands"])
assert all(0 <= c["at"] <= 1 for c in r["commands"] if c["op"] == "setClipKey")
r2 = oc.apply_animation("anim_spin_loop", "c1", 0, 4)
assert len(r2["commands"]) >= 2
print(f"overlay tests: OK ({len(f)} frames, {len(a)} animations)")
