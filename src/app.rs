use crate::misc::MarkdownFetcher;
use eframe::egui::{self, Color32, Response, RichText};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};

// Replace this with the URL of your file on raw.githubusercontent.com.
// Example: https://raw.githubusercontent.com/OWNER/REPO/main/CHANGELOG.md
const CHANGELOG_URL: &str = "";

#[derive(Default)]
pub struct MyApp {
    markdown_fetcher: MarkdownFetcher,
    changelog: Option<String>,
    changelog_loading: bool,
    changelog_error: Option<String>,
    changelog_started: bool,
    changelog_cache: CommonMarkCache,
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
        // Start the request once, not on every UI redraw.
        if !self.changelog_started {
            self.changelog_started = true;
            self.changelog_loading = true;
            self.markdown_fetcher
                .fetch(CHANGELOG_URL.to_owned(), ui.ctx().clone());
        }

        // Transfer completed HTTP results into app state on the UI thread.
        while let Ok(result) = self.markdown_fetcher.try_recv() {
            self.changelog_loading = false;
            match result {
                Ok(text) => {
                    self.changelog = Some(text);
                    self.changelog_error = None;
                }
                Err(err) => self.changelog_error = Some(err),
            }
        }

        egui::CentralPanel::default_margins().show(ui, |ui| {
            ui.style_mut().override_font_id =
                Some(egui::FontId::new(20.0, egui::FontFamily::Proportional));

            ui.add_space(20.0);
            ui.vertical_centered(|ui| {
                if button(ui, "Create Project").clicked() {
                    // TODO: create project
                }

                ui.add_space(28.0);
                ui.heading("Changelog");
            });

            ui.separator();
            /*
                        egui::ScrollArea::horizontal()
                            .max_height(280.0)
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                if self.changelog_loading {
                                    ui.horizontal(|ui| {
                                        ui.spinner();
                                        ui.label("Fetching changelog...");
                                    });
                                } else if let Some(markdown) = self.changelog.as_deref() {
                                    CommonMarkViewer::new().show(ui, &mut self.changelog_cache, markdown);
                                } else if let Some(error) = self.changelog_error.as_deref() {
                                    ui.colored_label(Color32::LIGHT_RED, error);
                                }
                            });
            */
            // Keep the secondary action aligned to the bottom of the window.
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add_space(20.0);
                if button(ui, "Browse Project").clicked() {
                    // TODO: browse projects
                }
            });
        });
    }
}
