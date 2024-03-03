mod generator;
mod ui;
mod gen_engine;

use eframe::egui;

fn main() -> eframe::Result<()>  {
    eframe::run_native(
        "calculator-wasm-rust-pwa",
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
            ui.label(r#"
            Это вымышленный калькулятов.
            Чтобы воспользоваться калькулятором вам необходимо воспользоваться воображением.            Представте себе любой интерфейс, наберите в нем математическое выражение и нажмите '='.
            Увидели результат, Да? - Поздравляю, ваш калькулятор работает хорошо.
            "#);
        });
    }
}

