pub mod ui {
    use eframe::egui;
    use crate::generator::generator::Generator;

    impl eframe::App for Generator {
        fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("Mammothcoding passgen");
                ui.separator();

                // Password length area
                let pass_len_label = ui.label("Password length:");
                ui.add(egui::Slider::new(&mut self.pwd_len, 4..=20)).labelled_by(pass_len_label.id);

                // Rules
                ui.checkbox(&mut self.letters, "include lowercase letters");
                ui.checkbox(&mut self.u_letters, "include capital letters");
                ui.checkbox(&mut self.numbs, "include numbers");
                ui.checkbox(&mut self.spec_symbs, "include special symbols");
                ui.checkbox(&mut self.let_num_drc_free, "all nums & letters exclude \"0oOiIlL1\"");
                ui.separator();

                // Gen button
                if ui.add_sized(
                    [90.0, 30.0],
                    egui::Button::new("generate").small(),
                ).clicked() {
                    self.submit_to_pwd();
                };
                ui.separator();

                // Password result area
                let mut pwd = self.pwd.clone();
                if pwd.len() > 40 {
                    pwd = format!("{}...", &pwd[..39].to_string());
                }
                if self.pwd != "" {
                    let text: String = if self.lang.as_str() == "en" {
                        if self.errors.0 != "" {
                            self.errors.0.clone()
                        } else {
                            "this password was copied to clipboard".to_string()
                        }
                    } else {
                        if self.errors.1 != "" {
                            self.errors.1.clone()
                        } else {
                            "пароль был скопирован в буфер обмена".to_string()
                        }
                    };

                    ui.add(egui::Label::new(
                        egui::RichText::new("Password is")
                            .font(egui::FontId::monospace(15.0))
                    ).wrap(true));

                    ui.add(egui::Label::new(
                        egui::RichText::new(pwd)
                            .font(egui::FontId::monospace(25.0))
                    ).wrap(true));

                    ui.add(egui::Label::new(
                        egui::RichText::new(text)
                            .font(egui::FontId::monospace(15.0))
                    ).wrap(true));
                }
            });
        }
    }
}
