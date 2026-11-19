import pathlib
p = pathlib.Path(r"C:\Users\Admin\Desktop\Capcut\KapCut\src\crates\concat-mcp\src\tools.rs")
lines = p.read_text(encoding="utf-8").split("\n")
# skip the rustfmt::skip helper body
start = next(i for i, l in enumerate(lines) if l.startswith("fn voice_edit("))
end = next(i for i, l in enumerate(lines[start:], start) if l == "}")
for i, line in enumerate(lines, 1):
    if start + 1 <= i <= end + 1:
        continue
    if len(line) > 110:
        print(i, len(line), line[:130])
print("checked", len(lines), "lines; helper", start + 1, "-", end + 1)
