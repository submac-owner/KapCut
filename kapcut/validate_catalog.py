#!/usr/bin/env python3
"""Validate the generated KapCut tool catalog before CI compiles it."""
import json, pathlib, re, sys

root = pathlib.Path(__file__).resolve().parents[1]
cat = json.loads((root / "kapcut" / "catalog.json").read_text(encoding="utf-8"))
tools = cat["tools"]
errors = []

names = [t["name"] for t in tools]
dupes = sorted({n for n in names if names.count(n) > 1})
if dupes:
    errors.append(f"duplicate tool names: {dupes}")

if cat["toolCount"] != len(tools):
    errors.append("toolCount mismatch")

valid_methods = {
    "project.list","project.create","project.open","project.save","project.close",
    "project.get","project.setVideo","project.document","media.probe","media.import",
    "edit.apply","edit.undo","edit.redo","export.run","export.status","export.cancel",
    "preview.frame","catalogue.list","template.list","template.instantiate",
}
for t in tools:
    if t["target_kind"] == "method" and t["target"] not in valid_methods:
        errors.append(f"{t['name']}: unknown method {t['target']}")
    if t["target_kind"] == "bridge" and t["target"] != "capcut_bridge":
        errors.append(f"{t['name']}: unknown bridge {t['target']}")
    if t["target_kind"] == "op" and not re.match(r"^[a-z][A-Za-z]+$", t["target"]):
        errors.append(f"{t['name']}: bad op {t['target']}")
    if not t["description"] or len(t["description"]) < 12:
        errors.append(f"{t['name']}: description too short")

rs = (root / "src/crates/concat-mcp/src/capcut_tools.rs").read_text(encoding="utf-8")
if rs.count('name: "') != len(tools):
    errors.append(f"Rust tool count mismatch: {rs.count('name: \"')} vs {len(tools)}")

print(f"tools={len(tools)} families={len(cat['families'])}")
print("families:", ", ".join(cat["families"]))
if errors:
    print("FAIL")
    for e in errors: print("  -", e)
    sys.exit(1)
print("OK")
