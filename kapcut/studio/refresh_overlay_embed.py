import pathlib, re
base = pathlib.Path(r"C:\Users\Admin\Desktop\Capcut\KapCut\kapcut\studio")
gen = base / "generate_studio_rs.py"
t = gen.read_text(encoding="utf-8")
frames = (base / "overlay_frames.json").read_text(encoding="utf-8").strip()
anims = (base / "overlay_animations.json").read_text(encoding="utf-8").strip()
t = re.sub(r'const FRAMES_JSON: &str = r##".*?"##;', lambda m: 'const FRAMES_JSON: &str = r##"' + frames + '"##;', t, count=1, flags=re.S)
t = re.sub(r'const ANIMS_JSON: &str = r##".*?"##;', lambda m: 'const ANIMS_JSON: &str = r##"' + anims + '"##;', t, count=1, flags=re.S)
gen.write_text(t, encoding="utf-8")
print("embedded JSON refreshed")
