//! Spreadsheet grid view component.

use gpui::{Entity, Window, Context};
use ui::prelude::*;
use ui::{Label, div, v_flex};
use office_core::sheet::Sheet;

/// A view for editing a spreadsheet.
pub struct SpreadsheetView {
    sheet: Entity<Sheet>,
}

impl SpreadsheetView {
    pub fn new(sheet: Entity<Sheet>, _cx: &mut Window) -> Self {
        Self { sheet }
    }
}

impl Render for SpreadsheetView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex().flex_1().child(
            div().id("spreadsheet-grid").child(
                Label::new("Spreadsheet Grid"),
            ),
        )
    }
}
