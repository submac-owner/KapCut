#!/usr/bin/env python3
"""CPU-first expression compiler: sample AE-style expressions into real
Concat keyframe commands. No JS engine, no GPU, deterministic output.
"""
import json, math, pathlib

HERE = pathlib.Path(__file__).resolve().parent
DATA = json.loads((HERE / "expressions.json").read_text(encoding="utf-8"))
EXPRS = {e["id"]: e for e in DATA["expressions"]}
PROPERTIES = {"scale", "offsetX", "offsetY", "rotation", "stretchX", "stretchY", "opacity", "volume"}
EASES = {"linear", "in", "out", "inOut"}


def list_expressions():
    return [dict(id=e["id"], name=e["name"], category=e["category"], description=e["description"])
            for e in EXPRS.values()]


def _rng(seed):
    s = (int(seed) & 0x7fffffff) or 1
    while True:
        s = (1103515245 * s + 12345) & 0x7fffffff
        yield s / 0x7fffffff


def value_at(expr_id, t, p, rand):
    if expr_id == "wiggle":
        f, a = float(p.get("frequency", 2)), float(p.get("amplitude", 0.05))
        return a * math.sin(2 * math.pi * f * t + next(rand) * 0.4)
    if expr_id == "sine":
        f, a, ph = float(p.get("frequency", 1)), float(p.get("amplitude", 0.1)), float(p.get("phase", 0))
        return a * math.sin(2 * math.pi * f * t + ph)
    if expr_id == "damped_sine":
        f, a, d = float(p.get("frequency", 3)), float(p.get("amplitude", 0.25)), float(p.get("damping", 4))
        return a * math.exp(-d * t) * math.sin(2 * math.pi * f * t)
    if expr_id == "pulse":
        per, duty = max(0.01, float(p.get("period", 0.5))), float(p.get("duty", 0.5))
        hi, lo = float(p.get("high", 1)), float(p.get("low", 0))
        return hi if (t % per) / per < duty else lo
    if expr_id == "random_steps":
        iv = max(0.01, float(p.get("interval", 0.25)))
        lo, hi = float(p.get("min", -0.1)), float(p.get("max", 0.1))
        return lo + (hi - lo) * next(rand) if int(t / iv) % 1 == 0 else lo + (hi - lo) * next(rand)
    if expr_id == "drift":
        return float(p.get("start", 0)) + float(p.get("slope", 0.02)) * t
    if expr_id == "loop_out":
        cyc = max(0.01, float(p.get("cycle", 1)))
        a, b = float(p.get("from", 0)), float(p.get("to", 1))
        phase = (t % cyc) / cyc
        mode = p.get("mode", "cycle")
        if mode == "pingpong":
            phase = 1 - abs(1 - 2 * phase)
        elif mode == "offset":
            a += (t // cyc) * (b - a)
        return a + (b - a) * phase
    if expr_id == "follow_path":
        return 0.0  # handled by compile_expression as a 2D path
    return 0.0


def _path_point(points, u):
    pts = [[float(x), float(y)] for x, y in points]
    if len(pts) == 1:
        return pts[0]
    seg = min(int(u * (len(pts) - 1)), len(pts) - 2)
    local = u * (len(pts) - 1) - seg
    a, b = pts[seg], pts[seg + 1]
    return [a[0] + (b[0] - a[0]) * local, a[1] + (b[1] - a[1]) * local]


def compile_expression(expr_id, clip_id, clip_duration, property="scale", params=None, start=0.0):
    e = EXPRS.get(expr_id)
    if not e:
        raise ValueError(f"unknown expression: {expr_id}")
    if property not in PROPERTIES:
        raise ValueError(f"bad property: {property}")
    if clip_duration <= 0:
        raise ValueError("clip_duration must be > 0")

    p = dict(e["params"])
    p.update(params or {})
    n = max(2, int(e.get("samples", 32)))
    rand = _rng(p.get("seed", 1)) if expr_id in ("wiggle", "random_steps") else _rng(1)
    ease = e.get("ease", "linear")
    if ease not in EASES:
        ease = "linear"

    commands = []
    if expr_id == "follow_path":
        points = p.get("points") or [[0, 0], [0, 0]]
        for i in range(n):
            u = i / (n - 1)
            x, y = _path_point(points, u)
            t = start + u * clip_duration
            at = max(0.0, min(1.0, t / clip_duration))
            commands.append({"op": "setClipKey", "clipId": clip_id, "property": "offsetX",
                             "at": round(at, 5), "value": round(x, 4), "ease": ease})
            commands.append({"op": "setClipKey", "clipId": clip_id, "property": "offsetY",
                             "at": round(at, 5), "value": round(y, 4), "ease": ease})
    else:
        for i in range(n):
            u = i / (n - 1)
            t = start + u * clip_duration
            v = value_at(expr_id, t, p, rand)
            at = max(0.0, min(1.0, t / clip_duration))
            commands.append({"op": "setClipKey", "clipId": clip_id, "property": property,
                             "at": round(at, 5), "value": round(v, 4), "ease": ease})

    return {"op": "batch", "commands": commands}


if __name__ == "__main__":
    import sys
    eid = sys.argv[1] if len(sys.argv) > 1 else "wiggle"
    print(json.dumps(compile_expression(eid, "clip1", 5.0, "rotation"), indent=2)[:2500])
