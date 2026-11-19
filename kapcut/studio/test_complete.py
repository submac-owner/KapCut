#!/usr/bin/env python3
"""Local CPU-only checks for the studio completion data + wiring."""
import json, pathlib

HERE = pathlib.Path(__file__).resolve().parent
ROOT = HERE.parents[1]

def load(name):
    return json.loads((HERE / name).read_text(encoding="utf-8"))

def main():
    curves = load("curves.json")
    three_d = load("three_d.json")
    tracking = load("tracking.json")
    catalog = json.loads((ROOT / "kapcut" / "catalog.json").read_text(encoding="utf-8"))
    tools = [t for t in catalog["tools"] if t["family"] in ("curves", "three_d", "tracking")]

    assert len(curves["presets"]) >= 12
    assert len(curves["interpolations"]) >= 10
    assert len(three_d["camera_rigs"]) >= 10
    assert len(three_d["particle_fields"]) >= 8
    assert len(three_d["composite_modes"]) >= 8
    assert len(tracking["trackers"]) >= 4
    assert len(tracking["roto_shapes"]) >= 4

    assert len(tools) == 18, len(tools)
    for t in tools:
        assert t["target"] == "capcut_studio_complete", t["name"]
        assert t["target_kind"] in ("studio_complete_query", "studio_complete_apply"), t["name"]
    names = {t["name"] for t in tools}
    for required in ("curve_list", "curve_apply", "camera_apply_rig", "particle_add",
                     "composite_layer", "track_apply", "roto_shape_add", "motion_stabilize"):
        assert required in names, required

    rs = ROOT / "src" / "crates" / "concat-mcp" / "src" / "tools" / "capcut_studio_complete.rs"
    text = rs.read_text(encoding="utf-8")
    for fn in ("list_curves", "curve_keys", "camera_keys", "particle_add", "roto_shape", "motion_stabilize"):
        assert f"pub fn {fn}(" in text, fn

    print(f"studio completion: OK ({len(tools)} tools, {len(curves['presets'])} curves, "
          f"{len(three_d['camera_rigs'])} camera rigs, {len(tracking['trackers'])} trackers)")

if __name__ == "__main__":
    main()
