#!/usr/bin/env python3
import pathlib, sys
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
import editing_style_engine as es

p = es.derive_plan("fast retention hormozi style with every word captions and whooshes")
assert p["pace"] == "fast" and p["text"] == "every-word" and p["sound"] == "designed", p
assert p["structure"] == "hook-payoff-cta", p
p2 = es.derive_plan("slow cinematic video essay with lower thirds", {"style_profile": {"pace": "slow", "median_shot": 8.0, "mean_rgb": [90, 30, 30]}})
assert p2["pace"] == "slow" and p2["shot_seconds"] == 8.0 and p2["text"] == "lower-thirds", p2
assert p2["reference_driven"], p2
r = es.compile_plan("vintage film with lower thirds and whooshes", title="FILM", duration=12)
assert len(r["commands"]) > 10, len(r["commands"])
assert any(c.get("effectId") == "concat.vintage" for c in r["commands"]), r["plan"]
assert any(c.get("op") == "addTransitionClip" for c in r["commands"])
print(f"editing style engine tests: OK ({len(r['commands'])} commands)")
