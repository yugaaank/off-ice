//! Office UI — GPUI components for the office suite.

pub mod document_view;
pub mod spreadsheet_view;
pub mod presentation_view;
pub mod toolbar;
pub mod sidebar;
pub mod status_bar;
pub mod actions;

use gpui::{App, Window, Entity, WindowOptions, Context, px, rgb};
use ui::prelude::*;
use ui::{div, h_flex, Button, IconButton, IconName, Label};
use crate::document_view::DocumentView;
use office_core::document::Document;

pub struct OfficeApp {
    editor: Option<Entity<DocumentView>>,
}

impl OfficeApp {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self { editor: None }
    }

    fn ensure_editor(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.editor.is_none() {
            let doc = cx.new(|_cx| Document::new("Untitled"));
            let editor = cx.new(|cx| DocumentView::new(doc, cx));
            self.editor = Some(editor.clone());
            let handle = editor.read(cx).focus_handle.clone();
            window.focus(&handle, cx);
        }
    }
}

impl Render for OfficeApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.ensure_editor(window, cx);
        let editor = self.editor.clone().unwrap();
        let status = editor.read(cx).status_text(&*cx);
        let title = "Untitled — Off-Ice";
        let bold_active = editor.read(cx).is_bold_active(&*cx);
        let italic_active = editor.read(cx).is_italic_active(&*cx);
        let underline_active = editor.read(cx).is_underline_active(&*cx);

        gpui::div()
            .id("office-app")
            .size_full()
            .flex()
            .flex_col()
            .bg(rgb(0x1e1e1e))
            .child(
                div()
                    .h(px(32.0))
                    .w_full()
                    .bg(rgb(0x252526))
                    .border_b_1()
                    .border_color(rgb(0x3a3a3a))
                    .flex()
                    .items_center()
                    .px(px(12.0))
                    .gap(px(8.0))
                    .child(div().size(px(12.0)).rounded_full().bg(rgb(0xff5f56)))
                    .child(div().size(px(12.0)).rounded_full().bg(rgb(0xffbd2e)))
                    .child(div().size(px(12.0)).rounded_full().bg(rgb(0x27c93f)))
                    .child(div().flex_1())
                    .child(Label::new(title).size(ui::LabelSize::Small).color(ui::Color::Muted))
                    .child(div().flex_1())
            )
            .child(
                h_flex()
                    .id("toolbar")
                    .w_full()
                    .h(px(44.))
                    .bg(rgb(0x252526))
                    .border_b_1()
                    .border_color(rgb(0x3a3a3a))
                    .px(px(12.))
                    .gap(px(8.))
                    .items_center()
                    .child(
                        h_flex().gap(px(6.))
                            .child(Button::new("new", "New").style(ui::ButtonStyle::Filled))
                            .child(Button::new("open", "Open"))
                            .child(Button::new("save", "Save"))
                    )
                    .child(div().w(px(1.)).h(px(20.)).bg(rgb(0x3a3a3a)).mx(px(4.)))
                    .child(
                        h_flex().gap(px(2.))
                            .child(
                                Button::new("bold", "B")
                                    .style(if bold_active { ui::ButtonStyle::Filled } else { ui::ButtonStyle::Outlined })
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        if let Some(ed) = this.editor.clone() {
                                            ed.update(cx, |ed, cx| ed.toggle_bold(cx));
                                        }
                                    }))
                            )
                            .child(
                                Button::new("italic", "I")
                                    .style(if italic_active { ui::ButtonStyle::Filled } else { ui::ButtonStyle::Outlined })
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        if let Some(ed) = this.editor.clone() {
                                            ed.update(cx, |ed, cx| ed.toggle_italic(cx));
                                        }
                                    }))
                            )
                            .child(
                                Button::new("underline", "U")
                                    .style(if underline_active { ui::ButtonStyle::Filled } else { ui::ButtonStyle::Outlined })
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        if let Some(ed) = this.editor.clone() {
                                            ed.update(cx, |ed, cx| ed.toggle_underline(cx));
                                        }
                                    }))
                            )
                    )
                    .child(div().w(px(1.)).h(px(20.)).bg(rgb(0x3a3a3a)).mx(px(4.)))
                    .child(
                        h_flex().gap(px(2.))
                            .child(IconButton::new("undo", IconName::Undo))
                            .child(IconButton::new("redo", IconName::ArrowRight))
                    )
                    .child(div().flex_1())
                    .child(Label::new("100%").size(ui::LabelSize::Small).color(ui::Color::Muted))
            )
            .child(
                gpui::div()
                    .flex()
                    .flex_row()
                    .flex_1()
                    .w_full()
                    .overflow_hidden()
                    .child(sidebar::Sidebar::new().build())
                    .child(
                        div()
                            .flex_1()
                            .h_full()
                            .overflow_hidden()
                            .child(editor.clone())
                    )
            )
            .child(status_bar::StatusBar::new(status).build())
    }
}

pub fn init_app(cx: &mut App) {
    cx.open_window(WindowOptions::default(), |_window, cx| {
        cx.new(|cx| OfficeApp::new(cx))
    }).unwrap();
}
