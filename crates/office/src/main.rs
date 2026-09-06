use gpui::{App, Font, Pixels};
use gpui_platform::application;
use office_ui::init_app;
use theme::{ThemeSettingsProvider, UiDensity, LoadThemes};

struct DummyProvider;
impl ThemeSettingsProvider for DummyProvider {
    fn ui_font<'a>(&'a self, _cx: &'a App) -> &'a Font {
        use std::sync::OnceLock;
        static F: OnceLock<Font> = OnceLock::new();
        F.get_or_init(|| Font::default()).into()
    }
    fn buffer_font<'a>(&'a self, _cx: &'a App) -> &'a Font { self.ui_font(_cx) }
    fn ui_font_size(&self, _cx: &App) -> Pixels { gpui::px(14.0) }
    fn buffer_font_size(&self, _cx: &App) -> Pixels { gpui::px(14.0) }
    fn ui_density(&self, _cx: &App) -> UiDensity { UiDensity::Compact }
}

fn main() {
    application().run(|cx: &mut App| {
        theme::init(LoadThemes::JustBase, cx);
        theme::set_theme_settings_provider(Box::new(DummyProvider), cx);
        init_app(cx);
    })
}
