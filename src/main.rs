mod app;

use app::MyApp;

const APP_NAME: &str = "Closet";

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        APP_NAME,
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}
