//! Presentation model and slide operations.

use crate::{DocumentId, SlideElement};
use serde::{Deserialize, Serialize};

/// A presentation document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Presentation {
    pub id: DocumentId,
    pub title: String,
    pub slides: Vec<Slide>,
    pub slide_width: u32,
    pub slide_height: u32,
}

impl Presentation {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            id: DocumentId::new_v4(),
            title: title.into(),
            slides: vec![],
            slide_width: 960,
            slide_height: 540,
        }
    }

    pub fn add_slide(&mut self, slide: Slide) {
        self.slides.push(slide);
    }

    pub fn slide(&self, index: usize) -> Option<&Slide> {
        self.slides.get(index)
    }

    pub fn slide_mut(&mut self, index: usize) -> Option<&mut Slide> {
        self.slides.get_mut(index)
    }

    pub fn len(&self) -> usize {
        self.slides.len()
    }

    pub fn is_empty(&self) -> bool {
        self.slides.is_empty()
    }
}

/// A single slide in a presentation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Slide {
    pub title: String,
    pub elements: Vec<SlideElement>,
}

impl Slide {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            elements: vec![],
        }
    }

    pub fn add_element(&mut self, element: SlideElement) {
        self.elements.push(element);
    }
}
