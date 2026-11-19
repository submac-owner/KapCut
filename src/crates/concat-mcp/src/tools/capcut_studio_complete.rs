// GENERATED FILE - do not edit. Run: python3 kapcut/studio/generate_studio_complete_rs.py
//! Studio completion: curve/graph engine, 3D camera-light-particle rigs,
//! and tracking/rotoscoping presets. All CPU data compiled to Concat commands.
use serde_json::{json, Value};

const CURVES_JSON: &str = r##"{"version":1,"interpolations":["linear","bezier","hold","easeIn","easeOut","easeInOut","elastic","bounce","spring","exponential"],"presets":[{"id":"smooth","name":"Smooth","ease":[0.42,0.0,0.58,1.0],"description":"Balanced ease in and out."},{"id":"snap","name":"Snap","ease":[0.2,0.9,0.3,1.0],"description":"Fast start, firm settle."},{"id":"glide","name":"Glide","ease":[0.25,0.1,0.25,1.0],"description":"Long, quiet deceleration."},{"id":"swift","name":"Swift","ease":[0.4,0.0,0.2,1.0],"description":"Quick acceleration then coast."},{"id":"settle","name":"Settle","ease":[0.34,1.56,0.64,1.0],"description":"Overshoot and settle."},{"id":"anticipate","name":"Anticipate","ease":[0.6,-0.5,0.4,1.0],"description":"Pull back before moving."},{"id":"expo_out","name":"Expo Out","ease":[0.16,1.0,0.3,1.0],"description":"Strong deceleration."},{"id":"expo_in","name":"Expo In","ease":[0.7,0.0,0.84,0.0],"description":"Strong acceleration."},{"id":"soft","name":"Soft","ease":[0.35,0.0,0.65,1.0],"description":"Gentle symmetric ease."},{"id":"linear","name":"Linear","ease":[0.0,0.0,1.0,1.0],"description":"Constant speed."},{"id":"hold","name":"Hold","ease":[1.0,0.0,1.0,0.0],"description":"Step at the end."},{"id":"pulse","name":"Pulse","ease":[0.4,0.0,0.6,1.0],"description":"Repeating eased cycle."}]}"##;
const THREE_D_JSON: &str = r##"{"version":1,"camera_rigs":[{"id":"dolly_in","name":"Dolly In","keys":[{"property":"scale","at":0.0,"value":1.0},{"property":"scale","at":1.0,"value":1.35}]},{"id":"dolly_out","name":"Dolly Out","keys":[{"property":"scale","at":0.0,"value":1.35},{"property":"scale","at":1.0,"value":1.0}]},{"id":"push_in","name":"Push In","keys":[{"property":"offsetZ","at":0.0,"value":-0.4},{"property":"offsetZ","at":1.0,"value":0.0}]},{"id":"pull_back","name":"Pull Back","keys":[{"property":"offsetZ","at":0.0,"value":0.0},{"property":"offsetZ","at":1.0,"value":-0.4}]},{"id":"orbit_left","name":"Orbit Left","keys":[{"property":"rotation","at":0.0,"value":8.0},{"property":"offsetX","at":0.5,"value":0.06},{"property":"rotation","at":1.0,"value":-8.0}]},{"id":"orbit_right","name":"Orbit Right","keys":[{"property":"rotation","at":0.0,"value":-8.0},{"property":"offsetX","at":0.5,"value":-0.06},{"property":"rotation","at":1.0,"value":8.0}]},{"id":"crane_up","name":"Crane Up","keys":[{"property":"offsetY","at":0.0,"value":0.3},{"property":"offsetY","at":1.0,"value":-0.3}]},{"id":"crane_down","name":"Crane Down","keys":[{"property":"offsetY","at":0.0,"value":-0.3},{"property":"offsetY","at":1.0,"value":0.3}]},{"id":"handheld","name":"Handheld","keys":[{"property":"offsetX","at":0.0,"value":0.01},{"property":"offsetY","at":0.3,"value":-0.015},{"property":"rotation","at":0.6,"value":0.6},{"property":"offsetX","at":1.0,"value":-0.01}]},{"id":"rack_focus","name":"Rack Focus","keys":[{"property":"blur","at":0.0,"value":0.0},{"property":"blur","at":0.5,"value":0.6},{"property":"blur","at":1.0,"value":0.0}]}],"light_rigs":[{"id":"three_point","name":"Three Point","layers":[{"kind":"effect","id":"light","params":{"key":0.8,"fill":0.4,"rim":0.6}}]},{"id":"noir","name":"Noir","layers":[{"kind":"effect","id":"light","params":{"key":0.9,"fill":0.1,"rim":0.7}}]},{"id":"soft_studio","name":"Soft Studio","layers":[{"kind":"effect","id":"light","params":{"key":0.5,"fill":0.5,"rim":0.2}}]},{"id":"golden_hour","name":"Golden Hour","layers":[{"kind":"effect","id":"light","params":{"key":0.7,"fill":0.3,"warm":0.8}}]},{"id":"backlit","name":"Backlit","layers":[{"kind":"effect","id":"light","params":{"key":0.2,"fill":0.1,"rim":1.0}}]}],"particle_fields":[{"id":"dust","name":"Dust","params":{"count":60,"size":0.01,"speed":0.05}},{"id":"sparks","name":"Sparks","params":{"count":40,"size":0.008,"speed":0.2}},{"id":"snow","name":"Snow","params":{"count":90,"size":0.012,"speed":0.08}},{"id":"bokeh","name":"Bokeh","params":{"count":30,"size":0.04,"speed":0.02}},{"id":"embers","name":"Embers","params":{"count":50,"size":0.01,"speed":0.12}},{"id":"stars","name":"Stars","params":{"count":120,"size":0.006,"speed":0.01}},{"id":"rain","name":"Rain","params":{"count":100,"size":0.005,"speed":0.4}},{"id":"smoke","name":"Smoke","params":{"count":25,"size":0.06,"speed":0.03}}],"composite_modes":[{"id":"normal","name":"Normal","blend":"normal"},{"id":"screen","name":"Screen","blend":"screen"},{"id":"multiply","name":"Multiply","blend":"multiply"},{"id":"overlay","name":"Overlay","blend":"overlay"},{"id":"add","name":"Add","blend":"add"},{"id":"soft_light","name":"Soft Light","blend":"soft-light"},{"id":"hard_light","name":"Hard Light","blend":"hard-light"},{"id":"difference","name":"Difference","blend":"difference"}]}"##;
const TRACKING_JSON: &str = r##"{"version":1,"trackers":[{"id":"point","name":"Point Track","points":1,"description":"Track one feature and follow it."},{"id":"two_point","name":"Two Point","points":2,"description":"Track scale and rotation from two features."},{"id":"corner_pin","name":"Corner Pin","points":4,"description":"Track a surface with four corners."},{"id":"planar","name":"Planar","points":4,"description":"Track a flat surface with perspective."}],"roto_shapes":[{"id":"bezier","name":"Bezier Shape","kind":"custom","points":4},{"id":"ellipse","name":"Ellipse","kind":"ellipse","points":0},{"id":"rectangle","name":"Rectangle","kind":"rectangle","points":4},{"id":"polygon","name":"Polygon","kind":"custom","points":6}],"smoothing":[{"id":"steady","name":"Steady","strength":0.4},{"id":"smooth","name":"Smooth","strength":0.7},{"id":"lock","name":"Lock","strength":0.95}]}"##;

fn curves() -> Value { serde_json::from_str(CURVES_JSON).unwrap_or(json!({})) }
fn three_d() -> Value { serde_json::from_str(THREE_D_JSON).unwrap_or(json!({})) }
fn tracking() -> Value { serde_json::from_str(TRACKING_JSON).unwrap_or(json!({})) }
fn s(v: &Value, k: &str) -> String { v.get(k).and_then(Value::as_str).unwrap_or("").to_string() }
fn n(v: &Value, k: &str, d: f64) -> f64 { v.get(k).and_then(Value::as_f64).unwrap_or(d) }
fn arr(v: &Value, k: &str) -> Vec<Value> { v.get(k).and_then(Value::as_array).cloned().unwrap_or_default() }
fn find(v: &Value, list: &str, id: &str) -> Option<Value> {
    arr(v, list).into_iter().find(|x| s(x, "id") == id)
}
fn ease_of(id: &str) -> Vec<f64> {
    find(&curves(), "presets", id).map(|p| arr(&p, "ease").iter().filter_map(Value::as_f64).collect())
        .unwrap_or_else(|| vec![0.42, 0.0, 0.58, 1.0])
}
fn bezier(a: f64, b: f64, u: f64) -> f64 {
    let v = 1.0 - u;
    3.0 * v * v * u * a + 3.0 * v * u * u * b + u * u * u
}
fn sample(ease: &[f64], t: f64) -> f64 {
    if ease.len() < 4 { return t; }
    let (x1, y1, x2, y2) = (ease[0], ease[1], ease[2], ease[3]);
    let mut lo = 0.0; let mut hi = 1.0;
    for _ in 0..24 {
        let mid = (lo + hi) / 2.0;
        if bezier(x1, x2, mid) < t { lo = mid; } else { hi = mid; }
    }
    let u = (lo + hi) / 2.0;
    bezier(y1, y2, u)
}

pub fn list_curves() -> Value {
    let items: Vec<Value> = arr(&curves(), "presets").iter().map(|p| json!({
        "id": s(p, "id"), "name": s(p, "name"), "ease": p.get("ease"),
        "description": s(p, "description")
    })).collect();
    json!({ "curves": items, "count": items.len(), "interpolations": curves().get("interpolations") })
}
pub fn list_interpolations() -> Value {
    curves().get("interpolations").cloned().unwrap_or(json!([]))
}
pub fn curve_sample(args: &Value) -> Value {
    let ease = ease_of(&s(args, "curve"));
    let count = n(args, "count", 10.0).max(2.0) as usize;
    let samples: Vec<f64> = (0..count).map(|i| sample(&ease, i as f64 / (count - 1) as f64)).collect();
    json!({ "curve": s(args, "curve"), "samples": samples })
}
pub fn curve_keys(args: &Value) -> Value {
    let ease = ease_of(&s(args, "curve"));
    let count = n(args, "count", 8.0).max(2.0) as usize;
    let from = n(args, "from", 0.0); let to = n(args, "to", 1.0);
    let prop = s(args, "property"); let clip = s(args, "clipId");
    let mut commands = Vec::new();
    for i in 0..count {
        let t = i as f64 / (count - 1) as f64;
        commands.push(json!({ "op": "setClipKey", "clipId": clip, "property": prop,
            "at": t, "value": from + (to - from) * sample(&ease, t) }));
    }
    json!({ "op": "batch", "commands": commands })
}
pub fn curve_smooth(args: &Value) -> Value {
    let strength = n(args, "strength", 0.5);
    json!({ "op": "batch", "commands": [json!({ "op": "setClipKeySmooth",
        "clipId": s(args, "clipId"), "property": s(args, "property"), "strength": strength })] })
}

pub fn list_camera_rigs() -> Value {
    let items: Vec<Value> = arr(&three_d(), "camera_rigs").iter()
        .map(|r| json!({ "id": s(r, "id"), "name": s(r, "name") })).collect();
    json!({ "rigs": items, "count": items.len() })
}
pub fn camera_keys(args: &Value) -> Value {
    let Some(rig) = find(&three_d(), "camera_rigs", &s(args, "rig")) else {
        return json!({ "op": "batch", "commands": [] });
    };
    let clip = s(args, "clipId"); let ease = ease_of(&s(args, "curve"));
    let mut commands = Vec::new();
    for key in arr(&rig, "keys") {
        commands.push(json!({ "op": "setClipKey", "clipId": clip,
            "property": s(&key, "property"), "at": n(&key, "at", 0.0),
            "value": n(&key, "value", 0.0), "ease": ease }));
    }
    json!({ "op": "batch", "commands": commands })
}
pub fn list_light_rigs() -> Value {
    let items: Vec<Value> = arr(&three_d(), "light_rigs").iter()
        .map(|r| json!({ "id": s(r, "id"), "name": s(r, "name") })).collect();
    json!({ "rigs": items, "count": items.len() })
}
pub fn light_apply(args: &Value) -> Value {
    let Some(rig) = find(&three_d(), "light_rigs", &s(args, "rig")) else {
        return json!({ "op": "batch", "commands": [] });
    };
    let mut commands = Vec::new();
    for layer in arr(&rig, "layers") {
        commands.push(json!({ "op": "addLayerClip", "start": n(args, "start", 0.0),
            "duration": n(args, "duration", 5.0), "effectId": s(&layer, "id"),
            "params": layer.get("params"), "name": s(&rig, "name") }));
    }
    json!({ "op": "batch", "commands": commands })
}
pub fn list_particles() -> Value {
    let items: Vec<Value> = arr(&three_d(), "particle_fields").iter()
        .map(|p| json!({ "id": s(p, "id"), "name": s(p, "name"), "params": p.get("params") })).collect();
    json!({ "particles": items, "count": items.len() })
}
pub fn particle_add(args: &Value) -> Value {
    let Some(p) = find(&three_d(), "particle_fields", &s(args, "particle")) else {
        return json!({ "op": "batch", "commands": [] });
    };
    json!({ "op": "batch", "commands": [json!({ "op": "addLayerClip",
        "start": n(args, "start", 0.0), "duration": n(args, "duration", 5.0),
        "effectId": s(&p, "id"), "params": p.get("params"), "name": s(&p, "name") })] })
}
pub fn list_composite() -> Value {
    let items: Vec<Value> = arr(&three_d(), "composite_modes").iter()
        .map(|m| json!({ "id": s(m, "id"), "name": s(m, "name"), "blend": s(m, "blend") })).collect();
    json!({ "modes": items, "count": items.len() })
}
pub fn composite_layer(args: &Value) -> Value {
    let mode = find(&three_d(), "composite_modes", &s(args, "mode"))
        .map(|m| s(&m, "blend")).unwrap_or_else(|| "normal".to_string());
    json!({ "op": "updateClip", "clipId": s(args, "clipId"),
        "patch": { "blend": mode } })
}

pub fn list_trackers() -> Value {
    let items: Vec<Value> = arr(&tracking(), "trackers").iter()
        .map(|t| json!({ "id": s(t, "id"), "name": s(t, "name"), "points": n(t, "points", 0.0),
                         "description": s(t, "description") })).collect();
    json!({ "trackers": items, "count": items.len() })
}
pub fn track_apply(args: &Value) -> Value {
    let points = arr(args, "points");
    let mut commands = Vec::new();
    for p in points {
        commands.push(json!({ "op": "setClipKey", "clipId": s(args, "clipId"),
            "property": s(&p, "property"), "at": n(&p, "at", 0.0).clamp(0.0, 1.0),
            "value": n(&p, "value", 0.0) }));
    }
    json!({ "op": "batch", "commands": commands })
}
pub fn list_roto_shapes() -> Value {
    let items: Vec<Value> = arr(&tracking(), "roto_shapes").iter()
        .map(|r| json!({ "id": s(r, "id"), "name": s(r, "name"), "kind": s(r, "kind") })).collect();
    json!({ "shapes": items, "count": items.len() })
}
pub fn roto_shape(args: &Value) -> Value {
    let shape = find(&tracking(), "roto_shapes", &s(args, "shape"));
    let kind = shape.as_ref().map(|x| s(x, "kind")).unwrap_or_else(|| "custom".to_string());
    let cutout = json!({ "source": { "kind": kind }, "feather": n(args, "feather", 0.02),
        "strokes": arr(args, "strokes") });
    json!({ "op": "setClipCutout", "clipId": s(args, "clipId"), "cutout": cutout })
}
pub fn motion_stabilize(args: &Value) -> Value {
    let level = find(&tracking(), "smoothing", &s(args, "level"));
    let strength = level.as_ref().map(|x| n(x, "strength", 0.5)).unwrap_or(0.5);
    json!({ "op": "setClipStabilize", "clipId": s(args, "clipId"),
        "stabilize": { "smoothness": strength, "maxZoom": n(args, "maxZoom", 1.1),
                       "tripod": args.get("tripod").and_then(Value::as_bool).unwrap_or(false) } })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn curves_and_rigs_are_populated() {
        assert!(arr(&curves(), "presets").len() >= 12);
        assert!(arr(&three_d(), "camera_rigs").len() >= 10);
        assert!(arr(&three_d(), "particle_fields").len() >= 8);
        assert!(arr(&tracking(), "trackers").len() >= 4);
        assert_eq!(s(&find(&curves(), "presets", "smooth").unwrap(), "name"), "Smooth");
    }
    #[test]
    fn curve_sampling_and_key_compilation() {
        let e = ease_of("smooth");
        let v0 = sample(&e, 0.0); let v1 = sample(&e, 1.0);
        assert!((v0 - 0.0).abs() < 0.01 && (v1 - 1.0).abs() < 0.01);
        let cmd = curve_keys(&json!({ "clipId": "c1", "property": "scale", "from": 0.0, "to": 1.0, "count": 5 }));
        assert_eq!(arr(&cmd, "commands").len(), 5);
    }
    #[test]
    fn rig_compiles_to_real_commands() {
        let cmd = camera_keys(&json!({ "clipId": "c1", "rig": "dolly_in" }));
        assert_eq!(arr(&cmd, "commands").len(), 2);
        let stab = motion_stabilize(&json!({ "clipId": "c1", "level": "smooth" }));
        assert_eq!(s(&stab, "op"), "setClipStabilize");
    }
}
