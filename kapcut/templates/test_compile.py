#!/usr/bin/env python3
import pathlib, sys
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
import compile as tpl

ts = tpl.list_templates()
assert len(ts) >= 4, len(ts)
b = tpl.compile_template("cinematic_reel")
assert b["op"] == "batch" and len(b["commands"]) == 4, b
assert all(c["op"] == "batch" for c in b["commands"]), b
print(f"template compiler tests: OK ({len(ts)} templates)")
