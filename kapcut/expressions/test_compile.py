#!/usr/bin/env python3
import json, pathlib, sys
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
import compile as expr

def check(c, m):
    if not c: raise AssertionError(m)

xs = expr.list_expressions()
check(len(xs) >= 8, f"count {len(xs)}")

w = expr.compile_expression("wiggle", "c1", 5.0, "rotation", {"amplitude": 0.2})
check(len(w["commands"]) == 32, w)
check(all(-0.2 <= c["value"] <= 0.2 for c in w["commands"]), w)

# deterministic
w2 = expr.compile_expression("wiggle", "c1", 5.0, "rotation", {"amplitude": 0.2})
check([c["value"] for c in w["commands"]] == [c["value"] for c in w2["commands"]], "not deterministic")

s = expr.compile_expression("sine", "c1", 4.0, "opacity", {"amplitude": 0.5})
check(any(c["value"] > 0.4 for c in s["commands"]) and any(c["value"] < -0.4 for c in s["commands"]), s)

d = expr.compile_expression("damped_sine", "c1", 3.0)
check(abs(d["commands"][-1]["value"]) < 0.05, d["commands"][-1])

p = expr.compile_expression("pulse", "c1", 2.0)
check(set(c["value"] for c in p["commands"]) <= {0.0, 1.0}, p)

lp = expr.compile_expression("loop_out", "c1", 4.0, "offsetX", {"mode": "pingpong"})
check(all(0 <= c["value"] <= 1 for c in lp["commands"]), lp)

path = expr.compile_expression("follow_path", "c1", 5.0)
check(len(path["commands"]) == 80, len(path["commands"]))
check({c["property"] for c in path["commands"]} == {"offsetX", "offsetY"}, path)

print(f"expression compiler tests: OK ({len(xs)} expressions)")
