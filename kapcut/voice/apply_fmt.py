import re, pathlib
log = pathlib.Path(r"C:\Users\Admin\AppData\Local\Temp\opencode\fmtlog.txt").read_text(encoding="utf-8", errors="replace")
target = pathlib.Path(r"C:\Users\Admin\Desktop\Capcut\KapCut\src\crates\concat-mcp\src\http.rs")
text = target.read_text(encoding="utf-8")

lines = log.split("\n")
hunks = []
cur = None
for raw in lines:
    m = re.match(r"^\S+Z (.*)$", raw)
    if not m:
        continue
    content = m.group(1)
    if content.startswith("Diff in "):
        cur = {"old": [], "new": []}
        hunks.append(cur)
        continue
    if cur is None:
        continue
    if content.startswith("-"):
        cur["old"].append(content[1:])
    elif content.startswith("+"):
        cur["new"].append(content[1:])

count = 0
for h in hunks:
    if not h["old"] or not h["new"]:
        continue
    old = "\n".join(h["old"])
    new = "\n".join(h["new"])
    if old in text:
        text = text.replace(old, new, 1)
        count += 1
    else:
        print("MISS:", repr(old[:90]))

target.write_text(text, encoding="utf-8")
print(f"applied {count}/{len(hunks)} hunks")
