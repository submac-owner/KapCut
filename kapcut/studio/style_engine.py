#!/usr/bin/env python3
"""Style invention engine: natural-language style -> composition recipe.

Uses the design tokens and skill vocabularies. The agent can describe any
style ("dark neon kinetic type with glitch cuts") and get a fresh combination
of palette, typography, animators, shapes, effects and rhythm — not a stored
template. Output is a Concat batch command.
"""
import json, pathlib

HERE = pathlib.Path(__file__).resolve().parent
DESIGN = json.loads((HERE / "design.json").read_text(encoding="utf-8"))
ANIMS = json.loads((HERE / "text_animators.json").read_text(encoding="utf-8"))["animators"]

VOCAB = {
    "palette": {
        "neon": "violet", "cyber": "violet", "glitch": "violet",
        "warm": "ember", "vintage": "ember", "film": "ember",
        "calm": "forest", "cinematic": "forest", "natural": "forest",
        "bold": "midnight", "clean": "midnight", "tech": "midnight",
    },
    "effect": {
        "neon": "concat.neon", "glow": "concat.glow", "glitch": "concat.glitch-warp",
        "grain": "concat.film-grain", "vintage": "concat.vintage", "noir": "concat.noir",
        "chromatic": "concat.chromatic-aberration", "halation": "concat.halation",
        "vivid": "concat.vibrance", "teal": "concat.teal-orange",
    },
    "animator": {
        "type": "type_on", "kinetic": "burst", "bounce": "burst", "pop": "highlight_pop",
        "wave": "wave", "slide": "slider", "flip": "flip_up", "highlight": "highlight_pop",
    },
    "shape": {
        "circle": "circle", "orb": "circle", "ring": "circle",
        "triangle": "triangle", "star": "star", "burst": "star",
        "line": "line", "card": "square", "square": "square",
    },
    "rhythm": {"fast": 140, "retention": 150, "calm": 70, "cinematic": 60, "medium": 110},
}


def invent(description, title="TITLE", duration=10.0):
    text = description.lower()
    palette = next((v for k, v in VOCAB["palette"].items() if k in text), "midnight")
    colors = DESIGN["palettes"][palette]
    effects = [v for k, v in VOCAB["effect"].items() if k in text][:3]
    if not effects:
        effects = ["concat.glow"]
    animator = next((v for k, v in VOCAB["animator"].items() if k in text), "burst")
    shapes = list(dict.fromkeys(v for k, v in VOCAB["shape"].items() if k in text))[:2]
    if not shapes:
        shapes = ["circle"]
    bpm = next((v for k, v in VOCAB["rhythm"].items() if k in text), 110)

    commands = [
        {"op": "addShapeClip", "start": 0, "duration": duration, "name": "Background",
         "style": {"kind": "square", "width": 2.0, "height": 2.0, "fill": colors["bg"]}},
    ]
    for i, shape in enumerate(shapes):
        commands.append({"op": "addShapeClip", "start": 0.5 + i * 0.5,
                         "duration": duration - 1 - i * 0.5, "name": f"Shape {i+1}",
                         "style": {"kind": shape, "width": 0.5, "fill": colors["surface"],
                                   "stroke": colors["primary"], "strokeWidth": 0.004,
                                   "shadow": {"color": colors["primary"], "blur": 0.05}}})
    commands.append({"op": "addTextClip", "start": 1.0, "duration": duration - 2,
                     "name": title, "style": {"content": title, "font_family": DESIGN["fonts"]["display"],
                     "font_size": DESIGN["typeScale"]["display"]["size"], "font_weight": 800,
                     "color": colors["text"], "align": "center"}})
    a = next((x for x in ANIMS if x["id"] == animator), ANIMS[0])
    cid = "title0"
    for key in a["keys"]:
        commands.append({"op": "setClipKey", "clipId": cid, "property": key["property"],
                         "at": float(key["at"]), "value": float(key["value"]),
                         "ease": DESIGN["motion"]["ease"].get(key.get("ease", "linear"), [0, 0, 1, 1])})
    for fx in effects:
        commands.append({"op": "addLayerClip", "start": 0, "duration": duration,
                         "effectId": fx, "name": fx, "sound": False})
    return {"op": "batch", "description": description, "palette": palette,
            "animator": animator, "bpm": bpm, "effects": effects, "commands": commands}


if __name__ == "__main__":
    r = invent("dark neon kinetic typography with glitch cuts and film grain", "NEON")
    print(f"palette={r['palette']} animator={r['animator']} bpm={r['bpm']} commands={len(r['commands'])}")
