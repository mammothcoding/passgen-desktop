#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use eframe::HardwareAcceleration::Off;
use std::env;
//use std::panic;

use crate::generator::generator::Generator;
use crate::ico::ico::{gen_icon_from_png_pixels_data, ICO_PNG_PXL_DATA};

mod gen_engine;
mod generator;
mod ico;
mod text_processor;
mod ui;

fn main() -> eframe::Result<()> {
    env::set_var("RUST_BACKTRACE", "full");

    let options = eframe::NativeOptions {
        hardware_acceleration: Off,
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([420.0, 300.0])
            //.with_min_inner_size([420.0, 300.0])
            //.with_max_inner_size([200.0, 200.0])
            .with_maximize_button(false)
            .with_resizable(false)
            .with_transparent(true)
            .with_icon(gen_icon_from_png_pixels_data(24, ICO_PNG_PXL_DATA)), //for X11 and mac, not for wayland programmatically way
        ..Default::default()
    };

    eframe::run_native(
        "McPassgen",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx); // This gives us image support
            Ok(Box::new(Generator::default()))
        }),
    )
}
