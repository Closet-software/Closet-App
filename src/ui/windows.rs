use eframe::egui;
pub struct About {
    pub is_open: bool,
}

impl Default for About {
    fn default() -> Self {
        Self { is_open: false }
    }
}

impl About {
    fn name(&self) -> &'static str {
        "About Closet"
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        let mut is_open = self.is_open;

        let center = ui.clip_rect().center();
        egui::Window::new(self.name())
            .default_width(220.0)
            .default_height(480.0)
            .open(&mut is_open)
            .resizable([false, false])
            .scroll(false)
            .constrain_to(ui.available_rect_before_wrap())
            .default_pos(center) // <-- add this
            .pivot(egui::Align2::CENTER_CENTER)
            .show(ui.ctx(), |ui| {
                self.ui(ui);
            });

        self.is_open = is_open;
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        use egui::special_emojis::{OS_APPLE, OS_LINUX, OS_WINDOWS};
        ui.vertical_centered(|ui| {
            ui.add_space(5.0);
            ui.add(
                egui::Image::new(egui::include_image!("../../assets/imgs/closet-logo.svg"))
                    .max_height(100.0)
                    .tint(ui.visuals().strong_text_color()),
            );
            ui.add_space(5.0);
            ui.horizontal_wrapped(|ui| {
                ui.label("Closet is a website editor written in rust using");
                ui.hyperlink_to("egui", "https://github.com/emilk/egui");
            });
        });
    }
}
