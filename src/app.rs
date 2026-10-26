use eframe::egui;

#[derive(Default)]
pub struct MyApp {}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default_margins().show(ui, |ui| {
            egui::Label::new("text");
            ui.scope(|ui| {
                ui.add_space(20.0);
                ui.vertical_centered(|ui| {
                    if ui
                        .button("Create Project")
                        .
                        .clicked()
                    {
                        //...
                    }
                });
                // any other buttons here also get this style
            });
            //outside style
        });
    }
}
