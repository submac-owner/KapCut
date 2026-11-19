#!/usr/bin/env python3
import pathlib, sys
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
import beat_sync as bs

r = bs.beat_keys("c1", "scale", 120, 8.0)
assert len(r["commands"]) > 20, len(r["commands"])
assert all(0 <= c["at"] <= 1 for c in r["commands"])
assert all(c["ease"] in bs.EASE.values() for c in r["commands"])
s = bs.beat_keys("c1", "opacity", 60, 4.0, base=0.5, amount=0.3)
assert any(c["value"] > 0.7 for c in s["commands"])
try:
    bs.beat_keys("c1", "bogus")
    raise AssertionError("bad property accepted")
except ValueError:
    pass
print(f"beat sync tests: OK ({len(r['commands'])} keys)")
