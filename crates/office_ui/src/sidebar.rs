use gpui::{px, rgb};
use ui::prelude::*;
use ui::{Label, div, v_flex, h_flex};

pub struct Sidebar;
impl Sidebar {
    pub fn new() -> Self { Self }
    pub fn build(&self) -> impl IntoElement {
        v_flex()
            .id("sidebar")
            .w(px(220.0))
            .h_full()
            .flex_shrink_0()
            .bg(rgb(0x252526))
            .border_r_1()
            .border_color(rgb(0x3a3a3a))
            .p(px(12.0))
            .gap(px(16.0))
            .child(
                v_flex()
                    .gap(px(6.0))
                    .child(Label::new("OFF-ICE").size(ui::LabelSize::XSmall).color(ui::Color::Muted))
                    .child(Label::new("Documents").size(ui::LabelSize::Small).color(ui::Color::Default))
            )
            .child(
                v_flex()
                    .gap(px(2.0))
                    .child(
                        h_flex()
                            .w_full()
                            .px(px(8.0))
                            .py(px(6.0))
                            .rounded_md()
                            .bg(rgb(0x2d2d30))
                            .border_1()
                            .border_color(rgb(0x3a3a3a))
                            .child(Label::new("Untitled").size(ui::LabelSize::Small))
                    )
                    .child(
                        h_flex()
                            .w_full()
                            .px(px(8.0))
                            .py(px(6.0))
                            .rounded_md()
                            .child(Label::new("Getting Started").size(ui::LabelSize::Small).color(ui::Color::Muted))
                    )
            )
            .child(div().flex_1())
            .child(
                div()
                    .text_size(px(10.0))
                    .child(Label::new("Click page to focus • Type to edit").size(ui::LabelSize::XSmall).color(ui::Color::Muted))
            )
    }
}
impl gpui::Render for Sidebar {
    fn render(&mut self, _w: &mut gpui::Window, _cx: &mut gpui::Context<Self>) -> impl IntoElement { self.build() }
}
