mod generator;
mod ui;
mod gen_engine;

use eframe::egui;

fn main() -> eframe::Result<()>  {
    eframe::run_native(
        "M-passgen",
        eframe::NativeOptions::default(),
        Box::new(|cc| Box::new(CalcApp::new(cc))),
    )
}

struct CalcApp {}

impl CalcApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        CalcApp {}
    }
}

impl eframe::App for CalcApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Mammothcoding passgen");
            ui.horizontal(|ui| {
                let _ = ui.add_sized(
                    [58.0, 48.0],
                    egui::Button::new("√").small(),
                );
                let _ = ui.small_button("C");
                let _ = ui.small_button("(");
                let _ = ui.small_button(")");
                let _ = ui.small_button("<=");
            });
            ui.horizontal(|ui| {
                let _ = ui.small_button("sin");
                let _ = ui.small_button("7");
                let _ = ui.small_button("8");
                let _ = ui.small_button("9");
                let _ = ui.small_button("*");
            });
            ui.horizontal(|ui| {
                let _ = ui.small_button("cos");
                let _ = ui.small_button("4");
                let _ = ui.small_button("5");
                let _ = ui.small_button("6");
                let _ = ui.small_button("/");
            });
            ui.horizontal(|ui| {
                let _ = ui.small_button("tg");
                let _ = ui.small_button("1");
                let _ = ui.small_button("2");
                let _ = ui.small_button("3");
                let _ = ui.small_button("-");
            });
            ui.horizontal(|ui| {
                let _ = ui.small_button("ctg");
                let _ = ui.small_button(".");
                let _ = ui.small_button("0");
                let _ = ui.small_button("=");
                let _ = ui.small_button("+");
            });
        });
    }
}

