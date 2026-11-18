#!/usr/bin/env python3
"""Phase E template compiler: ordered recipes -> one Concat batch command."""
import json, pathlib, sys

HERE = pathlib.Path(__file__).resolve().parent
TEMPLATES = {t["id"]: t for t in json.loads((HERE / "templates.json").read_text(encoding="utf-8"))["templates"]}

def list_templates():
    return [dict(id=t["id"], name=t["name"], description=t["description"]) for t in TEMPLATES.values()]

def compile_template(tid, clip_id="c1", start=0.0):
    t = TEMPLATES.get(tid)
    if not t:
        raise ValueError(f"unknown template: {tid}")
    commands, cursor = [], start
    for step in t["steps"]:
        dur = float(step.get("duration", 3.0))
        # Each step resolves to real Concat commands through the named
        # compiler's data; here we express the composition explicitly.
        if step["compiler"] in ("motion", "shape", "advanced", "expression"):
            commands.append({"op": "batch", "name": step["preset"],
                             "start": round(cursor, 3), "duration": dur,
                             "clipId": clip_id, "params": step.get("params", {}),
                             "compiler": step["compiler"]})
        cursor += dur
    return {"op": "batch", "template": t["name"], "commands": commands}

if __name__ == "__main__":
    print(json.dumps(compile_template(sys.argv[1] if len(sys.argv) > 1 else "tiktok_hook"), indent=2))
