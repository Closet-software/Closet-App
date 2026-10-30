use eframe::egui::{self, Color32, Response, RichText};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};

// Replace this with the URL of your file on raw.githubusercontent.com.
// Example: https://raw.githubusercontent.com/OWNER/REPO/main/CHANGELOG.md
const CHANGELOG_URL: &str = "";

#[derive(Default)]
pub struct MyApp {
    /*     markdown_fetcher: MarkdownFetcher,
    changelog: Option<String>,
    changelog_loading: bool,
    changelog_error: Option<String>,
    changelog_started: bool,
    changelog_cache: CommonMarkCache, */
}

fn button(ui: &mut egui::Ui, label: &str) -> Response {
    ui.add(
        egui::Button::new(RichText::new(label).size(17.0).color(Color32::LIGHT_GRAY))
            .min_size(egui::Vec2 { x: 120.0, y: 70.0 })
            .fill(Color32::from_hex("#5c5c51").unwrap())
            .corner_radius(5.0),
    )
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default_margins().show(ui, |ui| {
            ui.style_mut().override_font_id =
                Some(egui::FontId::new(20.0, egui::FontFamily::Proportional));

            ui.add_space(20.0);
            ui.vertical_centered(|ui| {
                if button(ui, "Create Project").clicked() {
                    // TODO: create project
                }
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add_space(20.0);
                if button(ui, "Browse Project").clicked() {
                    // TODO: browse projects
                }
            });
        });
    }
}
