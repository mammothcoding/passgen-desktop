mod generator;
mod ui;
mod gen_engine;

use eframe::egui;

fn main() {
    ui.heading("My egui Application");
    ui.horizontal(|ui| {
        ui.label("Your name: ");
        ui.text_edit_singleline(&mut name);
    });
    ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
    if ui.button("Increment").clicked() {
        age += 1;
    }
    ui.label(format!("Hello '{name}', age {age}"));
}
