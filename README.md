# Off-Ice — GPUI Office Suite (Zed-based)

Dark, Zed-styled office suite built on `gpui` + `ui` from [zed-industries/zed](https://github.com/zed-industries/zed). Writer / Sheet / Presentation share one `office_core` model and one `office_ui` GPUI chrome.

> LibreOffice is the reference. See `.agents/skills/libreoffice-reference` — Writer `sw`, Calc `sc`, Impress `sd`, `sfx2` chrome.

## Status

- **Writer MVP**: editable paginated page (800×900), dark ` #1e1e1e` canvas / `#252526` page, line-number gutter, blue cursor, `B/I/U` rich text (`TextRun {bold,italic,underline}` → `RichText::Paragraph`), `Shift+Arrow` selection (`bg #264f78`), `Ctrl+B/I/U`, toolbar wired, status `Ln Col • chars words • style`.
- **Sheet / Presentation**: stubs (`crates/office_ui/src/{spreadsheet,presentation}_view.rs`), models in `office_core`.
- **Theme**: dark (titlebar `#252526`, toolbar/sidebar same, status `#007acc`).

## Workspace

```
crates/office       — binary `office` (gpui Platform + ThemeSettingsProvider)
crates/office_core  — Document/RichText/Paragraph/TextRun, Sheet/Cell, Presentation/Slide, styles
crates/office_ui    — document_view (editor), toolbar, sidebar, status_bar, spreadsheet/presentation views
.agents/skills/libreoffice-reference — skill to consult LibreOffice before building
```

## Build

Pinned Zed rev `5a9b955` (main 2026-09-06). Requires Rust, Wayland.

```sh
cargo run --bin office          # opens dark editor
cargo check
cargo build --bin office
```

Dependencies are git-pinned, no local `zed/` checkout required after first fetch. A local `zed/` clone at `./zed` is `.gitignore`d and used only for offline iteration.

## Roadmap

1. **Formatting** (done): run splitting/merging via `RichText::styled_flat` → `insert_at`/`delete_range`/`toggle_*_range`.
2. **Save/Open** `*.off` JSON → ODT/DOCX (`oox`, `sw/source/filter` ref).
3. **Page Style + Ruler** (`Format > Page Style` — `PageDesc` margins/header).
4. **Sheet grid** (`ScDocument` — formula `SUM()`).
5. **Slides** (`SdPage`).

## Skill

`/libreoffice-reference` — consult Writer/Calc/Impress before inventing. See `help.libreoffice.org` or `gerrit.libreoffice.org/core` at `/tmp/libreoffice-core`.

## License

GPL-3.0-or-later (workspace)
