// GENERATED FILE - do not edit. Run: python3 kapcut/studio/generate_studio_rs.py
use serde_json::{json, Value};

const TEMPLATES_JSON: &str = r##"[{"id":"kinetic_title","name":"Kinetic Title Reveal","duration":8,"palette":"midnight","background":{"kind":"square","width":2.0,"height":2.0,"fill":"#070B14"},"elements":[{"type":"shape","kind":"circle","width":0.9,"fill":"","stroke":"#00E5FF","strokeWidth":0.002,"at":0.4,"duration":6.5,"anim":"draw_on","camera":true},{"type":"shape","kind":"circle","width":0.6,"fill":"#111A2E","stroke":"#7C5CFC","strokeWidth":0.003,"shadow":{"color":"#7C5CFC","blur":0.08},"at":0.8,"duration":6,"anim":"bounce_in","stagger":0.2},{"type":"text","content":"MOTION","font":"display","size":"display","color":"#F5F7FF","at":1.2,"duration":5.5,"anim":"typewriter","y":0.36},{"type":"text","content":"DESIGN SYSTEM","font":"heading","size":"caption","color":"#00E5FF","at":2.0,"duration":4.5,"anim":"fade_in_up","y":0.52},{"type":"shape","kind":"line","width":0.5,"stroke":"#FF3B6B","strokeWidth":0.002,"at":2.4,"duration":4,"anim":"draw_on","y":0.62}],"effects":[{"id":"concat.glow","at":0,"duration":8},{"id":"concat.film-grain","at":0,"duration":8}],"camera":{"scale":[[0,1.0],[0.5,1.04],[1,1.0]],"offsetX":[[0,-0.02],[1,0.02]]}},{"id":"particle_reveal","name":"Particle Field Reveal","duration":10,"palette":"violet","background":{"kind":"square","width":2.0,"height":2.0,"fill":"#0B0716"},"elements":[{"type":"particlefield","count":14,"kind":"circle","size":0.02,"fill":"#B388FF","at":0.3,"duration":9,"anim":"float","seed":11},{"type":"shape","kind":"star","points":6,"innerRadius":0.4,"width":0.7,"fill":"#1C1133","stroke":"#B388FF","strokeWidth":0.003,"at":1.5,"duration":7,"anim":"bounce_in"},{"type":"text","content":"AGENTIC","font":"display","size":"display","color":"#F6F0FF","at":2.0,"duration":6,"anim":"glitch_in","y":0.36},{"type":"text","content":"VIDEO PRODUCTION","font":"mono","size":"caption","color":"#00E5FF","at":3.0,"duration":5,"anim":"fade_in_up","y":0.52}],"effects":[{"id":"concat.neon","at":1.5,"duration":7},{"id":"concat.chromatic-aberration","at":2,"duration":6}],"camera":{"scale":[[0,0.96],[0.5,1.06],[1,1.0]]}},{"id":"color_grade_showcase","name":"Color Grade Showcase","duration":12,"palette":"ember","background":{"kind":"square","width":2.0,"height":2.0,"fill":"#120806"},"elements":[{"type":"shape","kind":"square","width":1.2,"height":0.7,"fill":"#2A140F","stroke":"#FF6B35","strokeWidth":0.002,"at":0.5,"duration":10,"anim":"fade_in_up","y":0.1},{"type":"text","content":"GRADE","font":"display","size":"display","color":"#FFD166","at":1.0,"duration":9,"anim":"bounce_in","y":0.36},{"type":"text","content":"TEAL · ORANGE · NOIR · VINTAGE","font":"heading","size":"body","color":"#FFF4E6","at":2.0,"duration":8,"anim":"typewriter","y":0.55},{"type":"shape","kind":"line","width":0.8,"stroke":"#FFD166","strokeWidth":0.003,"at":2.5,"duration":7,"anim":"draw_on","y":0.68}],"effects":[{"id":"concat.teal-orange","at":0,"duration":4},{"id":"concat.noir","at":4,"duration":4},{"id":"concat.vintage","at":8,"duration":4}],"camera":{"offsetX":[[0,-0.03],[1,0.03]],"scale":[[0,1.02],[1,1.08]]}},{"id":"logo_lockup","name":"Logo Lockup / End Card","duration":6,"palette":"forest","background":{"kind":"square","width":2.0,"height":2.0,"fill":"#04120D"},"elements":[{"type":"shape","kind":"circle","width":0.5,"fill":"","stroke":"#06D6A0","strokeWidth":0.005,"shadow":{"color":"#06D6A0","blur":0.06},"at":0.3,"duration":5,"anim":"draw_on"},{"type":"text","content":"KAPCUT","font":"display","size":"display","color":"#EAFFF8","at":0.8,"duration":4.5,"anim":"bounce_in","y":0.38},{"type":"text","content":"STUDIO","font":"heading","size":"h2","color":"#06D6A0","at":1.4,"duration":4,"anim":"fade_in_up","y":0.55},{"type":"shape","kind":"line","width":0.3,"stroke":"#118AB2","strokeWidth":0.002,"at":1.8,"duration":3.5,"anim":"draw_on","y":0.66}],"effects":[{"id":"concat.halation","at":0,"duration":6}],"camera":{"scale":[[0,1.1],[0.5,1.0],[1,1.02]]}}]"##;
const DESIGN_JSON: &str = r##"{"schemaVersion":1,"name":"KapCut Studio Design System","note":"Design tokens for studio-grade compositions. Everything maps to Concat's real shape/text/effect model; no GPU.","palettes":{"midnight":{"bg":"#070B14","surface":"#111A2E","primary":"#00E5FF","secondary":"#7C5CFC","accent":"#FF3B6B","text":"#F5F7FF","muted":"#8FA3C8"},"ember":{"bg":"#120806","surface":"#2A140F","primary":"#FF6B35","secondary":"#FFD166","accent":"#EF476F","text":"#FFF4E6","muted":"#C9A88E"},"forest":{"bg":"#04120D","surface":"#0C2A1F","primary":"#06D6A0","secondary":"#118AB2","accent":"#FFD166","text":"#EAFFF8","muted":"#8FBFAE"},"violet":{"bg":"#0B0716","surface":"#1C1133","primary":"#B388FF","secondary":"#00E5FF","accent":"#FF3B6B","text":"#F6F0FF","muted":"#A99BC4"}},"typeScale":{"display":{"size":0.14,"weight":800},"h1":{"size":0.09,"weight":700},"h2":{"size":0.06,"weight":600},"body":{"size":0.035,"weight":400},"caption":{"size":0.024,"weight":400}},"fonts":{"display":"Anton","heading":"Montserrat","body":"Inter","mono":"JetBrains Mono","script":"Dancing Script"},"motion":{"ease":{"linear":[0,0,1,1],"out":[0,0,0.58,1],"in":[0.42,0,1,1],"inOut":[0.42,0,0.58,1]},"durations":{"in":0.8,"out":0.6,"loop":2.0,"stagger":0.12,"transition":0.7},"principles":["anticipation","overshoot","follow-through","stagger","secondary-action"]},"layout":{"safeMargin":0.08,"gridColumns":12,"titleY":0.36,"subtitleY":0.52,"accentY":0.68}}"##;
const ANIMATORS_JSON: &str = r##"[{"id":"type_on","name":"Type On","unit":"char","stagger":0.06,"keys":[{"property":"opacity","at":0,"value":0},{"property":"opacity","at":0.08,"value":1},{"property":"offsetY","at":0,"value":0.04,"ease":"out"},{"property":"offsetY","at":1,"value":0}]},{"id":"wave","name":"Wave","unit":"word","stagger":0.12,"keys":[{"property":"offsetY","at":0,"value":0.02,"ease":"inOut"},{"property":"offsetY","at":0.5,"value":-0.03,"ease":"inOut"},{"property":"offsetY","at":1,"value":0.02}]},{"id":"burst","name":"Burst","unit":"word","stagger":0.09,"keys":[{"property":"scale","at":0,"value":0.2,"ease":"out"},{"property":"scale","at":0.6,"value":1.2,"ease":"in"},{"property":"scale","at":1,"value":1.0}]},{"id":"slider","name":"Slider","unit":"word","stagger":0.1,"keys":[{"property":"offsetX","at":0,"value":0.3,"ease":"out"},{"property":"offsetX","at":1,"value":0}]},{"id":"flip_up","name":"Flip Up","unit":"word","stagger":0.08,"keys":[{"property":"rotation","at":0,"value":-45,"ease":"out"},{"property":"rotation","at":1,"value":0},{"property":"opacity","at":0,"value":0},{"property":"opacity","at":0.2,"value":1}]},{"id":"highlight_pop","name":"Highlight Pop","unit":"word","stagger":0.15,"keys":[{"property":"scale","at":0,"value":1,"ease":"out"},{"property":"scale","at":0.5,"value":1.25,"ease":"inOut"},{"property":"scale","at":1,"value":1}]}]"##;

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

pub fn list_animators() -> Value {
    let a: Vec<Value> = serde_json::from_str(ANIMATORS_JSON).unwrap_or_default();
    let items: Vec<Value> = a.iter().map(|x| json!({
        "id": x["id"], "name": x["name"], "unit": x["unit"],
    })).collect();
    json!({ "animators": items })
}

pub fn animate_text(args: &Value) -> Value {
    let text = args.get("text").and_then(Value::as_str).unwrap_or("TEXT");
    let aid = args.get("animator").and_then(Value::as_str).unwrap_or("burst");
    let start = f(args, "start", 0.0);
    let dur = f(args, "duration", 4.0);
    let a: Vec<Value> = serde_json::from_str(ANIMATORS_JSON).unwrap_or_default();
    let Some(anim) = a.into_iter().find(|x| x["id"] == aid) else {
        return json!({ "op":"batch", "commands":[] });
    };
    let words: Vec<&str> = text.split_whitespace().collect();
    let stagger = f(&anim, "stagger", 0.1);
    let mut commands: Vec<Value> = Vec::new();
    for (i, word) in words.iter().enumerate() {
        let at = start + i as f64 * stagger;
        let life = (dur - i as f64 * stagger).max(0.2);
        commands.push(json!({ "op":"addTextClip", "start":at, "duration":life,
            "name":word, "style":{ "content":word, "font_family":"Inter",
            "font_size":0.08, "font_weight":700, "color":"#FFFFFF", "align":"center" } }));
        if let Some(keys) = anim.get("keys").and_then(Value::as_array) {
            for key in keys {
                commands.push(json!({ "op":"setClipKey", "clipId":format!("w{}", i),
                    "property":key["property"], "at":f(key,"at",0.0), "value":f(key,"value",0.0),
                    "ease":key.get("ease").and_then(Value::as_str).unwrap_or("linear") }));
            }
        }
    }
    json!({ "op":"batch", "text":text, "animator":aid, "commands":commands })
}

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

pub fn beat_keys(args: &Value) -> Value {
    let clip = args.get("clipId").and_then(Value::as_str).unwrap_or("c1");
    let prop = args.get("property").and_then(Value::as_str).unwrap_or("scale");
    let bpm = f(args, "bpm", 120.0).max(1.0);
    let dur = f(args, "duration", 8.0).max(0.1);
    let base = f(args, "base", 1.0);
    let amount = f(args, "amount", 0.12);
    let offset = f(args, "offset", 0.0);
    let beat = 60.0 / bpm;
    let mut commands: Vec<Value> = Vec::new();
    let mut t = offset;
    let mut i = 0;
    while t < dur && i < 256 {
        let at = (t / dur).clamp(0.0, 1.0);
        commands.push(json!({ "op":"setClipKey", "clipId":clip, "property":prop,
            "at":at, "value":base + amount, "ease":ease("out") }));
        commands.push(json!({ "op":"setClipKey", "clipId":clip, "property":prop,
            "at":(at + (beat/dur*0.6).min(0.12)).min(1.0), "value":base - amount*0.35, "ease":ease("inOut") }));
        commands.push(json!({ "op":"setClipKey", "clipId":clip, "property":prop,
            "at":(at + beat*0.9/dur).min(1.0), "value":base, "ease":ease("linear") }));
        t += beat; i += 1;
    }
    json!({ "op":"batch", "bpm":bpm, "property":prop, "commands":commands })
}

pub fn invent_style(args: &Value) -> Value {
    let description = args.get("description").and_then(Value::as_str).unwrap_or("").to_ascii_lowercase();
    let title = args.get("title").and_then(Value::as_str).unwrap_or("TITLE");
    let dur = f(args, "duration", 10.0).max(0.1);
    let design = design();
    let palette = if description.contains("neon") || description.contains("cyber") || description.contains("glitch") { "violet" }
        else if description.contains("warm") || description.contains("vintage") || description.contains("film") { "ember" }
        else if description.contains("calm") || description.contains("cinematic") || description.contains("natural") { "forest" }
        else { "midnight" };
    let colors = design.get("palettes").and_then(|p| p.get(palette)).cloned().unwrap_or(json!({}));
    let bg = colors.get("bg").and_then(Value::as_str).unwrap_or("#070B14");
    let surface = colors.get("surface").and_then(Value::as_str).unwrap_or("#111A2E");
    let primary = colors.get("primary").and_then(Value::as_str).unwrap_or("#00E5FF");
    let text_color = colors.get("text").and_then(Value::as_str).unwrap_or("#FFFFFF");
    let animator = if description.contains("wave") { "wave" } else if description.contains("slide") { "slider" }
        else if description.contains("flip") { "flip_up" } else if description.contains("highlight") || description.contains("pop") { "highlight_pop" }
        else { "burst" };
    let mut effects: Vec<&str> = Vec::new();
    for (word, fx) in [("neon","concat.neon"),("glow","concat.glow"),("glitch","concat.glitch-warp"),
        ("grain","concat.film-grain"),("vintage","concat.vintage"),("noir","concat.noir"),
        ("chromatic","concat.chromatic-aberration"),("halation","concat.halation"),("vivid","concat.vibrance"),("teal","concat.teal-orange")] {
        if description.contains(word) && effects.len() < 3 { effects.push(fx); }
    }
    if effects.is_empty() { effects.push("concat.glow"); }
    let shape = if description.contains("triangle") { "triangle" } else if description.contains("star") || description.contains("burst") { "star" }
        else if description.contains("line") { "line" } else if description.contains("square") || description.contains("card") { "square" } else { "circle" };
    let bpm = if description.contains("fast") || description.contains("retention") { 145.0 }
        else if description.contains("calm") || description.contains("cinematic") { 65.0 } else { 110.0 };
    let mut commands: Vec<Value> = Vec::new();
    commands.push(json!({ "op":"addShapeClip", "start":0.0, "duration":dur, "name":"Background",
        "style":{ "kind":"square", "width":2.0, "height":2.0, "fill":bg } }));
    commands.push(json!({ "op":"addShapeClip", "start":0.5, "duration":dur-1.0, "name":"Shape 1",
        "style":{ "kind":shape, "width":0.5, "fill":surface, "stroke":primary, "strokeWidth":0.004,
        "shadow":{ "color":primary, "blur":0.05 } } }));
    commands.push(json!({ "op":"addTextClip", "start":1.0, "duration":dur-2.0, "name":title,
        "style":{ "content":title, "font_family":"Anton", "font_size":0.14, "font_weight":800,
        "color":text_color, "align":"center" } }));
    commands.extend(keys_for(animator, "title0", 1.0, dur - 2.0, 0.0));
    for fx in effects {
        commands.push(json!({ "op":"addLayerClip", "start":0.0, "duration":dur,
            "effectId":fx, "name":fx, "sound":false }));
    }
    json!({ "op":"batch", "description":description, "palette":palette, "animator":animator,
        "bpm":bpm, "effects":effects, "commands":commands })
}

pub fn invent_editing_style(args: &Value) -> Value {
    let desc = args.get("description").and_then(Value::as_str).unwrap_or("").to_ascii_lowercase();
    let title = args.get("title").and_then(Value::as_str).unwrap_or("TITLE");
    let dur = f(args, "duration", 20.0).max(0.1);
    let design = design();
    let has = |words: &[&str]| words.iter().any(|w| desc.contains(w));
    let profile = args.get("profile").cloned().unwrap_or(json!({}));
    let prof = profile.get("style_profile").cloned().unwrap_or(json!({}));
    let pace = if has(&["hormozi","retention","brainrot","mrbeast","fast","rapid"]) { "fast" }
        else if has(&["documentary","cinematic","slow","calm","video essay"]) { "slow" }
        else if has(&["medium","balanced","vlog","tutorial"]) { "medium" }
        else { prof.get("pace").and_then(Value::as_str).unwrap_or("medium") };
    let shot = if let Some(s) = prof.get("median_shot").and_then(Value::as_f64) { s } else {
        match pace { "fast" => 1.2, "slow" => 9.0, _ => 4.0 } };
    let text_style = if has(&["every word","word by word","kinetic captions","hormozi"]) { "every-word" }
        else if has(&["keyword","callout","highlight"]) { "keywords" }
        else if has(&["lower third","caption","subtitle"]) { "lower-thirds" }
        else if has(&["no text","clean","minimal"]) { "none" } else { "keywords" };
    let graphics = if has(&["heavy graphics","collage","brainrot","motion graphics"]) { "heavy" }
        else if has(&["b-roll","overlay","graphics","map","chart"]) { "medium" }
        else if has(&["minimal","clean","talking head"]) { "light" } else { "medium" };
    let sound = if has(&["sfx","sound effect","whoosh","hit","riser"]) { "designed" } else { "bed" };
    let palette = if let Some(rgb) = prof.get("mean_rgb").and_then(Value::as_array) {
        let r = rgb.get(0).and_then(Value::as_f64).unwrap_or(0.0);
        let b = rgb.get(2).and_then(Value::as_f64).unwrap_or(0.0);
        if r > b + 30.0 { "ember" } else { "midnight" }
    } else if has(&["neon","cyber","glitch","violet"]) { "violet" }
      else if has(&["warm","vintage","film","ember"]) { "ember" }
      else if has(&["cinematic","calm","forest","natural"]) { "forest" } else { "midnight" };
    let colors = design.get("palettes").and_then(|p| p.get(palette)).cloned().unwrap_or(json!({}));
    let bg = colors.get("bg").and_then(Value::as_str).unwrap_or("#070B14");
    let surface = colors.get("surface").and_then(Value::as_str).unwrap_or("#111A2E");
    let primary = colors.get("primary").and_then(Value::as_str).unwrap_or("#00E5FF");
    let tcolor = colors.get("text").and_then(Value::as_str).unwrap_or("#FFFFFF");
    let grade = if palette == "ember" { "concat.vintage" } else if palette == "violet" { "concat.neon" } else { "concat.teal-orange" };
    let mut commands: Vec<Value> = Vec::new();
    commands.push(json!({ "op":"addShapeClip", "start":0.0, "duration":dur, "name":"Background",
        "style":{ "kind":"square", "width":2.0, "height":2.0, "fill":bg } }));
    let steps = match graphics { "heavy" => 10, "light" => 2, _ => 5 };
    for i in 0..steps {
        commands.push(json!({ "op":"addShapeClip", "start":i as f64*dur/steps as f64,
            "duration":dur/steps as f64, "name":format!("Accent {}", i+1),
            "style":{ "kind":if i%2==0 {"square"} else {"circle"}, "width":0.35,
            "fill":surface, "stroke":primary, "strokeWidth":0.003 } }));
    }
    if text_style != "none" {
        let count = match text_style { "every-word" => 8, "lower-thirds" => 2, _ => 4 };
        for i in 0..count {
            let content = if i == 0 { title.to_string() } else { format!("POINT {}", i+1) };
            commands.push(json!({ "op":"addTextClip", "start":i as f64*dur/count as f64,
                "duration":dur/count as f64 - 0.1, "name":content,
                "style":{ "content":content, "font_family":"Montserrat", "font_size":0.08,
                "font_weight":700, "color":tcolor, "align":"center" } }));
        }
    }
    let n = (dur / shot.max(0.5)) as usize;
    for i in 1..n {
        commands.push(json!({ "op":"addTransitionClip", "start":i as f64*shot,
            "transitionId":if pace == "fast" { "concat.zoom-transition" } else { "concat.dissolve" },
            "duration":0.5, "name":"Cut" }));
    }
    if sound == "designed" {
        let mut t = 0.0;
        while t < dur {
            commands.push(json!({ "op":"addLayerClip", "start":t, "duration":0.4,
                "effectId":"concat.impact", "name":"Hit", "sound":true }));
            t += 3.0;
        }
    }
    commands.push(json!({ "op":"addLayerClip", "start":0.0, "duration":dur,
        "effectId":grade, "name":grade, "sound":false }));
    json!({ "op":"batch", "description":desc, "title":title, "duration":dur,
        "plan":json!({ "pace":pace, "shot_seconds":shot, "text":text_style,
        "graphics":graphics, "sound":sound, "look":palette, "grade":grade,
        "reference_driven":profile.get("style_profile").is_some() }),
        "commands":commands })
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
