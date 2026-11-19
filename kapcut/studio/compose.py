#!/usr/bin/env python3
"""Phase F composition compiler: studio templates -> layered Concat commands.

CPU-only. Uses Concat's real shape styles (gradients via fill pairs, strokes,
shadows), text styles, keyframed properties, effect layers and camera rigs.
"""
import json, math, pathlib

HERE = pathlib.Path(__file__).resolve().parent
DESIGN = json.loads((HERE / "design.json").read_text(encoding="utf-8"))
TEMPLATES = {t["id"]: t for t in json.loads((HERE / "templates.json").read_text(encoding="utf-8"))["templates"]}
EASE = DESIGN["motion"]["ease"]
TYPE = DESIGN["typeScale"]


def list_templates():
    return [dict(id=t["id"], name=t["name"], duration=t["duration"], palette=t["palette"]) for t in TEMPLATES.values()]


def _style(el):
    s = {"kind": str(el.get("kind", "square")).lower()}
    for key in ("width", "height", "fill", "stroke", "strokeWidth", "cornerRadius", "points", "innerRadius", "dash"):
        if key in el:
            s[key] = el[key]
    if el.get("shadow"):
        s["shadow"] = el["shadow"]
    return s


def _keys(clip_id, anim, at, duration, stagger=0.0, ease="out"):
    """Return staggered keyframes for an animation preset."""
    e = EASE.get(ease, EASE["out"])
    start = at + stagger
    out = []
    recipes = {
        "bounce_in": [("scale", 0, 0.25, EASE["out"]), ("scale", 0.55, 1.14, EASE["in"]),
                      ("scale", 0.8, 0.97, EASE["out"]), ("scale", 1, 1.0, EASE["linear"]),
                      ("opacity", 0, 0), ("opacity", 0.3, 1)],
        "fade_in_up": [("opacity", 0, 0, EASE["out"]), ("opacity", 1, 1),
                       ("offsetY", 0, 0.16, EASE["out"]), ("offsetY", 1, 0)],
        "draw_on": [("opacity", 0, 0), ("opacity", 0.15, 1),
                    ("scale", 0, 0.7, EASE["out"]), ("scale", 1, 1.0)],
        "typewriter": [("opacity", 0, 0), ("opacity", 0.02, 1),
                       ("offsetX", 0, -0.03, EASE["out"]), ("offsetX", 1, 0)],
        "glitch_in": [("opacity", 0, 0.2), ("opacity", 0.12, 1),
                      ("offsetX", 0, 0.07, EASE["out"]), ("offsetX", 0.5, -0.03), ("offsetX", 1, 0)],
        "float": [("offsetY", 0, -0.03, EASE["inOut"]), ("offsetY", 0.5, 0.03, EASE["inOut"]), ("offsetY", 1, -0.03)],
    }
    for prop, pos, value, *ez in recipes.get(anim, recipes["fade_in_up"]):
        out.append({"op": "setClipKey", "clipId": clip_id, "property": prop,
                    "at": round(max(0, min(1, start / max(duration, 0.01) + pos)), 5),
                    "value": float(value), "ease": ez[0] if ez else e})
    return out


def _camera(commands, rig, duration):
    if not rig:
        return
    for prop, points in rig.items():
        if prop not in ("scale", "offsetX", "offsetY", "rotation"):
            continue
        # Apply to every clip created so far (shared camera move)
        ids = {c["clipId"] for c in commands if c.get("op") == "setClipKey"}
        for cid in ids:
            for pos, value in points:
                commands.append({"op": "setClipKey", "clipId": cid, "property": prop,
                                 "at": round(float(pos) * duration / max(duration, 0.01), 5),
                                 "value": float(value), "ease": EASE["inOut"]})


def compose(template_id):
    t = TEMPLATES.get(template_id)
    if not t:
        raise ValueError(f"unknown template {template_id}")
    pal = DESIGN["palettes"][t["palette"]]
    dur = float(t["duration"])
    commands = []

    bg = t.get("background")
    if bg:
        commands.append({"op": "addShapeClip", "start": 0, "duration": dur,
                         "name": "Background", "style": _style(bg)})

    for i, el in enumerate(t.get("elements", [])):
        at = float(el.get("at", 0))
        edur = float(el.get("duration", max(1, dur - at)))
        y = el.get("y")
        if y is not None:
            el = dict(el)
            el["_y"] = y

        if el["type"] == "particlefield":
            count = int(el.get("count", 10))
            for j in range(count):
                angle = 2 * math.pi * j / count
                radius = 0.18 + 0.05 * (j % 3)
                sid = f"p{j}"
                commands.append({"op": "addShapeClip", "start": round(at + j * 0.03, 3),
                                 "duration": round(edur, 3), "name": f"Particle {j+1}",
                                 "style": {"kind": str(el.get("kind", "circle")).lower(),
                                           "width": float(el.get("size", 0.02)), "fill": el.get("fill", pal["primary"])}})
                commands.extend(_keys(sid, el.get("anim", "float"), at, edur, j * 0.02))
                commands.append({"op": "setClipKey", "clipId": sid, "property": "offsetX", "at": 0, "value": round(radius * math.cos(angle), 4), "ease": EASE["linear"]})
                commands.append({"op": "setClipKey", "clipId": sid, "property": "offsetY", "at": 0, "value": round(radius * math.sin(angle), 4), "ease": EASE["linear"]})
        elif el["type"] == "text":
            content = el.get("content", "TEXT")
            size = TYPE.get(el.get("size", "h1"), TYPE["h1"])["size"] if isinstance(el.get("size"), str) else float(el.get("size", TYPE["h1"]["size"]))
            family = DESIGN["fonts"].get(el.get("font", "heading"), "Inter")
            commands.append({"op": "addTextClip", "start": round(at, 3), "duration": round(edur, 3),
                             "name": content[:12],
                             "style": {"content": content, "font_family": family,
                                       "font_size": float(el.get("size_override", size)),
                                       "font_weight": 700, "color": el.get("color", pal["text"]),
                                       "align": "center"}})
            cid = f"text{i}"
            commands.extend(_keys(cid, el.get("anim", "fade_in_up"), at, edur, i * DESIGN["motion"]["durations"]["stagger"]))
            if "_y" in el:
                commands.append({"op": "setClipKey", "clipId": cid, "property": "offsetY", "at": 0, "value": float(el["_y"]) - 0.5, "ease": EASE["linear"]})
        else:
            commands.append({"op": "addShapeClip", "start": round(at, 3), "duration": round(edur, 3),
                             "name": el.get("name", f"Element {i+1}"), "style": _style(el)})
            commands.extend(_keys(f"shape{i}", el.get("anim", "fade_in_up"), at, edur, i * DESIGN["motion"]["durations"]["stagger"]))

    for fx in t.get("effects", []):
        commands.append({"op": "addLayerClip", "start": float(fx["at"]), "duration": float(fx["duration"]),
                         "effectId": fx["id"], "name": fx["id"], "sound": False})

    _camera(commands, t.get("camera"), dur)
    return {"op": "batch", "template": t["name"], "duration": dur, "commands": commands}


if __name__ == "__main__":
    import sys
    r = compose(sys.argv[1] if len(sys.argv) > 1 else "kinetic_title")
    print(f"template={r['template']} commands={len(r['commands'])}")
    print(json.dumps(r["commands"][:6], indent=2)[:1800])
