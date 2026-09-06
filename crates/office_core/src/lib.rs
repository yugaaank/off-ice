//! Office Core — document model, serializers, and shared types.
//!
//! This crate defines the core data structures for the office suite:
//! documents, spreadsheets, presentations, and their serialization formats.
//!
//! Built on top of Zed's GPUI framework and component system.

pub mod document;
pub mod sheet;
pub mod presentation;
pub mod rich_text;
pub mod styles;

pub use document::Document;
pub use sheet::Sheet;
pub use presentation::Presentation;

use gpui::Entity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A unique identifier for an office document.
pub type DocumentId = Uuid;

/// The root enum for all office document types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OfficeDocument {
    Text(Document),
    Spreadsheet(Sheet),
    Presentation(Presentation),
}

/// A single cell in a spreadsheet.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cell {
    pub value: CellValue,
    pub formula: Option<String>,
    pub style: CellStyle,
}

impl Cell {
    pub fn empty() -> Self {
        Self {
            value: CellValue::Empty,
            formula: None,
            style: CellStyle::default(),
        }
    }

    pub fn text(text: impl Into<String>) -> Self {
        Self {
            value: CellValue::Text(text.into()),
            formula: None,
            style: CellStyle::default(),
        }
    }

    pub fn number(value: f64) -> Self {
        Self {
            value: CellValue::Number(value),
            formula: None,
            style: CellStyle::default(),
        }
    }
}

/// The type of value a cell can hold.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CellValue {
    Empty,
    Text(String),
    Number(f64),
    Boolean(bool),
    Formula(String),
}

/// Style applied to a cell.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CellStyle {
    pub bold: bool,
    pub italic: bool,
    pub font_size: u16,
    pub background: Option<String>,
}

/// A single slide in a presentation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Slide {
    pub title: String,
    pub elements: Vec<SlideElement>,
}

/// An element on a slide.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SlideElement {
    Text { content: String, position: (f32, f32) },
    Image { path: String, position: (f32, f32) },
    Shape { kind: ShapeKind, position: (f32, f32), size: (f32, f32) },
}

/// A geometric shape kind.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShapeKind {
    Rectangle,
    Circle,
    Triangle,
    Arrow,
}

/// Styles shared across document types.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DocumentStyles {
    pub font_family: String,
    pub font_size: u16,
    pub line_height: f32,
}

/// A handle to an open document view.
pub type DocumentHandle = Entity<DocumentView>;

/// The view state for an open document.
#[derive(Debug)]
pub struct DocumentView {
    pub document: OfficeDocument,
    pub scroll_offset: gpui::Point<f32>,
    pub zoom: f32,
}

impl DocumentView {
    pub fn new(document: OfficeDocument) -> Self {
        Self {
            document,
            scroll_offset: gpui::Point::<f32>::new(0.0, 0.0),
            zoom: 1.0,
        }
    }
}

pub use document::{Paragraph, TextRun};
