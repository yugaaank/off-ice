//! Document styling and theme integration.

use gpui::Hsla;

/// Shared styles across document types.
#[derive(Debug, Default, Clone)]
pub struct DocumentStyles {
    pub font_family: String,
    pub font_size: u16,
    pub line_height: f32,
}

/// Text style for rich text runs.
#[derive(Debug, Default, Clone)]
pub struct TextStyle {
    pub font_family: String,
    pub font_size: u16,
    pub bold: bool,
    pub italic: bool,
    pub color: Hsla,
    pub background: Option<Hsla>,
}

/// Alignment options.
#[derive(Debug, Default, Clone, Copy)]
pub enum Alignment {
    #[default]
    Left,
    Center,
    Right,
    Justify,
}
