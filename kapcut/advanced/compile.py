#!/usr/bin/env python3
"""Phase D advanced preset compiler -> Concat batches (CPU only)."""
import json, pathlib

HERE = pathlib.Path(__file__).resolve().parent
DATA = json.loads((HERE / "presets.json").read_text(encoding="utf-8"))
FAMILIES = DATA["families"]
PROPERTIES = {"scale", "offsetX", "offsetY", "rotation", "opacity"}


def list_presets():
    return [dict(id=p["id"], name=p["name"], family=f) for f, ps in FAMILIES.items() for p in ps]


def find(pid):
    for f, ps in FAMILIES.items():
        for p in ps:
            if p["id"] == pid:
                return f, p
    return None, None


def compile_advanced(pid, clip_id="c1", start=0.0, duration=3.0, adjust_entry=0):
    family, p = find(pid)
    if not p:
        raise ValueError(f"unknown preset: {pid}")
    commands = []
    if p.get("adjust"):
        for key, value in p["adjust"].items():
            commands.append({"op": "setEffectKey", "clipId": clip_id, "entry": adjust_entry,
                             "key": key, "at": 0.0, "value": float(value)})
    if p.get("effect"):
        commands.append({"op": "addLayerClip", "start": round(start, 3), "duration": round(duration, 3),
                         "effectId": p["effect"], "name": p["name"], "sound": False})
    if p.get("keys"):
        for prop, keys in p["keys"].items():
            if prop not in PROPERTIES:
                continue
            for key in keys:
                at = max(0.0, min(1.0, start / max(duration, 0.001) + float(key["at"])))
                commands.append({"op": "setClipKey", "clipId": clip_id, "property": prop,
                                 "at": round(at, 5), "value": float(key["value"]),
                                 "ease": key.get("ease", "linear")})
    return {"op": "batch", "commands": commands}


if __name__ == "__main__":
    import sys
    print(json.dumps(compile_advanced(sys.argv[1] if len(sys.argv) > 1 else "grade_noir"), indent=2))
