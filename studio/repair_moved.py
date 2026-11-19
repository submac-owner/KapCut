import pathlib
p = pathlib.Path(r"C:\Users\Admin\Desktop\Capcut\KapCut\kapcut\studio\generate_studio_rs.py")
t = p.read_text(encoding="utf-8")
old = r'\"duration\":dur/count as f64 - 0.1, \"name\":content,'
new = r'\"duration\":dur/count as f64 - 0.1, \"name\":content.clone(),'
assert old in t
p.write_text(t.replace(old, new), encoding="utf-8")
print("repaired moved content")
