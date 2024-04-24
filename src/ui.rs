pub mod ui {
    use eframe::egui;
    use eframe::egui::{Direction, Widget};
    use egui_extras::{Column, TableBuilder};
    use crate::generator::generator::Generator;

    impl eframe::App for Generator {
        fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

            // Title
            egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
                ui.with_layout(egui::Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                    ui.label(egui::RichText::new("Mammothcoding passgen")
                        .heading()
                        .color(egui::Color32::LIGHT_BLUE)
                    );
                });
            });

            // Footer
            egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
                TableBuilder::new(ui)
                    .column(Column::initial(10.0))
                    .column(Column::remainder())
                    .column(Column::initial(10.0))
                    .body(|mut body| {
                        body.row(25.0, |mut row| {

                            // Dark-light switcher
                            row.col(|ui| {
                                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                    egui::widgets::global_dark_light_mode_switch(ui);
                                });
                            });

                            // Central bottom text
                            row.col(|ui| {
                                let footer_text = if self.lang.as_str() == "en" {
                                    "Homepage: https://github.com/mammothcoding"
                                } else {
                                    "Домашняя страница: https://github.com/mammothcoding"
                                };
                                ui.with_layout(egui::Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                                    //let lab = ui.label(egui::RichText::new(footer_text).color(egui::Color32::LIGHT_BLUE));
                                    let hyp = egui::Hyperlink::from_label_and_url(footer_text, "https://github.com/mammothcoding")
                                        .open_in_new_tab(true);
                                    ui.add(hyp);
                                });
                            });

                            // Lang indicator
                            row.col(|ui| {
                                let ind_text = if self.lang.as_str() == "en" {
                                    egui::RichText::new("Ru").color(egui::Color32::WHITE)
                                } else {
                                    egui::RichText::new("En").color(egui::Color32::WHITE)
                                };
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    let ind_btn = ui.add_sized(
                                        [20.0, 20.0],
                                        egui::Button::new(ind_text).small().rounding(egui::Rounding::same(60.0)).fill(egui::Color32::from_rgb(0, 115, 153)),
                                    );
                                    if ind_btn.clicked() {
                                        self.switch_lang();
                                    };
                                });
                            });
                        });
                    });
            });

            egui::CentralPanel::default().show(ctx, |ui| {
                TableBuilder::new(ui)
                    .column(Column::remainder())
                    .column(Column::initial(110.0))
                    .body(|mut body| {
                        body.row(150.0, |mut row| {

                            // Options
                            row.col(|ui| {
                                // Password length area
                                ui.vertical(|ui| {
                                    ui.spacing_mut().slider_width = 200.0;
                                    let pass_len_label = ui.label("Password length:");
                                    let sli = egui::Slider::new(&mut self.pwd_len, 4..=10000).logarithmic(true);
                                    ui.add(sli).labelled_by(pass_len_label.id);
                                });

                                // Rules
                                ui.checkbox(&mut self.letters, "include lowercase letters");
                                ui.checkbox(&mut self.u_letters, "include capital letters");
                                ui.checkbox(&mut self.numbs, "include numbers");
                                ui.checkbox(&mut self.spec_symbs, "include special symbols");
                                ui.checkbox(&mut self.let_num_drc_free, "all nums & letters exclude \"0oOiIlL1\"");
                            });

                            // Gen button
                            row.col(|ui| {
                                ui.horizontal_centered(|ui| {
                                    if ui.add_sized([100.0, 45.0], egui::Button::new(
                                        egui::RichText::new("GENERATE\n(press Enter)")
                                            .color(egui::Color32::LIGHT_GRAY)
                                            .strong()
                                        ).small()
                                    )
                                        .clicked() {
                                        self.submit_to_pwd();
                                    };
                                });
                            });
                        });
                    });

                ui.separator();

                // Password result area
                let mut pwd = self.pwd.clone();
                if pwd.len() > 16 {
                    pwd = format!("{}...", &pwd[..15].to_string());
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

                    ui.vertical_centered(|ui| {
                        ui.add(egui::Label::new(
                            egui::RichText::new(text)
                                .font(egui::FontId::monospace(10.0))
                                .color(egui::Color32::LIGHT_GRAY)
                        ));
                        ui.separator();

                        ui.add(egui::Label::new(
                            egui::RichText::new(pwd)
                                .font(egui::FontId::monospace(32.0))
                                .color(egui::Color32::LIGHT_YELLOW)
                                .strong()
                        ));
                    });
                }
            });
        }
    }
}
