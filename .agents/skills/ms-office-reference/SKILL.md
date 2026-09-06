---
name: ms-office-reference
description: Consult Microsoft Office (Word/Excel/PowerPoint) as the canonical reference for MS OOXML file design and Fluent/Ribbon UI looks — ribbon tabs, groups, galleries, status bar, dark theme, and .docx/.xlsx/.pptx structure. Use whenever designing, implementing, or matching Off-Ice UI or .docx/.xlsx/.pptx fidelity to MS Office.
---

# MS Office Reference — Design & UI

Use this skill whenever you need to decide how Off-Ice should *look* or how its `.docx/.xlsx/.pptx` should be *structured* to match Microsoft Office. MS Office is the visual and file-format ground truth — check it before inventing.

## When to activate

- Designing or matching UI to Word/Excel/PowerPoint: ribbon, tabs, groups, galleries, quick-access toolbar, status bar, rulers, sidebars, dark mode, Fluent icons, typography
- Choosing layout, spacing, colors, shadows, or interaction that should feel like MS Office
- Implementing or debugging `.docx/.xlsx/.pptx` Save/Open fidelity vs MS Word/Excel/PowerPoint
- Answering "how does MS Office do/look like X?"

## Reference sources (priority order)

1. **Running MS Office / M365 (fastest visual):** Word/Excel/PowerPoint desktop or `office.com` web. Inspect Ribbon tabs (`Home`, `Insert`, `Draw`, `Design`, `Layout`, `References`, `Review`, `View`), groups (`Clipboard`, `Font`, `Paragraph`, `Styles`), and status bar. Screenshot and note pixel spacing.
2. **Fluent UI / Fluent 2 Docs (design system):** `https://fluent2.microsoft.design/` and `https://developer.microsoft.com/fluentui` — Ribbon, Command Bar, tokens (color, typography, elevation, motion). Prefer over generic UI kits.
3. **Microsoft Learn — Office Add-ins / Open XML:** `https://learn.microsoft.com/en-us/office/open-xml/` — `w:document`, `w:body/w:p/w:r/w:rPr (w:b/w:i/w:u)`, `xl/worksheets`, `ppt/slides`. Reference for file structure.
4. **Local file samples (deep dive):** create `samples/word/sample.docx` in Word, unzip and inspect:
   ```sh
   unzip -l samples/word/sample.docx | head -n 30
   unzip -p samples/word/sample.docx word/document.xml | xmllint --format - | head -n 120
   # same for xl/worksheets/sheet1.xml and ppt/slides/slide1.xml
   ```
5. **LO `oox` as cross-check:** `sw/source/filter/ww8`, `oox/source/docx` show how LO maps to the same OOXML — use to validate your `w:rPr` mapping.

## Workflow

1. **Name the MS Office counterpart:** Off-Ice Document → Word, Sheet → Excel, Presentation → PowerPoint. Map feature to Ribbon tab/group:
   - Word `Home > Font` → Off-Ice `TextRun {bold,italic,underline}` (`w:rPr`)
   - Word `Home > Paragraph` → alignment, line spacing
   - Word `Insert > Tables/Pictures` → `w:tbl` / `w:drawing`
   - Excel `Home > Number/Styles` → `CellStyle`
   - PowerPoint `Home > Slides` → `Slide/SlideElement`

2. **Observe MS Office first:**
   - Open Word/Excel/PowerPoint, reproduce flow, note Ribbon path (`Home > Font > B I U`), gallery, and status text (`Page 1 of 1  Words: 123  Zoom 100%`).
   - One-sentence expectation: "Word shows `B` pressed with `ButtonStyle::Filled` and `w:b` in `document.xml` when selection is bold."

3. **Capture design tokens (UI) or OOXML snippet (file):**
   ```sh
   # UI: screenshot ribbon, measure 4/8px gaps, note #252526/#1e1e1e dark, #007acc accent, Segoe UI Variable
   # File: save minimal docx with single bold word, unzip and record w:rPr
   ```

4. **Translate to Off-Ice (GPUI + office_core):**
   - **UI in `crates/office_ui`:** GPUI `div()/h_flex()/v_flex()` with Fluent tokens — `44px` ribbon, `8px` gaps, `12px` traffic lights, `rounded_md shadow_lg` page, `Segoe UI`/`Inter` via `Label`. Match Word dark (`#1e1e1e` canvas, `#252526` page/chrome, `#3a3a3a` borders). Use `Label.weight(BOLD).italic().underline()` to mirror `w:rPr`.
   - **File in `crates/office_core`:** `document.rs` `RichText` ↔ `word/document.xml` `w:p/w:r`, `sheet.rs` ↔ `xl/worksheets`, `presentation.rs` ↔ `ppt/slides`. Keep element names close to `w:`/`c:` spec.
   - Verify side-by-side vs Word screenshot and `unzip -p word/document.xml`.

5. **Cite source:** `MS ref: Word Home>Font B -> w:b val=true (document.xml:42)` or `Fluent 2: Ribbon 44px, 8px gap`.

## Off-Ice mapping

| Off-Ice | MS Office | Key refs |
|---|---|---|
| `office_ui/toolbar.rs` (ribbon) | Word Ribbon `Home > Font/Paragraph` | Fluent 2 Ribbon, `44px` bar |
| `office_ui/sidebar.rs` | Word Navigation/Styles pane | Fluent Panel |
| `office_ui/status_bar.rs` | Word status `Page/Words/Language/Zoom` | Word status bar |
| `office_ui/document_view.rs` page | Word page `800×900` `shadow_lg` | Word canvas |
| `office_core/document.rs` `RichText` | `word/document.xml` `w:p/w:r/w:rPr` | `learn.microsoft.com/office/open-xml` |
| `office_core/sheet.rs` | `xl/workbook.xml + worksheets/sheet1.xml` | Open XML Spreadsheet |
| `office_core/presentation.rs` | `ppt/presentation.xml + slides/slide1.xml` | Open XML Presentation |

## Rules

- Prefer MS Office visuals until Off-Ice has a stated reason to diverge — note divergence.
- Don't invent ribbon terms LO already names differently ("Page Style" vs Word "Layout > Margins"); use MS names for MS files.
- File work must round-trip in Word/Excel/PowerPoint desktop, not just LO — test open/save there.
- If MS Office app unavailable, state which of the 5 sources you used and what was missing.

