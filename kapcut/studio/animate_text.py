#!/usr/bin/env python3
"""Compile AE-style text animators into staggered per-word text clips.

Each word becomes a separate title clip with its own style and keyframes,
giving per-character/word animation without an engine change.
"""
import json, pathlib

HERE = pathlib.Path(__file__).resolve().parent
ANIM = {a["id"]: a for a in json.loads((HERE / "text_animators.json").read_text(encoding="utf-8"))["animators"]}
DESIGN = json.loads((HERE / "design.json").read_text(encoding="utf-8"))
EASE = DESIGN["motion"]["ease"]


def list_animators():
    return [dict(id=a["id"], name=a["name"], unit=a["unit"]) for a in ANIM.values()]


def animate_text(text, animator, start=0.0, duration=4.0, color="#FFFFFF", size=0.08, font="heading"):
    a = ANIM.get(animator)
    if not a:
        raise ValueError(f"unknown animator {animator}")
    words = text.split(" ")
    stagger = float(a["stagger"])
    commands = []
    for i, word in enumerate(words):
        at = start + i * stagger
        life = max(0.2, duration - i * stagger)
        commands.append({
            "op": "addTextClip", "start": round(at, 3), "duration": round(life, 3),
            "name": word,
            "style": {"content": word, "font_family": DESIGN["fonts"].get(font, "Inter"),
                      "font_size": size, "font_weight": 700, "color": color, "align": "center"},
        })
        cid = f"w{i}"
        for key in a["keys"]:
            commands.append({
                "op": "setClipKey", "clipId": cid, "property": key["property"],
                "at": round(float(key["at"]), 5), "value": float(key["value"]),
                "ease": EASE.get(key.get("ease", "linear"), EASE["linear"]),
            })
    return {"op": "batch", "text": text, "animator": animator, "commands": commands}


if __name__ == "__main__":
    import sys
    r = animate_text(sys.argv[1] if len(sys.argv) > 1 else "MOTION DESIGN", "burst")
    print(f"words={len(r['commands'])//4} commands={len(r['commands'])}")
