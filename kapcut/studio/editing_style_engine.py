#!/usr/bin/env python3
"""Full video-editing style engine (not just motion graphics).

Turns a user's description and/or a decoded reference profile into a complete
editing plan across the five dials (pace, text, graphics, sound, look) plus
structure. Emits real Concat commands so the agent can reproduce or invent any
editing style without a stored template.
"""
import json, pathlib

HERE = pathlib.Path(__file__).resolve().parent
DESIGN = json.loads((HERE / "design.json").read_text(encoding="utf-8"))
EASE = DESIGN["motion"]["ease"]


def _has(text, *words):
    return any(w in text for w in words)


def _palette_from_rgb(rgb):
    palettes = DESIGN["palettes"]
    def dist(key):
        c = palettes[key]["primary"].lstrip("#")
        target = tuple(int(rgb[i]) for i in range(3))
        ref = tuple(int(c[i:i+2], 16) for i in (0, 2, 4))
        return sum((a - b) ** 2 for a, b in zip(target, ref))
    return min(palettes, key=dist)


def derive_plan(description="", profile=None):
    text = description.lower()
    prof = (profile or {}).get("style_profile", {}) if profile else {}
    ref_pace = prof.get("pace")
    ref_shot = prof.get("median_shot")

    # Pace
    if _has(text, "hormozi", "retention", "brainrot", "mrbeast", "fast", "rapid"):
        pace = "fast"
    elif _has(text, "documentary", "cinematic", "slow", "calm", "video essay"):
        pace = "slow"
    elif _has(text, "medium", "balanced", "vlog", "tutorial"):
        pace = "medium"
    else:
        pace = ref_pace or "medium"
    shot = {"fast": 1.2, "medium": 4.0, "slow": 9.0}[pace]
    if ref_shot:
        shot = round(float(ref_shot), 2)

    # Text
    if _has(text, "every word", "word by word", "kinetic captions", "hormozi"):
        text_style = "every-word"
    elif _has(text, "keyword", "callout", "highlight"):
        text_style = "keywords"
    elif _has(text, "lower third", "caption", "subtitle"):
        text_style = "lower-thirds"
    elif _has(text, "no text", "clean", "minimal"):
        text_style = "none"
    else:
        text_style = "keywords"

    # Graphics density
    if _has(text, "heavy graphics", "collage", "brainrot", "motion graphics"):
        graphics = "heavy"
    elif _has(text, "b-roll", "overlay", "graphics", "map", "chart"):
        graphics = "medium"
    elif _has(text, "minimal", "clean", "talking head"):
        graphics = "light"
    else:
        graphics = "medium"
    if ref_pace == "fast" and not description:
        graphics = "heavy"

    # Sound
    if _has(text, "sfx", "sound effect", "whoosh", "hit", "riser"):
        sound = "designed"
    elif _has(text, "music", "bed", "score"):
        sound = "bed"
    else:
        sound = "bed"

    # Look
    if prof.get("mean_rgb"):
        palette = _palette_from_rgb(prof["mean_rgb"])
    elif _has(text, "neon", "cyber", "glitch", "violet"):
        palette = "violet"
    elif _has(text, "warm", "vintage", "film", "ember"):
        palette = "ember"
    elif _has(text, "cinematic", "calm", "forest", "natural"):
        palette = "forest"
    else:
        palette = "midnight"
    grade = "concat.vintage" if palette == "ember" else ("concat.teal-orange" if palette in ("forest", "midnight") else "concat.neon")

    # Structure
    if _has(text, "hook", "short", "reel", "tiktok", "shorts"):
        structure = "hook-payoff-cta"
    elif _has(text, "essay", "documentary", "explain", "thesis"):
        structure = "thesis-evidence-conclusion"
    elif _has(text, "tutorial", "steps", "how to"):
        structure = "step-by-step"
    else:
        structure = "hook-payoff-cta"

    return {"pace": pace, "shot_seconds": shot, "text": text_style,
            "graphics": graphics, "sound": sound, "look": palette,
            "grade": grade, "structure": structure,
            "reference_driven": bool(profile)}


def compile_plan(description="", profile=None, title="TITLE", duration=20.0):
    plan = derive_plan(description, profile)
    colors = DESIGN["palettes"][plan["look"]]
    commands = []

    # Background
    commands.append({"op": "addShapeClip", "start": 0, "duration": duration,
                     "name": "Background",
                     "style": {"kind": "square", "width": 2.0, "height": 2.0, "fill": colors["bg"]}})

    # Graphics density -> shape accents every few seconds
    density_steps = {"light": 2, "medium": 5, "heavy": 10}[plan["graphics"]]
    for i in range(density_steps):
        at = i * duration / density_steps
        commands.append({"op": "addShapeClip", "start": round(at, 3),
                         "duration": round(duration / density_steps, 3),
                         "name": f"Accent {i+1}",
                         "style": {"kind": "circle" if i % 2 else "square", "width": 0.35,
                                   "fill": colors["surface"], "stroke": colors["primary"],
                                   "strokeWidth": 0.003}})

    # Text treatment
    if plan["text"] != "none":
        count = {"every-word": 8, "keywords": 4, "lower-thirds": 2}[plan["text"]]
        for i in range(count):
            at = i * duration / count
            content = title if i == 0 else f"POINT {i+1}"
            commands.append({"op": "addTextClip", "start": round(at, 3),
                             "duration": round(duration / count - 0.1, 3),
                             "name": content,
                             "style": {"content": content, "font_family": DESIGN["fonts"]["heading"],
                                       "font_size": 0.06 if plan["text"] == "lower-thirds" else 0.09,
                                       "font_weight": 700, "color": colors["text"], "align": "center"}})
            commands.append({"op": "setClipKey", "clipId": f"text{i}", "property": "opacity",
                             "at": 0.0, "value": 0.0, "ease": EASE["out"]})
            commands.append({"op": "setClipKey", "clipId": f"text{i}", "property": "opacity",
                             "at": 0.2, "value": 1.0, "ease": EASE["linear"]})

    # Cut cadence -> transitions at shot boundaries
    n = max(1, int(duration / max(0.5, plan["shot_seconds"])))
    for i in range(1, n):
        commands.append({"op": "addTransitionClip", "start": round(i * plan["shot_seconds"], 3),
                         "transitionId": "concat.zoom-transition" if plan["pace"] == "fast" else "concat.dissolve",
                         "duration": 0.5, "name": "Cut"})
    # Camera micro-move
    commands.append({"op": "setClipKey", "clipId": "accent0", "property": "scale",
                     "at": 0.0, "value": 1.0, "ease": EASE["inOut"]})
    commands.append({"op": "setClipKey", "clipId": "accent0", "property": "scale",
                     "at": 1.0, "value": 1.08 if plan["pace"] != "slow" else 1.03, "ease": EASE["linear"]})

    # Sound + look
    if plan["sound"] == "designed":
        for i in range(0, int(duration), 3):
            commands.append({"op": "addLayerClip", "start": float(i), "duration": 0.4,
                             "effectId": "concat.impact", "name": "Hit", "sound": True})
    commands.append({"op": "addLayerClip", "start": 0, "duration": duration,
                     "effectId": plan["grade"], "name": plan["grade"], "sound": False})

    return {"op": "batch", "plan": plan, "title": title, "duration": duration, "commands": commands}


if __name__ == "__main__":
    r = compile_plan("fast retention hormozi style with every word captions and whooshes", title="HOOK", duration=15)
    print(json.dumps(r["plan"], indent=2))
    print("commands:", len(r["commands"]))
