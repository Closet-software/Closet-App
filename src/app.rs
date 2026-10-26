use eframe::egui::{self, Color32, FontData, FontDefinitions, Response, RichText};
use std::sync::Arc;

#[derive(Default)]
pub struct MyApp {}

pub fn set_font(ctx: &egui::Context) {
    let mut font = FontDefinitions::default();

    font.font_data.insert(
        "font".to_owned(),
        Arc::new(FontData::from_static(include_bytes!(
            "../assets/fonts/JetBrainsMono-Regular.ttf"
        ))),
    );

    font.families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "font".to_owned());

    font.families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .insert(0, "my_font".to_owned());

    ctx.set_fonts(font)
}

fn button(ui: &mut egui::Ui, label: &str) -> Response {
    ui.add(
        egui::Button::new(RichText::new(label).size(17.0).color(Color32::LIGHT_GRAY))
            .min_size(egui::Vec2 { x: 120.0, y: 70.0 })
            .fill(egui::Color32::from_hex("#5c5c51").unwrap())
            .corner_radius(5.0),
    )
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default_margins().show(ui, |ui| {
            ui.style_mut().override_font_id =
                Some(egui::FontId::new(20.0, egui::FontFamily::Proportional));

            // Top button
            ui.add_space(20.0);
            ui.vertical_centered(|ui| {
                if button(ui, "Create Project").clicked() {
                    //...
                }
                ui.add_space(40.0);
                ui.add(egui::Label::new("changelog")); //ADD CHANGELOG HERE
            });

            // Bottom button — anchored to the bottom of the panel
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add_space(20.0); // padding from the window edge
                if button(ui, "Browse Project").clicked() {
                    //...
                }
            });
        });
    }
}
