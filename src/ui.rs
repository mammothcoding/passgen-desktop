pub mod ui {
    use eframe::egui;
    use eframe::egui::Direction;
    use crate::generator::generator::Generator;

    impl eframe::App for Generator {
        fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

            // Title
            egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
                ui.with_layout(egui::Layout::centered_and_justified(Direction::TopDown), |ui| {
                    ui.label(egui::RichText::new("Mammothcoding passgen")
                        .heading()
                        .color(egui::Color32::LIGHT_BLUE)
                    );
                });
            });

            // Footer
            /*egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
                egui::Grid::new("bottom_table").num_columns(3).min_col_width(10.0).show(ui, |ui| {
                    egui::widgets::global_dark_light_mode_switch(ui);

                    let footer_text = if self.lang.as_str() == "en" {
                        "Made with RUST | 2024 | https://github.com/mammothcoding"
                    } else {
                        "Создано на языке RUST | 2024 | https://github.com/mammothcoding"
                    };
                    ui.with_layout(egui::Layout::centered_and_justified(Direction::RightToLeft), |ui| {
                        ui.label(egui::RichText::new(footer_text).color(egui::Color32::LIGHT_BLUE));
                    });

                    // Lang indicator
                    let ind_text = if self.lang.as_str() == "en" {
                        egui::RichText::new("Ru").color(egui::Color32::BLACK)
                    } else {
                        egui::RichText::new("En").color(egui::Color32::BLACK)
                    };
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        let ind_btn = ui.add_sized(
                            [20.0, 20.0],
                            egui::Button::new(ind_text).small().rounding(egui::Rounding::same(60.0)),
                        );
                        if ind_btn.clicked() {
                            self.switch_lang();
                        };
                    });
                });*/



                /*ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                /*let la = egui::Layout {
                    main_dir: Direction::LeftToRight,
                    main_wrap: false,
                    main_align: egui::Align::Center,
                    main_justify: false,
                    cross_align: egui::Align::Center,
                    cross_justify: false,
                };*/
                //ui.with_layout(la, |ui| {
                    egui::widgets::global_dark_light_mode_switch(ui);

                    let footer_text = if self.lang.as_str() == "en" {
                        "Made with RUST | 2024 | https://github.com/mammothcoding"
                    } else {
                        "Создано на языке RUST | 2024 | https://github.com/mammothcoding"
                    };
                    ui.label(egui::RichText::new(footer_text).color(egui::Color32::LIGHT_BLUE)
                    );

                    // Lang indicator
                    let ind_text = if self.lang.as_str() == "en" {
                        egui::RichText::new("Ru").color(egui::Color32::BLACK)
                    } else {
                        egui::RichText::new("En").color(egui::Color32::BLACK)
                    };
                    let ind_btn = ui.add_sized(
                        [20.0, 20.0],
                        egui::Button::new(ind_text).small().rounding(egui::Rounding::same(60.0)),
                    );
                    if ind_btn.clicked() {
                        self.switch_lang();
                    };
                });*/
            //});

            egui::CentralPanel::default().show(ctx, |ui| {
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
