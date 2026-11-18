#!/usr/bin/env python3
"""CapCut material -> Concat edit-command bridge.

This is the piece that makes the CapCut-derived tools actually do something:
it converts CapCut effect/adjust material descriptions into real Concat
`edit.apply` commands using Concat's own effect packages.

Reference implementation in Python. The same tables are emitted as Rust in
capcut_bridge.rs for the MCP server.
"""
import json, pathlib

HERE = pathlib.Path(__file__).resolve().parent
BRIDGE = json.loads((HERE / "bridge.json").read_text(encoding="utf-8"))
REGISTRY = json.loads((HERE / "concat-packages.json").read_text(encoding="utf-8"))
PACKAGES = {p["id"]: p for p in REGISTRY["packages"]}


def scale(value: float, mode) -> float:
    """Map a CapCut normalized slider value onto Concat's parameter range."""
    if mode == "kelvin":
        return round(6500 + float(value) * 4500, 1)
    if mode == "invert_percent":
        return -100.0 if float(value) > 0 else 0.0
    return round(float(value) * float(mode), 3)


def map_adjust(capcut_type: str, value: float, clip_id: str, entry: int = 0, at: float = 0.0):
    """Translate one CapCut materials.effects[] entry to a setEffectKey command."""
    rule = BRIDGE["adjustMap"].get(str(capcut_type).lower())
    if not rule:
        return None
    package, key, mode = rule
    package = "concat.adjust" if mode == "invert_percent" else package
    if mode == "invert_percent":
        key = "saturation"
    if package not in PACKAGES or key not in {p["key"] for p in PACKAGES[package]["params"]}:
        return None
    return {
        "op": "setEffectKey",
        "clipId": clip_id,
        "entry": entry,
        "key": key,
        "at": at,
        "value": scale(value, "invert_percent" if mode == -100 else mode),
    }


def map_effect(name_or_id: str, start: float, duration: float,
               adjust_params=None, name: str = ""):
    """Translate a CapCut effect/transition/sticker name to Concat layer commands."""
    key = str(name_or_id or "").lower()
    package = None
    for needle, pkg in BRIDGE["effectMap"].items():
        if needle in key:
            package = pkg
            break
    if not package or package not in PACKAGES:
        return []

    commands = [{
        "op": "addLayerClip",
        "start": round(float(start), 3),
        "duration": round(float(duration), 3),
        "effectId": package,
        "name": name or package,
        "sound": False,
    }]
    # CapCut effect_adjust_params: map known keys onto Concat package params.
    touched = set()
    for p in adjust_params or []:
        pname = p.get("name")
        ckey = BRIDGE["effectParamMap"].get(pname)
        if ckey and ckey not in touched and ckey in {x["key"] for x in PACKAGES[package]["params"]}:
            touched.add(ckey)
            commands.append({
                "op": "setEffectKey", "clipId": "@created", "entry": -1,
                "key": ckey, "at": 0.0, "value": float(p.get("value", 0)),
            })
    return commands


def translate_draft(draft: dict):
    """Translate every recognized CapCut material in a draft into one batch."""
    commands = []
    segments = {}
    for track in draft.get("tracks", []):
        for seg in track.get("segments", []):
            segments[seg["id"]] = seg

    for mat in draft.get("materials", {}).get("effects", []):
        seg_id = ""
        for sid, seg in segments.items():
            if mat["id"] in (seg.get("extra_material_refs") or []):
                seg_id = sid
                break
        if not seg_id:
            continue
        cmd = map_adjust(mat.get("type", ""), mat.get("value", 0), seg_id)
        if cmd:
            commands.append(cmd)

    for mat in draft.get("materials", {}).get("video_effects", []):
        seg_id = ""
        for sid, seg in segments.items():
            if mat["id"] in (seg.get("extra_material_refs") or []):
                seg_id = sid
                break
        tr = segments.get(seg_id, {}).get("target_timerange", {}) if seg_id else {}
        start, duration = tr.get("start", 0) / 1e6, tr.get("duration", 0) / 1e6
        commands += map_effect(mat.get("name", ""), start, duration,
                               mat.get("adjust_params"), mat.get("name", ""))

    return {"op": "batch", "commands": commands}


if __name__ == "__main__":
    import sys
    p = pathlib.Path(sys.argv[1] if len(sys.argv) > 1 else
        r"C:\Users\Admin\Desktop\Capcut\notes\capcut-draft-1118-v2.json")
    d = json.loads(p.read_text(encoding="utf-8"))
    result = translate_draft(d)
    print(json.dumps(result, indent=2, ensure_ascii=False)[:6000])
    print(f"\ncommands: {len(result['commands'])}")
