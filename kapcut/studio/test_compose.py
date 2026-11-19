#!/usr/bin/env python3
import pathlib, sys
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
import compose as st

ts = st.list_templates()
assert len(ts) >= 4, len(ts)
for t in ts:
    r = st.compose(t["id"])
    assert r["op"] == "batch"
    assert len(r["commands"]) > 8, (t["id"], len(r["commands"]))
    kinds = {c.get("op") for c in r["commands"]}
    assert "addShapeClip" in kinds and "setClipKey" in kinds, (t["id"], kinds)
    assert any(c["style"].get("kind") in ("circle","square","triangle","star","line") for c in r["commands"] if c.get("op")=="addShapeClip")
    assert all("ease" in c for c in r["commands"] if c.get("op")=="setClipKey")
p = st.compose("particle_reveal")
assert len([c for c in p["commands"] if c.get("op")=="addShapeClip" and "Particle" in c.get("name","")]) >= 10
print(f"studio composition tests: OK ({len(ts)} templates)")
