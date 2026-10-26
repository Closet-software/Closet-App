mod app;

use app::MyApp;

const APP_NAME: &str = "Closet";

fn main() -> eframe::Result {
    egui_logger::builder().init().unwrap();
    let options = eframe::NativeOptions {
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        APP_NAME,
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}
