use eframe::{self, egui};
#[derive(Default)]
struct MyApp {}
impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {});
    }
}

fn main() -> Result<(), eframe::Error> {
    println!("Hello, world!");
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Closet",
        native_options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    );
}
