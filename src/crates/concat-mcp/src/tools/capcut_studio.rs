// GENERATED FILE - do not edit. Run: python3 kapcut/studio/generate_studio_rs.py
use serde_json::{json, Value};

const TEMPLATES_JSON: &str = r##"[{"id":"kinetic_title","name":"Kinetic Title Reveal","duration":8,"palette":"midnight","background":{"kind":"square","width":2.0,"height":2.0,"fill":"#070B14"},"elements":[{"type":"shape","kind":"circle","width":0.9,"fill":"","stroke":"#00E5FF","strokeWidth":0.002,"at":0.4,"duration":6.5,"anim":"draw_on","camera":true},{"type":"shape","kind":"circle","width":0.6,"fill":"#111A2E","stroke":"#7C5CFC","strokeWidth":0.003,"shadow":{"color":"#7C5CFC","blur":0.08},"at":0.8,"duration":6,"anim":"bounce_in","stagger":0.2},{"type":"text","content":"MOTION","font":"display","size":"display","color":"#F5F7FF","at":1.2,"duration":5.5,"anim":"typewriter","y":0.36},{"type":"text","content":"DESIGN SYSTEM","font":"heading","size":"caption","color":"#00E5FF","at":2.0,"duration":4.5,"anim":"fade_in_up","y":0.52},{"type":"shape","kind":"line","width":0.5,"stroke":"#FF3B6B","strokeWidth":0.002,"at":2.4,"duration":4,"anim":"draw_on","y":0.62}],"effects":[{"id":"concat.glow","at":0,"duration":8},{"id":"concat.film-grain","at":0,"duration":8}],"camera":{"scale":[[0,1.0],[0.5,1.04],[1,1.0]],"offsetX":[[0,-0.02],[1,0.02]]}},{"id":"particle_reveal","name":"Particle Field Reveal","duration":10,"palette":"violet","background":{"kind":"square","width":2.0,"height":2.0,"fill":"#0B0716"},"elements":[{"type":"particlefield","count":14,"kind":"circle","size":0.02,"fill":"#B388FF","at":0.3,"duration":9,"anim":"float","seed":11},{"type":"shape","kind":"star","points":6,"innerRadius":0.4,"width":0.7,"fill":"#1C1133","stroke":"#B388FF","strokeWidth":0.003,"at":1.5,"duration":7,"anim":"bounce_in"},{"type":"text","content":"AGENTIC","font":"display","size":"display","color":"#F6F0FF","at":2.0,"duration":6,"anim":"glitch_in","y":0.36},{"type":"text","content":"VIDEO PRODUCTION","font":"mono","size":"caption","color":"#00E5FF","at":3.0,"duration":5,"anim":"fade_in_up","y":0.52}],"effects":[{"id":"concat.neon","at":1.5,"duration":7},{"id":"concat.chromatic-aberration","at":2,"duration":6}],"camera":{"scale":[[0,0.96],[0.5,1.06],[1,1.0]]}},{"id":"color_grade_showcase","name":"Color Grade Showcase","duration":12,"palette":"ember","background":{"kind":"square","width":2.0,"height":2.0,"fill":"#120806"},"elements":[{"type":"shape","kind":"square","width":1.2,"height":0.7,"fill":"#2A140F","stroke":"#FF6B35","strokeWidth":0.002,"at":0.5,"duration":10,"anim":"fade_in_up","y":0.1},{"type":"text","content":"GRADE","font":"display","size":"display","color":"#FFD166","at":1.0,"duration":9,"anim":"bounce_in","y":0.36},{"type":"text","content":"TEAL · ORANGE · NOIR · VINTAGE","font":"heading","size":"body","color":"#FFF4E6","at":2.0,"duration":8,"anim":"typewriter","y":0.55},{"type":"shape","kind":"line","width":0.8,"stroke":"#FFD166","strokeWidth":0.003,"at":2.5,"duration":7,"anim":"draw_on","y":0.68}],"effects":[{"id":"concat.teal-orange","at":0,"duration":4},{"id":"concat.noir","at":4,"duration":4},{"id":"concat.vintage","at":8,"duration":4}],"camera":{"offsetX":[[0,-0.03],[1,0.03]],"scale":[[0,1.02],[1,1.08]]}},{"id":"logo_lockup","name":"Logo Lockup / End Card","duration":6,"palette":"forest","background":{"kind":"square","width":2.0,"height":2.0,"fill":"#04120D"},"elements":[{"type":"shape","kind":"circle","width":0.5,"fill":"","stroke":"#06D6A0","strokeWidth":0.005,"shadow":{"color":"#06D6A0","blur":0.06},"at":0.3,"duration":5,"anim":"draw_on"},{"type":"text","content":"KAPCUT","font":"display","size":"display","color":"#EAFFF8","at":0.8,"duration":4.5,"anim":"bounce_in","y":0.38},{"type":"text","content":"STUDIO","font":"heading","size":"h2","color":"#06D6A0","at":1.4,"duration":4,"anim":"fade_in_up","y":0.55},{"type":"shape","kind":"line","width":0.3,"stroke":"#118AB2","strokeWidth":0.002,"at":1.8,"duration":3.5,"anim":"draw_on","y":0.66}],"effects":[{"id":"concat.halation","at":0,"duration":6}],"camera":{"scale":[[0,1.1],[0.5,1.0],[1,1.02]]}}]"##;
const DESIGN_JSON: &str = r##"{"schemaVersion":1,"name":"KapCut Studio Design System","note":"Design tokens for studio-grade compositions. Everything maps to Concat's real shape/text/effect model; no GPU.","palettes":{"midnight":{"bg":"#070B14","surface":"#111A2E","primary":"#00E5FF","secondary":"#7C5CFC","accent":"#FF3B6B","text":"#F5F7FF","muted":"#8FA3C8"},"ember":{"bg":"#120806","surface":"#2A140F","primary":"#FF6B35","secondary":"#FFD166","accent":"#EF476F","text":"#FFF4E6","muted":"#C9A88E"},"forest":{"bg":"#04120D","surface":"#0C2A1F","primary":"#06D6A0","secondary":"#118AB2","accent":"#FFD166","text":"#EAFFF8","muted":"#8FBFAE"},"violet":{"bg":"#0B0716","surface":"#1C1133","primary":"#B388FF","secondary":"#00E5FF","accent":"#FF3B6B","text":"#F6F0FF","muted":"#A99BC4"}},"typeScale":{"display":{"size":0.14,"weight":800},"h1":{"size":0.09,"weight":700},"h2":{"size":0.06,"weight":600},"body":{"size":0.035,"weight":400},"caption":{"size":0.024,"weight":400}},"fonts":{"display":"Anton","heading":"Montserrat","body":"Inter","mono":"JetBrains Mono","script":"Dancing Script"},"motion":{"ease":{"linear":[0,0,1,1],"out":[0,0,0.58,1],"in":[0.42,0,1,1],"inOut":[0.42,0,0.58,1]},"durations":{"in":0.8,"out":0.6,"loop":2.0,"stagger":0.12,"transition":0.7},"principles":["anticipation","overshoot","follow-through","stagger","secondary-action"]},"layout":{"safeMargin":0.08,"gridColumns":12,"titleY":0.36,"subtitleY":0.52,"accentY":0.68}}"##;

fn templates() -> Vec<Value> { serde_json::from_str(TEMPLATES_JSON).unwrap_or_default() }
fn design() -> Value { serde_json::from_str(DESIGN_JSON).unwrap_or_default() }
fn f(v: &Value, k: &str, d: f64) -> f64 { v.get(k).and_then(Value::as_f64).unwrap_or(d) }

pub fn list_templates() -> Value {
    let items: Vec<Value> = templates().iter().map(|t| json!({
        "id": t["id"], "name": t["name"], "duration": t["duration"], "palette": t["palette"],
    })).collect();
    json!({ "templates": items, "cpuOnly": true })
}

pub fn design_system() -> Value { design() }

fn ease(name: &str) -> Value {
    let d = design();
    let e = d.get("motion").and_then(|m| m.get("ease")).and_then(|x| x.get(name)).cloned();
    e.unwrap_or(json!([0.0,0.0,0.58,1.0]))
}

fn keys_for(anim: &str, clip: &str, at: f64, dur: f64, stagger: f64) -> Vec<Value> {
    let recipes: Vec<(&str,&str,f64,f64,Value)> = match anim {
        "bounce_in" => vec![
            ("scale","",0.0,0.25,ease("out")),("scale","",0.55,1.14,ease("in")),
            ("scale","",0.8,0.97,ease("out")),("scale","",1.0,1.0,ease("linear")),
            ("opacity","",0.0,0.0,ease("linear")),("opacity","",0.3,1.0,ease("linear"))],
        "fade_in_up" => vec![
            ("opacity","",0.0,0.0,ease("out")),("opacity","",1.0,1.0,ease("linear")),
            ("offsetY","",0.0,0.16,ease("out")),("offsetY","",1.0,0.0,ease("linear"))],
        "draw_on" => vec![
            ("opacity","",0.0,0.0,ease("linear")),("opacity","",0.15,1.0,ease("linear")),
            ("scale","",0.0,0.7,ease("out")),("scale","",1.0,1.0,ease("linear"))],
        "typewriter" => vec![
            ("opacity","",0.0,0.0,ease("linear")),("opacity","",0.02,1.0,ease("linear")),
            ("offsetX","",0.0,-0.03,ease("out")),("offsetX","",1.0,0.0,ease("linear"))],
        "glitch_in" => vec![
            ("opacity","",0.0,0.2,ease("linear")),("opacity","",0.12,1.0,ease("linear")),
            ("offsetX","",0.0,0.07,ease("out")),("offsetX","",0.5,-0.03,ease("linear")),("offsetX","",1.0,0.0,ease("linear"))],
        _ => vec![
            ("opacity","",0.0,0.0,ease("out")),("opacity","",1.0,1.0,ease("linear")),
            ("offsetY","",0.0,0.12,ease("out")),("offsetY","",1.0,0.0,ease("linear"))],
    };
    let mut out = Vec::new();
    for (prop, _, pos, value, ez) in recipes {
        let at_pos = ((at + stagger) / dur.max(0.01) + pos).clamp(0.0, 1.0);
        out.push(json!({ "op":"setClipKey", "clipId":clip, "property":prop,
            "at":at_pos, "value":value, "ease":ez }));
    }
    out
}

fn style_of(el: &Value) -> Value {
    let mut s = json!({ "kind": el.get("kind").and_then(Value::as_str).unwrap_or("square").to_ascii_lowercase() });
    if let Some(obj) = s.as_object_mut() {
        for k in ["width","height","fill","stroke","strokeWidth","cornerRadius","points","innerRadius","dash"] {
            if let Some(v) = el.get(k) { obj.insert(k.to_string(), v.clone()); }
        }
        if let Some(v) = el.get("shadow") { obj.insert("shadow".to_string(), v.clone()); }
    }
    s
}

pub fn compose(args: &Value) -> Value {
    let id = args.get("templateId").and_then(Value::as_str).unwrap_or("");
    let Some(t) = templates().into_iter().find(|x| x["id"] == id) else {
        return json!({ "op":"batch", "commands":[] });
    };
    let dur = f(&t, "duration", 8.0);
    let mut commands: Vec<Value> = Vec::new();
    if let Some(bg) = t.get("background") {
        commands.push(json!({ "op":"addShapeClip", "start":0.0, "duration":dur,
            "name":"Background", "style":style_of(bg) }));
    }
    if let Some(items) = t.get("elements").and_then(Value::as_array) {
        for (i, el) in items.iter().enumerate() {
            let at = f(el, "at", 0.0);
            let ed = f(el, "duration", dur - at).max(0.1);
            let anim = el.get("anim").and_then(Value::as_str).unwrap_or("fade_in_up");
            let stagger = i as f64 * 0.12;
            let cid = format!("el{}", i);
            if el["type"] == "text" {
                let content = el.get("content").and_then(Value::as_str).unwrap_or("TEXT");
                let color = el.get("color").and_then(Value::as_str).unwrap_or("#FFFFFF");
                let family = el.get("font").and_then(Value::as_str).unwrap_or("heading");
                commands.push(json!({ "op":"addTextClip", "start":at, "duration":ed,
                    "name":content, "style":{ "content":content, "font_family":family,
                    "font_size":0.09, "font_weight":700, "color":color, "align":"center" } }));
            } else if el["type"] == "particlefield" {
                let count = f(el, "count", 10.0) as usize;
                for j in 0..count {
                    let pid = format!("p{}", j);
                    commands.push(json!({ "op":"addShapeClip", "start":at + j as f64*0.03,
                        "duration":ed, "name":format!("Particle {}", j+1),
                        "style":{ "kind":"circle", "width":f(el,"size",0.02),
                        "fill":el.get("fill").and_then(Value::as_str).unwrap_or("#FFFFFF") } }));
                    commands.extend(keys_for(anim, &pid, at, ed, j as f64 * 0.02));
                }
            } else {
                commands.push(json!({ "op":"addShapeClip", "start":at, "duration":ed,
                    "name":el.get("name").and_then(Value::as_str).unwrap_or("Element"),
                    "style":style_of(el) }));
                commands.extend(keys_for(anim, &cid, at, ed, stagger));
            }
        }
    }
    if let Some(fxs) = t.get("effects").and_then(Value::as_array) {
        for fx in fxs {
            commands.push(json!({ "op":"addLayerClip", "start":f(fx,"at",0.0),
                "duration":f(fx,"duration",dur), "effectId":fx["id"],
                "name":fx["id"], "sound":false }));
        }
    }
    json!({ "op":"batch", "template":t["name"], "duration":dur, "commands":commands })
}
