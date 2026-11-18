// GENERATED FILE - do not edit. Run: python3 kapcut/shapes/generate_shape_rs.py
use serde_json::{json, Value};
use std::f64::consts::PI;

const SHAPES_JSON: &str = r##"[{"id":"neon_ring","name":"Neon Ring","kind":"drawing","style":{"kind":"Circle","width":0.4,"fill":"","stroke":"#00E5FF","strokeWidth":0.006,"shadow":{"color":"#00E5FF","blur":0.05}},"trim":{"start":0,"end":1},"keys":{"rotation":[{"at":0,"value":0,"ease":"linear"},{"at":1,"value":360,"ease":"linear"}]}},{"id":"progress_arc","name":"Progress Arc","kind":"drawing","style":{"kind":"Circle","width":0.35,"fill":"","stroke":"#FFFFFF","strokeWidth":0.01},"trim":{"start":0,"end":0.75},"keys":{"rotation":[{"at":0,"value":-90,"ease":"linear"},{"at":1,"value":270,"ease":"linear"}]}},{"id":"dashed_trail","name":"Dashed Trail","kind":"drawing","style":{"kind":"Line","width":0.6,"fill":"","stroke":"#FF3B6B","strokeWidth":0.004,"dash":"dash"},"trim":{"start":0,"end":1},"keys":{"scale":[{"at":0,"value":0.8,"ease":"out"},{"at":1,"value":1.0}]}},{"id":"gradient_orb","name":"Gradient Orb","kind":"figure","style":{"kind":"Circle","width":0.3,"fill":"#6C5CE7","gradient":{"from":"#6C5CE7","to":"#00E5FF","angle":45}},"keys":{"scale":[{"at":0,"value":0.9,"ease":"inOut"},{"at":0.5,"value":1.08,"ease":"inOut"},{"at":1,"value":0.9}]}},{"id":"star_burst","name":"Star Burst","kind":"figure","style":{"kind":"Star","sides":8,"points":8,"innerRadius":0.45,"width":0.4,"fill":"#FFD166"},"keys":{"rotation":[{"at":0,"value":0,"ease":"out"},{"at":1,"value":180,"ease":"linear"}],"opacity":[{"at":0,"value":0},{"at":0.15,"value":1}]}},{"id":"pill_label","name":"Pill Label","kind":"figure","style":{"kind":"Square","width":0.35,"height":0.12,"cornerRadius":1.0,"fill":"#111827","stroke":"#FFFFFF","strokeWidth":0.002},"keys":{"opacity":[{"at":0,"value":0,"ease":"out"},{"at":1,"value":1}]}},{"id":"hex_badge","name":"Hex Badge","kind":"figure","style":{"kind":"Polygon","sides":6,"width":0.25,"fill":"#2563EB","stroke":"#60A5FA","strokeWidth":0.004},"keys":{"scale":[{"at":0,"value":0,"ease":"out"},{"at":1,"value":1}]}},{"id":"heart_pop","name":"Heart Pop","kind":"figure","style":{"kind":"Heart","width":0.25,"fill":"#EF476F"},"keys":{"scale":[{"at":0,"value":0.2,"ease":"out"},{"at":0.6,"value":1.15,"ease":"in"},{"at":1,"value":1}]}},{"id":"arrow_sweep","name":"Arrow Sweep","kind":"drawing","style":{"kind":"Arrow","width":0.5,"fill":"","stroke":"#FFFFFF","strokeWidth":0.008,"endHead":"triangle"},"trim":{"start":0,"end":1},"keys":{"offsetX":[{"at":0,"value":-0.4,"ease":"out"},{"at":1,"value":0.4}]}},{"id":"triangle_wipe","name":"Triangle Wipe","kind":"figure","style":{"kind":"Triangle","width":0.5,"fill":"#06D6A0"},"keys":{"offsetY":[{"at":0,"value":0.6,"ease":"out"},{"at":1,"value":0}]}},{"id":"glow_frame","name":"Glow Frame","kind":"drawing","style":{"kind":"Square","width":0.8,"height":0.5,"fill":"","stroke":"#FFFFFF","strokeWidth":0.003,"shadow":{"color":"#FFFFFF","blur":0.03}},"trim":{"start":0,"end":1},"keys":{"opacity":[{"at":0,"value":0},{"at":0.2,"value":1}]}},{"id":"orbit_dots","name":"Orbit Dots","kind":"compound","style":{"kind":"Circle","width":0.06,"fill":"#A78BFA"},"repeat":{"count":8,"mode":"radial","radius":0.3},"keys":{"rotation":[{"at":0,"value":0,"ease":"linear"},{"at":1,"value":360,"ease":"linear"}]}}]"##;

fn shapes() -> Vec<Value> { serde_json::from_str(SHAPES_JSON).unwrap_or_default() }

pub fn list_shapes() -> Value {
    let items: Vec<Value> = shapes().iter().map(|s| json!({
        "id": s["id"], "name": s["name"], "kind": s["kind"],
    })).collect();
    json!({ "shapes": items, "cpuOnly": true })
}

fn num(v: &Value, k: &str, d: f64) -> f64 { v.get(k).and_then(Value::as_f64).unwrap_or(d) }

fn keys(shape: &Value, start: f64, duration: f64, ox: f64, oy: f64) -> Vec<Value> {
    let mut out: Vec<Value> = Vec::new();
    if let Some(map) = shape.get("keys").and_then(Value::as_object) {
        for (prop, arr) in map {
            let Some(list) = arr.as_array() else { continue };
            for k in list {
                let at = ((start + num(k,"at",0.0)*duration) / duration.max(0.001)).clamp(0.0,1.0);
                out.push(json!({ "op":"setClipKey", "clipId":"@created", "property":prop,
                    "at":at, "value":num(k,"value",0.0), "ease":k.get("ease").and_then(Value::as_str).unwrap_or("linear") }));
            }
        }
    }
    if ox != 0.0 || oy != 0.0 {
        out.push(json!({ "op":"setClipKey", "clipId":"@created", "property":"offsetX",
            "at":0.0, "value":ox, "ease":"linear" }));
        out.push(json!({ "op":"setClipKey", "clipId":"@created", "property":"offsetY",
            "at":0.0, "value":oy, "ease":"linear" }));
    }
    out
}

pub fn compile(args: &Value) -> Value {
    let id = args.get("shapeId").and_then(Value::as_str).unwrap_or("");
    let start = num(args, "start", 0.0);
    let dur = num(args, "duration", 3.0).max(0.01);
    let Some(s) = shapes().into_iter().find(|x| x["id"] == id) else {
        return json!({ "op": "batch", "commands": [] });
    };
    let style = s.get("style").cloned().unwrap_or(json!({}));
    let mut commands: Vec<Value> = Vec::new();
    if let Some(rep) = s.get("repeat") {
        let count = num(rep, "count", 6.0).max(2.0) as usize;
        let radius = num(rep, "radius", 0.2);
        for i in 0..count {
            let a = 2.0 * PI * i as f64 / count as f64;
            commands.push(json!({ "op":"addShapeClip", "start":start, "duration":dur,
                "style":style, "name":format!("{} {}", s["name"], i+1) }));
            commands.extend(keys(&s, start, dur, radius*a.cos(), radius*a.sin()));
        }
    } else if s.get("trim").is_some() && s["kind"] == "drawing" {
        let tr = s.get("trim").unwrap();
        let a = num(tr, "start", 0.0); let b = num(tr, "end", 1.0);
        let steps = num(args, "steps", 8.0).max(1.0) as usize;
        for i in 0..steps {
            let u = (i + 1) as f64 / steps as f64;
            let mut part = style.clone();
            part["trimStart"] = json!(a);
            part["trimEnd"] = json!(a + (b - a) * u);
            let st = start + i as f64 * dur / steps as f64;
            commands.push(json!({ "op":"addShapeClip", "start":st, "duration":dur/steps as f64,
                "style":part, "name":format!("{} {}%", s["name"], (u*100.0) as usize) }));
            commands.extend(keys(&s, st, dur/steps as f64, 0.0, 0.0));
        }
    } else {
        commands.push(json!({ "op":"addShapeClip", "start":start, "duration":dur,
            "style":style, "name":s["name"] }));
        commands.extend(keys(&s, start, dur, 0.0, 0.0));
    }
    json!({ "op": "batch", "commands": commands })
}
