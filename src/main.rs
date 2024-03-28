//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod generator;
mod ui;
mod gen_engine;

use eframe::egui;
use crate::generator::generator::Generator;

fn main() -> eframe::Result<()>  {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([480.0, 640.0])
            .with_min_inner_size([480.0, 640.0])
            .with_max_inner_size([480.0, 640.0])
            .with_transparent(true),
        ..Default::default()
    };

    eframe::run_native(
        "Mammothcoding passgen",
        options,
        Box::new(|_| Box::new(Generator::default())),
    )
}
