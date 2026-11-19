#!/usr/bin/env python3
"""Append a dark-dossier document pack to the overlay libraries (idempotent)."""
import json, pathlib

HERE = pathlib.Path(__file__).resolve().parent
fp = HERE / "overlay_frames.json"
ap = HERE / "overlay_animations.json"
frames = json.loads(fp.read_text(encoding="utf-8"))
anims = json.loads(ap.read_text(encoding="utf-8"))

DOSSIER_FRAMES = [
    ("dossier_sheet", "Dossier Sheet", "#11151C", "#C9D3E0", 4),
    ("classified_file", "Classified File", "#161616", "#D6D6D6", 3),
    ("redacted_doc", "Redacted Document", "#0D1117", "#8FA3C8", 5),
    ("evidence_photo", "Evidence Photo", "#101418", "#E0E0E0", 2),
    ("case_folder", "Case Folder", "#1A1A1A", "#C0A060", 3),
    ("fingerprint_card", "Fingerprint Card", "#0E1117", "#6FA8DC", 4),
    ("news_clipping", "News Clipping", "#E8E2D4", "#333333", 2),
    ("police_report", "Police Report", "#F2F0EA", "#2B2B2B", 3),
    ("lab_result", "Lab Result", "#EFF6F5", "#1F7A6B", 4),
    ("map_pin", "Evidence Map", "#0B1A24", "#5EA9FF", 2),
    ("wiretap", "Wiretap Transcript", "#121212", "#9CC5E9", 3),
    ("stamp_approved", "Approved Stamp", "#0E1117", "#E0533B", 5),
]
for fid, name, fill, stroke, lines in DOSSIER_FRAMES:
    if any(f["id"] == f"frame_{fid}" for f in frames["frames"]):
        continue
    els = [{"kind": "square", "width": 0.6, "height": 0.78, "fill": fill,
            "stroke": stroke, "strokeWidth": 0.003, "rotation": 0.0,
            "shadow": {"color": stroke, "blur": 0.02}}]
    for i in range(lines):
        els.append({"kind": "line", "width": 0.42 - (i % 3) * 0.06, "fill": "#00000000",
                    "stroke": stroke, "strokeWidth": 0.0015, "rotation": 0.0,
                    "_y": 0.12 + i * 0.07})
    frames["frames"].append({"id": f"frame_{fid}", "name": name, "family": "dossier_pack",
                             "category": "overlay-frame", "elements": els})

DOSSIER_ANIMS = [
    ("page_flip", "Page Flip", [("rotation", 0, -28), ("rotation", 0.5, 8), ("rotation", 1, 0)]),
    ("stamp", "Stamp Down", [("scale", 0, 2.2), ("scale", 0.4, 0.92), ("scale", 1, 1)]),
    ("type_lines", "Type Lines", [("opacity", 0, 0), ("opacity", 0.3, 1), ("offsetY", 0, 0.06), ("offsetY", 1, 0)]),
    ("highlight", "Highlight Sweep", [("offsetX", 0, -0.3), ("offsetX", 1, 0.3)]),
    ("redact", "Redact", [("scale", 0, 1.4), ("scale", 1, 1)]),
    ("scan", "Document Scan", [("offsetY", 0, 0.35), ("offsetY", 1, 0)]),
]
for aid, name, keys in DOSSIER_ANIMS:
    for phase in ("in", "out", "loop"):
        iid = f"anim_{aid}_{phase}"
        if any(a["id"] == iid for a in anims["animations"]):
            continue
        anims["animations"].append({
            "id": iid, "name": f"{name} — {phase.upper()}", "family": aid, "phase": phase,
            "category": "overlay-animation",
            "keys": [{"property": p, "at": float(at), "value": float(v),
                      "ease": "out" if phase == "in" else ("in" if phase == "out" else "inOut")}
                     for p, at, v in keys],
        })

frames["count"] = len(frames["frames"])
anims["count"] = len(anims["animations"])
fp.write_text(json.dumps(frames, indent=1), encoding="utf-8")
ap.write_text(json.dumps(anims, indent=1), encoding="utf-8")
print(f"frames={frames['count']} animations={anims['count']}")
