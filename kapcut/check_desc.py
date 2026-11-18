import pathlib, re
t = pathlib.Path(r"C:\Users\Admin\Desktop\Capcut\KapCut\src\crates\concat-mcp\src\tools\capcut_tools.rs").read_text(encoding="utf-8")
ds = re.findall(r'description: "([^"]*)"', t)
print("descriptions:", len(ds), "min:", min(len(x) for x in ds), "max:", max(len(x) for x in ds))
