// GENERATED FILE - do not edit. Run: python3 kapcut/motion/generate_motion_rs.py
use serde_json::{json, Value};

const PRESETS_JSON: &str = r#"[{"id":"slice_rebuild","name":"Slice Rebuild","kind":"in","duration":1.0,"tracks":[{"property":"opacity","keys":[{"at":0,"value":0,"ease":"out"},{"at":1,"value":1}]},{"property":"scale","keys":[{"at":0,"value":1.35,"ease":"out"},{"at":1,"value":1}]},{"property":"offsetX","keys":[{"at":0,"value":-0.12,"ease":"out"},{"at":1,"value":0}]}],"ffmpeg":[{"filter":"fade","args":"t=in:st=0:d=1"},{"filter":"scale","args":"iw*1.35:ih*1.35"}]},{"id":"glare_shrink","name":"Glare Shrink","kind":"out","duration":1.2,"tracks":[{"property":"opacity","keys":[{"at":0,"value":1},{"at":1,"value":0,"ease":"in"}]},{"property":"scale","keys":[{"at":0,"value":1},{"at":1,"value":0.78,"ease":"in"}]},{"property":"rotation","keys":[{"at":0,"value":0},{"at":1,"value":-6,"ease":"in"}]}],"ffmpeg":[{"filter":"fade","args":"t=out:st=0:d=1.2"},{"filter":"scale","args":"iw*0.78:ih*0.78"}]},{"id":"popping_candy","name":"Popping Candy","kind":"loop","duration":1.0,"tracks":[{"property":"scale","keys":[{"at":0,"value":1,"ease":"inOut"},{"at":0.5,"value":1.14,"ease":"inOut"},{"at":1,"value":1}]},{"property":"rotation","keys":[{"at":0,"value":-2,"ease":"inOut"},{"at":0.5,"value":2,"ease":"inOut"},{"at":1,"value":-2}]}],"ffmpeg":[{"filter":"zoompan","args":"z='min(zoom+0.0008,1.14)':d=1:s=iwxih"}]},{"id":"fade_in_up","name":"Fade In Up","kind":"in","duration":0.8,"tracks":[{"property":"opacity","keys":[{"at":0,"value":0,"ease":"out"},{"at":1,"value":1}]},{"property":"offsetY","keys":[{"at":0,"value":0.18,"ease":"out"},{"at":1,"value":0}]}],"ffmpeg":[{"filter":"fade","args":"t=in:st=0:d=0.8"}]},{"id":"bounce_in","name":"Bounce In","kind":"in","duration":1.0,"tracks":[{"property":"scale","keys":[{"at":0,"value":0.2,"ease":"out"},{"at":0.55,"value":1.12,"ease":"in"},{"at":0.8,"value":0.96,"ease":"out"},{"at":1,"value":1}]},{"property":"opacity","keys":[{"at":0,"value":0},{"at":0.3,"value":1}]}],"ffmpeg":[{"filter":"scale","args":"iw*1.12:ih*1.12"}]},{"id":"typewriter","name":"Typewriter","kind":"in","duration":1.4,"text":true,"tracks":[{"property":"opacity","keys":[{"at":0,"value":0},{"at":0.02,"value":1}]},{"property":"offsetX","keys":[{"at":0,"value":-0.03,"ease":"out"},{"at":1,"value":0}]}],"ffmpeg":[{"filter":"drawtext","args":"text='Hello':fontsize=48:alpha='if(lt(t,1.4),1,0)'"}]},{"id":"neon_pulse","name":"Neon Pulse","kind":"loop","duration":1.5,"effect":"concat.neon","tracks":[{"property":"opacity","keys":[{"at":0,"value":0.85,"ease":"inOut"},{"at":0.5,"value":1,"ease":"inOut"},{"at":1,"value":0.85}]}],"ffmpeg":[{"filter":"eq","args":"brightness=0.05*sin(2*PI*t/1.5)"}]},{"id":"whip_pan","name":"Whip Pan","kind":"in","duration":0.5,"tracks":[{"property":"offsetX","keys":[{"at":0,"value":-0.9,"ease":"in"},{"at":1,"value":0,"ease":"out"}]},{"property":"rotation","keys":[{"at":0,"value":3},{"at":1,"value":0}]}],"ffmpeg":[{"filter":"crop","args":"iw/2:ih:(iw/2)*sin(2*PI*t/0.5):0"}]},{"id":"ken_burns","name":"Ken Burns","kind":"loop","duration":4.0,"tracks":[{"property":"scale","keys":[{"at":0,"value":1,"ease":"inOut"},{"at":1,"value":1.18}]},{"property":"offsetX","keys":[{"at":0,"value":-0.04,"ease":"inOut"},{"at":1,"value":0.04}]},{"property":"offsetY","keys":[{"at":0,"value":0.03,"ease":"inOut"},{"at":1,"value":-0.03}]}],"ffmpeg":[{"filter":"zoompan","args":"z='min(zoom+0.0005,1.18)':x='iw/2-(iw/zoom/2)':y='ih/2-(ih/zoom/2)':d=1:s=iwxih"}]},{"id":"kinetic_words","name":"Kinetic Words","kind":"in","duration":2.0,"text":true,"tracks":[{"property":"offsetY","keys":[{"at":0,"value":0.25,"ease":"out"},{"at":1,"value":0}]},{"property":"opacity","keys":[{"at":0,"value":0},{"at":0.2,"value":1}]},{"property":"rotation","keys":[{"at":0,"value":-4,"ease":"out"},{"at":1,"value":0}]}],"ffmpeg":[{"filter":"drawtext","args":"text='Hello':fontsize=64:y='h/2+40*sin(2*PI*t/2)'"}]},{"id":"glitch_in","name":"Glitch In","kind":"in","duration":0.7,"effect":"concat.glitch-warp","tracks":[{"property":"offsetX","keys":[{"at":0,"value":0.08,"ease":"out"},{"at":0.5,"value":-0.04},{"at":1,"value":0}]},{"property":"opacity","keys":[{"at":0,"value":0.3},{"at":0.1,"value":1}]}],"ffmpeg":[{"filter":"noise","args":"alls=20:seed=1"}]},{"id":"smooth_zoom_loop","name":"Smooth Zoom Loop","kind":"loop","duration":3.0,"tracks":[{"property":"scale","keys":[{"at":0,"value":1,"ease":"inOut"},{"at":0.5,"value":1.1,"ease":"inOut"},{"at":1,"value":1}]}],"ffmpeg":[{"filter":"zoompan","args":"z='1+0.1*sin(2*PI*t/3)':d=1:s=iwxih"}]}]"#;

fn presets() -> Vec<Value> {
    serde_json::from_str(PRESETS_JSON).unwrap_or_default()
}

pub fn list_presets() -> Value {
    let items: Vec<Value> = presets().iter().map(|p| json!({
        "id": p["id"], "name": p["name"],
        "kind": p["kind"], "duration": p["duration"],
    })).collect();
    json!({ "presets": items, "cpuOnly": true })
}

fn num(v: &Value, key: &str, default: f64) -> f64 {
    v.get(key).and_then(Value::as_f64).unwrap_or(default)
}

pub fn compile(args: &Value) -> Value {
    let pid = args.get("presetId").and_then(Value::as_str).unwrap_or("");
    let clip = args.get("clipId").and_then(Value::as_str).unwrap_or("");
    let clip_dur = num(args, "clipDuration", 1.0).max(0.001);
    let start = num(args, "start", 0.0);
    let loops = num(args, "loops", 1.0).max(1.0) as usize;
    let Some(preset) = presets().into_iter().find(|p| p["id"] == pid) else {
        return json!({ "op": "batch", "commands": [] });
    };
    let pd = num(&preset, "duration", 1.0);
    let cycles = if preset["kind"] == "loop" { loops } else { 1 };
    let mut commands: Vec<Value> = Vec::new();
    if let Some(tracks) = preset.get("tracks").and_then(Value::as_array) {
        for track in tracks {
            let prop = track["property"].as_str().unwrap_or("");
            let Some(keys) = track.get("keys").and_then(Value::as_array) else { continue };
            for cycle in 0..cycles {
                for key in keys {
                    let t = start + (cycle as f64 + num(key, "at", 0.0)) * pd;
                    let at = (t / clip_dur).clamp(0.0, 1.0);
                    commands.push(json!({
                        "op": "setClipKey", "clipId": clip, "property": prop,
                        "at": at, "value": num(key, "value", 0.0),
                        "ease": key.get("ease").and_then(Value::as_str).unwrap_or("linear"),
                    }));
                }
            }
        }
    }
    if let Some(effect) = preset.get("effect").and_then(Value::as_str) {
        commands.push(json!({
            "op": "addLayerClip", "start": start, "duration": pd * cycles as f64,
            "effectId": effect, "name": preset["name"], "sound": false,
        }));
    }
    json!({ "op": "batch", "commands": commands })
}

pub fn ffmpeg_graph(args: &Value) -> Value {
    let pid = args.get("presetId").and_then(Value::as_str).unwrap_or("");
    let start = num(args, "start", 0.0);
    let Some(preset) = presets().into_iter().find(|p| p["id"] == pid) else {
        return json!({ "graph": "null", "cpuOnly": true });
    };
    let mut parts: Vec<String> = Vec::new();
    if let Some(list) = preset.get("ffmpeg").and_then(Value::as_array) {
        for f in list {
            let mut a = f["args"].as_str().unwrap_or("").to_string();
            if start != 0.0 { a = a.replace("st=0", &format!("st={}", start)); }
            parts.push(format!("{}={}", f["filter"].as_str().unwrap_or("null"), a));
        }
    }
    json!({ "graph": if parts.is_empty() { "null".to_string() } else { parts.join(",") }, "cpuOnly": true })
}
