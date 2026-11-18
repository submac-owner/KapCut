# Font library

27 open-licensed fonts for motion graphics and video editing, downloaded from
the [google/fonts](https://github.com/google/fonts) repository.

**Licenses:** SIL Open Font License 1.1 or Apache 2.0 — free for personal and
commercial use. Each font has a matching `*-LICENSE.txt` next to it. Keep the
license files with the fonts.

## Categories

| Category | Families |
|---|---|
| Display / impact (titles, kinetic text) | Anton, Bebas Neue, Archivo Black, Oswald, Alfa Slab One, Abril Fatface, Bangers, Teko |
| Clean sans (captions, UI, lower thirds) | Inter, Montserrat, Poppins, Roboto, Open Sans, Lato, Raleway, Work Sans |
| Serif (elegant, editorial) | Playfair Display, Merriweather, Lora, Cormorant Garamond |
| Script / handwriting | Dancing Script, Pacifico, Caveat |
| Mono / tech | JetBrains Mono, Fira Code, Roboto Mono, Space Mono |

`fonts.json` is the machine-readable registry (family, file, category, license,
source URL). Several families use Google's variable-font TTFs (`[wght]` etc.);
the registry records the actual downloaded filename.

## Use in KapCut

Titles and captions reference fonts by family or by absolute path. Register a
font in a Concat project with the `font_add` MCP tool (Concat's `addFont`
command), then set a title's font with `title_set_font`. Motion presets with
`text: true` (Typewriter, Kinetic Words) can be combined with any of these
fonts.

To refresh the library, run `python3 kapcut/assets/download_fonts.py`.
