mod app;
use app::MyApp;
use log::*;

pub const APP_NAME: &str = "Closet";

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        APP_NAME,
        options,
        Box::new(|cc| {
            app::set_font(&cc.egui_ctx);
            Ok(Box::new(MyApp::default()))
        }),
    )
}
