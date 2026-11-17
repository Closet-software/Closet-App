use crate::ui::panels::TopBar;
use crate::ui::windows::About;
use eframe::egui;

#[derive(Default)]
struct Windows {
    about: About,
}

#[derive(Default)]
pub struct MyApp {
    windows: Windows,
    top_bar: TopBar,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Setup font
        ui.style_mut().override_font_id =
            Some(egui::FontId::new(17.0, egui::FontFamily::Proportional));
        self.top_bar.show(ui, &mut self.windows.about);
        self.windows.about.show(ui);
    }
}
