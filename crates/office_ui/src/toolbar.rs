//! Application toolbar component.

use gpui::{px, rgb};
use ui::prelude::*;
use ui::{h_flex, Button, IconButton, IconName, div, Label};

/// The main application toolbar.
pub struct Toolbar;

impl Toolbar {
    pub fn new() -> Self {
        Self
    }

    pub fn build(&self) -> impl IntoElement {
        h_flex()
            .id("toolbar")
            .w_full()
            .h(px(44.0))
            .bg(rgb(0x252526))
            .border_b_1()
            .border_color(rgb(0x3a3a3a))
            .px(px(12.0))
            .gap(px(8.0))
            .items_center()
            .child(
                h_flex().gap(px(6.0))
                    .child(Button::new("new", "New").style(ui::ButtonStyle::Filled))
                    .child(Button::new("open", "Open"))
                    .child(Button::new("save", "Save"))
            )
            .child(div().w(px(1.0)).h(px(20.0)).bg(rgb(0x3a3a3a)).mx(px(4.0)))
            .child(
                h_flex().gap(px(4.0))
                    .child(Button::new("bold", "B").style(ui::ButtonStyle::Outlined))
                    .child(Button::new("italic", "I").style(ui::ButtonStyle::Outlined))
                    .child(Button::new("underline", "U").style(ui::ButtonStyle::Outlined))
            )
            .child(div().w(px(1.0)).h(px(20.0)).bg(rgb(0x3a3a3a)).mx(px(4.0)))
            .child(
                h_flex().gap(px(2.0))
                    .child(IconButton::new("undo", IconName::Undo))
                    .child(IconButton::new("redo", IconName::ArrowRight))
            )
            .child(div().flex_1())
            .child(Label::new("100%").size(ui::LabelSize::Small).color(ui::Color::Muted))
    }
}
