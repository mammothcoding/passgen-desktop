//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod generator;
mod ui;
mod gen_engine;
mod ico;

use std::{env, io};
use std::io::Write;
use std::path::Path;
use eframe::egui;
use eframe::egui::Shape::Vec;
use image::{GenericImageView, ImageBuffer, ImageFormat, RgbImage, Pixel};
use crate::generator::generator::Generator;
use crate::ico::ico::{ICO_IMG, ICO_PXL_DATA};

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
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();*/
        //(rgba, width, height)



        use std::vec::Vec;
        let mut pixels: Vec<Vec<u8>> = Vec::new();
        let mut pix: Vec<u8> = Vec::new();
        for i in 0..ICO_PXL_DATA.len() {
            pix.push(ICO_PXL_DATA[i] as u8);
            println!("ICO_PXL_DATA[i] = {}", ICO_PXL_DATA[i]);
            if pix.len() == 4 {
                pixels.push(pix.clone());
                pix = Vec::new();
            }
        }
        //println!("ICO_PXL_DATA.len() = {}", ICO_PXL_DATA.len());
        println!("pixels.len() = {}", pixels.len());
        //println!("{:?}", pixels);

        let mut imgbuf = image::ImageBuffer::new(32, 32);
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            //let pixels_idx: usize = (x + ((y) * 32)) as usize;
            //println!("pixels_idx = {}  x={} y={}", pixels_idx, x, y);
            let pix_data: Vec<u8> = pixels[(x + ((y) * 32)) as usize].clone();
            *pixel = image::Rgba([pix_data[0], pix_data[1], pix_data[2], pix_data[3]]);
            //*pixel = image::Rgba([3, 2 , 1, 255]);
        }
        let (width, height) = imgbuf.dimensions();
        let rgba = imgbuf.into_raw();

        (rgba, width, height)
    };

    egui::viewport::IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }

        /*Рабочий вариант с include_bytes
    let a: &[u8] = include_bytes!("../icon_32x32.png");
    let image = image::load_from_memory_with_format(a, ImageFormat::Png).unwrap();
    let (width, height) = (32, 32);
    let rgba = image.to_rgba8().into_raw();
    (rgba, width, height)*/


    //Байты
    /*let image2 = image::open("icon_32x32.png")
        .expect("Failed to open icon path")
        .into_rgba8();
    let s = image2.into_raw();
    println!("{:?}", s);*/

    //Тесктом
    //let image2: &[u8] = include_bytes!("../icon_32x32.png");
    //println!("{:?}", image2);
    //print!("{}", String::from_utf8_lossy(image2));

    /*Пиксели
    let img = image::open("icon_32x32.png").expect("File not found!");
    let (w, h) = img.dimensions();
    println!("{:?}", img.pixels());*/

    /*Изменяем изображение попиксельно
        let img = image::open("icon_32x32.png").expect("File not found!");
        let (w, h) = img.dimensions();
        let mut output = ImageBuffer::new(w, h); // create a new buffer for our output
        for (x, y, pixel) in img.pixels() {
            output.put_pixel(x, y,
                             // pixel.map will iterate over the r, g, b, a values of the pixel
                             pixel.map(|p| p.saturating_sub(65))
            );
        }
        let (width, height) = output.dimensions();
        let rgba = output.into_raw();
        (rgba, width, height)*/
}
