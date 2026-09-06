//! Rich text representation.

use crate::document::{Paragraph, TextRun};
use serde::{Deserialize, Serialize};

/// A rich text buffer supporting styled text runs.
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
}

impl Default for RichText {
    fn default() -> Self {
        Self::new()
    }
}
