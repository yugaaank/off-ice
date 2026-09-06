//! Presentation slide view component.

use gpui::{Entity, Window, Context};
use ui::prelude::*;
use ui::{Label, div, v_flex};
use office_core::presentation::Presentation;

/// A view for editing a presentation.
pub struct PresentationView {
    presentation: Entity<Presentation>,
    current_slide: usize,
}

impl PresentationView {
    pub fn new(presentation: Entity<Presentation>, _cx: &mut Window) -> Self {
        Self {
            presentation,
            current_slide: 0,
        }
    }
}

impl Render for PresentationView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex().flex_1().child(
            div().id("presentation-view").child(
                Label::new(&format!("Slide {}", self.current_slide + 1)).size(ui::LabelSize::Large),
            ),
        )
    }
}
