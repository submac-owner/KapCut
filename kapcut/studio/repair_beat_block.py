import pathlib
p = pathlib.Path(r"C:\Users\Admin\Desktop\Capcut\KapCut\kapcut\studio\generate_studio_rs.py")
t = p.read_text(encoding="utf-8")
start = t.index(' "pub fn beat_keys(args: &Value) -> Value {"')
end = t.index(' "pub fn compose(args: &Value) -> Value {"', start)
lines = r''' "pub fn beat_keys(args: &Value) -> Value {",
 "    let clip = args.get(\"clipId\").and_then(Value::as_str).unwrap_or(\"c1\");",
 "    let prop = args.get(\"property\").and_then(Value::as_str).unwrap_or(\"scale\");",
 "    let bpm = f(args, \"bpm\", 120.0).max(1.0);",
 "    let dur = f(args, \"duration\", 8.0).max(0.1);",
 "    let base = f(args, \"base\", 1.0);",
 "    let amount = f(args, \"amount\", 0.12);",
 "    let offset = f(args, \"offset\", 0.0);",
 "    let beat = 60.0 / bpm;",
 "    let mut commands: Vec<Value> = Vec::new();",
 "    let mut t = offset;",
 "    let mut i = 0;",
 "    while t < dur && i < 256 {",
 "        let at = (t / dur).clamp(0.0, 1.0);",
 "        commands.push(json!({ \"op\":\"setClipKey\", \"clipId\":clip, \"property\":prop,",
 "            \"at\":at, \"value\":base + amount, \"ease\":ease(\"out\") }));",
 "        commands.push(json!({ \"op\":\"setClipKey\", \"clipId\":clip, \"property\":prop,",
 "            \"at\":(at + (beat/dur*0.6).min(0.12)).min(1.0), \"value\":base - amount*0.35, \"ease\":ease(\"inOut\") }));",
 "        commands.push(json!({ \"op\":\"setClipKey\", \"clipId\":clip, \"property\":prop,",
 "            \"at\":(at + beat*0.9/dur).min(1.0), \"value\":base, \"ease\":ease(\"linear\") }));",
 "        t += beat; i += 1;",
 "    }",
 "    json!({ \"op\":\"batch\", \"bpm\":bpm, \"property\":prop, \"commands\":commands })",
 "}",
 "",
'''
p.write_text(t[:start] + lines + t[end:], encoding="utf-8")
print("beat block repaired")
