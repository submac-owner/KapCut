#!/usr/bin/env python3
"""Compile AE-style shape presets into real Concat addShapeClip + keyframe commands.

All output targets Concat's existing ShapeStyle vocabulary; no engine change.
CPU-only: shapes are data, rendered by the editor's software path.
"""
import json, math, pathlib

HERE = pathlib.Path(__file__).resolve().parent
SHAPES = {s["id"]: s for s in json.loads((HERE / "shapes.json").read_text(encoding="utf-8"))["shapes"]}
PROPERTIES = {"scale", "offsetX", "offsetY", "rotation", "opacity", "stretchX", "stretchY"}


def list_shapes():
    return [dict(id=s["id"], name=s["name"], kind=s["kind"]) for s in SHAPES.values()]


def _keys(shape, start, duration, offsets=None):
    out = []
    for prop, keys in (shape.get("keys") or {}).items():
        if prop not in PROPERTIES:
            continue
        for key in keys:
            at = (start + float(key["at"]) * duration) / max(duration, 0.001)
            out.append({"op": "setClipKey", "clipId": "@created", "property": prop,
                        "at": round(max(0.0, min(1.0, at)), 5),
                        "value": float(key["value"]), "ease": key.get("ease", "linear")})
    for prop, value in (offsets or {}).items():
        out.append({"op": "setClipKey", "clipId": "@created", "property": prop,
                    "at": 0.0, "value": round(value, 4), "ease": "linear"})
    return out


def compile_shape(shape_id, start=0.0, duration=3.0, steps=8):
    s = SHAPES.get(shape_id)
    if not s:
        raise ValueError(f"unknown shape: {shape_id}")
    commands = []
    style = dict(s.get("style") or {})

    if s.get("repeat"):
        rep = s["repeat"]
        count = max(2, int(rep.get("count", 6)))
        radius = float(rep.get("radius", 0.2))
        for i in range(count):
            angle = 2 * math.pi * i / count
            x, y = radius * math.cos(angle), radius * math.sin(angle)
            commands.append({"op": "addShapeClip", "start": round(start, 3),
                             "duration": round(duration, 3), "style": style,
                             "name": f"{s['name']} {i+1}"})
            commands.extend(_keys(s, start, duration, {"offsetX": x, "offsetY": y}))
        return {"op": "batch", "commands": commands}

    if s.get("trim") and s.get("kind") == "drawing":
        tr = s["trim"]
        a, b = float(tr.get("start", 0)), float(tr.get("end", 1))
        for i in range(steps):
            u = (i + 1) / steps
            part = dict(style)
            part["trimStart"] = a
            part["trimEnd"] = a + (b - a) * u
            commands.append({"op": "addShapeClip", "start": round(start + i * duration / steps, 3),
                             "duration": round(duration / steps, 3), "style": part,
                             "name": f"{s['name']} {int(u*100)}%"})
            commands.extend(_keys(s, start + i * duration / steps, duration / steps))
        return {"op": "batch", "commands": commands}

    commands.append({"op": "addShapeClip", "start": round(start, 3),
                     "duration": round(duration, 3), "style": style, "name": s["name"]})
    commands.extend(_keys(s, start, duration))
    return {"op": "batch", "commands": commands}


if __name__ == "__main__":
    import sys
    sid = sys.argv[1] if len(sys.argv) > 1 else "neon_ring"
    print(json.dumps(compile_shape(sid), indent=2)[:2500])
