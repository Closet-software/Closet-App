use crate::ui::windows::About;
use eframe::egui::{self};

//const CHANGELOG_URL: &str = "https://raw.githubusercontent.com/emilk/egui/refs/heads/main/README.md";

#[derive(Default)]
struct Windows {
    about: About,
}

#[derive(Default)]
pub struct MyApp {
    windows: Windows,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        //setup font
        ui.style_mut().override_font_id =
            Some(egui::FontId::new(17.0, egui::FontFamily::Proportional));

        egui::Panel::top("top_panel").show(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("Closet", |ui| {
                    if ui.button("About").clicked() {
                        self.windows.about.is_open = true;
                        ui.close();
                    }
                });
            });
        });
        self.windows.about.show(ui);
    }
}
