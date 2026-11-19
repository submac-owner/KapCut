// GENERATED FILE - do not edit. Run: python3 kapcut/studio/generate_studio_rs.py
use serde_json::{json, Value};

const TEMPLATES_JSON: &str = r##"[{"id":"kinetic_title","name":"Kinetic Title Reveal","duration":8,"palette":"midnight","background":{"kind":"square","width":2.0,"height":2.0,"fill":"#070B14"},"elements":[{"type":"shape","kind":"circle","width":0.9,"fill":"","stroke":"#00E5FF","strokeWidth":0.002,"at":0.4,"duration":6.5,"anim":"draw_on","camera":true},{"type":"shape","kind":"circle","width":0.6,"fill":"#111A2E","stroke":"#7C5CFC","strokeWidth":0.003,"shadow":{"color":"#7C5CFC","blur":0.08},"at":0.8,"duration":6,"anim":"bounce_in","stagger":0.2},{"type":"text","content":"MOTION","font":"display","size":"display","color":"#F5F7FF","at":1.2,"duration":5.5,"anim":"typewriter","y":0.36},{"type":"text","content":"DESIGN SYSTEM","font":"heading","size":"caption","color":"#00E5FF","at":2.0,"duration":4.5,"anim":"fade_in_up","y":0.52},{"type":"shape","kind":"line","width":0.5,"stroke":"#FF3B6B","strokeWidth":0.002,"at":2.4,"duration":4,"anim":"draw_on","y":0.62}],"effects":[{"id":"concat.glow","at":0,"duration":8},{"id":"concat.film-grain","at":0,"duration":8}],"camera":{"scale":[[0,1.0],[0.5,1.04],[1,1.0]],"offsetX":[[0,-0.02],[1,0.02]]}},{"id":"particle_reveal","name":"Particle Field Reveal","duration":10,"palette":"violet","background":{"kind":"square","width":2.0,"height":2.0,"fill":"#0B0716"},"elements":[{"type":"particlefield","count":14,"kind":"circle","size":0.02,"fill":"#B388FF","at":0.3,"duration":9,"anim":"float","seed":11},{"type":"shape","kind":"star","points":6,"innerRadius":0.4,"width":0.7,"fill":"#1C1133","stroke":"#B388FF","strokeWidth":0.003,"at":1.5,"duration":7,"anim":"bounce_in"},{"type":"text","content":"AGENTIC","font":"display","size":"display","color":"#F6F0FF","at":2.0,"duration":6,"anim":"glitch_in","y":0.36},{"type":"text","content":"VIDEO PRODUCTION","font":"mono","size":"caption","color":"#00E5FF","at":3.0,"duration":5,"anim":"fade_in_up","y":0.52}],"effects":[{"id":"concat.neon","at":1.5,"duration":7},{"id":"concat.chromatic-aberration","at":2,"duration":6}],"camera":{"scale":[[0,0.96],[0.5,1.06],[1,1.0]]}},{"id":"color_grade_showcase","name":"Color Grade Showcase","duration":12,"palette":"ember","background":{"kind":"square","width":2.0,"height":2.0,"fill":"#120806"},"elements":[{"type":"shape","kind":"square","width":1.2,"height":0.7,"fill":"#2A140F","stroke":"#FF6B35","strokeWidth":0.002,"at":0.5,"duration":10,"anim":"fade_in_up","y":0.1},{"type":"text","content":"GRADE","font":"display","size":"display","color":"#FFD166","at":1.0,"duration":9,"anim":"bounce_in","y":0.36},{"type":"text","content":"TEAL · ORANGE · NOIR · VINTAGE","font":"heading","size":"body","color":"#FFF4E6","at":2.0,"duration":8,"anim":"typewriter","y":0.55},{"type":"shape","kind":"line","width":0.8,"stroke":"#FFD166","strokeWidth":0.003,"at":2.5,"duration":7,"anim":"draw_on","y":0.68}],"effects":[{"id":"concat.teal-orange","at":0,"duration":4},{"id":"concat.noir","at":4,"duration":4},{"id":"concat.vintage","at":8,"duration":4}],"camera":{"offsetX":[[0,-0.03],[1,0.03]],"scale":[[0,1.02],[1,1.08]]}},{"id":"logo_lockup","name":"Logo Lockup / End Card","duration":6,"palette":"forest","background":{"kind":"square","width":2.0,"height":2.0,"fill":"#04120D"},"elements":[{"type":"shape","kind":"circle","width":0.5,"fill":"","stroke":"#06D6A0","strokeWidth":0.005,"shadow":{"color":"#06D6A0","blur":0.06},"at":0.3,"duration":5,"anim":"draw_on"},{"type":"text","content":"KAPCUT","font":"display","size":"display","color":"#EAFFF8","at":0.8,"duration":4.5,"anim":"bounce_in","y":0.38},{"type":"text","content":"STUDIO","font":"heading","size":"h2","color":"#06D6A0","at":1.4,"duration":4,"anim":"fade_in_up","y":0.55},{"type":"shape","kind":"line","width":0.3,"stroke":"#118AB2","strokeWidth":0.002,"at":1.8,"duration":3.5,"anim":"draw_on","y":0.66}],"effects":[{"id":"concat.halation","at":0,"duration":6}],"camera":{"scale":[[0,1.1],[0.5,1.0],[1,1.02]]}}]"##;
const DESIGN_JSON: &str = r##"{"schemaVersion":1,"name":"KapCut Studio Design System","note":"Design tokens for studio-grade compositions. Everything maps to Concat's real shape/text/effect model; no GPU.","palettes":{"midnight":{"bg":"#070B14","surface":"#111A2E","primary":"#00E5FF","secondary":"#7C5CFC","accent":"#FF3B6B","text":"#F5F7FF","muted":"#8FA3C8"},"ember":{"bg":"#120806","surface":"#2A140F","primary":"#FF6B35","secondary":"#FFD166","accent":"#EF476F","text":"#FFF4E6","muted":"#C9A88E"},"forest":{"bg":"#04120D","surface":"#0C2A1F","primary":"#06D6A0","secondary":"#118AB2","accent":"#FFD166","text":"#EAFFF8","muted":"#8FBFAE"},"violet":{"bg":"#0B0716","surface":"#1C1133","primary":"#B388FF","secondary":"#00E5FF","accent":"#FF3B6B","text":"#F6F0FF","muted":"#A99BC4"}},"typeScale":{"display":{"size":0.14,"weight":800},"h1":{"size":0.09,"weight":700},"h2":{"size":0.06,"weight":600},"body":{"size":0.035,"weight":400},"caption":{"size":0.024,"weight":400}},"fonts":{"display":"Anton","heading":"Montserrat","body":"Inter","mono":"JetBrains Mono","script":"Dancing Script"},"motion":{"ease":{"linear":[0,0,1,1],"out":[0,0,0.58,1],"in":[0.42,0,1,1],"inOut":[0.42,0,0.58,1]},"durations":{"in":0.8,"out":0.6,"loop":2.0,"stagger":0.12,"transition":0.7},"principles":["anticipation","overshoot","follow-through","stagger","secondary-action"]},"layout":{"safeMargin":0.08,"gridColumns":12,"titleY":0.36,"subtitleY":0.52,"accentY":0.68}}"##;
const ANIMATORS_JSON: &str = r##"[{"id":"type_on","name":"Type On","unit":"char","stagger":0.06,"keys":[{"property":"opacity","at":0,"value":0},{"property":"opacity","at":0.08,"value":1},{"property":"offsetY","at":0,"value":0.04,"ease":"out"},{"property":"offsetY","at":1,"value":0}]},{"id":"wave","name":"Wave","unit":"word","stagger":0.12,"keys":[{"property":"offsetY","at":0,"value":0.02,"ease":"inOut"},{"property":"offsetY","at":0.5,"value":-0.03,"ease":"inOut"},{"property":"offsetY","at":1,"value":0.02}]},{"id":"burst","name":"Burst","unit":"word","stagger":0.09,"keys":[{"property":"scale","at":0,"value":0.2,"ease":"out"},{"property":"scale","at":0.6,"value":1.2,"ease":"in"},{"property":"scale","at":1,"value":1.0}]},{"id":"slider","name":"Slider","unit":"word","stagger":0.1,"keys":[{"property":"offsetX","at":0,"value":0.3,"ease":"out"},{"property":"offsetX","at":1,"value":0}]},{"id":"flip_up","name":"Flip Up","unit":"word","stagger":0.08,"keys":[{"property":"rotation","at":0,"value":-45,"ease":"out"},{"property":"rotation","at":1,"value":0},{"property":"opacity","at":0,"value":0},{"property":"opacity","at":0.2,"value":1}]},{"id":"highlight_pop","name":"Highlight Pop","unit":"word","stagger":0.15,"keys":[{"property":"scale","at":0,"value":1,"ease":"out"},{"property":"scale","at":0.5,"value":1.25,"ease":"inOut"},{"property":"scale","at":1,"value":1}]}]"##;
const FRAMES_JSON: &str = r##"{
 "count": 67,
 "frames": [
  {
   "id": "frame_paper_original",
   "name": "Paper \u2014 Original",
   "family": "paper",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#F5EFE0",
     "stroke": "#D8CDBB",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": 0.02,
     "shadow": {
      "color": "#D8CDBB",
      "blur": 0.03
     }
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#D8CDBB",
     "strokeWidth": 0.002,
     "rotation": 0.02
    }
   ]
  },
  {
   "id": "frame_paper_warm",
   "name": "Paper \u2014 Warm",
   "family": "paper",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#FFD9A0",
     "stroke": "#A86B32",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": 0.02,
     "shadow": {
      "color": "#A86B32",
      "blur": 0.03
     }
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#A86B32",
     "strokeWidth": 0.002,
     "rotation": 0.02
    }
   ]
  },
  {
   "id": "frame_paper_cold",
   "name": "Paper \u2014 Cold",
   "family": "paper",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#CFE8FF",
     "stroke": "#3E7FB5",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": 0.02,
     "shadow": {
      "color": "#3E7FB5",
      "blur": 0.03
     }
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#3E7FB5",
     "strokeWidth": 0.002,
     "rotation": 0.02
    }
   ]
  },
  {
   "id": "frame_paper_mono",
   "name": "Paper \u2014 Mono",
   "family": "paper",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#E8E8E8",
     "stroke": "#444444",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": 0.02,
     "shadow": {
      "color": "#444444",
      "blur": 0.03
     }
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#444444",
     "strokeWidth": 0.002,
     "rotation": 0.02
    }
   ]
  },
  {
   "id": "frame_paper_accent",
   "name": "Paper \u2014 Accent",
   "family": "paper",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#FFE066",
     "stroke": "#E0533B",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": 0.02,
     "shadow": {
      "color": "#E0533B",
      "blur": 0.03
     }
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#E0533B",
     "strokeWidth": 0.002,
     "rotation": 0.02
    }
   ]
  },
  {
   "id": "frame_vintage_original",
   "name": "Camera Vintage \u2014 Original",
   "family": "vintage",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#3A2E24",
     "stroke": "#C9A227",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": 0.0,
     "shadow": {
      "color": "#C9A227",
      "blur": 0.03
     }
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#C9A227",
     "strokeWidth": 0.002,
     "rotation": 0.0
    }
   ]
  },
  {
   "id": "frame_vintage_warm",
   "name": "Camera Vintage \u2014 Warm",
   "family": "vintage",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#FFD9A0",
     "stroke": "#A86B32",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": 0.0,
     "shadow": {
      "color": "#A86B32",
      "blur": 0.03
     }
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#A86B32",
     "strokeWidth": 0.002,
     "rotation": 0.0
    }
   ]
  },
  {
   "id": "frame_vintage_cold",
   "name": "Camera Vintage \u2014 Cold",
   "family": "vintage",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#CFE8FF",
     "stroke": "#3E7FB5",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": 0.0,
     "shadow": {
      "color": "#3E7FB5",
      "blur": 0.03
     }
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#3E7FB5",
     "strokeWidth": 0.002,
     "rotation": 0.0
    }
   ]
  },
  {
   "id": "frame_vintage_mono",
   "name": "Camera Vintage \u2014 Mono",
   "family": "vintage",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#E8E8E8",
     "stroke": "#444444",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": 0.0,
     "shadow": {
      "color": "#444444",
      "blur": 0.03
     }
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#444444",
     "strokeWidth": 0.002,
     "rotation": 0.0
    }
   ]
  },
  {
   "id": "frame_vintage_accent",
   "name": "Camera Vintage \u2014 Accent",
   "family": "vintage",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#FFE066",
     "stroke": "#E0533B",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": 0.0,
     "shadow": {
      "color": "#E0533B",
      "blur": 0.03
     }
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#E0533B",
     "strokeWidth": 0.002,
     "rotation": 0.0
    }
   ]
  },
  {
   "id": "frame_cutpaper_original",
   "name": "Cutting Paper \u2014 Original",
   "family": "cutpaper",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#FFFFFF",
     "stroke": "#E6E6E6",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": -0.04,
     "shadow": {
      "color": "#E6E6E6",
      "blur": 0.03
     }
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#E6E6E6",
     "strokeWidth": 0.002,
     "rotation": -0.04
    }
   ]
  },
  {
   "id": "frame_cutpaper_warm",
   "name": "Cutting Paper \u2014 Warm",
   "family": "cutpaper",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#FFD9A0",
     "stroke": "#A86B32",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": -0.04,
     "shadow": {
      "color": "#A86B32",
      "blur": 0.03
     }
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#A86B32",
     "strokeWidth": 0.002,
     "rotation": -0.04
    }
   ]
  },
  {
   "id": "frame_cutpaper_cold",
   "name": "Cutting Paper \u2014 Cold",
   "family": "cutpaper",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#CFE8FF",
     "stroke": "#3E7FB5",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": -0.04,
     "shadow": {
      "color": "#3E7FB5",
      "blur": 0.03
     }
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#3E7FB5",
     "strokeWidth": 0.002,
     "rotation": -0.04
    }
   ]
  },
  {
   "id": "frame_cutpaper_mono",
   "name": "Cutting Paper \u2014 Mono",
   "family": "cutpaper",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#E8E8E8",
     "stroke": "#444444",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": -0.04,
     "shadow": {
      "color": "#444444",
      "blur": 0.03
     }
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#444444",
     "strokeWidth": 0.002,
     "rotation": -0.04
    }
   ]
  },
  {
   "id": "frame_cutpaper_accent",
   "name": "Cutting Paper \u2014 Accent",
   "family": "cutpaper",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#FFE066",
     "stroke": "#E0533B",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": -0.04,
     "shadow": {
      "color": "#E0533B",
      "blur": 0.03
     }
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#E0533B",
     "strokeWidth": 0.002,
     "rotation": -0.04
    }
   ]
  },
  {
   "id": "frame_polaroid_original",
   "name": "Polaroid \u2014 Original",
   "family": "polaroid",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#FAFAFA",
     "stroke": "#DDDDDD",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": 0.03,
     "shadow": {
      "color": "#DDDDDD",
      "blur": 0.03
     }
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#DDDDDD",
     "strokeWidth": 0.002,
     "rotation": 0.03
    }
   ]
  },
  {
   "id": "frame_polaroid_warm",
   "name": "Polaroid \u2014 Warm",
   "family": "polaroid",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#FFD9A0",
     "stroke": "#A86B32",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": 0.03,
     "shadow": {
      "color": "#A86B32",
      "blur": 0.03
     }
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#A86B32",
     "strokeWidth": 0.002,
     "rotation": 0.03
    }
   ]
  },
  {
   "id": "frame_polaroid_cold",
   "name": "Polaroid \u2014 Cold",
   "family": "polaroid",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#CFE8FF",
     "stroke": "#3E7FB5",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": 0.03,
     "shadow": {
      "color": "#3E7FB5",
      "blur": 0.03
     }
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#3E7FB5",
     "strokeWidth": 0.002,
     "rotation": 0.03
    }
   ]
  },
  {
   "id": "frame_polaroid_mono",
   "name": "Polaroid \u2014 Mono",
   "family": "polaroid",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#E8E8E8",
     "stroke": "#444444",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": 0.03,
     "shadow": {
      "color": "#444444",
      "blur": 0.03
     }
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#444444",
     "strokeWidth": 0.002,
     "rotation": 0.03
    }
   ]
  },
  {
   "id": "frame_polaroid_accent",
   "name": "Polaroid \u2014 Accent",
   "family": "polaroid",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#FFE066",
     "stroke": "#E0533B",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": 0.03,
     "shadow": {
      "color": "#E0533B",
      "blur": 0.03
     }
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#E0533B",
     "strokeWidth": 0.002,
     "rotation": 0.03
    }
   ]
  },
  {
   "id": "frame_film_original",
   "name": "Film Strip \u2014 Original",
   "family": "film",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#111111",
     "stroke": "#FFFFFF",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": 0.0,
     "shadow": {
      "color": "#FFFFFF",
      "blur": 0.03
     }
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#FFFFFF",
     "strokeWidth": 0.002,
     "rotation": 0.0
    }
   ]
  },
  {
   "id": "frame_film_warm",
   "name": "Film Strip \u2014 Warm",
   "family": "film",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#FFD9A0",
     "stroke": "#A86B32",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": 0.0,
     "shadow": {
      "color": "#A86B32",
      "blur": 0.03
     }
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#A86B32",
     "strokeWidth": 0.002,
     "rotation": 0.0
    }
   ]
  },
  {
   "id": "frame_film_cold",
   "name": "Film Strip \u2014 Cold",
   "family": "film",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#CFE8FF",
     "stroke": "#3E7FB5",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": 0.0,
     "shadow": {
      "color": "#3E7FB5",
      "blur": 0.03
     }
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#3E7FB5",
     "strokeWidth": 0.002,
     "rotation": 0.0
    }
   ]
  },
  {
   "id": "frame_film_mono",
   "name": "Film Strip \u2014 Mono",
   "family": "film",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#E8E8E8",
     "stroke": "#444444",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": 0.0,
     "shadow": {
      "color": "#444444",
      "blur": 0.03
     }
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#444444",
     "strokeWidth": 0.002,
     "rotation": 0.0
    }
   ]
  },
  {
   "id": "frame_film_accent",
   "name": "Film Strip \u2014 Accent",
   "family": "film",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#FFE066",
     "stroke": "#E0533B",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": 0.0,
     "shadow": {
      "color": "#E0533B",
      "blur": 0.03
     }
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#E0533B",
     "strokeWidth": 0.002,
     "rotation": 0.0
    }
   ]
  },
  {
   "id": "frame_neon_original",
   "name": "Neon Frame \u2014 Original",
   "family": "neon",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#0B0F1A",
     "stroke": "#00E5FF",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": 0.0,
     "shadow": null
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#00E5FF",
     "strokeWidth": 0.002,
     "rotation": 0.0
    }
   ]
  },
  {
   "id": "frame_neon_warm",
   "name": "Neon Frame \u2014 Warm",
   "family": "neon",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#FFD9A0",
     "stroke": "#A86B32",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": 0.0,
     "shadow": null
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#A86B32",
     "strokeWidth": 0.002,
     "rotation": 0.0
    }
   ]
  },
  {
   "id": "frame_neon_cold",
   "name": "Neon Frame \u2014 Cold",
   "family": "neon",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#CFE8FF",
     "stroke": "#3E7FB5",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": 0.0,
     "shadow": null
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#3E7FB5",
     "strokeWidth": 0.002,
     "rotation": 0.0
    }
   ]
  },
  {
   "id": "frame_neon_mono",
   "name": "Neon Frame \u2014 Mono",
   "family": "neon",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#E8E8E8",
     "stroke": "#444444",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": 0.0,
     "shadow": null
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#444444",
     "strokeWidth": 0.002,
     "rotation": 0.0
    }
   ]
  },
  {
   "id": "frame_neon_accent",
   "name": "Neon Frame \u2014 Accent",
   "family": "neon",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#FFE066",
     "stroke": "#E0533B",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": 0.0,
     "shadow": null
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#E0533B",
     "strokeWidth": 0.002,
     "rotation": 0.0
    }
   ]
  },
  {
   "id": "frame_dossier_original",
   "name": "Dark Dossier \u2014 Original",
   "family": "dossier",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#0E1117",
     "stroke": "#8FA3C8",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": 0.0,
     "shadow": {
      "color": "#8FA3C8",
      "blur": 0.03
     }
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#8FA3C8",
     "strokeWidth": 0.002,
     "rotation": 0.0
    }
   ]
  },
  {
   "id": "frame_dossier_warm",
   "name": "Dark Dossier \u2014 Warm",
   "family": "dossier",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#FFD9A0",
     "stroke": "#A86B32",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": 0.0,
     "shadow": {
      "color": "#A86B32",
      "blur": 0.03
     }
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#A86B32",
     "strokeWidth": 0.002,
     "rotation": 0.0
    }
   ]
  },
  {
   "id": "frame_dossier_cold",
   "name": "Dark Dossier \u2014 Cold",
   "family": "dossier",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#CFE8FF",
     "stroke": "#3E7FB5",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": 0.0,
     "shadow": {
      "color": "#3E7FB5",
      "blur": 0.03
     }
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#3E7FB5",
     "strokeWidth": 0.002,
     "rotation": 0.0
    }
   ]
  },
  {
   "id": "frame_dossier_mono",
   "name": "Dark Dossier \u2014 Mono",
   "family": "dossier",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#E8E8E8",
     "stroke": "#444444",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": 0.0,
     "shadow": {
      "color": "#444444",
      "blur": 0.03
     }
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#444444",
     "strokeWidth": 0.002,
     "rotation": 0.0
    }
   ]
  },
  {
   "id": "frame_dossier_accent",
   "name": "Dark Dossier \u2014 Accent",
   "family": "dossier",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#FFE066",
     "stroke": "#E0533B",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": 0.0,
     "shadow": {
      "color": "#E0533B",
      "blur": 0.03
     }
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#E0533B",
     "strokeWidth": 0.002,
     "rotation": 0.0
    }
   ]
  },
  {
   "id": "frame_blueprint_original",
   "name": "Blueprint \u2014 Original",
   "family": "blueprint",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#0A2340",
     "stroke": "#5EA9FF",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": 0.0,
     "shadow": null
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#5EA9FF",
     "strokeWidth": 0.002,
     "rotation": 0.0
    }
   ]
  },
  {
   "id": "frame_blueprint_warm",
   "name": "Blueprint \u2014 Warm",
   "family": "blueprint",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#FFD9A0",
     "stroke": "#A86B32",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": 0.0,
     "shadow": null
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#A86B32",
     "strokeWidth": 0.002,
     "rotation": 0.0
    }
   ]
  },
  {
   "id": "frame_blueprint_cold",
   "name": "Blueprint \u2014 Cold",
   "family": "blueprint",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#CFE8FF",
     "stroke": "#3E7FB5",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": 0.0,
     "shadow": null
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#3E7FB5",
     "strokeWidth": 0.002,
     "rotation": 0.0
    }
   ]
  },
  {
   "id": "frame_blueprint_mono",
   "name": "Blueprint \u2014 Mono",
   "family": "blueprint",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#E8E8E8",
     "stroke": "#444444",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": 0.0,
     "shadow": null
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#444444",
     "strokeWidth": 0.002,
     "rotation": 0.0
    }
   ]
  },
  {
   "id": "frame_blueprint_accent",
   "name": "Blueprint \u2014 Accent",
   "family": "blueprint",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#FFE066",
     "stroke": "#E0533B",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": 0.0,
     "shadow": null
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#E0533B",
     "strokeWidth": 0.002,
     "rotation": 0.0
    }
   ]
  },
  {
   "id": "frame_polaroid_dark_original",
   "name": "Night Polaroid \u2014 Original",
   "family": "polaroid_dark",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#1A1A1A",
     "stroke": "#E0E0E0",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": 0.03,
     "shadow": {
      "color": "#E0E0E0",
      "blur": 0.03
     }
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#E0E0E0",
     "strokeWidth": 0.002,
     "rotation": 0.03
    }
   ]
  },
  {
   "id": "frame_polaroid_dark_warm",
   "name": "Night Polaroid \u2014 Warm",
   "family": "polaroid_dark",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#FFD9A0",
     "stroke": "#A86B32",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": 0.03,
     "shadow": {
      "color": "#A86B32",
      "blur": 0.03
     }
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#A86B32",
     "strokeWidth": 0.002,
     "rotation": 0.03
    }
   ]
  },
  {
   "id": "frame_polaroid_dark_cold",
   "name": "Night Polaroid \u2014 Cold",
   "family": "polaroid_dark",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#CFE8FF",
     "stroke": "#3E7FB5",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": 0.03,
     "shadow": {
      "color": "#3E7FB5",
      "blur": 0.03
     }
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#3E7FB5",
     "strokeWidth": 0.002,
     "rotation": 0.03
    }
   ]
  },
  {
   "id": "frame_polaroid_dark_mono",
   "name": "Night Polaroid \u2014 Mono",
   "family": "polaroid_dark",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#E8E8E8",
     "stroke": "#444444",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": 0.03,
     "shadow": {
      "color": "#444444",
      "blur": 0.03
     }
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#444444",
     "strokeWidth": 0.002,
     "rotation": 0.03
    }
   ]
  },
  {
   "id": "frame_polaroid_dark_accent",
   "name": "Night Polaroid \u2014 Accent",
   "family": "polaroid_dark",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#FFE066",
     "stroke": "#E0533B",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": 0.03,
     "shadow": {
      "color": "#E0533B",
      "blur": 0.03
     }
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#E0533B",
     "strokeWidth": 0.002,
     "rotation": 0.03
    }
   ]
  },
  {
   "id": "frame_kraft_original",
   "name": "Kraft Cutout \u2014 Original",
   "family": "kraft",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#C8A97E",
     "stroke": "#8B6F47",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": -0.02,
     "shadow": {
      "color": "#8B6F47",
      "blur": 0.03
     }
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#8B6F47",
     "strokeWidth": 0.002,
     "rotation": -0.02
    }
   ]
  },
  {
   "id": "frame_kraft_warm",
   "name": "Kraft Cutout \u2014 Warm",
   "family": "kraft",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#FFD9A0",
     "stroke": "#A86B32",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": -0.02,
     "shadow": {
      "color": "#A86B32",
      "blur": 0.03
     }
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#A86B32",
     "strokeWidth": 0.002,
     "rotation": -0.02
    }
   ]
  },
  {
   "id": "frame_kraft_cold",
   "name": "Kraft Cutout \u2014 Cold",
   "family": "kraft",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#CFE8FF",
     "stroke": "#3E7FB5",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": -0.02,
     "shadow": {
      "color": "#3E7FB5",
      "blur": 0.03
     }
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#3E7FB5",
     "strokeWidth": 0.002,
     "rotation": -0.02
    }
   ]
  },
  {
   "id": "frame_kraft_mono",
   "name": "Kraft Cutout \u2014 Mono",
   "family": "kraft",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#E8E8E8",
     "stroke": "#444444",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": -0.02,
     "shadow": {
      "color": "#444444",
      "blur": 0.03
     }
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#444444",
     "strokeWidth": 0.002,
     "rotation": -0.02
    }
   ]
  },
  {
   "id": "frame_kraft_accent",
   "name": "Kraft Cutout \u2014 Accent",
   "family": "kraft",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#FFE066",
     "stroke": "#E0533B",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": -0.02,
     "shadow": {
      "color": "#E0533B",
      "blur": 0.03
     }
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#E0533B",
     "strokeWidth": 0.002,
     "rotation": -0.02
    }
   ]
  },
  {
   "id": "frame_glass_original",
   "name": "Glass Pane \u2014 Original",
   "family": "glass",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#EAF4FF",
     "stroke": "#9CC5E9",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": 0.0,
     "shadow": null
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#9CC5E9",
     "strokeWidth": 0.002,
     "rotation": 0.0
    }
   ]
  },
  {
   "id": "frame_glass_warm",
   "name": "Glass Pane \u2014 Warm",
   "family": "glass",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#FFD9A0",
     "stroke": "#A86B32",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": 0.0,
     "shadow": null
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#A86B32",
     "strokeWidth": 0.002,
     "rotation": 0.0
    }
   ]
  },
  {
   "id": "frame_glass_cold",
   "name": "Glass Pane \u2014 Cold",
   "family": "glass",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#CFE8FF",
     "stroke": "#3E7FB5",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": 0.0,
     "shadow": null
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#3E7FB5",
     "strokeWidth": 0.002,
     "rotation": 0.0
    }
   ]
  },
  {
   "id": "frame_glass_mono",
   "name": "Glass Pane \u2014 Mono",
   "family": "glass",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#E8E8E8",
     "stroke": "#444444",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": 0.0,
     "shadow": null
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#444444",
     "strokeWidth": 0.002,
     "rotation": 0.0
    }
   ]
  },
  {
   "id": "frame_glass_accent",
   "name": "Glass Pane \u2014 Accent",
   "family": "glass",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.62,
     "height": 0.78,
     "fill": "#FFE066",
     "stroke": "#E0533B",
     "strokeWidth": 0.003,
     "cornerRadius": 0.02,
     "rotation": 0.0,
     "shadow": null
    },
    {
     "kind": "square",
     "width": 0.5,
     "height": 0.64,
     "fill": "#00000000",
     "stroke": "#E0533B",
     "strokeWidth": 0.002,
     "rotation": 0.0
    }
   ]
  },
  {
   "id": "frame_dossier_sheet",
   "name": "Dossier Sheet",
   "family": "dossier_pack",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.6,
     "height": 0.78,
     "fill": "#11151C",
     "stroke": "#C9D3E0",
     "strokeWidth": 0.003,
     "rotation": 0.0,
     "shadow": {
      "color": "#C9D3E0",
      "blur": 0.02
     }
    },
    {
     "kind": "line",
     "width": 0.42,
     "fill": "#00000000",
     "stroke": "#C9D3E0",
     "strokeWidth": 0.0015,
     "rotation": 0.0,
     "_y": 0.12
    },
    {
     "kind": "line",
     "width": 0.36,
     "fill": "#00000000",
     "stroke": "#C9D3E0",
     "strokeWidth": 0.0015,
     "rotation": 0.0,
     "_y": 0.19
    },
    {
     "kind": "line",
     "width": 0.3,
     "fill": "#00000000",
     "stroke": "#C9D3E0",
     "strokeWidth": 0.0015,
     "rotation": 0.0,
     "_y": 0.26
    },
    {
     "kind": "line",
     "width": 0.42,
     "fill": "#00000000",
     "stroke": "#C9D3E0",
     "strokeWidth": 0.0015,
     "rotation": 0.0,
     "_y": 0.33
    }
   ]
  },
  {
   "id": "frame_classified_file",
   "name": "Classified File",
   "family": "dossier_pack",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.6,
     "height": 0.78,
     "fill": "#161616",
     "stroke": "#D6D6D6",
     "strokeWidth": 0.003,
     "rotation": 0.0,
     "shadow": {
      "color": "#D6D6D6",
      "blur": 0.02
     }
    },
    {
     "kind": "line",
     "width": 0.42,
     "fill": "#00000000",
     "stroke": "#D6D6D6",
     "strokeWidth": 0.0015,
     "rotation": 0.0,
     "_y": 0.12
    },
    {
     "kind": "line",
     "width": 0.36,
     "fill": "#00000000",
     "stroke": "#D6D6D6",
     "strokeWidth": 0.0015,
     "rotation": 0.0,
     "_y": 0.19
    },
    {
     "kind": "line",
     "width": 0.3,
     "fill": "#00000000",
     "stroke": "#D6D6D6",
     "strokeWidth": 0.0015,
     "rotation": 0.0,
     "_y": 0.26
    }
   ]
  },
  {
   "id": "frame_redacted_doc",
   "name": "Redacted Document",
   "family": "dossier_pack",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.6,
     "height": 0.78,
     "fill": "#0D1117",
     "stroke": "#8FA3C8",
     "strokeWidth": 0.003,
     "rotation": 0.0,
     "shadow": {
      "color": "#8FA3C8",
      "blur": 0.02
     }
    },
    {
     "kind": "line",
     "width": 0.42,
     "fill": "#00000000",
     "stroke": "#8FA3C8",
     "strokeWidth": 0.0015,
     "rotation": 0.0,
     "_y": 0.12
    },
    {
     "kind": "line",
     "width": 0.36,
     "fill": "#00000000",
     "stroke": "#8FA3C8",
     "strokeWidth": 0.0015,
     "rotation": 0.0,
     "_y": 0.19
    },
    {
     "kind": "line",
     "width": 0.3,
     "fill": "#00000000",
     "stroke": "#8FA3C8",
     "strokeWidth": 0.0015,
     "rotation": 0.0,
     "_y": 0.26
    },
    {
     "kind": "line",
     "width": 0.42,
     "fill": "#00000000",
     "stroke": "#8FA3C8",
     "strokeWidth": 0.0015,
     "rotation": 0.0,
     "_y": 0.33
    },
    {
     "kind": "line",
     "width": 0.36,
     "fill": "#00000000",
     "stroke": "#8FA3C8",
     "strokeWidth": 0.0015,
     "rotation": 0.0,
     "_y": 0.4
    }
   ]
  },
  {
   "id": "frame_evidence_photo",
   "name": "Evidence Photo",
   "family": "dossier_pack",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.6,
     "height": 0.78,
     "fill": "#101418",
     "stroke": "#E0E0E0",
     "strokeWidth": 0.003,
     "rotation": 0.0,
     "shadow": {
      "color": "#E0E0E0",
      "blur": 0.02
     }
    },
    {
     "kind": "line",
     "width": 0.42,
     "fill": "#00000000",
     "stroke": "#E0E0E0",
     "strokeWidth": 0.0015,
     "rotation": 0.0,
     "_y": 0.12
    },
    {
     "kind": "line",
     "width": 0.36,
     "fill": "#00000000",
     "stroke": "#E0E0E0",
     "strokeWidth": 0.0015,
     "rotation": 0.0,
     "_y": 0.19
    }
   ]
  },
  {
   "id": "frame_case_folder",
   "name": "Case Folder",
   "family": "dossier_pack",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.6,
     "height": 0.78,
     "fill": "#1A1A1A",
     "stroke": "#C0A060",
     "strokeWidth": 0.003,
     "rotation": 0.0,
     "shadow": {
      "color": "#C0A060",
      "blur": 0.02
     }
    },
    {
     "kind": "line",
     "width": 0.42,
     "fill": "#00000000",
     "stroke": "#C0A060",
     "strokeWidth": 0.0015,
     "rotation": 0.0,
     "_y": 0.12
    },
    {
     "kind": "line",
     "width": 0.36,
     "fill": "#00000000",
     "stroke": "#C0A060",
     "strokeWidth": 0.0015,
     "rotation": 0.0,
     "_y": 0.19
    },
    {
     "kind": "line",
     "width": 0.3,
     "fill": "#00000000",
     "stroke": "#C0A060",
     "strokeWidth": 0.0015,
     "rotation": 0.0,
     "_y": 0.26
    }
   ]
  },
  {
   "id": "frame_fingerprint_card",
   "name": "Fingerprint Card",
   "family": "dossier_pack",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.6,
     "height": 0.78,
     "fill": "#0E1117",
     "stroke": "#6FA8DC",
     "strokeWidth": 0.003,
     "rotation": 0.0,
     "shadow": {
      "color": "#6FA8DC",
      "blur": 0.02
     }
    },
    {
     "kind": "line",
     "width": 0.42,
     "fill": "#00000000",
     "stroke": "#6FA8DC",
     "strokeWidth": 0.0015,
     "rotation": 0.0,
     "_y": 0.12
    },
    {
     "kind": "line",
     "width": 0.36,
     "fill": "#00000000",
     "stroke": "#6FA8DC",
     "strokeWidth": 0.0015,
     "rotation": 0.0,
     "_y": 0.19
    },
    {
     "kind": "line",
     "width": 0.3,
     "fill": "#00000000",
     "stroke": "#6FA8DC",
     "strokeWidth": 0.0015,
     "rotation": 0.0,
     "_y": 0.26
    },
    {
     "kind": "line",
     "width": 0.42,
     "fill": "#00000000",
     "stroke": "#6FA8DC",
     "strokeWidth": 0.0015,
     "rotation": 0.0,
     "_y": 0.33
    }
   ]
  },
  {
   "id": "frame_news_clipping",
   "name": "News Clipping",
   "family": "dossier_pack",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.6,
     "height": 0.78,
     "fill": "#E8E2D4",
     "stroke": "#333333",
     "strokeWidth": 0.003,
     "rotation": 0.0,
     "shadow": {
      "color": "#333333",
      "blur": 0.02
     }
    },
    {
     "kind": "line",
     "width": 0.42,
     "fill": "#00000000",
     "stroke": "#333333",
     "strokeWidth": 0.0015,
     "rotation": 0.0,
     "_y": 0.12
    },
    {
     "kind": "line",
     "width": 0.36,
     "fill": "#00000000",
     "stroke": "#333333",
     "strokeWidth": 0.0015,
     "rotation": 0.0,
     "_y": 0.19
    }
   ]
  },
  {
   "id": "frame_police_report",
   "name": "Police Report",
   "family": "dossier_pack",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.6,
     "height": 0.78,
     "fill": "#F2F0EA",
     "stroke": "#2B2B2B",
     "strokeWidth": 0.003,
     "rotation": 0.0,
     "shadow": {
      "color": "#2B2B2B",
      "blur": 0.02
     }
    },
    {
     "kind": "line",
     "width": 0.42,
     "fill": "#00000000",
     "stroke": "#2B2B2B",
     "strokeWidth": 0.0015,
     "rotation": 0.0,
     "_y": 0.12
    },
    {
     "kind": "line",
     "width": 0.36,
     "fill": "#00000000",
     "stroke": "#2B2B2B",
     "strokeWidth": 0.0015,
     "rotation": 0.0,
     "_y": 0.19
    },
    {
     "kind": "line",
     "width": 0.3,
     "fill": "#00000000",
     "stroke": "#2B2B2B",
     "strokeWidth": 0.0015,
     "rotation": 0.0,
     "_y": 0.26
    }
   ]
  },
  {
   "id": "frame_lab_result",
   "name": "Lab Result",
   "family": "dossier_pack",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.6,
     "height": 0.78,
     "fill": "#EFF6F5",
     "stroke": "#1F7A6B",
     "strokeWidth": 0.003,
     "rotation": 0.0,
     "shadow": {
      "color": "#1F7A6B",
      "blur": 0.02
     }
    },
    {
     "kind": "line",
     "width": 0.42,
     "fill": "#00000000",
     "stroke": "#1F7A6B",
     "strokeWidth": 0.0015,
     "rotation": 0.0,
     "_y": 0.12
    },
    {
     "kind": "line",
     "width": 0.36,
     "fill": "#00000000",
     "stroke": "#1F7A6B",
     "strokeWidth": 0.0015,
     "rotation": 0.0,
     "_y": 0.19
    },
    {
     "kind": "line",
     "width": 0.3,
     "fill": "#00000000",
     "stroke": "#1F7A6B",
     "strokeWidth": 0.0015,
     "rotation": 0.0,
     "_y": 0.26
    },
    {
     "kind": "line",
     "width": 0.42,
     "fill": "#00000000",
     "stroke": "#1F7A6B",
     "strokeWidth": 0.0015,
     "rotation": 0.0,
     "_y": 0.33
    }
   ]
  },
  {
   "id": "frame_map_pin",
   "name": "Evidence Map",
   "family": "dossier_pack",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.6,
     "height": 0.78,
     "fill": "#0B1A24",
     "stroke": "#5EA9FF",
     "strokeWidth": 0.003,
     "rotation": 0.0,
     "shadow": {
      "color": "#5EA9FF",
      "blur": 0.02
     }
    },
    {
     "kind": "line",
     "width": 0.42,
     "fill": "#00000000",
     "stroke": "#5EA9FF",
     "strokeWidth": 0.0015,
     "rotation": 0.0,
     "_y": 0.12
    },
    {
     "kind": "line",
     "width": 0.36,
     "fill": "#00000000",
     "stroke": "#5EA9FF",
     "strokeWidth": 0.0015,
     "rotation": 0.0,
     "_y": 0.19
    }
   ]
  },
  {
   "id": "frame_wiretap",
   "name": "Wiretap Transcript",
   "family": "dossier_pack",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.6,
     "height": 0.78,
     "fill": "#121212",
     "stroke": "#9CC5E9",
     "strokeWidth": 0.003,
     "rotation": 0.0,
     "shadow": {
      "color": "#9CC5E9",
      "blur": 0.02
     }
    },
    {
     "kind": "line",
     "width": 0.42,
     "fill": "#00000000",
     "stroke": "#9CC5E9",
     "strokeWidth": 0.0015,
     "rotation": 0.0,
     "_y": 0.12
    },
    {
     "kind": "line",
     "width": 0.36,
     "fill": "#00000000",
     "stroke": "#9CC5E9",
     "strokeWidth": 0.0015,
     "rotation": 0.0,
     "_y": 0.19
    },
    {
     "kind": "line",
     "width": 0.3,
     "fill": "#00000000",
     "stroke": "#9CC5E9",
     "strokeWidth": 0.0015,
     "rotation": 0.0,
     "_y": 0.26
    }
   ]
  },
  {
   "id": "frame_stamp_approved",
   "name": "Approved Stamp",
   "family": "dossier_pack",
   "category": "overlay-frame",
   "elements": [
    {
     "kind": "square",
     "width": 0.6,
     "height": 0.78,
     "fill": "#0E1117",
     "stroke": "#E0533B",
     "strokeWidth": 0.003,
     "rotation": 0.0,
     "shadow": {
      "color": "#E0533B",
      "blur": 0.02
     }
    },
    {
     "kind": "line",
     "width": 0.42,
     "fill": "#00000000",
     "stroke": "#E0533B",
     "strokeWidth": 0.0015,
     "rotation": 0.0,
     "_y": 0.12
    },
    {
     "kind": "line",
     "width": 0.36,
     "fill": "#00000000",
     "stroke": "#E0533B",
     "strokeWidth": 0.0015,
     "rotation": 0.0,
     "_y": 0.19
    },
    {
     "kind": "line",
     "width": 0.3,
     "fill": "#00000000",
     "stroke": "#E0533B",
     "strokeWidth": 0.0015,
     "rotation": 0.0,
     "_y": 0.26
    },
    {
     "kind": "line",
     "width": 0.42,
     "fill": "#00000000",
     "stroke": "#E0533B",
     "strokeWidth": 0.0015,
     "rotation": 0.0,
     "_y": 0.33
    },
    {
     "kind": "line",
     "width": 0.36,
     "fill": "#00000000",
     "stroke": "#E0533B",
     "strokeWidth": 0.0015,
     "rotation": 0.0,
     "_y": 0.4
    }
   ]
  }
 ]
}"##;
const ANIMS_JSON: &str = r##"{
 "count": 76,
 "animations": [
  {
   "id": "anim_fade_in",
   "name": "Fade \u2014 IN",
   "family": "fade",
   "phase": "in",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "opacity",
     "at": 0.0,
     "value": 0.0,
     "ease": "out"
    },
    {
     "property": "opacity",
     "at": 1.0,
     "value": 1.0,
     "ease": "out"
    }
   ]
  },
  {
   "id": "anim_fade_out",
   "name": "Fade \u2014 OUT",
   "family": "fade",
   "phase": "out",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "opacity",
     "at": 0.0,
     "value": 0.0,
     "ease": "in"
    },
    {
     "property": "opacity",
     "at": 1.0,
     "value": 0.0,
     "ease": "in"
    }
   ]
  },
  {
   "id": "anim_fade_loop",
   "name": "Fade \u2014 LOOP",
   "family": "fade",
   "phase": "loop",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "opacity",
     "at": 0.0,
     "value": 0.0,
     "ease": "inOut"
    },
    {
     "property": "opacity",
     "at": 1.0,
     "value": 1.0,
     "ease": "inOut"
    }
   ]
  },
  {
   "id": "anim_slide_up_in",
   "name": "Slide Up \u2014 IN",
   "family": "slide_up",
   "phase": "in",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "offsetY",
     "at": 0.0,
     "value": 0.3,
     "ease": "out"
    },
    {
     "property": "offsetY",
     "at": 1.0,
     "value": 0.0,
     "ease": "out"
    }
   ]
  },
  {
   "id": "anim_slide_up_out",
   "name": "Slide Up \u2014 OUT",
   "family": "slide_up",
   "phase": "out",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "offsetY",
     "at": 0.0,
     "value": 0.0,
     "ease": "in"
    },
    {
     "property": "offsetY",
     "at": 1.0,
     "value": 0.0,
     "ease": "in"
    }
   ]
  },
  {
   "id": "anim_slide_up_loop",
   "name": "Slide Up \u2014 LOOP",
   "family": "slide_up",
   "phase": "loop",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "offsetY",
     "at": 0.0,
     "value": 0.3,
     "ease": "inOut"
    },
    {
     "property": "offsetY",
     "at": 1.0,
     "value": 0.0,
     "ease": "inOut"
    }
   ]
  },
  {
   "id": "anim_slide_down_in",
   "name": "Slide Down \u2014 IN",
   "family": "slide_down",
   "phase": "in",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "offsetY",
     "at": 0.0,
     "value": -0.3,
     "ease": "out"
    },
    {
     "property": "offsetY",
     "at": 1.0,
     "value": 0.0,
     "ease": "out"
    }
   ]
  },
  {
   "id": "anim_slide_down_out",
   "name": "Slide Down \u2014 OUT",
   "family": "slide_down",
   "phase": "out",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "offsetY",
     "at": 0.0,
     "value": 0.0,
     "ease": "in"
    },
    {
     "property": "offsetY",
     "at": 1.0,
     "value": 0.0,
     "ease": "in"
    }
   ]
  },
  {
   "id": "anim_slide_down_loop",
   "name": "Slide Down \u2014 LOOP",
   "family": "slide_down",
   "phase": "loop",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "offsetY",
     "at": 0.0,
     "value": -0.3,
     "ease": "inOut"
    },
    {
     "property": "offsetY",
     "at": 1.0,
     "value": 0.0,
     "ease": "inOut"
    }
   ]
  },
  {
   "id": "anim_slide_left_in",
   "name": "Slide Left \u2014 IN",
   "family": "slide_left",
   "phase": "in",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "offsetX",
     "at": 0.0,
     "value": 0.35,
     "ease": "out"
    },
    {
     "property": "offsetX",
     "at": 1.0,
     "value": 0.0,
     "ease": "out"
    }
   ]
  },
  {
   "id": "anim_slide_left_out",
   "name": "Slide Left \u2014 OUT",
   "family": "slide_left",
   "phase": "out",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "offsetX",
     "at": 0.0,
     "value": 0.0,
     "ease": "in"
    },
    {
     "property": "offsetX",
     "at": 1.0,
     "value": 0.0,
     "ease": "in"
    }
   ]
  },
  {
   "id": "anim_slide_left_loop",
   "name": "Slide Left \u2014 LOOP",
   "family": "slide_left",
   "phase": "loop",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "offsetX",
     "at": 0.0,
     "value": 0.35,
     "ease": "inOut"
    },
    {
     "property": "offsetX",
     "at": 1.0,
     "value": 0.0,
     "ease": "inOut"
    }
   ]
  },
  {
   "id": "anim_slide_right_in",
   "name": "Slide Right \u2014 IN",
   "family": "slide_right",
   "phase": "in",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "offsetX",
     "at": 0.0,
     "value": -0.35,
     "ease": "out"
    },
    {
     "property": "offsetX",
     "at": 1.0,
     "value": 0.0,
     "ease": "out"
    }
   ]
  },
  {
   "id": "anim_slide_right_out",
   "name": "Slide Right \u2014 OUT",
   "family": "slide_right",
   "phase": "out",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "offsetX",
     "at": 0.0,
     "value": 0.0,
     "ease": "in"
    },
    {
     "property": "offsetX",
     "at": 1.0,
     "value": 0.0,
     "ease": "in"
    }
   ]
  },
  {
   "id": "anim_slide_right_loop",
   "name": "Slide Right \u2014 LOOP",
   "family": "slide_right",
   "phase": "loop",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "offsetX",
     "at": 0.0,
     "value": -0.35,
     "ease": "inOut"
    },
    {
     "property": "offsetX",
     "at": 1.0,
     "value": 0.0,
     "ease": "inOut"
    }
   ]
  },
  {
   "id": "anim_pop_in",
   "name": "Pop \u2014 IN",
   "family": "pop",
   "phase": "in",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "scale",
     "at": 0.0,
     "value": 0.2,
     "ease": "out"
    },
    {
     "property": "scale",
     "at": 0.6,
     "value": 1.2,
     "ease": "out"
    },
    {
     "property": "scale",
     "at": 1.0,
     "value": 1.0,
     "ease": "out"
    }
   ]
  },
  {
   "id": "anim_pop_out",
   "name": "Pop \u2014 OUT",
   "family": "pop",
   "phase": "out",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "scale",
     "at": 0.0,
     "value": 1.0,
     "ease": "in"
    },
    {
     "property": "scale",
     "at": 0.6,
     "value": 1.0,
     "ease": "in"
    },
    {
     "property": "scale",
     "at": 1.0,
     "value": 1.0,
     "ease": "in"
    }
   ]
  },
  {
   "id": "anim_pop_loop",
   "name": "Pop \u2014 LOOP",
   "family": "pop",
   "phase": "loop",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "scale",
     "at": 0.0,
     "value": 0.2,
     "ease": "inOut"
    },
    {
     "property": "scale",
     "at": 0.6,
     "value": 1.2,
     "ease": "inOut"
    },
    {
     "property": "scale",
     "at": 1.0,
     "value": 1.0,
     "ease": "inOut"
    }
   ]
  },
  {
   "id": "anim_zoom_in_in",
   "name": "Zoom In \u2014 IN",
   "family": "zoom_in",
   "phase": "in",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "scale",
     "at": 0.0,
     "value": 0.6,
     "ease": "out"
    },
    {
     "property": "scale",
     "at": 1.0,
     "value": 1.05,
     "ease": "out"
    }
   ]
  },
  {
   "id": "anim_zoom_in_out",
   "name": "Zoom In \u2014 OUT",
   "family": "zoom_in",
   "phase": "out",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "scale",
     "at": 0.0,
     "value": 1.0,
     "ease": "in"
    },
    {
     "property": "scale",
     "at": 1.0,
     "value": 1.0,
     "ease": "in"
    }
   ]
  },
  {
   "id": "anim_zoom_in_loop",
   "name": "Zoom In \u2014 LOOP",
   "family": "zoom_in",
   "phase": "loop",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "scale",
     "at": 0.0,
     "value": 0.6,
     "ease": "inOut"
    },
    {
     "property": "scale",
     "at": 1.0,
     "value": 1.05,
     "ease": "inOut"
    }
   ]
  },
  {
   "id": "anim_zoom_out_in",
   "name": "Zoom Out \u2014 IN",
   "family": "zoom_out",
   "phase": "in",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "scale",
     "at": 0.0,
     "value": 1.4,
     "ease": "out"
    },
    {
     "property": "scale",
     "at": 1.0,
     "value": 1.0,
     "ease": "out"
    }
   ]
  },
  {
   "id": "anim_zoom_out_out",
   "name": "Zoom Out \u2014 OUT",
   "family": "zoom_out",
   "phase": "out",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "scale",
     "at": 0.0,
     "value": 1.0,
     "ease": "in"
    },
    {
     "property": "scale",
     "at": 1.0,
     "value": 1.0,
     "ease": "in"
    }
   ]
  },
  {
   "id": "anim_zoom_out_loop",
   "name": "Zoom Out \u2014 LOOP",
   "family": "zoom_out",
   "phase": "loop",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "scale",
     "at": 0.0,
     "value": 1.4,
     "ease": "inOut"
    },
    {
     "property": "scale",
     "at": 1.0,
     "value": 1.0,
     "ease": "inOut"
    }
   ]
  },
  {
   "id": "anim_spin_in",
   "name": "Spin \u2014 IN",
   "family": "spin",
   "phase": "in",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "rotation",
     "at": 0.0,
     "value": -180.0,
     "ease": "out"
    },
    {
     "property": "rotation",
     "at": 1.0,
     "value": 0.0,
     "ease": "out"
    }
   ]
  },
  {
   "id": "anim_spin_out",
   "name": "Spin \u2014 OUT",
   "family": "spin",
   "phase": "out",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "rotation",
     "at": 0.0,
     "value": 0.0,
     "ease": "in"
    },
    {
     "property": "rotation",
     "at": 1.0,
     "value": 0.0,
     "ease": "in"
    }
   ]
  },
  {
   "id": "anim_spin_loop",
   "name": "Spin \u2014 LOOP",
   "family": "spin",
   "phase": "loop",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "rotation",
     "at": 0.0,
     "value": -180.0,
     "ease": "inOut"
    },
    {
     "property": "rotation",
     "at": 1.0,
     "value": 0.0,
     "ease": "inOut"
    }
   ]
  },
  {
   "id": "anim_spin_back_in",
   "name": "Spin Back \u2014 IN",
   "family": "spin_back",
   "phase": "in",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "rotation",
     "at": 0.0,
     "value": 180.0,
     "ease": "out"
    },
    {
     "property": "rotation",
     "at": 1.0,
     "value": 0.0,
     "ease": "out"
    }
   ]
  },
  {
   "id": "anim_spin_back_out",
   "name": "Spin Back \u2014 OUT",
   "family": "spin_back",
   "phase": "out",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "rotation",
     "at": 0.0,
     "value": 0.0,
     "ease": "in"
    },
    {
     "property": "rotation",
     "at": 1.0,
     "value": 0.0,
     "ease": "in"
    }
   ]
  },
  {
   "id": "anim_spin_back_loop",
   "name": "Spin Back \u2014 LOOP",
   "family": "spin_back",
   "phase": "loop",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "rotation",
     "at": 0.0,
     "value": 180.0,
     "ease": "inOut"
    },
    {
     "property": "rotation",
     "at": 1.0,
     "value": 0.0,
     "ease": "inOut"
    }
   ]
  },
  {
   "id": "anim_flip_x_in",
   "name": "Flip X \u2014 IN",
   "family": "flip_x",
   "phase": "in",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "stretchX",
     "at": 0.0,
     "value": -1.0,
     "ease": "out"
    },
    {
     "property": "stretchX",
     "at": 1.0,
     "value": 1.0,
     "ease": "out"
    }
   ]
  },
  {
   "id": "anim_flip_x_out",
   "name": "Flip X \u2014 OUT",
   "family": "flip_x",
   "phase": "out",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "stretchX",
     "at": 0.0,
     "value": 0.0,
     "ease": "in"
    },
    {
     "property": "stretchX",
     "at": 1.0,
     "value": 0.0,
     "ease": "in"
    }
   ]
  },
  {
   "id": "anim_flip_x_loop",
   "name": "Flip X \u2014 LOOP",
   "family": "flip_x",
   "phase": "loop",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "stretchX",
     "at": 0.0,
     "value": -1.0,
     "ease": "inOut"
    },
    {
     "property": "stretchX",
     "at": 1.0,
     "value": 1.0,
     "ease": "inOut"
    }
   ]
  },
  {
   "id": "anim_flip_y_in",
   "name": "Flip Y \u2014 IN",
   "family": "flip_y",
   "phase": "in",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "stretchY",
     "at": 0.0,
     "value": -1.0,
     "ease": "out"
    },
    {
     "property": "stretchY",
     "at": 1.0,
     "value": 1.0,
     "ease": "out"
    }
   ]
  },
  {
   "id": "anim_flip_y_out",
   "name": "Flip Y \u2014 OUT",
   "family": "flip_y",
   "phase": "out",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "stretchY",
     "at": 0.0,
     "value": 0.0,
     "ease": "in"
    },
    {
     "property": "stretchY",
     "at": 1.0,
     "value": 0.0,
     "ease": "in"
    }
   ]
  },
  {
   "id": "anim_flip_y_loop",
   "name": "Flip Y \u2014 LOOP",
   "family": "flip_y",
   "phase": "loop",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "stretchY",
     "at": 0.0,
     "value": -1.0,
     "ease": "inOut"
    },
    {
     "property": "stretchY",
     "at": 1.0,
     "value": 1.0,
     "ease": "inOut"
    }
   ]
  },
  {
   "id": "anim_bounce_in",
   "name": "Bounce \u2014 IN",
   "family": "bounce",
   "phase": "in",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "scale",
     "at": 0.0,
     "value": 0.2,
     "ease": "out"
    },
    {
     "property": "scale",
     "at": 0.5,
     "value": 1.18,
     "ease": "out"
    },
    {
     "property": "scale",
     "at": 0.75,
     "value": 0.94,
     "ease": "out"
    },
    {
     "property": "scale",
     "at": 1.0,
     "value": 1.0,
     "ease": "out"
    }
   ]
  },
  {
   "id": "anim_bounce_out",
   "name": "Bounce \u2014 OUT",
   "family": "bounce",
   "phase": "out",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "scale",
     "at": 0.0,
     "value": 1.0,
     "ease": "in"
    },
    {
     "property": "scale",
     "at": 0.5,
     "value": 1.0,
     "ease": "in"
    },
    {
     "property": "scale",
     "at": 0.75,
     "value": 1.0,
     "ease": "in"
    },
    {
     "property": "scale",
     "at": 1.0,
     "value": 1.0,
     "ease": "in"
    }
   ]
  },
  {
   "id": "anim_bounce_loop",
   "name": "Bounce \u2014 LOOP",
   "family": "bounce",
   "phase": "loop",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "scale",
     "at": 0.0,
     "value": 0.2,
     "ease": "inOut"
    },
    {
     "property": "scale",
     "at": 0.5,
     "value": 1.18,
     "ease": "inOut"
    },
    {
     "property": "scale",
     "at": 0.75,
     "value": 0.94,
     "ease": "inOut"
    },
    {
     "property": "scale",
     "at": 1.0,
     "value": 1.0,
     "ease": "inOut"
    }
   ]
  },
  {
   "id": "anim_glitch_in",
   "name": "Glitch \u2014 IN",
   "family": "glitch",
   "phase": "in",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "offsetX",
     "at": 0.0,
     "value": 0.08,
     "ease": "out"
    },
    {
     "property": "offsetX",
     "at": 0.3,
     "value": -0.05,
     "ease": "out"
    },
    {
     "property": "offsetX",
     "at": 0.6,
     "value": 0.03,
     "ease": "out"
    },
    {
     "property": "offsetX",
     "at": 1.0,
     "value": 0.0,
     "ease": "out"
    }
   ]
  },
  {
   "id": "anim_glitch_out",
   "name": "Glitch \u2014 OUT",
   "family": "glitch",
   "phase": "out",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "offsetX",
     "at": 0.0,
     "value": 0.0,
     "ease": "in"
    },
    {
     "property": "offsetX",
     "at": 0.3,
     "value": 0.0,
     "ease": "in"
    },
    {
     "property": "offsetX",
     "at": 0.6,
     "value": 0.0,
     "ease": "in"
    },
    {
     "property": "offsetX",
     "at": 1.0,
     "value": 0.0,
     "ease": "in"
    }
   ]
  },
  {
   "id": "anim_glitch_loop",
   "name": "Glitch \u2014 LOOP",
   "family": "glitch",
   "phase": "loop",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "offsetX",
     "at": 0.0,
     "value": 0.08,
     "ease": "inOut"
    },
    {
     "property": "offsetX",
     "at": 0.3,
     "value": -0.05,
     "ease": "inOut"
    },
    {
     "property": "offsetX",
     "at": 0.6,
     "value": 0.03,
     "ease": "inOut"
    },
    {
     "property": "offsetX",
     "at": 1.0,
     "value": 0.0,
     "ease": "inOut"
    }
   ]
  },
  {
   "id": "anim_drop_in",
   "name": "Drop In \u2014 IN",
   "family": "drop",
   "phase": "in",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "offsetY",
     "at": 0.0,
     "value": -0.5,
     "ease": "out"
    },
    {
     "property": "offsetY",
     "at": 0.6,
     "value": 0.06,
     "ease": "out"
    },
    {
     "property": "offsetY",
     "at": 1.0,
     "value": 0.0,
     "ease": "out"
    }
   ]
  },
  {
   "id": "anim_drop_out",
   "name": "Drop In \u2014 OUT",
   "family": "drop",
   "phase": "out",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "offsetY",
     "at": 0.0,
     "value": 0.0,
     "ease": "in"
    },
    {
     "property": "offsetY",
     "at": 0.6,
     "value": 0.0,
     "ease": "in"
    },
    {
     "property": "offsetY",
     "at": 1.0,
     "value": 0.0,
     "ease": "in"
    }
   ]
  },
  {
   "id": "anim_drop_loop",
   "name": "Drop In \u2014 LOOP",
   "family": "drop",
   "phase": "loop",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "offsetY",
     "at": 0.0,
     "value": -0.5,
     "ease": "inOut"
    },
    {
     "property": "offsetY",
     "at": 0.6,
     "value": 0.06,
     "ease": "inOut"
    },
    {
     "property": "offsetY",
     "at": 1.0,
     "value": 0.0,
     "ease": "inOut"
    }
   ]
  },
  {
   "id": "anim_swing_in",
   "name": "Swing \u2014 IN",
   "family": "swing",
   "phase": "in",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "rotation",
     "at": 0.0,
     "value": -12.0,
     "ease": "out"
    },
    {
     "property": "rotation",
     "at": 0.5,
     "value": 8.0,
     "ease": "out"
    },
    {
     "property": "rotation",
     "at": 1.0,
     "value": 0.0,
     "ease": "out"
    }
   ]
  },
  {
   "id": "anim_swing_out",
   "name": "Swing \u2014 OUT",
   "family": "swing",
   "phase": "out",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "rotation",
     "at": 0.0,
     "value": 0.0,
     "ease": "in"
    },
    {
     "property": "rotation",
     "at": 0.5,
     "value": 0.0,
     "ease": "in"
    },
    {
     "property": "rotation",
     "at": 1.0,
     "value": 0.0,
     "ease": "in"
    }
   ]
  },
  {
   "id": "anim_swing_loop",
   "name": "Swing \u2014 LOOP",
   "family": "swing",
   "phase": "loop",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "rotation",
     "at": 0.0,
     "value": -12.0,
     "ease": "inOut"
    },
    {
     "property": "rotation",
     "at": 0.5,
     "value": 8.0,
     "ease": "inOut"
    },
    {
     "property": "rotation",
     "at": 1.0,
     "value": 0.0,
     "ease": "inOut"
    }
   ]
  },
  {
   "id": "anim_reveal_in",
   "name": "Reveal \u2014 IN",
   "family": "reveal",
   "phase": "in",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "opacity",
     "at": 0.0,
     "value": 0.0,
     "ease": "out"
    },
    {
     "property": "opacity",
     "at": 0.2,
     "value": 1.0,
     "ease": "out"
    },
    {
     "property": "scale",
     "at": 0.0,
     "value": 0.85,
     "ease": "out"
    },
    {
     "property": "scale",
     "at": 1.0,
     "value": 1.0,
     "ease": "out"
    }
   ]
  },
  {
   "id": "anim_reveal_out",
   "name": "Reveal \u2014 OUT",
   "family": "reveal",
   "phase": "out",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "opacity",
     "at": 0.0,
     "value": 0.0,
     "ease": "in"
    },
    {
     "property": "opacity",
     "at": 0.2,
     "value": 0.0,
     "ease": "in"
    },
    {
     "property": "scale",
     "at": 0.0,
     "value": 1.0,
     "ease": "in"
    },
    {
     "property": "scale",
     "at": 1.0,
     "value": 1.0,
     "ease": "in"
    }
   ]
  },
  {
   "id": "anim_reveal_loop",
   "name": "Reveal \u2014 LOOP",
   "family": "reveal",
   "phase": "loop",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "opacity",
     "at": 0.0,
     "value": 0.0,
     "ease": "inOut"
    },
    {
     "property": "opacity",
     "at": 0.2,
     "value": 1.0,
     "ease": "inOut"
    },
    {
     "property": "scale",
     "at": 0.0,
     "value": 0.85,
     "ease": "inOut"
    },
    {
     "property": "scale",
     "at": 1.0,
     "value": 1.0,
     "ease": "inOut"
    }
   ]
  },
  {
   "id": "anim_pulse_in",
   "name": "Pulse \u2014 IN",
   "family": "pulse",
   "phase": "in",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "scale",
     "at": 0.0,
     "value": 1.0,
     "ease": "out"
    },
    {
     "property": "scale",
     "at": 0.5,
     "value": 1.12,
     "ease": "out"
    },
    {
     "property": "scale",
     "at": 1.0,
     "value": 1.0,
     "ease": "out"
    }
   ]
  },
  {
   "id": "anim_pulse_out",
   "name": "Pulse \u2014 OUT",
   "family": "pulse",
   "phase": "out",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "scale",
     "at": 0.0,
     "value": 1.0,
     "ease": "in"
    },
    {
     "property": "scale",
     "at": 0.5,
     "value": 1.0,
     "ease": "in"
    },
    {
     "property": "scale",
     "at": 1.0,
     "value": 1.0,
     "ease": "in"
    }
   ]
  },
  {
   "id": "anim_pulse_loop",
   "name": "Pulse \u2014 LOOP",
   "family": "pulse",
   "phase": "loop",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "scale",
     "at": 0.0,
     "value": 1.0,
     "ease": "inOut"
    },
    {
     "property": "scale",
     "at": 0.5,
     "value": 1.12,
     "ease": "inOut"
    },
    {
     "property": "scale",
     "at": 1.0,
     "value": 1.0,
     "ease": "inOut"
    }
   ]
  },
  {
   "id": "anim_float_in",
   "name": "Float \u2014 IN",
   "family": "float",
   "phase": "in",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "offsetY",
     "at": 0.0,
     "value": -0.02,
     "ease": "out"
    },
    {
     "property": "offsetY",
     "at": 0.5,
     "value": 0.03,
     "ease": "out"
    },
    {
     "property": "offsetY",
     "at": 1.0,
     "value": -0.02,
     "ease": "out"
    }
   ]
  },
  {
   "id": "anim_float_out",
   "name": "Float \u2014 OUT",
   "family": "float",
   "phase": "out",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "offsetY",
     "at": 0.0,
     "value": 0.0,
     "ease": "in"
    },
    {
     "property": "offsetY",
     "at": 0.5,
     "value": 0.0,
     "ease": "in"
    },
    {
     "property": "offsetY",
     "at": 1.0,
     "value": 0.0,
     "ease": "in"
    }
   ]
  },
  {
   "id": "anim_float_loop",
   "name": "Float \u2014 LOOP",
   "family": "float",
   "phase": "loop",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "offsetY",
     "at": 0.0,
     "value": -0.02,
     "ease": "inOut"
    },
    {
     "property": "offsetY",
     "at": 0.5,
     "value": 0.03,
     "ease": "inOut"
    },
    {
     "property": "offsetY",
     "at": 1.0,
     "value": -0.02,
     "ease": "inOut"
    }
   ]
  },
  {
   "id": "anim_dossier_reveal_in",
   "name": "Dossier Reveal \u2014 IN",
   "family": "dossier",
   "phase": "in",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "opacity",
     "at": 0,
     "value": 0,
     "ease": "out"
    },
    {
     "property": "opacity",
     "at": 0.25,
     "value": 1,
     "ease": "linear"
    },
    {
     "property": "offsetY",
     "at": 0,
     "value": 0.12,
     "ease": "out"
    },
    {
     "property": "offsetY",
     "at": 1,
     "value": 0,
     "ease": "linear"
    },
    {
     "property": "rotation",
     "at": 0,
     "value": 3,
     "ease": "out"
    },
    {
     "property": "rotation",
     "at": 1,
     "value": 0,
     "ease": "linear"
    }
   ]
  },
  {
   "id": "anim_page_flip_in",
   "name": "Page Flip \u2014 IN",
   "family": "page_flip",
   "phase": "in",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "rotation",
     "at": 0.0,
     "value": -28.0,
     "ease": "out"
    },
    {
     "property": "rotation",
     "at": 0.5,
     "value": 8.0,
     "ease": "out"
    },
    {
     "property": "rotation",
     "at": 1.0,
     "value": 0.0,
     "ease": "out"
    }
   ]
  },
  {
   "id": "anim_page_flip_out",
   "name": "Page Flip \u2014 OUT",
   "family": "page_flip",
   "phase": "out",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "rotation",
     "at": 0.0,
     "value": -28.0,
     "ease": "in"
    },
    {
     "property": "rotation",
     "at": 0.5,
     "value": 8.0,
     "ease": "in"
    },
    {
     "property": "rotation",
     "at": 1.0,
     "value": 0.0,
     "ease": "in"
    }
   ]
  },
  {
   "id": "anim_page_flip_loop",
   "name": "Page Flip \u2014 LOOP",
   "family": "page_flip",
   "phase": "loop",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "rotation",
     "at": 0.0,
     "value": -28.0,
     "ease": "inOut"
    },
    {
     "property": "rotation",
     "at": 0.5,
     "value": 8.0,
     "ease": "inOut"
    },
    {
     "property": "rotation",
     "at": 1.0,
     "value": 0.0,
     "ease": "inOut"
    }
   ]
  },
  {
   "id": "anim_stamp_in",
   "name": "Stamp Down \u2014 IN",
   "family": "stamp",
   "phase": "in",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "scale",
     "at": 0.0,
     "value": 2.2,
     "ease": "out"
    },
    {
     "property": "scale",
     "at": 0.4,
     "value": 0.92,
     "ease": "out"
    },
    {
     "property": "scale",
     "at": 1.0,
     "value": 1.0,
     "ease": "out"
    }
   ]
  },
  {
   "id": "anim_stamp_out",
   "name": "Stamp Down \u2014 OUT",
   "family": "stamp",
   "phase": "out",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "scale",
     "at": 0.0,
     "value": 2.2,
     "ease": "in"
    },
    {
     "property": "scale",
     "at": 0.4,
     "value": 0.92,
     "ease": "in"
    },
    {
     "property": "scale",
     "at": 1.0,
     "value": 1.0,
     "ease": "in"
    }
   ]
  },
  {
   "id": "anim_stamp_loop",
   "name": "Stamp Down \u2014 LOOP",
   "family": "stamp",
   "phase": "loop",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "scale",
     "at": 0.0,
     "value": 2.2,
     "ease": "inOut"
    },
    {
     "property": "scale",
     "at": 0.4,
     "value": 0.92,
     "ease": "inOut"
    },
    {
     "property": "scale",
     "at": 1.0,
     "value": 1.0,
     "ease": "inOut"
    }
   ]
  },
  {
   "id": "anim_type_lines_in",
   "name": "Type Lines \u2014 IN",
   "family": "type_lines",
   "phase": "in",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "opacity",
     "at": 0.0,
     "value": 0.0,
     "ease": "out"
    },
    {
     "property": "opacity",
     "at": 0.3,
     "value": 1.0,
     "ease": "out"
    },
    {
     "property": "offsetY",
     "at": 0.0,
     "value": 0.06,
     "ease": "out"
    },
    {
     "property": "offsetY",
     "at": 1.0,
     "value": 0.0,
     "ease": "out"
    }
   ]
  },
  {
   "id": "anim_type_lines_out",
   "name": "Type Lines \u2014 OUT",
   "family": "type_lines",
   "phase": "out",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "opacity",
     "at": 0.0,
     "value": 0.0,
     "ease": "in"
    },
    {
     "property": "opacity",
     "at": 0.3,
     "value": 1.0,
     "ease": "in"
    },
    {
     "property": "offsetY",
     "at": 0.0,
     "value": 0.06,
     "ease": "in"
    },
    {
     "property": "offsetY",
     "at": 1.0,
     "value": 0.0,
     "ease": "in"
    }
   ]
  },
  {
   "id": "anim_type_lines_loop",
   "name": "Type Lines \u2014 LOOP",
   "family": "type_lines",
   "phase": "loop",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "opacity",
     "at": 0.0,
     "value": 0.0,
     "ease": "inOut"
    },
    {
     "property": "opacity",
     "at": 0.3,
     "value": 1.0,
     "ease": "inOut"
    },
    {
     "property": "offsetY",
     "at": 0.0,
     "value": 0.06,
     "ease": "inOut"
    },
    {
     "property": "offsetY",
     "at": 1.0,
     "value": 0.0,
     "ease": "inOut"
    }
   ]
  },
  {
   "id": "anim_highlight_in",
   "name": "Highlight Sweep \u2014 IN",
   "family": "highlight",
   "phase": "in",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "offsetX",
     "at": 0.0,
     "value": -0.3,
     "ease": "out"
    },
    {
     "property": "offsetX",
     "at": 1.0,
     "value": 0.3,
     "ease": "out"
    }
   ]
  },
  {
   "id": "anim_highlight_out",
   "name": "Highlight Sweep \u2014 OUT",
   "family": "highlight",
   "phase": "out",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "offsetX",
     "at": 0.0,
     "value": -0.3,
     "ease": "in"
    },
    {
     "property": "offsetX",
     "at": 1.0,
     "value": 0.3,
     "ease": "in"
    }
   ]
  },
  {
   "id": "anim_highlight_loop",
   "name": "Highlight Sweep \u2014 LOOP",
   "family": "highlight",
   "phase": "loop",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "offsetX",
     "at": 0.0,
     "value": -0.3,
     "ease": "inOut"
    },
    {
     "property": "offsetX",
     "at": 1.0,
     "value": 0.3,
     "ease": "inOut"
    }
   ]
  },
  {
   "id": "anim_redact_in",
   "name": "Redact \u2014 IN",
   "family": "redact",
   "phase": "in",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "scale",
     "at": 0.0,
     "value": 1.4,
     "ease": "out"
    },
    {
     "property": "scale",
     "at": 1.0,
     "value": 1.0,
     "ease": "out"
    }
   ]
  },
  {
   "id": "anim_redact_out",
   "name": "Redact \u2014 OUT",
   "family": "redact",
   "phase": "out",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "scale",
     "at": 0.0,
     "value": 1.4,
     "ease": "in"
    },
    {
     "property": "scale",
     "at": 1.0,
     "value": 1.0,
     "ease": "in"
    }
   ]
  },
  {
   "id": "anim_redact_loop",
   "name": "Redact \u2014 LOOP",
   "family": "redact",
   "phase": "loop",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "scale",
     "at": 0.0,
     "value": 1.4,
     "ease": "inOut"
    },
    {
     "property": "scale",
     "at": 1.0,
     "value": 1.0,
     "ease": "inOut"
    }
   ]
  },
  {
   "id": "anim_scan_in",
   "name": "Document Scan \u2014 IN",
   "family": "scan",
   "phase": "in",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "offsetY",
     "at": 0.0,
     "value": 0.35,
     "ease": "out"
    },
    {
     "property": "offsetY",
     "at": 1.0,
     "value": 0.0,
     "ease": "out"
    }
   ]
  },
  {
   "id": "anim_scan_out",
   "name": "Document Scan \u2014 OUT",
   "family": "scan",
   "phase": "out",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "offsetY",
     "at": 0.0,
     "value": 0.35,
     "ease": "in"
    },
    {
     "property": "offsetY",
     "at": 1.0,
     "value": 0.0,
     "ease": "in"
    }
   ]
  },
  {
   "id": "anim_scan_loop",
   "name": "Document Scan \u2014 LOOP",
   "family": "scan",
   "phase": "loop",
   "category": "overlay-animation",
   "keys": [
    {
     "property": "offsetY",
     "at": 0.0,
     "value": 0.35,
     "ease": "inOut"
    },
    {
     "property": "offsetY",
     "at": 1.0,
     "value": 0.0,
     "ease": "inOut"
    }
   ]
  }
 ]
}"##;

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
    for fx in &effects {
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
                "duration":dur/count as f64 - 0.1, "name":content.clone(),
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

fn frames() -> Vec<Value> { serde_json::from_str(FRAMES_JSON).unwrap_or_default() }
fn anims() -> Vec<Value> { serde_json::from_str(ANIMS_JSON).unwrap_or_default() }

pub fn list_overlay_frames() -> Value {
    let items: Vec<Value> = frames().iter().map(|f| json!({
        "id": f["id"], "name": f["name"], "family": f["family"],
    })).collect();
    json!({ "frames": items, "count": items.len() })
}

pub fn list_overlay_animations() -> Value {
    let items: Vec<Value> = anims().iter().map(|a| json!({
        "id": a["id"], "name": a["name"], "phase": a["phase"],
    })).collect();
    json!({ "animations": items, "count": items.len() })
}

pub fn add_overlay_frame(args: &Value) -> Value {
    let fid = args.get("frameId").and_then(Value::as_str).unwrap_or("");
    let start = f(args, "start", 0.0);
    let dur = f(args, "duration", 5.0).max(0.1);
    let aid = args.get("animationId").and_then(Value::as_str);
    let Some(frame) = frames().into_iter().find(|x| x["id"] == fid) else {
        return json!({ "op":"batch", "commands":[] });
    };
    let anim = aid.and_then(|id| anims().into_iter().find(|x| x["id"] == id));
    let mut commands: Vec<Value> = Vec::new();
    if let Some(items) = frame.get("elements").and_then(Value::as_array) {
        for (i, el) in items.iter().enumerate() {
            let cid = format!("ov{}", i);
            commands.push(json!({ "op":"addShapeClip", "start":start, "duration":dur,
                "name":format!("{} {}", frame["name"], i+1), "style":el }));
            if let Some(a) = &anim {
                if let Some(keys) = a.get("keys").and_then(Value::as_array) {
                    for key in keys {
                        commands.push(json!({ "op":"setClipKey", "clipId":cid,
                            "property":key["property"], "at":f(key,"at",0.0),
                            "value":f(key,"value",0.0),
                            "ease":key.get("ease").and_then(Value::as_str).unwrap_or("linear") }));
                    }
                }
            }
        }
    }
    json!({ "op":"batch", "frame":frame["name"], "animation":aid, "commands":commands })
}

pub fn apply_overlay_animation(args: &Value) -> Value {
    let aid = args.get("animationId").and_then(Value::as_str).unwrap_or("");
    let clip = args.get("clipId").and_then(Value::as_str).unwrap_or("ov0");
    let start = f(args, "start", 0.0);
    let dur = f(args, "duration", 5.0).max(0.1);
    let Some(a) = anims().into_iter().find(|x| x["id"] == aid) else {
        return json!({ "op":"batch", "commands":[] });
    };
    let mut commands: Vec<Value> = Vec::new();
    if let Some(keys) = a.get("keys").and_then(Value::as_array) {
        for key in keys {
            let at = ((start + f(key,"at",0.0)*dur) / dur).clamp(0.0,1.0);
            commands.push(json!({ "op":"setClipKey", "clipId":clip,
                "property":key["property"], "at":at, "value":f(key,"value",0.0),
                "ease":key.get("ease").and_then(Value::as_str).unwrap_or("linear") }));
        }
    }
    json!({ "op":"batch", "animation":a["name"], "commands":commands })
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
