import pathlib, re, sys
p = pathlib.Path(sys.argv[1]) if len(sys.argv) > 1 else pathlib.Path(
    r"C:\Users\Admin\Desktop\Capcut\KapCut\src\crates\concat-mcp\src\tools\capcut_voice.rs")
t = p.read_text(encoding="utf-8")
s = re.sub(r'r##".*?"##', "", t, flags=re.S)
s = re.sub(r'"(?:\\.|[^"\\])*"', "", s)
s = re.sub(r"'(?:\\.|[^'\\])'", "", s)
s = re.sub(r"//.*", "", s)
depth = 0
for ch in s:
    if ch == "{": depth += 1
    if ch == "}": depth -= 1
print(p.name, "brace depth", depth)
