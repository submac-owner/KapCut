#!/usr/bin/env python3
"""Download open-licensed fonts from the google/fonts GitHub repo (real TTFs).

Output: fonts/*.ttf, fonts/*-LICENSE.txt, fonts.json registry.
Licenses: SIL OFL 1.1 or Apache 2.0 - free for personal and commercial use.
"""
import json, pathlib, urllib.request

HERE = pathlib.Path(__file__).resolve().parent
HEADERS = {"User-Agent": "opencode"}

FONTS = {
    "Display / impact (titles, kinetic text)": [
        "Anton", "Bebas Neue", "Archivo Black", "Oswald", "Alfa Slab One",
        "Abril Fatface", "Bangers", "Teko",
    ],
    "Clean sans (captions, UI, lower thirds)": [
        "Inter", "Montserrat", "Poppins", "Roboto", "Open Sans", "Lato",
        "Raleway", "Work Sans",
    ],
    "Serif (elegant, editorial)": [
        "Playfair Display", "Merriweather", "Lora", "Cormorant Garamond",
    ],
    "Script / handwriting": [
        "Dancing Script", "Pacifico", "Caveat",
    ],
    "Mono / tech": [
        "JetBrains Mono", "Fira Code", "Roboto Mono", "Space Mono",
    ],
}


def get(url):
    req = urllib.request.Request(url, headers=HEADERS)
    with urllib.request.urlopen(req, timeout=60) as r:
        return r.read()


def listing(slug):
    for base in ("ofl", "apache"):
        try:
            data = json.loads(get(f"https://api.github.com/repos/google/fonts/contents/{base}/{slug}"))
            if isinstance(data, list):
                return base, data
        except Exception:
            pass
    return None, []


manifest = []
for category, families in FONTS.items():
    for family in families:
        slug = family.lower().replace(" ", "")
        try:
            base, files = listing(slug)
            if not files:
                print(f"SKIP {family}: no repo folder")
                continue
            ttfs = [f for f in files if f["name"].lower().endswith(".ttf") and "italic" not in f["name"].lower()]
            if not ttfs:
                ttfs = [f for f in files if f["name"].lower().endswith(".ttf")]
            if not ttfs:
                print(f"SKIP {family}: no TTF")
                continue
            chosen = ttfs[0]
            data = get(chosen["download_url"])
            if data[:4] not in (b"\x00\x01\x00\x00", b"OTTO", b"true"):
                print(f"SKIP {family}: not a TTF")
                continue
            fname = f"{slug}-Regular.ttf".replace("[", "").replace("]", "").replace(",", "-")
            (HERE / fname).write_bytes(data)
            lic = "Open Font License 1.1"
            for lf in files:
                if lf["name"].lower() in ("ofl.txt", "license.txt"):
                    lic = get(lf["download_url"]).decode("utf-8", "replace")
                    break
            lname = f"{fname}-LICENSE.txt"
            (HERE / lname).write_text(lic, encoding="utf-8")
            manifest.append({
                "family": family, "file": fname, "category": category,
                "licenseFile": lname, "licenseType": "OFL" if base == "ofl" else "Apache-2.0",
                "source": chosen["download_url"], "bytes": len(data),
            })
            print(f"OK   {family:<20} {len(data)//1024:>5} KB  {chosen['name']}")
        except Exception as e:
            print(f"SKIP {family}: {e}")

(HERE / "fonts.json").write_text(json.dumps({
    "count": len(manifest), "source": "google/fonts GitHub (SIL OFL 1.1 / Apache 2.0)",
    "note": "Free for personal and commercial use. Keep license files with the fonts.",
    "fonts": manifest,
}, indent=2), encoding="utf-8")
print(f"\nDownloaded {len(manifest)} fonts. Registry: {HERE / 'fonts.json'}")
