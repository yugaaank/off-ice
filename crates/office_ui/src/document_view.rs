use gpui::{Entity, FocusHandle, Window, Context, KeyDownEvent, px, rgb, FontWeight};
use ui::prelude::*;
use ui::{div, v_flex, h_flex, Label};
use office_core::document::Document;

pub struct DocumentView {
    document: Entity<Document>,
    cursor: usize, // char offset in flat text (including '\n')
    anchor: Option<usize>,
    pending_bold: bool,
    pending_italic: bool,
    pending_underline: bool,
    pub focus_handle: FocusHandle,
}

impl DocumentView {
    pub fn new(document: Entity<Document>, cx: &mut Context<Self>) -> Self {
        let flat_len;
        let needs_init;
        {
            let body = &document.read(cx).body;
            let text = body.to_text();
            flat_len = text.chars().count();
            needs_init = text.trim().is_empty();
        }
        if needs_init {
            document.update(cx, |doc, _| {
                doc.body.set_text("Welcome to Off-Ice — Word\n\nStart typing here. This is a fully editable text area.\n\nSelect text and press B / I / U in the toolbar to format.\n• Bold  • Italic  • Underline\n• Enter for new paragraph  • Shift+Arrow to select");
            });
        }
        let len = document.read(cx).body.len_chars();
        // cursor at end
        let style = document.read(cx).body.style_at(len.saturating_sub(1));
        Self {
            document,
            cursor: len,
            anchor: None,
            pending_bold: style.0,
            pending_italic: style.1,
            pending_underline: style.2,
            focus_handle: cx.focus_handle(),
        }
    }

    fn len_chars(&self, cx: &App) -> usize { self.document.read(cx).body.len_chars() }

    fn selection_range(&self) -> Option<(usize, usize)> {
        self.anchor.map(|a| {
            let s = a.min(self.cursor);
            let e = a.max(self.cursor);
            (s, e)
        }).filter(|(s,e)| s != e)
    }

    fn has_selection(&self) -> bool { self.selection_range().is_some() }

    fn cursor_line_col(&self, cx: &App) -> (usize, usize) {
        let text = self.document.read(cx).body.to_text();
        let chars: Vec<char> = text.chars().collect();
        let mut line = 0usize;
        let mut col = 0usize;
        for i in 0..self.cursor.min(chars.len()) {
            if chars[i] == '\n' { line += 1; col = 0; } else { col += 1; }
        }
        (line, col)
    }

    fn sync_pending(&mut self, cx: &App) {
        if self.anchor.is_none() {
            let (b,i,u) = self.document.read(cx).body.style_at(self.cursor);
            // if cursor at 0 and doc empty, style is default; else use style at cursor-1 for typing continuity
            let len = self.document.read(cx).body.len_chars();
            if self.cursor > 0 && self.cursor == len {
                let (pb, pi, pu) = self.document.read(cx).body.style_at(self.cursor - 1);
                self.pending_bold = pb;
                self.pending_italic = pi;
                self.pending_underline = pu;
            } else if self.cursor < len {
                self.pending_bold = b;
                self.pending_italic = i;
                self.pending_underline = u;
            }
        }
    }

    pub fn toggle_bold(&mut self, cx: &mut Context<Self>) {
        if let Some((s,e)) = self.selection_range() {
            self.document.update(cx, |doc,_| doc.body.toggle_bold_range(s,e));
            self.anchor = None;
        } else {
            self.pending_bold = !self.pending_bold;
        }
        cx.notify();
    }
    pub fn toggle_italic(&mut self, cx: &mut Context<Self>) {
        if let Some((s,e)) = self.selection_range() {
            self.document.update(cx, |doc,_| doc.body.toggle_italic_range(s,e));
            self.anchor = None;
        } else {
            self.pending_italic = !self.pending_italic;
        }
        cx.notify();
    }
    pub fn toggle_underline(&mut self, cx: &mut Context<Self>) {
        if let Some((s,e)) = self.selection_range() {
            self.document.update(cx, |doc,_| doc.body.toggle_underline_range(s,e));
            self.anchor = None;
        } else {
            self.pending_underline = !self.pending_underline;
        }
        cx.notify();
    }

    fn handle_key(&mut self, event: &KeyDownEvent, _window: &mut Window, cx: &mut Context<Self>) {
        let key = event.keystroke.key.as_str();
        let has_shift = event.keystroke.modifiers.shift;
        let has_ctrl = event.keystroke.modifiers.control;
        let has_alt = event.keystroke.modifiers.alt;
        if has_ctrl && !has_alt {
            match key.to_ascii_lowercase().as_str() {
                "b" => { self.toggle_bold(cx); return; }
                "i" => { self.toggle_italic(cx); return; }
                "u" => { self.toggle_underline(cx); return; }
                _ => return,
            }
        }
        if has_alt {
            return;
        }
        let len = self.document.read(cx).body.len_chars();
        let mut moved = false;

        match key {
            "backspace" => {
                if let Some((s,e)) = self.selection_range() {
                    self.document.update(cx, |doc,_| doc.body.delete_range(s,e));
                    self.cursor = s;
                    self.anchor = None;
                } else if self.cursor > 0 {
                    // delete char before cursor (handle unicode char count 1)
                    self.document.update(cx, |doc,_| doc.body.delete_range(self.cursor-1, self.cursor));
                    self.cursor -= 1;
                }
            }
            "delete" => {
                if let Some((s,e)) = self.selection_range() {
                    self.document.update(cx, |doc,_| doc.body.delete_range(s,e));
                    self.cursor = s;
                    self.anchor = None;
                } else if self.cursor < len {
                    self.document.update(cx, |doc,_| doc.body.delete_range(self.cursor, self.cursor+1));
                }
            }
            "enter" => {
                if let Some((s,e)) = self.selection_range() {
                    self.document.update(cx, |doc,_| doc.body.delete_range(s,e));
                    self.cursor = s;
                    self.anchor = None;
                }
                self.document.update(cx, |doc,_| doc.body.insert_at(self.cursor, "\n", false, false));
                self.cursor += 1;
                self.sync_pending(cx);
            }
            "arrowleft" | "left" => {
                if has_shift {
                    if self.anchor.is_none() { self.anchor = Some(self.cursor); }
                    if self.cursor > 0 { self.cursor -= 1; }
                } else {
                    if self.has_selection() { 
                        let (s,_) = self.selection_range().unwrap();
                        self.cursor = s;
                        self.anchor = None;
                    } else if self.cursor > 0 { self.cursor -= 1; self.anchor = None; }
                    else { self.anchor = None; }
                    self.sync_pending(cx);
                }
                moved = true;
            }
            "arrowright" | "right" => {
                if has_shift {
                    if self.anchor.is_none() { self.anchor = Some(self.cursor); }
                    if self.cursor < len { self.cursor += 1; }
                } else {
                    if self.has_selection() {
                        let (_,e) = self.selection_range().unwrap();
                        self.cursor = e;
                        self.anchor = None;
                    } else if self.cursor < len { self.cursor += 1; self.anchor=None; }
                    else { self.anchor=None; }
                    self.sync_pending(cx);
                }
                moved = true;
            }
            "arrowup" | "up" => {
                // compute line/col then move to prev line same col
                let text = self.document.read(cx).body.to_text();
                let chars: Vec<char> = text.chars().collect();
                let (line, col) = self.cursor_line_col(cx);
                if line > 0 {
                    // find start of previous line
                    let lines: Vec<Vec<char>> = {
                        let mut ls = Vec::new();
                        let mut cur = Vec::new();
                        for &c in &chars { if c=='\n' { ls.push(cur); cur=Vec::new(); } else { cur.push(c); } }
                        ls.push(cur); ls
                    };
                    let target_col = col.min(lines[line-1].len());
                    let offset_before: usize = lines[..line-1].iter().map(|l| l.len()+1).sum();
                    let new_cursor = offset_before + target_col;
                    if has_shift {
                        if self.anchor.is_none() { self.anchor = Some(self.cursor); }
                    } else { self.anchor=None; self.sync_pending(cx); }
                    self.cursor = new_cursor;
                } else if !has_shift { self.anchor=None; }
                moved = true;
            }
            "arrowdown" | "down" => {
                let text = self.document.read(cx).body.to_text();
                let chars: Vec<char> = text.chars().collect();
                let (line, col) = self.cursor_line_col(cx);
                let lines: Vec<Vec<char>> = {
                    let mut ls = Vec::new();
                    let mut cur = Vec::new();
                    for &c in &chars { if c=='\n' { ls.push(cur); cur=Vec::new(); } else { cur.push(c); } }
                    ls.push(cur); ls
                };
                if line +1 < lines.len() {
                    let target_col = col.min(lines[line+1].len());
                    let offset_before: usize = lines[..=line].iter().map(|l| l.len()+1).sum();
                    let new_cursor = offset_before + target_col;
                    if has_shift {
                        if self.anchor.is_none() { self.anchor = Some(self.cursor); }
                    } else { self.anchor=None; self.sync_pending(cx); }
                    self.cursor = new_cursor;
                } else if !has_shift { self.anchor=None; }
                moved = true;
            }
            "home" => {
                let (line, _) = self.cursor_line_col(cx);
                let text = self.document.read(cx).body.to_text();
                let chars: Vec<char> = text.chars().collect();
                let mut offset = 0;
                let mut cur_line = 0;
                for i in 0..chars.len() {
                    if cur_line == line { break; }
                    if chars[i]=='\n' { cur_line+=1; offset = i+1; }
                }
                if has_shift { if self.anchor.is_none() { self.anchor=Some(self.cursor); } } else { self.anchor=None; }
                self.cursor = offset;
                if !has_shift { self.sync_pending(cx); }
                moved=true;
            }
            "end" => {
                let (line, _) = self.cursor_line_col(cx);
                let text = self.document.read(cx).body.to_text();
                let chars: Vec<char> = text.chars().collect();
                let mut offset = 0;
                let mut cur_line=0;
                let mut line_start=0;
                for i in 0..chars.len() {
                    if cur_line==line {
                        if chars[i]=='\n' { offset=i; break; } else { offset=i+1; }
                    }
                    if chars[i]=='\n' { cur_line+=1; line_start=i+1; if cur_line>line { offset=line_start-1; break; } }
                }
                if line == chars.iter().filter(|&&c| c=='\n').count() { // last line
                    offset = chars.len();
                }
                if has_shift { if self.anchor.is_none() { self.anchor=Some(self.cursor); } } else { self.anchor=None; }
                self.cursor = offset.min(len);
                if !has_shift { self.sync_pending(cx); }
                moved=true;
            }
            _ => {
                if let Some(ch) = &event.keystroke.key_char {
                    if ch.chars().all(|c| !c.is_control() || c=='\n' || c=='\t') {
                        let insert = if ch=="\t" { "    " } else { ch.as_str() };
                        // delete selection first
                        if let Some((s,e)) = self.selection_range() {
                            self.document.update(cx, |doc,_| doc.body.delete_range(s,e));
                            self.cursor = s;
                            self.anchor=None;
                        }
                        let b = self.pending_bold;
                        let it = self.pending_italic;
                        // underline pending
                        let u = self.pending_underline;
                        // insert_at handles char offset; for multi-char insert, loop
                        self.document.update(cx, |doc,_| {
                            doc.body.insert_at(self.cursor, insert, b, it);
                            // if underline pending, toggle it for inserted range
                            if u {
                                let end = self.cursor + insert.chars().count();
                                doc.body.toggle_underline_range(self.cursor, end);
                            }
                        });
                        self.cursor += insert.chars().count();
                    }
                }
            }
        }
        if moved {
            // clamp
            let nlen = self.document.read(cx).body.len_chars();
            self.cursor = self.cursor.min(nlen);
        }
        cx.notify();
    }

    pub fn status_text(&self, cx: &App) -> String {
        let (line, col) = self.cursor_line_col(cx);
        let len = self.document.read(cx).body.len_chars();
        let words = self.document.read(cx).body.to_text().split_whitespace().count();
        let sel = if let Some((s,e)) = self.selection_range() { format!("  Sel {}  ", e-s) } else { "".to_string() };
        let style = {
            let mut s = Vec::new();
            if self.pending_bold { s.push("B"); }
            if self.pending_italic { s.push("I"); }
            if self.pending_underline { s.push("U"); }
            if s.is_empty() { "Normal".to_string() } else { s.join("+") }
        };
        format!("Ln {}, Col {}  •  {} chars  •  {} words{} • {}", line+1, col+1, len, words, sel, style)
    }

    pub fn is_bold_active(&self, cx: &App) -> bool {
        if let Some((s,e)) = self.selection_range() {
            // active if any char in selection is bold? Use first
            let (_chars, styles) = {
                let body = &self.document.read(cx).body;
                let flat = body.to_text().chars().collect::<Vec<_>>();
                let (_, st) = {
                    // private, so peek via style_at range
                    let mut v = Vec::new();
                    for i in s..e.min(flat.len()) { if flat[i]!='\n' { v.push(body.style_at(i).0); } }
                    (flat, v)
                };
                (s, st)
            };
            !styles.is_empty() && styles.iter().all(|&b| b)
        } else {
            self.pending_bold
        }
    }
    pub fn is_italic_active(&self, cx: &App) -> bool {
        if let Some((s,e)) = self.selection_range() {
            let body = &self.document.read(cx).body;
            let flat = body.to_text().chars().collect::<Vec<_>>();
            let mut v = Vec::new();
            for i in s..e.min(flat.len()) { if i<flat.len() && flat[i]!='\n' { v.push(body.style_at(i).1); } }
            !v.is_empty() && v.iter().all(|&b| b)
        } else { self.pending_italic }
    }
    pub fn is_underline_active(&self, cx: &App) -> bool {
        if let Some((s,e)) = self.selection_range() {
            let body = &self.document.read(cx).body;
            let flat = body.to_text().chars().collect::<Vec<_>>();
            let mut v = Vec::new();
            for i in s..e.min(flat.len()) { if i<flat.len() && flat[i]!='\n' { v.push(body.style_at(i).2); } }
            !v.is_empty() && v.iter().all(|&b| b)
        } else { self.pending_underline }
    }
}

impl Render for DocumentView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let body = self.document.read(cx).body.clone();
        let paragraphs = body.paragraphs().to_vec();
        let is_focused = self.focus_handle.is_focused(window);
        let sel = self.selection_range();
        // compute flat offsets for each para run to handle selection/cursor
        let mut para_start: usize = 0;
        let mut para_rows: Vec<gpui::AnyElement> = Vec::new();

        for (pi, para) in paragraphs.iter().enumerate() {
            let para_len_chars: usize = para.runs.iter().map(|r| r.text.chars().count()).sum();
            // Build inline h_flex for this paragraph
            let mut inline_children: Vec<gpui::AnyElement> = Vec::new();
            let mut run_start = para_start;
            if para.runs.is_empty() {
                // empty paragraph — still need to show cursor if at para_start
                let cursor_here = self.cursor == para_start && sel.is_none();
                if cursor_here {
                    inline_children.push(
                        div().w(px(2.)).h(px(18.)).bg(if is_focused { rgb(0x4fc1ff)} else { rgb(0x858585)}).into_any_element()
                    );
                } else if para_start == 0 && self.cursor==0 {
                    inline_children.push(div().w(px(2.)).h(px(18.)).bg(rgb(0x4fc1ff)).into_any_element());
                }
                // placeholder for empty line height
                if inline_children.is_empty() {
                    inline_children.push(div().h(px(18.)).into_any_element());
                }
            } else {
                for run in &para.runs {
                    let run_chars: Vec<char> = run.text.chars().collect();
                    let run_len = run_chars.len();
                    let run_end = run_start + run_len;
                    // Determine selection overlap for this run
                    let (sel_start, sel_end) = if let Some((s,e)) = sel {
                        let ss = s.max(run_start).saturating_sub(run_start);
                        let ee = e.min(run_end).saturating_sub(run_start);
                        if ss < ee { (ss, ee) } else { (0,0) }
                    } else { (0,0) };
                    let has_sel = sel_start < sel_end;
                    let cursor_in_run = self.cursor >= run_start && self.cursor <= run_end && sel.is_none();
                    let cursor_local = if cursor_in_run { self.cursor - run_start } else { usize::MAX };

                    // Split run into up to 3 pieces: before sel, sel, after sel, with cursor injected
                    // Build pieces boundaries
                    let mut bounds = vec![0, run_len];
                    if has_sel { bounds.push(sel_start); bounds.push(sel_end); }
                    if cursor_in_run { bounds.push(cursor_local); }
                    bounds.sort_unstable(); bounds.dedup();
                    // Now iterate intervals [bounds[i], bounds[i+1])
                    for w in bounds.windows(2) {
                        let a = w[0]; let b = w[1];
                        if a >= b { continue; }
                        // cursor bar before this segment if cursor == a and cursor_in_run
                        if cursor_in_run && a == cursor_local {
                            inline_children.push(
                                div().w(px(2.)).h(px(18.)).bg(if is_focused { rgb(0x4fc1ff)} else { rgb(0x858585)}).into_any_element()
                            );
                        }
                        let segment: String = run_chars[a..b].iter().collect();
                        let is_selected = has_sel && a >= sel_start && b <= sel_end;
                        let mut label = Label::new(segment).size(LabelSize::Small);
                        if run.bold { label = label.weight(FontWeight::BOLD); }
                        if run.italic { label = label.italic(); }
                        if run.underline { label = label.underline(); }
                        if is_selected {
                            inline_children.push(
                                div().bg(rgb(0x264f78)).px(px(1.)).child(label.color(Color::Default)).into_any_element()
                            );
                        } else {
                            inline_children.push(label.color(Color::Default).into_any_element());
                        }
                    }
                    // cursor at end of run
                    if cursor_in_run && cursor_local == run_len {
                        inline_children.push(
                            div().w(px(2.)).h(px(18.)).bg(if is_focused { rgb(0x4fc1ff)} else { rgb(0x858585)}).into_any_element()
                        );
                    }
                    run_start = run_end;
                }
            }
            // cursor at para boundary (between paras) — when run loop didn't handle because at newline pos
            // newline char is at offset para_start + para_len
            let newline_pos = para_start + para_len_chars;
            if sel.is_none() && self.cursor == newline_pos && pi +1 < paragraphs.len() {
                // cursor at end of this paragraph (before newline) already handled as end-of-last-run; newline case is when cursor is exactly at newline char (pos of \n)
                // Actually newline itself is not in runs, so cursor at newline_pos is same as end-of-para; handled.
            }

            let line_no = pi + 1;
            let row = h_flex()
                .id(("line", pi))
                .w_full()
                .min_h(px(22.))
                .child(
                    div().w(px(48.)).flex_shrink_0().pr(px(12.)).text_align(gpui::TextAlign::Right)
                        .child(Label::new(format!("{}", line_no)).size(LabelSize::XSmall).color(Color::Muted))
                )
                .child(
                    h_flex().flex_1().flex_wrap().gap(px(0.)).children(inline_children)
                )
                .into_any_element();
            para_rows.push(row);
            para_start += para_len_chars + 1; // +1 for '\n'
        }

        // toolbar state handled in OfficeApp; kept for future inline controls
        // for status bar we already have pending, but row needed? just for page
        v_flex()
            .id("doc-view")
            .flex_1()
            .w_full()
            .h_full()
            .bg(rgb(0x1e1e1e))
            .items_center()
            .overflow_y_scroll()
            .child(
                div()
                    .id("page")
                    .w(px(800.)).max_w_full().min_h(px(900.)).my(px(24.)).mx(px(24.))
                    .bg(rgb(0x252526)).rounded_md().shadow_lg().border_1().border_color(rgb(0x3a3a3a)).p(px(48.))
                    .track_focus(&self.focus_handle)
                    .on_mouse_down(gpui::MouseButton::Left, cx.listener(|this, _ev, window, cx| { window.focus(&this.focus_handle, cx); }))
                    .on_key_down(cx.listener(|this, ev, window, cx| this.handle_key(ev, window, cx)))
                    .child(v_flex().w_full().gap(px(1.)).children(para_rows))
            )
    }
}
