//! Text document model and rich text operations.

use crate::{DocumentId, DocumentStyles};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A text document with rich text content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: DocumentId,
    pub title: String,
    pub body: RichText,
    pub styles: DocumentStyles,
}

impl Document {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            title: title.into(),
            body: RichText::new(),
            styles: DocumentStyles::default(),
        }
    }

    pub fn insert_text(&mut self, text: impl Into<String>) {
        self.body.push_str(&text.into());
    }
}

/// A paragraph in a rich text document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paragraph {
    pub runs: Vec<TextRun>,
}

/// A single styled text run within a paragraph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextRun {
    pub text: String,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub font_size: u16,
    pub color: Option<String>,
}

/// Operations for manipulating rich text.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichText {
    paragraphs: Vec<Paragraph>,
}

impl RichText {
    pub fn new() -> Self {
        Self {
            paragraphs: vec![Paragraph { runs: vec![] }],
        }
    }

    pub fn push_str(&mut self, text: &str) {
        if let Some(last) = self.paragraphs.last_mut() {
            if let Some(run) = last.runs.last_mut() {
                run.text.push_str(text);
            } else {
                last.runs.push(TextRun {
                    text: text.to_string(),
                    bold: false,
                    italic: false,
                    underline: false,
                    font_size: 12,
                    color: None,
                });
            }
        }
    }

    pub fn paragraphs(&self) -> &[Paragraph] {
        &self.paragraphs
    }

    pub fn add_paragraph(&mut self) {
        self.paragraphs.push(Paragraph { runs: vec![] });
    }

    pub fn set_text(&mut self, text: &str) {
        self.paragraphs.clear();
        for line in text.split('\n') {
            let mut p = Paragraph { runs: vec![] };
            if !line.is_empty() {
                p.runs.push(TextRun {
                    text: line.to_string(),
                    bold: false,
                    italic: false,
                    underline: false,
                    font_size: 12,
                    color: None,
                });
            }
            self.paragraphs.push(p);
        }
        if self.paragraphs.is_empty() {
            self.paragraphs.push(Paragraph { runs: vec![] });
        }
    }

    pub fn to_text(&self) -> String {
        self.paragraphs
            .iter()
            .map(|p| {
                p.runs.iter().map(|r| r.text.as_str()).collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    // --- Rich text editing helpers (char-index based, '\n' = 1 char) ---

    pub fn len_chars(&self) -> usize {
        self.to_text().chars().count()
    }

    fn styled_flat(&self) -> (Vec<char>, Vec<(bool, bool, bool)>) {
        let flat: Vec<char> = self.to_text().chars().collect();
        let mut styles: Vec<(bool, bool, bool)> = Vec::with_capacity(flat.len());
        let mut flat_idx = 0usize;
        for (pi, para) in self.paragraphs.iter().enumerate() {
            for run in &para.runs {
                for _ in run.text.chars() {
                    styles.push((run.bold, run.italic, run.underline));
                    flat_idx += 1;
                }
            }
            if pi + 1 < self.paragraphs.len() {
                styles.push((false, false, false));
                flat_idx += 1;
            }
        }
        while styles.len() < flat.len() {
            styles.push((false, false, false));
        }
        (flat, styles)
    }

    fn from_styled_flat(chars: &[char], styles: &[(bool, bool, bool)]) -> Self {
        let mut paragraphs: Vec<Paragraph> = Vec::new();
        let mut cur_para = Paragraph { runs: vec![] };
        let mut cur_run: Option<TextRun> = None;
        let mut cur_bold = false;
        let mut cur_italic = false;
        let mut cur_underline = false;
        let mut has_run = false;

        for (i, &ch) in chars.iter().enumerate() {
            if ch == '\n' {
                if let Some(run) = cur_run.take() {
                    cur_para.runs.push(run);
                    has_run = false;
                }
                paragraphs.push(cur_para);
                cur_para = Paragraph { runs: vec![] };
            } else {
                let (b, it, u) = styles[i];
                if has_run && cur_bold == b && cur_italic == it && cur_underline == u {
                    cur_run.as_mut().unwrap().text.push(ch);
                } else {
                    if let Some(run) = cur_run.take() {
                        cur_para.runs.push(run);
                    }
                    cur_run = Some(TextRun {
                        text: ch.to_string(),
                        bold: b,
                        italic: it,
                        underline: u,
                        font_size: 12,
                        color: None,
                    });
                    cur_bold = b;
                    cur_italic = it;
                    cur_underline = u;
                    has_run = true;
                }
            }
        }
        if let Some(run) = cur_run {
            cur_para.runs.push(run);
        }
        paragraphs.push(cur_para);
        if paragraphs.is_empty() {
            paragraphs.push(Paragraph { runs: vec![] });
        }
        Self { paragraphs }
    }

    pub fn style_at(&self, char_offset: usize) -> (bool, bool, bool) {
        let (chars, styles) = self.styled_flat();
        if chars.is_empty() {
            return (false, false, false);
        }
        let idx = char_offset.min(chars.len().saturating_sub(1));
        if chars[idx] == '\n' && idx > 0 {
            return styles[idx - 1];
        }
        styles[idx]
    }

    pub fn insert_at(&mut self, char_offset: usize, text: &str, bold: bool, italic: bool) {
        let (mut chars, mut styles) = self.styled_flat();
        let insert_chars: Vec<char> = text.chars().collect();
        let insert_styles: Vec<(bool, bool, bool)> = vec![(bold, italic, false); insert_chars.len()];
        let off = char_offset.min(chars.len());
        // Adjust newlines: inserted text may contain '\n' already, keep dummy style for them
        chars.splice(off..off, insert_chars.clone());
        styles.splice(off..off, insert_styles);
        // For any inserted '\n', ensure style dummy (already false,false) — keep as is
        *self = Self::from_styled_flat(&chars, &styles);
    }

    pub fn delete_range(&mut self, start: usize, end: usize) {
        if start >= end {
            return;
        }
        let (mut chars, mut styles) = self.styled_flat();
        let len = chars.len();
        let s = start.min(len);
        let e = end.min(len);
        if s >= e {
            return;
        }
        chars.drain(s..e);
        styles.drain(s..e);
        if chars.is_empty() {
            *self = Self::new();
            return;
        }
        *self = Self::from_styled_flat(&chars, &styles);
    }

    pub fn toggle_bold_range(&mut self, start: usize, end: usize) {
        if start >= end {
            return;
        }
        let (chars, mut styles) = self.styled_flat();
        let len = chars.len();
        let s = start.min(len);
        let e = end.min(len);
        for i in s..e {
            if chars[i] == '\n' {
                continue;
            }
            styles[i].0 = !styles[i].0;
        }
        *self = Self::from_styled_flat(&chars, &styles);
    }

    pub fn toggle_underline_range(&mut self, start: usize, end: usize) {
        if start >= end {
            return;
        }
        let (chars, mut styles) = self.styled_flat();
        let len = chars.len();
        let s = start.min(len);
        let e = end.min(len);
        for i in s..e {
            if chars[i] == '\n' { continue; }
            styles[i].2 = !styles[i].2;
        }
        *self = Self::from_styled_flat(&chars, &styles);
    }

    pub fn toggle_italic_range(&mut self, start: usize, end: usize) {
        if start >= end {
            return;
        }
        let (chars, mut styles) = self.styled_flat();
        let len = chars.len();
        let s = start.min(len);
        let e = end.min(len);
        for i in s..e {
            if chars[i] == '\n' {
                continue;
            }
            styles[i].1 = !styles[i].1;
        }
        *self = Self::from_styled_flat(&chars, &styles);
    }
}

impl Default for RichText {
    fn default() -> Self {
        Self::new()
    }
}
