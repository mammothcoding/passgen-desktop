//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod generator;
mod ui;
mod gen_engine;
mod ico;

use std::{env};
use eframe::egui;
use crate::generator::generator::Generator;
use crate::ico::ico::{ICO_PNG_PXL_DATA, gen_icon_from_png_pixels_data};

fn main() -> eframe::Result<()>  {
    env::set_var("RUST_BACKTRACE", "full");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([450.0, 450.0])
            //.with_min_inner_size([200.0, 200.0])
            //.with_max_inner_size([200.0, 200.0])
            .with_resizable(true)
            .with_transparent(true)
            .with_icon(gen_icon_from_png_pixels_data(24, ICO_PNG_PXL_DATA)), //for X11 and mac, not for wayland programmaticaly way
        ..Default::default()
    };

    eframe::run_native(
        "Mc passgen",
        options,
        Box::new(|_| Box::new(Generator::default())),
    )
}
