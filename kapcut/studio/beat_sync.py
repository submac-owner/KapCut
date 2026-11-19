#!/usr/bin/env python3
"""Beat-synced motion compiler: BPM -> keyframes on any clip property.

CapCut/Premiere-style audio-reactive editing without audio analysis at runtime:
the caller supplies BPM (and optional downbeat offset); we generate a
deterministic pulse envelope on scale/rotation/opacity/offset.
"""
import json, pathlib

HERE = pathlib.Path(__file__).resolve().parent
EASE = json.loads((HERE / "design.json").read_text(encoding="utf-8"))["motion"]["ease"]
PROPS = {"scale", "rotation", "opacity", "offsetX", "offsetY"}


def beat_keys(clip_id, property, bpm=120, duration=8.0, base=1.0, amount=0.12,
              offset=0.0, every=1, ease="out"):
    if property not in PROPS:
        raise ValueError(f"bad property {property}")
    beat = 60.0 / max(1.0, float(bpm))
    commands = []
    t = float(offset)
    i = 0
    while t < duration:
        at = max(0.0, min(1.0, t / duration))
        peak = round(base + amount, 4)
        rest = round(base - amount * 0.35, 4)
        ez = EASE.get(ease, EASE["out"])
        commands.append({"op": "setClipKey", "clipId": clip_id, "property": property,
                         "at": round(at, 5), "value": peak, "ease": ez})
        commands.append({"op": "setClipKey", "clipId": clip_id, "property": property,
                         "at": round(min(1.0, at + min(0.12, beat / duration * 0.6)), 5),
                         "value": rest, "ease": EASE["inOut"]})
        commands.append({"op": "setClipKey", "clipId": clip_id, "property": property,
                         "at": round(min(1.0, at + beat * 0.9 / duration), 5),
                         "value": base, "ease": EASE["linear"]})
        t += beat * max(1, int(every))
        i += 1
        if i > 256:
            break
    return {"op": "batch", "bpm": bpm, "property": property, "commands": commands}


if __name__ == "__main__":
    r = beat_keys("c1", "scale", 128, 8.0)
    print(f"bpm={r['bpm']} keys={len(r['commands'])}")
