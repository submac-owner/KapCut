#!/usr/bin/env python3
"""Generate the overlay frame + animation libraries (CPU, data only)."""
import json, pathlib

HERE = pathlib.Path(__file__).resolve().parent
EASE = {"linear": [0, 0, 1, 1], "out": [0, 0, 0.58, 1], "in": [0.42, 0, 1, 1], "inOut": [0.42, 0, 0.58, 1]}

# 11 frame families x 5 colorways = 55 frames
FAMILIES = [
    ("paper", "Paper", "square", "cream", "#F5EFE0", "#D8CDBB", 0.02, True),
    ("vintage", "Camera Vintage", "square", "sepia", "#3A2E24", "#C9A227", 0.0, True),
    ("cutpaper", "Cutting Paper", "square", "white", "#FFFFFF", "#E6E6E6", -0.04, True),
    ("polaroid", "Polaroid", "square", "white", "#FAFAFA", "#DDDDDD", 0.03, True),
    ("film", "Film Strip", "square", "black", "#111111", "#FFFFFF", 0.0, True),
    ("neon", "Neon Frame", "square", "cyan", "#0B0F1A", "#00E5FF", 0.0, False),
    ("dossier", "Dark Dossier", "square", "ink", "#0E1117", "#8FA3C8", 0.0, True),
    ("blueprint", "Blueprint", "square", "blue", "#0A2340", "#5EA9FF", 0.0, False),
    ("polaroid_dark", "Night Polaroid", "square", "charcoal", "#1A1A1A", "#E0E0E0", 0.03, True),
    ("kraft", "Kraft Cutout", "square", "kraft", "#C8A97E", "#8B6F47", -0.02, True),
    ("glass", "Glass Pane", "square", "glass", "#EAF4FF", "#9CC5E9", 0.0, False),
]
COLORWAYS = [
    ("original", None, None),
    ("warm", "#FFD9A0", "#A86B32"),
    ("cold", "#CFE8FF", "#3E7FB5"),
    ("mono", "#E8E8E8", "#444444"),
    ("accent", "#FFE066", "#E0533B"),
]

frames = []
for fid, fname, kind, base_name, base_fill, base_stroke, rot, shadow in FAMILIES:
    for cname, cover_fill, cover_stroke in COLORWAYS:
        fill = cover_fill or base_fill
        stroke = cover_stroke or base_stroke
        frames.append({
            "id": f"frame_{fid}_{cname}",
            "name": f"{fname} — {cname.title()}",
            "family": fid,
            "category": "overlay-frame",
            "elements": [
                {"kind": kind, "width": 0.62, "height": 0.78, "fill": fill,
                 "stroke": stroke, "strokeWidth": 0.003, "cornerRadius": 0.02,
                 "rotation": rot, "shadow": {"color": stroke, "blur": 0.03} if shadow else None},
                {"kind": "square", "width": 0.50, "height": 0.64, "fill": "#00000000",
                 "stroke": stroke, "strokeWidth": 0.002, "rotation": rot},
            ],
        })

# 18 animation families x 3 phases = 54, + one extra = 55
ANIM = [
    ("fade", "Fade", [("opacity", 0, 0), ("opacity", 1, 1)]),
    ("slide_up", "Slide Up", [("offsetY", 0, 0.3), ("offsetY", 1, 0)]),
    ("slide_down", "Slide Down", [("offsetY", 0, -0.3), ("offsetY", 1, 0)]),
    ("slide_left", "Slide Left", [("offsetX", 0, 0.35), ("offsetX", 1, 0)]),
    ("slide_right", "Slide Right", [("offsetX", 0, -0.35), ("offsetX", 1, 0)]),
    ("pop", "Pop", [("scale", 0, 0.2), ("scale", 0.6, 1.2), ("scale", 1, 1)]),
    ("zoom_in", "Zoom In", [("scale", 0, 0.6), ("scale", 1, 1.05)]),
    ("zoom_out", "Zoom Out", [("scale", 0, 1.4), ("scale", 1, 1)]),
    ("spin", "Spin", [("rotation", 0, -180), ("rotation", 1, 0)]),
    ("spin_back", "Spin Back", [("rotation", 0, 180), ("rotation", 1, 0)]),
    ("flip_x", "Flip X", [("stretchX", 0, -1), ("stretchX", 1, 1)]),
    ("flip_y", "Flip Y", [("stretchY", 0, -1), ("stretchY", 1, 1)]),
    ("bounce", "Bounce", [("scale", 0, 0.2), ("scale", 0.5, 1.18), ("scale", 0.75, 0.94), ("scale", 1, 1)]),
    ("glitch", "Glitch", [("offsetX", 0, 0.08), ("offsetX", 0.3, -0.05), ("offsetX", 0.6, 0.03), ("offsetX", 1, 0)]),
    ("drop", "Drop In", [("offsetY", 0, -0.5), ("offsetY", 0.6, 0.06), ("offsetY", 1, 0)]),
    ("swing", "Swing", [("rotation", 0, -12), ("rotation", 0.5, 8), ("rotation", 1, 0)]),
    ("reveal", "Reveal", [("opacity", 0, 0), ("opacity", 0.2, 1), ("scale", 0, 0.85), ("scale", 1, 1)]),
    ("pulse", "Pulse", [("scale", 0, 1), ("scale", 0.5, 1.12), ("scale", 1, 1)]),
    ("float", "Float", [("offsetY", 0, -0.02), ("offsetY", 0.5, 0.03), ("offsetY", 1, -0.02)]),
]
animations = []
for aid, aname, keys in ANIM:
    for phase in ("in", "out", "loop"):
        klist = []
        for prop, at, val in keys:
            klist.append({"property": prop, "at": float(at), "value": float(val),
                          "ease": "out" if phase == "in" else ("in" if phase == "out" else "inOut")})
        if phase == "out":
            # reverse the value path for an exit
            klist = [{"property": k["property"], "at": k["at"],
                      "value": (0.0 if k["property"] == "opacity" else (1.0 if k["property"] == "scale" else 0.0)),
                      "ease": k["ease"]} for k in klist]
        animations.append({"id": f"anim_{aid}_{phase}", "name": f"{aname} — {phase.upper()}",
                           "family": aid, "phase": phase, "category": "overlay-animation",
                           "keys": klist})
# 55th: dossier document reveal
animations.append({"id": "anim_dossier_reveal_in", "name": "Dossier Reveal — IN", "family": "dossier",
                   "phase": "in", "category": "overlay-animation",
                   "keys": [{"property": "opacity", "at": 0, "value": 0, "ease": "out"},
                            {"property": "opacity", "at": 0.25, "value": 1, "ease": "linear"},
                            {"property": "offsetY", "at": 0, "value": 0.12, "ease": "out"},
                            {"property": "offsetY", "at": 1, "value": 0, "ease": "linear"},
                            {"property": "rotation", "at": 0, "value": 3, "ease": "out"},
                            {"property": "rotation", "at": 1, "value": 0, "ease": "linear"}]})

(HERE / "overlay_frames.json").write_text(json.dumps({"count": len(frames), "frames": frames}, indent=1), encoding="utf-8")
(HERE / "overlay_animations.json").write_text(json.dumps({"count": len(animations), "animations": animations}, indent=1), encoding="utf-8")
print(f"frames={len(frames)} animations={len(animations)}")
