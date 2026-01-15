#![doc = include_str!("../README.md")]

// import
use eframe::egui;
use egui_extras;
use engin::config::WorldConfig;
use engin::fly_view::MyApp;
use util::what_panic;

// main entry point
fn main() -> eframe::Result<()> {
    // debug: panic information at /tmp/what_panic.log
    what_panic();

    // start app
    // initial app windows size
    let config = WorldConfig::default();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([config.width, config.height]),
        ..Default::default()
    };

    eframe::run_native(
        "Fly Rust", // my app name
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(MyApp::new(cc)))
        }),
    )
}
