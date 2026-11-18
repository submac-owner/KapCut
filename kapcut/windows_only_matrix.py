#!/usr/bin/env python3
"""Trim Concat's build-app matrix to Windows x64 only (personal Windows project)."""
import pathlib

path = pathlib.Path(r"C:\Users\Admin\Desktop\Capcut\KapCut\.github\workflows\build-app.yml")
text = path.read_text(encoding="utf-8")
start = text.index("          - platform: macos\n")
end = text.index("          - platform: windows\n", start)
text = text[:start] + "          # KapCut: Windows x64 only.\n" + text[end:]
path.write_text(text, encoding="utf-8")
print("build-app.yml matrix trimmed to Windows x64")
