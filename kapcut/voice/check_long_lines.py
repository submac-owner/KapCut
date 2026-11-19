import pathlib, re
p = pathlib.Path(r"C:\Users\Admin\Desktop\Capcut\KapCut\src\crates\concat-mcp\src\tools.rs")
lines = p.read_text(encoding="utf-8").split("\n")
# find bodies of functions marked #[rustfmt::skip]
skip = set()
for i, line in enumerate(lines):
    if line.strip() == "#[rustfmt::skip]":
        j = i + 1
        while j < len(lines) and not lines[j].startswith("fn "):
            j += 1
        if j < len(lines):
            depth = 0
            started = False
            for k in range(j, len(lines)):
                for ch in lines[k]:
                    if ch == "{": depth += 1; started = True
                    if ch == "}": depth -= 1
                skip.add(k + 1)
                if started and depth == 0: break
bad = [(i, len(l)) for i, l in enumerate(lines, 1) if i not in skip and len(l) > 110]
print("long lines outside skipped blocks:", len(bad))
for i, n in bad[:20]: print(i, n)
