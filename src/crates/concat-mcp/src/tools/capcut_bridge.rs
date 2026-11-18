// GENERATED FILE - do not edit. Run: python3 kapcut/materials/generate_bridge_rs.py
use serde_json::{json, Value};

fn has_param(package: &str, key: &str) -> bool {
    let keys: &[&str] = match package {
        "concat.adjust" => &["blacks", "blue", "brightness", "contrast", "exposure", "fade", "gain", "gamma", "green", "highlights", "lift", "luma", "red", "saturation", "shadows", "sharpen", "temperature", "tint", "vignette", "whites"],
        "concat.after-hours" => &["depth", "grain"],
        "concat.aquamarine" => &["brightness", "water"],
        "concat.autumn" => &[],
        "concat.bass" => &["gain"],
        "concat.beat-shake" => &["amount", "blur", "bpm", "length", "punch"],
        "concat.bleach-bypass" => &[],
        "concat.bloom-pulse" => &["amount", "speed"],
        "concat.blur-dissolve" => &["strength"],
        "concat.blush" => &["glow", "softness"],
        "concat.bright" => &["gain"],
        "concat.bronze" => &["contrast", "tone"],
        "concat.cathedral" => &["size"],
        "concat.chipmunk" => &["pitch"],
        "concat.chroma-key" => &["balance", "clip_black", "clip_white", "color", "feather", "grow", "restore", "shadows", "spill", "strength", "view"],
        "concat.chromatic-aberration" => &["shift"],
        "concat.city-lights" => &["depth", "lights"],
        "concat.classic-chrome" => &[],
        "concat.clean" => &["amount"],
        "concat.clock-wipe" => &[],
        "concat.cloudless-sky" => &["greens", "sky"],
        "concat.color-wheels" => &["gain", "gamma", "lift"],
        "concat.contrast" => &["amount"],
        "concat.contrast-pop" => &["contrast"],
        "concat.cool" => &["temperature"],
        "concat.cool-blue" => &["chill", "contrast"],
        "concat.corona" => &["amount", "cool", "radius", "threshold"],
        "concat.cube" => &[],
        "concat.curves" => &["blue", "green", "luma", "red"],
        "concat.cyberpunk" => &[],
        "concat.deep" => &["body", "pitch"],
        "concat.delicious" => &[],
        "concat.denoise" => &["strength"],
        "concat.dissolve" => &[],
        "concat.distorted" => &["drive"],
        "concat.dreamy" => &[],
        "concat.echo" => &["decay", "delay"],
        "concat.enhance-voice" => &["amount"],
        "concat.exposure" => &["stops"],
        "concat.fade-black" => &[],
        "concat.fade-white" => &[],
        "concat.falling-dust" => &["dust", "leak"],
        "concat.film" => &["grain", "warmth"],
        "concat.film-burn" => &["heat"],
        "concat.film-grain" => &["amount"],
        "concat.fisheye" => &["strength"],
        "concat.fresh" => &["brightness", "pop"],
        "concat.gaussian-blur" => &["radius"],
        "concat.glitch-transition" => &["amount"],
        "concat.glitch-warp" => &["amount"],
        "concat.glow" => &["amount"],
        "concat.golden-hour" => &["glow", "warmth"],
        "concat.halation" => &["amount", "radius", "threshold", "tint"],
        "concat.halftone" => &["size"],
        "concat.hall" => &["size"],
        "concat.high-contrast-mono" => &[],
        "concat.impact" => &["amount", "blur", "direction", "length", "punch"],
        "concat.indie" => &["amount"],
        "concat.instant-print" => &["fade", "warmth"],
        "concat.iris" => &["softness"],
        "concat.landscape-pop" => &[],
        "concat.light-leak-flash" => &["strength"],
        "concat.light-leak-transition" => &["warmth"],
        "concat.linear-wipe" => &["direction", "softness"],
        "concat.linen" => &["matte", "warmth"],
        "concat.lofi" => &["bits"],
        "concat.loudness" => &["target"],
        "concat.luma-key" => &["invert", "softness", "threshold"],
        "concat.luminous" => &["brighten", "even"],
        "concat.megaphone" => &["drive"],
        "concat.mint-haze" => &["amount"],
        "concat.mono" => &["colour", "contrast", "grain"],
        "concat.moody" => &["depth", "warmth"],
        "concat.motion-blur" => &["angle", "length"],
        "concat.natural" => &[],
        "concat.neon" => &["glow", "punch"],
        "concat.night" => &["clarity", "lift"],
        "concat.noir" => &[],
        "concat.pink-fuzz" => &["blush", "haze"],
        "concat.pitch" => &["semitones"],
        "concat.pixelate" => &["size"],
        "concat.plate" => &["size"],
        "concat.point-and-shoot" => &["flash", "noise"],
        "concat.posterize" => &["levels"],
        "concat.prism-shake" => &["amount", "speed", "split"],
        "concat.push" => &["direction"],
        "concat.radio" => &["noise"],
        "concat.remove-black" => &["strength", "threshold"],
        "concat.retro-70s" => &[],
        "concat.retro-cartoon" => &["ink", "levels"],
        "concat.rgb-split-wipe" => &["spread"],
        "concat.robot" => &["depth"],
        "concat.room" => &["size"],
        "concat.rosy" => &[],
        "concat.saturation" => &["amount"],
        "concat.sci-fi" => &[],
        "concat.shake" => &["amount", "blur", "rotation", "speed", "style"],
        "concat.shape-wipe" => &["shape", "softness"],
        "concat.sharpen" => &["amount"],
        "concat.silver" => &[],
        "concat.slide" => &["direction"],
        "concat.soft-light" => &["lift", "warmth"],
        "concat.spin" => &["turns"],
        "concat.spooky" => &["dread"],
        "concat.stadium" => &["size"],
        "concat.super-8" => &[],
        "concat.sweet" => &["amount", "pitch"],
        "concat.swirl" => &["twist"],
        "concat.teal-orange" => &["amount", "contrast", "skin"],
        "concat.telephone" => &["drive"],
        "concat.treble" => &["gain"],
        "concat.tropical" => &[],
        "concat.underwater" => &["depth"],
        "concat.vaporwave" => &[],
        "concat.vhs" => &["bleed", "lines"],
        "concat.vibrance" => &["amount"],
        "concat.vignette" => &["strength"],
        "concat.vintage" => &["age", "tint"],
        "concat.vinyl" => &["warmth"],
        "concat.vivid" => &["colour", "punch"],
        "concat.walnut" => &["soften", "warmth"],
        "concat.warm" => &["temperature"],
        "concat.wet-street" => &["chill", "haze"],
        "concat.whip-pan" => &["speed"],
        "concat.whisper" => &["air"],
        "concat.white-balance" => &["temperature", "tint"],
        "concat.whitewash" => &["cool", "matte"],
        "concat.window-seat" => &["haze", "warmth"],
        "concat.winter-cool" => &[],
        "concat.zoom-blur" => &["amount", "x", "y"],
        "concat.zoom-blur-transition" => &["strength"],
        "concat.zoom-transition" => &["direction"],
        _ => &[],
    };
    keys.contains(&key)
}

fn scale(v: f64, mode: &str) -> f64 {
    match mode {
        "kelvin" => 6500.0 + v * 4500.0,
        "invert_percent" => if v > 0.0 { -100.0 } else { 0.0 },
        _ => v * mode.parse::<f64>().unwrap_or(1.0),
    }
}

pub fn map_adjust(capcut_type: &str, value: f64, clip_id: &str, entry: usize) -> Option<Value> {
    let (package, key, mode) = match capcut_type.to_ascii_lowercase().as_str() {
        "black_white" => ("concat.adjust", "saturation", "-100"),
        "brightness" => ("concat.adjust", "brightness", "100"),
        "clear" => ("concat.adjust", "sharpen", "100"),
        "contrast" => ("concat.adjust", "contrast", "100"),
        "fade" => ("concat.adjust", "fade", "100"),
        "highlight" => ("concat.adjust", "highlights", "100"),
        "hsl" => ("concat.adjust", "saturation", "100"),
        "light_sensation" => ("concat.adjust", "exposure", "3"),
        "saturation" => ("concat.adjust", "saturation", "100"),
        "shadow" => ("concat.adjust", "shadows", "100"),
        "sharpen" => ("concat.adjust", "sharpen", "100"),
        "temperature" => ("concat.adjust", "temperature", "kelvin"),
        "tone" => ("concat.adjust", "temperature", "kelvin"),
        "vignetting" => ("concat.adjust", "vignette", "100"),
        _ => return None,
    };
    let (package, key) = if mode == "invert_percent" { ("concat.adjust", "saturation") } else { (package, key) };
    if !has_param(package, key) { return None; }
    Some(json!({
        "op": "setEffectKey", "clipId": clip_id, "entry": entry,
        "key": key, "at": 0.0, "value": scale(value, mode),
    }))
}

pub fn map_effect(name: &str, start: f64, duration: f64) -> Vec<Value> {
    let lower = name.to_ascii_lowercase();
    let mut package = "";
    if package.is_empty() && lower.contains("after hours") { package = "concat.after-hours"; }
    if package.is_empty() && lower.contains("aquamarine") { package = "concat.aquamarine"; }
    if package.is_empty() && lower.contains("autumn") { package = "concat.autumn"; }
    if package.is_empty() && lower.contains("beat shake") { package = "concat.beat-shake"; }
    if package.is_empty() && lower.contains("bloom") { package = "concat.bloom-pulse"; }
    if package.is_empty() && lower.contains("blur") { package = "concat.gaussian-blur"; }
    if package.is_empty() && lower.contains("blush") { package = "concat.blush"; }
    if package.is_empty() && lower.contains("box blur") { package = "concat.gaussian-blur"; }
    if package.is_empty() && lower.contains("chipmunk") { package = "concat.chipmunk"; }
    if package.is_empty() && lower.contains("chroma") { package = "concat.chroma-key"; }
    if package.is_empty() && lower.contains("chromatic aberration") { package = "concat.chromatic-aberration"; }
    if package.is_empty() && lower.contains("clock wipe") { package = "concat.clock-wipe"; }
    if package.is_empty() && lower.contains("color wheels") { package = "concat.color-wheels"; }
    if package.is_empty() && lower.contains("contrast") { package = "concat.contrast"; }
    if package.is_empty() && lower.contains("cube") { package = "concat.cube"; }
    if package.is_empty() && lower.contains("denoise") { package = "concat.denoise"; }
    if package.is_empty() && lower.contains("dissolve") { package = "concat.dissolve"; }
    if package.is_empty() && lower.contains("dust") { package = "concat.falling-dust"; }
    if package.is_empty() && lower.contains("echo") { package = "concat.echo"; }
    if package.is_empty() && lower.contains("enhance voice") { package = "concat.enhance-voice"; }
    if package.is_empty() && lower.contains("exposure") { package = "concat.exposure"; }
    if package.is_empty() && lower.contains("fade black") { package = "concat.fade-black"; }
    if package.is_empty() && lower.contains("fade white") { package = "concat.fade-white"; }
    if package.is_empty() && lower.contains("film grain") { package = "concat.film-grain"; }
    if package.is_empty() && lower.contains("fisheye") { package = "concat.fisheye"; }
    if package.is_empty() && lower.contains("glitch") { package = "concat.glitch-warp"; }
    if package.is_empty() && lower.contains("glow") { package = "concat.glow"; }
    if package.is_empty() && lower.contains("green screen") { package = "concat.chroma-key"; }
    if package.is_empty() && lower.contains("green_screen") { package = "concat.chroma-key"; }
    if package.is_empty() && lower.contains("halation") { package = "concat.halation"; }
    if package.is_empty() && lower.contains("halftone") { package = "concat.halftone"; }
    if package.is_empty() && lower.contains("impact") { package = "concat.impact"; }
    if package.is_empty() && lower.contains("iris") { package = "concat.iris"; }
    if package.is_empty() && lower.contains("light leak") { package = "concat.light-leak-flash"; }
    if package.is_empty() && lower.contains("linear wipe") { package = "concat.linear-wipe"; }
    if package.is_empty() && lower.contains("loudness") { package = "concat.loudness"; }
    if package.is_empty() && lower.contains("luma key") { package = "concat.luma-key"; }
    if package.is_empty() && lower.contains("megaphone") { package = "concat.megaphone"; }
    if package.is_empty() && lower.contains("motion blur") { package = "concat.motion-blur"; }
    if package.is_empty() && lower.contains("neon") { package = "concat.neon"; }
    if package.is_empty() && lower.contains("noise reduction") { package = "concat.denoise"; }
    if package.is_empty() && lower.contains("pixelate") { package = "concat.pixelate"; }
    if package.is_empty() && lower.contains("posterize") { package = "concat.posterize"; }
    if package.is_empty() && lower.contains("push") { package = "concat.push"; }
    if package.is_empty() && lower.contains("remove black") { package = "concat.remove-black"; }
    if package.is_empty() && lower.contains("rgb split wipe") { package = "concat.rgb-split-wipe"; }
    if package.is_empty() && lower.contains("robot") { package = "concat.robot"; }
    if package.is_empty() && lower.contains("saturation") { package = "concat.saturation"; }
    if package.is_empty() && lower.contains("sepia") { package = "concat.vintage"; }
    if package.is_empty() && lower.contains("shake") { package = "concat.shake"; }
    if package.is_empty() && lower.contains("shape wipe") { package = "concat.shape-wipe"; }
    if package.is_empty() && lower.contains("sharpen") { package = "concat.sharpen"; }
    if package.is_empty() && lower.contains("slide") { package = "concat.slide"; }
    if package.is_empty() && lower.contains("swirl") { package = "concat.swirl"; }
    if package.is_empty() && lower.contains("telephone") { package = "concat.telephone"; }
    if package.is_empty() && lower.contains("vhs") { package = "concat.vhs"; }
    if package.is_empty() && lower.contains("vignette") { package = "concat.vignette"; }
    if package.is_empty() && lower.contains("zoom blur") { package = "concat.zoom-blur"; }
    if package.is_empty() { return Vec::new(); }
    vec![json!({
        "op": "addLayerClip", "start": start, "duration": duration,
        "effectId": package, "name": name, "sound": false,
    })]
}

/// Translate a CapCut-style material request into one Concat edit command.
pub fn translate(args: &Value) -> Value {
    let path = args.get("path").cloned().unwrap_or(Value::Null);
    let mut commands: Vec<Value> = Vec::new();
    if let Some(t) = args.get("capcutType").and_then(Value::as_str) {
        let v = args.get("value").and_then(Value::as_f64).unwrap_or(0.0);
        let clip = args.get("clipId").and_then(Value::as_str).unwrap_or("");
        if let Some(c) = map_adjust(t, v, clip, 0) { commands.push(c); }
    }
    if let Some(n) = args.get("effectName").and_then(Value::as_str) {
        let start = args.get("start").and_then(Value::as_f64).unwrap_or(0.0);
        let dur = args.get("duration").and_then(Value::as_f64).unwrap_or(5.0);
        commands.extend(map_effect(n, start, dur));
    }
    json!({ "op": "batch", "commands": commands })
}
