//! Status bar component at the bottom of the application.

use gpui::{px, rgb};
use ui::prelude::*;
use ui::{Label, div, h_flex};

/// Status bar showing document info and cursor position.
pub struct StatusBar {
    text: String,
}

impl StatusBar {
    pub fn new(text: String) -> Self {
        Self { text }
    }

    pub fn build(&self) -> impl IntoElement {
        h_flex()
            .id("status-bar")
            .w_full()
            .h(px(24.0))
            .bg(rgb(0x007acc))
            .px(px(12.0))
            .gap(px(12.0))
            .items_center()
            .child(Label::new("Ready").size(ui::LabelSize::XSmall).color(ui::Color::Default))
            .child(div().flex_1())
            .child(Label::new(self.text.clone()).size(ui::LabelSize::XSmall).color(ui::Color::Default))
            .child(Label::new("UTF-8").size(ui::LabelSize::XSmall).color(ui::Color::Default))
    }
}
