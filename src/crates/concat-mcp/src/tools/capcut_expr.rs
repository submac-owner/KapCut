// GENERATED FILE - do not edit. Run: python3 kapcut/expressions/generate_expr_rs.py
use serde_json::{json, Value};
use std::f64::consts::PI;

const EXPRESSIONS_JSON: &str = r#"[{"id":"wiggle","name":"Wiggle","category":"procedural","description":"Smooth pseudo-random oscillation. Seeded, so it is reproducible.","params":{"frequency":2.0,"amplitude":0.05,"seed":1},"samples":32,"ease":"linear"},{"id":"sine","name":"Sine Wave","category":"wave","description":"Pure sine motion for loops, pulses and oscillation.","params":{"frequency":1.0,"amplitude":0.1,"phase":0.0},"samples":32,"ease":"inOut"},{"id":"damped_sine","name":"Damped Sine","category":"wave","description":"Bounce that settles - spring, recoil, elastic ease.","params":{"frequency":3.0,"amplitude":0.25,"damping":4.0},"samples":40,"ease":"linear"},{"id":"pulse","name":"Pulse","category":"step","description":"Duty-cycle pulse between two values - strobes and blinks.","params":{"period":0.5,"duty":0.5,"high":1.0,"low":0.0},"samples":48,"ease":"linear"},{"id":"random_steps","name":"Random Steps","category":"step","description":"Seeded random values held between steps - glitch and vernacular motion.","params":{"interval":0.25,"min":-0.1,"max":0.1,"seed":7},"samples":40,"ease":"linear"},{"id":"drift","name":"Drift","category":"ramp","description":"Linear ramp from start to start+slope*time.","params":{"start":0.0,"slope":0.02},"samples":8,"ease":"linear"},{"id":"loop_out","name":"Loop Out","category":"loop","description":"Repeat a source span; pingpong alternates direction.","params":{"cycle":1.0,"mode":"cycle","from":0.0,"to":1.0},"samples":48,"ease":"inOut"},{"id":"follow_path","name":"Follow Path","category":"path","description":"Move along control points; compiles to offsetX + offsetY keyframes.","params":{"points":[[0,0],[0.3,0.2],[-0.2,-0.1],[0,0]]},"samples":40,"ease":"inOut"}]"#;

fn expressions() -> Vec<Value> { serde_json::from_str(EXPRESSIONS_JSON).unwrap_or_default() }

pub fn list_expressions() -> Value {
    let items: Vec<Value> = expressions().iter().map(|e| json!({
        "id": e["id"], "name": e["name"],
        "category": e["category"], "description": e["description"],
    })).collect();
    json!({ "expressions": items, "cpuOnly": true, "runtime": "sampled-keyframes" })
}

struct Rng { s: u64 }
impl Rng {
    fn new(seed: i64) -> Self { Self { s: ((seed as u64) & 0x7fff_ffff).max(1) } }
    fn next(&mut self) -> f64 {
        self.s = (1_103_515_245u64.wrapping_mul(self.s).wrapping_add(12345)) & 0x7fff_ffff;
        self.s as f64 / 0x7fff_ffff as f64
    }
}

fn num(p: &Value, k: &str, d: f64) -> f64 { p.get(k).and_then(Value::as_f64).unwrap_or(d) }

fn value_at(id: &str, t: f64, p: &Value, rng: &mut Rng) -> f64 {
    match id {
        "wiggle" => num(p,"amplitude",0.05) * (2.0*PI*num(p,"frequency",2.0)*t + rng.next()*0.4).sin(),
        "sine" => num(p,"amplitude",0.1) * (2.0*PI*num(p,"frequency",1.0)*t + num(p,"phase",0.0)).sin(),
        "damped_sine" => num(p,"amplitude",0.25) * (-num(p,"damping",4.0)*t).exp()
            * (2.0*PI*num(p,"frequency",3.0)*t).sin(),
        "pulse" => {
            let per = num(p,"period",0.5).max(0.01);
            let duty = num(p,"duty",0.5);
            if (t % per) / per < duty { num(p,"high",1.0) } else { num(p,"low",0.0) }
        }
        "random_steps" => {
            let iv = num(p,"interval",0.25).max(0.01);
            let lo = num(p,"min",-0.1); let hi = num(p,"max",0.1);
            let _step = (t / iv) as i64;
            lo + (hi - lo) * rng.next()
        }
        "drift" => num(p,"start",0.0) + num(p,"slope",0.02) * t,
        "loop_out" => {
            let cyc = num(p,"cycle",1.0).max(0.01);
            let mut a = num(p,"from",0.0); let b = num(p,"to",1.0);
            let mut phase = (t % cyc) / cyc;
            match p.get("mode").and_then(Value::as_str).unwrap_or("cycle") {
                "pingpong" => phase = 1.0 - (1.0 - 2.0*phase).abs(),
                "offset" => a += (t / cyc).floor() * (b - a),
                _ => {}
            }
            a + (b - a) * phase
        }
        _ => 0.0,
    }
}

fn path_point(points: &Value, u: f64) -> (f64, f64) {
    let Some(arr) = points.as_array() else { return (0.0, 0.0) };
    if arr.is_empty() { return (0.0, 0.0) }
    if arr.len() == 1 {
        return (arr[0][0].as_f64().unwrap_or(0.0), arr[0][1].as_f64().unwrap_or(0.0));
    }
    let pos = u * (arr.len() - 1) as f64;
    let seg = (pos as usize).min(arr.len() - 2);
    let local = pos - seg as f64;
    let a = &arr[seg]; let b = &arr[seg + 1];
    let ax = a[0].as_f64().unwrap_or(0.0); let ay = a[1].as_f64().unwrap_or(0.0);
    let bx = b[0].as_f64().unwrap_or(0.0); let by = b[1].as_f64().unwrap_or(0.0);
    (ax + (bx - ax) * local, ay + (by - ay) * local)
}

pub fn compile(args: &Value) -> Value {
    let id = args.get("expressionId").and_then(Value::as_str).unwrap_or("");
    let clip = args.get("clipId").and_then(Value::as_str).unwrap_or("");
    let dur = num(args, "clipDuration", 1.0).max(0.001);
    let property = args.get("property").and_then(Value::as_str).unwrap_or("scale");
    let start = num(args, "start", 0.0);
    let Some(e) = expressions().into_iter().find(|x| x["id"] == id) else {
        return json!({ "op": "batch", "commands": [] });
    };
    let mut p = e.get("params").cloned().unwrap_or(json!({}));
    if let Some(obj) = args.get("params").and_then(Value::as_object) {
        for (k, v) in obj { p[k.clone()] = v.clone(); }
    }
    let n = (num(&e, "samples", 32.0) as usize).max(2);
    let ease = e.get("ease").and_then(Value::as_str).unwrap_or("linear");
    let mut rng = Rng::new(num(&p, "seed", 1.0) as i64);
    let mut commands: Vec<Value> = Vec::new();
    if id == "follow_path" {
        for i in 0..n {
            let u = i as f64 / (n - 1) as f64;
            let (x, y) = path_point(p.get("points").unwrap_or(&Value::Null), u);
            let at = ((start + u * dur) / dur).clamp(0.0, 1.0);
            commands.push(json!({ "op": "setClipKey", "clipId": clip, "property": "offsetX",
                "at": at, "value": x, "ease": ease }));
            commands.push(json!({ "op": "setClipKey", "clipId": clip, "property": "offsetY",
                "at": at, "value": y, "ease": ease }));
        }
    } else {
        for i in 0..n {
            let u = i as f64 / (n - 1) as f64;
            let t = start + u * dur;
            let v = value_at(id, t, &p, &mut rng);
            let at = (t / dur).clamp(0.0, 1.0);
            commands.push(json!({ "op": "setClipKey", "clipId": clip, "property": property,
                "at": at, "value": v, "ease": ease }));
        }
    }
    json!({ "op": "batch", "commands": commands })
}
