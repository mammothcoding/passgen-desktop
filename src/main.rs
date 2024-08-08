//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

//use std::env;
//use eframe::egui;
use wasm_bindgen_futures::wasm_bindgen::prelude::wasm_bindgen;
use crate::generator::generator::Generator;

mod gen_engine;
mod generator;
mod ico;
mod text_processor;
mod ui;

/*#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    use std::env;
    env::set_var("RUST_BACKTRACE", "full");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([420.0, 300.0])
            //.with_min_inner_size([200.0, 200.0])
            //.with_max_inner_size([200.0, 200.0])
            .with_resizable(true)
            .with_transparent(true)
            .with_icon(gen_icon_from_png_pixels_data(24, ICO_PNG_PXL_DATA)), //for X11 and mac, not for wayland programmaticaly way
        ..Default::default()
    };

    eframe::run_native(
        "McPassgen",
        options,
        //Box::new(|_| Box::new(Generator::default())),
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx); // This gives us image support:
                                                              //Box::<Generator>::default()
            Box::new(Generator::default())
        }),
    )
}

#[cfg(target_arch = "wasm32")]*/
fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let runner = eframe::WebRunner::new();

        runner.start(
            "McPassgen",
            web_options,
            Box::new(|cc| {
                egui_extras::install_image_loaders(&cc.egui_ctx); // This gives us image support:
                Ok (Box::new(Generator::default()))
            }),
        )
            .await
            .expect("failed to start McPassgen");
    });
}


/*
#[derive(Clone)]
#[wasm_bindgen]
pub struct WebHandle {
    runner: eframe::WebRunner,
}

#[wasm_bindgen]
impl WebHandle {
    /// Installs a panic hook, then returns.
    #[allow(clippy::new_without_default)]
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        // Redirect [`log`] message to `console.log` and friends:
        eframe::WebLogger::init(log::LevelFilter::Debug).ok();

        Self {
            runner: eframe::WebRunner::new(),
        }
    }

    /// Call this once from JavaScript to start your app.
    #[wasm_bindgen]
    pub async fn start(&self) -> Result<(), wasm_bindgen::JsValue> {
        self.runner
            .start(
                "McPassgen",
                eframe::WebOptions::default(),
                Box::new(|cc| {
                    egui_extras::install_image_loaders(&cc.egui_ctx); // This gives us image support:
                    Ok (Box::new(Generator::default()))
                }),
            )
            .await
    }
}*/
