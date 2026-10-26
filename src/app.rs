use eframe::egui::{self, Color32, Response, RichText};

#[derive(Default)]
pub struct MyApp {}

fn button(ui: &mut egui::Ui, label: &str) -> Response {
    ui.add(
        egui::Button::new(RichText::new(label).size(20.0).color(Color32::LIGHT_GRAY))
            .min_size(egui::Vec2 { x: 120.0, y: 70.0 })
            .fill(egui::Color32::BLACK)
            .corner_radius(5.0),
    )
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default_margins().show(ui, |ui| {
            ui.scope(|ui| {
                ui.add_space(20.0);
                ui.vertical_centered(|ui| {
                    if button(ui, "Browse Projects").clicked() {
                        //...
                    }
                });
                // any other buttons here also get this style
            });
            //outside style
        });
    }
}
