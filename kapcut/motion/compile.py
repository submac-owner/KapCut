#!/usr/bin/env python3
"""Compile a motion preset into real Concat edit commands + a CPU FFmpeg graph.

Presets are data (presets.json). The Concat path emits setClipKey /
addLayerClip commands the engine already understands. The FFmpeg path emits a
CPU-only filter chain for rendering on machines with no GPU (this box included).
"""
import json, pathlib

HERE = pathlib.Path(__file__).resolve().parent
PRESETS = {p["id"]: p for p in json.loads((HERE / "presets.json").read_text(encoding="utf-8"))["presets"]}
PACKAGES = {p["id"] for p in json.loads((HERE.parent / "materials" / "concat-packages.json").read_text(encoding="utf-8"))["packages"]}
PROPERTIES = {"scale", "offsetX", "offsetY", "rotation", "stretchX", "stretchY", "opacity", "volume"}
EASES = {"linear", "in", "out", "inOut"}


def list_presets():
    return [dict(id=p["id"], name=p["name"], kind=p["kind"], duration=p["duration"]) for p in PRESETS.values()]


def compile_preset(preset_id: str, clip_id: str, clip_duration: float,
                   start: float = 0.0, loops: int = 1, effect_entry: int = 0):
    """Return a Concat `batch` command with all keys from the preset."""
    p = PRESETS.get(preset_id)
    if not p:
        raise ValueError(f"unknown preset: {preset_id}")
    if clip_duration <= 0:
        raise ValueError("clip_duration must be > 0")

    pd = float(p["duration"])
    cycles = max(1, int(loops)) if p["kind"] == "loop" else 1
    commands = []
    for track in p["tracks"]:
        prop = track["property"]
        if prop not in PROPERTIES:
            continue
        for cycle in range(cycles):
            for key in track["keys"]:
                t = start + (cycle + float(key["at"])) * pd
                at = max(0.0, min(1.0, t / clip_duration))
                ease = key.get("ease", "linear")
                if ease not in EASES:
                    ease = "linear"
                commands.append({
                    "op": "setClipKey", "clipId": clip_id, "property": prop,
                    "at": round(at, 5), "value": float(key["value"]), "ease": ease,
                })

    if p.get("effect"):
        package = p["effect"]
        if package in PACKAGES:
            commands.append({
                "op": "addLayerClip", "start": round(start, 3),
                "duration": round(pd * cycles, 3),
                "effectId": package, "name": p["name"], "sound": False,
            })

    return {"op": "batch", "commands": commands}


def compile_ffmpeg(preset_id: str, start: float = 0.0, loops: int = 1):
    """Return a CPU-only FFmpeg filter chain for the preset's visual motif."""
    p = PRESETS.get(preset_id)
    if not p:
        raise ValueError(f"unknown preset: {preset_id}")
    filters = []
    for f in p.get("ffmpeg", []):
        filt, args = f["filter"], f["args"]
        if start:
            args = args.replace("st=0", f"st={start:g}")
        filters.append(f"{filt}={args}")
    return ",".join(filters) if filters else "null"


if __name__ == "__main__":
    import sys
    pid = sys.argv[1] if len(sys.argv) > 1 else "popping_candy"
    print(json.dumps(compile_preset(pid, "clip1", 20.2, loops=3), indent=2)[:3000])
    print("\nFFmpeg CPU graph:", compile_ffmpeg(pid))
