use crate::ui::windows::About;
use eframe::egui;

#[derive(Default)]
pub struct TopBar;


pub struct Panels{
    project_files: bool,
    elements: bool,
    console: bool,
}

impl Default for Panels {
    fn default() -> Self {
        Self {
            project_files: true,
            elements: true,
            console: false,
        }
    }
}

impl TopBar {
    pub fn show(&mut self, ui: &mut egui::Ui, about: &mut About) {
        egui::Panel::top("top_panel").show(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("Closet", |ui| {
                    if ui.button("About").clicked() {
                        about.is_open = true;
                        ui.close();
                    }

                    if ui.button("Check for updates").clicked() {}

                    if ui.button("Quit").clicked() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                ui.menu_button("File", |ui| {
                    if ui.button("New File").clicked() {}
                    if ui.button("Open File").clicked() {}
                });
            });
        });
    }
}

impl Panels {
    fn ui(&mut self, ui: &mut egui::Ui) {

        let Self {
            left,

        }
    }
}
