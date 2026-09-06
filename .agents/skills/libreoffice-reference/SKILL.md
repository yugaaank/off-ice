---
name: libreoffice-reference
description: Consult LibreOffice as the canonical reference for how an office suite works — Writer layout/pagination/styles, Calc grid/formulas, Impress slides, and shared chrome (toolbars, sidebars,一 status). Use whenever designing, implementing, or debugging any Off-Ice feature that exists in LibreOffice.
---

# LibreOffice Reference

Use this skill whenever you need to decide *how* an office suite should behave or *what* to build next in Off-Ice. LibreOffice is the ground truth — check it before inventing.

## When to activate

- Designing or implementing Writer / Calc / Impress behavior, file formats, or UI
- Choosing toolbar/sidebar/panel layout, shortcuts, or interaction
- Debugging fidelity gaps vs. expected office behavior
- Answering "how does LibreOffice do X?"

## Reference sources (in priority order)

1. **Running LibreOffice (fastest):** `soffice --writer`, `--calc`, `--impress`. Inspect menus, sidebars, status bar, dialogs. Screenshot if needed.
   ```sh
   which soffice; soffice --help | head -n 40
   libreoffice --writer /tmp/test.odt &
   ```
2. **Help & docs:** `https://help.libreoffice.org/` — Writer Guide, Calc Guide, Impress Guide. Prefer over web search.
3. **Local source mirror (deep dives):** clone once, grep as needed:
   ```sh
   # shallow clone (~2GB) — skip if already exists
   test -d /tmp/libreoffice-core || git clone --depth 1 https://gerrit.libreoffice.org/core /tmp/libreoffice-core
   grep -rn "SwPageFrame\|ScTable\|SdPage" /tmp/libreoffice-core/sw /tmp/libreoffice-core/sc /tmp/libreoffice-core/sd --include="*.cxx" --include="*.hxx" | head
   ```
4. **ODF spec / filter code:** `oox/`, `sw/source/filter/`, `sc/source/filter/` for .odt/.ods/.odp and .docx/.xlsx/.pptx interop.

## Workflow

1. **Name the LibreOffice counterpart:** Writer ↔ Off-Ice Document, Calc ↔ Sheet, Impress ↔ Presentation. Map the feature to its LO module:
   - `sw/` = Writer (paragraphs, styles, pagination, headers/footers)
   - `sc/` = Calc (grid, formulas, cell styles, sheets)
   - `sd/` + `sd/source/` = Impress/Draw (slides, masters, transitions)
   - `sfx2/`, `vcl/`, `svx/` = shared UI (toolbars, sidebars, status)

2. **Observe LibreOffice first:**
   - Open LO, reproduce the flow, note menu path (`Format > Page Style`), sidebar deck, and status indicators.
   - Record expected behavior in one sentence: "Writer wraps at page margin and paginates with widow control."

3. **Look up the implementation cue (if building):**
   ```sh
   grep -rn "WidowControl\|PageBreak\|LOK" /tmp/libreoffice-core/sw --include="*.cxx" | head -n 20
   ```
   Use the LO name for your Off-Ice type/method where sensible — keeps mapping obvious.

4. **Translate to Off-Ice (GPUI):**
   - Model in `crates/office_core` (e.g., `document.rs` ≈ `SwDoc`, `sheet.rs` ≈ `ScDocument`, `presentation.rs` ≈ `SdPage`)
   - View in `crates/office_ui` with GPUI `div()/v_flex()/h_flex()` matching LO chrome: top toolbar, left Navigator/Styles sidebar, bottom status bar, central canvas.
   - Verify by side-by-side screenshot with LO.

5. **Cite the source:** In PR/code comment, note `LO ref: sw/source/core/layout/pagechg.cxx:HeaderFooter` or `Help: Writer > Page Style > Borders`.

## Off-Ice mapping

| Off-Ice | LibreOffice | Key refs |
|---|---|---|
| `crates/office_core/src/document.rs` `RichText/Paragraph/TextRun` | `sw` Writer, `SwTextNode` | `sw/source/core/txtnode/` |
| `styles.rs` `DocumentStyles` | `StyleFamilies` | `sw/source/core/doc/styles.cxx` |
| `sheet.rs` `Sheet/Cell` | `sc` Calc `ScDocument/ScTable` | `sc/source/core/data/document.cxx` |
| `presentation.rs` `Slide/SlideElement` | `sd` Impress `SdPage` | `sd/source/core/` |
| `office_ui/toolbar.rs` `sidebar.rs` `status_bar.rs` | `sfx2/sidebar`, `svx/tbcontrl` | `sfx2/source/sidebar/` |

## Rules

- Prefer LO's UX until Off-Ice has a stated reason to diverge — note the divergence.
- Don't invent office concepts that LO already names (e.g., "Page Style" not "Page Theme").
- If LO source is unavailable, state which of the 4 sources you used and what was missing.

