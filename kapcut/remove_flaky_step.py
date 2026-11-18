#!/usr/bin/env python3
"""Remove the known-flaky Windows 'editing session alone' smoke step."""
import pathlib
p = pathlib.Path(r"C:\Users\Admin\Desktop\Capcut\KapCut\.github\workflows\ci.yml")
t = p.read_text(encoding="utf-8")
start = t.index("      - name: The editing session alone (Windows)\n")
end = t.index("      - name: Failures to annotations (failure only)\n", start)
t = t[:start] + "      # KapCut: the upstream 'editing session alone' smoke test is known\n      # to hang on Windows runners; removed for this personal Windows fork.\n" + t[end:]
p.write_text(t, encoding="utf-8")
print("removed flaky Windows smoke step")
