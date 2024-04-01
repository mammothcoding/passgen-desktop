//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod generator;
mod ui;
mod gen_engine;
mod ico;

use std::{env, io};
use std::io::Write;
use std::path::Path;
use eframe::egui;
use image::{ImageBuffer, ImageFormat, RgbImage};
use crate::generator::generator::Generator;
use crate::ico::ico::ICO_IMG;

fn main() -> eframe::Result<()>  {
    env::set_var("RUST_BACKTRACE", "full");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([480.0, 640.0])
            .with_min_inner_size([480.0, 640.0])
            .with_max_inner_size([480.0, 640.0])
            .with_transparent(true)
            .with_icon(load_icon()),
        ..Default::default()
    };

    eframe::run_native(
        "Mammothcoding passgen",
        options,
        Box::new(|_| Box::new(Generator::default())),
    )
}

fn load_icon() -> egui::viewport::IconData {
    let (icon_rgba, icon_width, icon_height) = {
        /*let image = image::open("icon.ico")
            .expect("Failed to open icon path")
            .into_rgba8();*/

        let mut a = Vec::<u8>::new();
        ICO_IMG.iter().for_each(|val| a.extend_from_slice(&val.to_be_bytes()));
        //let c: &[u8] = &a;
        let image = RgbImage::from_raw(32, 32, a).unwrap();
        let (width, height) = (32, 32);//image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    /*let image2 = image::open("icon_32x32.png")
        .expect("Failed to open icon path")
        .into_rgba8();
    let s = image2.into_raw();
    println!("{:?}", s);*/

    egui::viewport::IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}
