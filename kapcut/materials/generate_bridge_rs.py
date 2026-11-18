#!/usr/bin/env python3
"""Emit capcut_bridge.rs from bridge.json + concat-packages.json."""
import json, pathlib

HERE = pathlib.Path(__file__).resolve().parent
OUT = pathlib.Path(__file__).resolve().parents[2] / "src/crates/concat-mcp/src/tools/capcut_bridge.rs"
b = json.loads((HERE / "bridge.json").read_text(encoding="utf-8"))
reg = json.loads((HERE / "concat-packages.json").read_text(encoding="utf-8"))
params = {p["id"]: sorted(x["key"] for x in p["params"]) for p in reg["packages"]}

def s(x): return '"' + str(x).replace('\\', '\\\\').replace('"', '\\"') + '"'

L = [
 "// GENERATED FILE - do not edit. Run: python3 kapcut/materials/generate_bridge_rs.py",
 "use serde_json::{json, Value};",
 "",
 "fn has_param(package: &str, key: &str) -> bool {",
 "    let keys: &[&str] = match package {",
]
for pkg in sorted(params):
    L.append(f"        {s(pkg)} => &[{', '.join(s(k) for k in params[pkg])}],")
L += [
 "        _ => &[],",
 "    };",
 "    keys.contains(&key)",
 "}",
 "",
 "fn scale(v: f64, mode: &str) -> f64 {",
 "    match mode {",
 '        "kelvin" => 6500.0 + v * 4500.0,',
 '        "invert_percent" => if v > 0.0 { -100.0 } else { 0.0 },',
 "        _ => v * mode.parse::<f64>().unwrap_or(1.0),",
 "    }",
 "}",
 "",
 "pub fn map_adjust(capcut_type: &str, value: f64, clip_id: &str, entry: usize) -> Option<Value> {",
 "    let (package, key, mode) = match capcut_type.to_ascii_lowercase().as_str() {",
]
for k, (pkg, param, mode) in sorted(b["adjustMap"].items()):
    L.append(f"        {s(k)} => ({s(pkg)}, {s(param)}, {s(mode)}),")
L += [
 "        _ => return None,",
 "    };",
 '    let (package, key) = if mode == "invert_percent" { ("concat.adjust", "saturation") } else { (package, key) };',
 "    if !has_param(package, key) { return None; }",
 "    Some(json!({",
 '        "op": "setEffectKey", "clipId": clip_id, "entry": entry,',
 '        "key": key, "at": 0.0, "value": scale(value, mode),',
 "    }))",
 "}",
 "",
 "pub fn map_effect(name: &str, start: f64, duration: f64) -> Vec<Value> {",
 "    let lower = name.to_ascii_lowercase();",
 "    let mut package = \"\";",
]
for needle, pkg in sorted(b["effectMap"].items()):
    L.append(f'    if package.is_empty() && lower.contains({s(needle)}) {{ package = {s(pkg)}; }}')
L += [
 "    if package.is_empty() { return Vec::new(); }",
 "    vec![json!({",
 '        "op": "addLayerClip", "start": start, "duration": duration,',
 '        "effectId": package, "name": name, "sound": false,',
 "    })]",
 "}",
 "",
 "/// Translate a CapCut-style material request into one Concat edit command.",
 "pub fn translate(args: &Value) -> Value {",
 "    let path = args.get(\"path\").cloned().unwrap_or(Value::Null);",
 "    let mut commands: Vec<Value> = Vec::new();",
 "    if let Some(t) = args.get(\"capcutType\").and_then(Value::as_str) {",
 "        let v = args.get(\"value\").and_then(Value::as_f64).unwrap_or(0.0);",
 "        let clip = args.get(\"clipId\").and_then(Value::as_str).unwrap_or(\"\");",
 "        if let Some(c) = map_adjust(t, v, clip, 0) { commands.push(c); }",
 "    }",
 "    if let Some(n) = args.get(\"effectName\").and_then(Value::as_str) {",
 "        let start = args.get(\"start\").and_then(Value::as_f64).unwrap_or(0.0);",
 "        let dur = args.get(\"duration\").and_then(Value::as_f64).unwrap_or(5.0);",
 "        commands.extend(map_effect(n, start, dur));",
 "    }",
 "    json!({ \"op\": \"batch\", \"commands\": commands })",
 "}",
 "",
]
OUT.parent.mkdir(parents=True, exist_ok=True)
OUT.write_text("\n".join(L), encoding="utf-8")
print(f"wrote {OUT} ({len(L)} lines)")
