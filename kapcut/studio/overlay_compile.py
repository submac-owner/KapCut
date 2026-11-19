#!/usr/bin/env python3
"""Compile overlay frames + animations into real Concat edit commands."""
import json, pathlib

HERE = pathlib.Path(__file__).resolve().parent
FRAMES = {f["id"]: f for f in json.loads((HERE / "overlay_frames.json").read_text(encoding="utf-8"))["frames"]}
ANIMS = {a["id"]: a for a in json.loads((HERE / "overlay_animations.json").read_text(encoding="utf-8"))["animations"]}


def list_frames():
    return [{"id": f["id"], "name": f["name"], "family": f["family"]} for f in FRAMES.values()]


def list_animations():
    return [{"id": a["id"], "name": a["name"], "phase": a["phase"]} for a in ANIMS.values()]


def add_frame(frame_id, start=0.0, duration=5.0, animation_id=None):
    f = FRAMES.get(frame_id)
    if not f:
        raise ValueError(f"unknown frame {frame_id}")
    commands = []
    for i, el in enumerate(f["elements"]):
        cid = f"ov{i}"
        style = {k: v for k, v in el.items() if v is not None}
        commands.append({"op": "addShapeClip", "start": round(start, 3),
                         "duration": round(duration, 3), "name": f"{f['name']} {i+1}",
                         "style": style})
        if animation_id:
            a = ANIMS.get(animation_id)
            if a:
                for key in a["keys"]:
                    commands.append({"op": "setClipKey", "clipId": cid,
                                     "property": key["property"], "at": float(key["at"]),
                                     "value": float(key["value"]), "ease": key["ease"]})
    return {"op": "batch", "frame": f["name"], "animation": animation_id, "commands": commands}


def apply_animation(animation_id, clip_id="ov0", start=0.0, duration=5.0):
    a = ANIMS.get(animation_id)
    if not a:
        raise ValueError(f"unknown animation {animation_id}")
    commands = []
    for key in a["keys"]:
        t = start + float(key["at"]) * duration
        at = max(0.0, min(1.0, t / max(duration, 0.01)))
        commands.append({"op": "setClipKey", "clipId": clip_id, "property": key["property"],
                         "at": round(at, 5), "value": float(key["value"]), "ease": key["ease"]})
    return {"op": "batch", "animation": a["name"], "commands": commands}


if __name__ == "__main__":
    r = add_frame("frame_dossier_original", 1.0, 6.0, "anim_dossier_reveal_in")
    print(f"frame={r['frame']} commands={len(r['commands'])}")
