#!/usr/bin/env python3
import pathlib, sys
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
import style_engine as se

r = se.invent("warm vintage cinematic with grain and wave text", "WARM")
assert r["palette"] == "ember", r["palette"]
assert r["animator"] == "wave", r["animator"]
assert "concat.film-grain" in r["effects"], r["effects"]
assert "concat.vintage" in r["effects"], r["effects"]
assert any(c.get("op") == "addTextClip" for c in r["commands"])
assert any(c.get("op") == "addLayerClip" for c in r["commands"])
r2 = se.invent("calm natural minimal")
assert r2["palette"] == "forest", r2["palette"]
print(f"style engine tests: OK ({len(r['commands'])} commands)")
