pub mod ui {
    use eframe::egui;
    use crate::generator::generator::Generator;

    impl eframe::App for Generator {
        fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("Mammothcoding passgen");
                let pass_len_label = ui.label("Password length:");
                //ui.text_edit_singleline(&mut self.pwd_len).labelled_by(pass_len_label.id);

                ui.add(egui::Slider::new(&mut self.pwd_len, 4..=20)).labelled_by(pass_len_label.id);


                /*ui.horizontal(|ui| {
                    let _ = ui.add_sized(
                        [58.0, 48.0],
                        egui::Button::new("√").small(),
                    );
                    let _ = ui.small_button("C");
                    let _ = ui.small_button("(");
                    let _ = ui.small_button(")");
                    let _ = ui.small_button("<=");
                });*/
            });
        }
    }
}
