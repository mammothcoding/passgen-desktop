pub mod ui {
    use crate::generator::generator::Generator;
    use eframe::egui::text::LayoutJob;
    use eframe::egui::Align::Center;
    use eframe::egui::{
        Direction, FontSelection, Key, Style
    };
    use eframe::{egui};
    use egui::OpenUrl;
    use egui_extras::{Column, TableBuilder};

    impl eframe::App for Generator {
        fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
            // Title
            egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
                TableBuilder::new(ui)
                    .column(Column::initial(10.0))
                    .column(Column::remainder())
                    .column(Column::initial(10.0))
                    .body(|mut body| {
                        body.row(24.0, |mut row| {
                            // Dark-light switcher
                            row.col(|ui| {
                                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                    egui::widgets::global_theme_preference_switch(ui);
                                    /*if ui.button("☀").clicked() {
                                        let visuals = if ui.visuals().dark_mode {
                                            Visuals::light()
                                        } else {
                                            Visuals::dark()
                                        };
                                        ctx.set_visuals(visuals);
                                    }*/
                                });
                            });

                            // Central top text
                            row.col(|ui| {
                                ui.with_layout(
                                    egui::Layout::centered_and_justified(Direction::LeftToRight),
                                    |ui| {
                                        ui.label(
                                            egui::RichText::new("Mammothcoding passgen")
                                                .heading()
                                                .color(egui::Color32::LIGHT_BLUE),
                                        );
                                    },
                                );
                            });

                            // About button
                            row.col(|ui| {
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        let ind_btn = ui
                                            .add_sized(
                                                [20.0, 20.0],
                                                egui::Button::new(egui::RichText::new("i").color(egui::Color32::GREEN))
                                                    .small()
                                                    .rounding(egui::Rounding::same(45.0)),
                                            )
                                            .on_hover_text(self.get_lang_text("but_about"));
                                        if ind_btn.clicked() {
                                            self.switch_lang();
                                        };
                                    },
                                );
                            });
                        });
                    });
            });

            // Footer
            egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
                TableBuilder::new(ui)
                    .column(Column::initial(25.0))
                    .column(Column::remainder())
                    .column(Column::initial(25.0))
                    .body(|mut body| {
                        body.row(24.0, |mut row| {

                            // Labeled button send to home
                            row.col(|ui| {
                                let img = egui::Image::new(egui::include_image!("../icon_24x24.png"));
                                if ui
                                    .add_sized(
                                        [25.0, 25.0],
                                        egui::ImageButton::new(img)
                                            .rounding(0.0)//бесполез
                                            //.uv()
                                    )
                                    .on_hover_text("Mammothcoding passgen home".to_string())
                                    .clicked()
                                {
                                    let url = OpenUrl {
                                        url: "https://github.com/mammothcoding".to_string(),
                                        new_tab: true,
                                    };
                                    ui.ctx().open_url(url);
                                };
                            });

                            // Central bottom text
                            row.col(|ui| {
                                ui.with_layout(
                                    egui::Layout::centered_and_justified(Direction::LeftToRight),
                                    |ui| {
                                        //let lab = ui.label(egui::RichText::new(footer_text).color(egui::Color32::LIGHT_BLUE));
                                        let hyp = egui::Hyperlink::from_label_and_url(
                                            self.get_lang_text("homepage"),
                                            "https://github.com/mammothcoding",
                                        )
                                        .open_in_new_tab(true);
                                        ui.add(hyp);
                                    },
                                );
                            });

                            // Lang indicator
                            row.col(|ui| {
                                let ind_text = if self.lang.as_str() == "en" {
                                    egui::RichText::new("Ru").color(egui::Color32::WHITE)
                                } else {
                                    egui::RichText::new("En").color(egui::Color32::WHITE)
                                };
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        let ind_btn = ui
                                            .add_sized(
                                                [25.0, 25.0],
                                                egui::Button::new(ind_text)
                                                    .small()
                                                    .rounding(egui::Rounding::same(60.0))
                                                    .fill(egui::Color32::from_rgb(0, 115, 153)),
                                            )
                                            .on_hover_text(self.get_lang_text("lang_ttip"));
                                        if ind_btn.clicked() {
                                            self.switch_lang();
                                        };
                                    },
                                );
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
                                    //let pass_len_label = ui.label("Password length:");
                                    let pass_len_label = ui.label(self.get_lang_text("pass_len"));
                                    let sli = egui::Slider::new(&mut self.pwd_len, 4..=10000)
                                        .logarithmic(true);
                                    ui.add(sli).labelled_by(pass_len_label.id);
                                });

                                // Rules
                                let t_letters = self.get_lang_text("inc_lcase");
                                let t_u_letters = self.get_lang_text("inc_cap");
                                let t_numbs = self.get_lang_text("inc_num");
                                let t_spec_symbs = self.get_lang_text("inc_ss");
                                let t_let_num_drc_free = self.get_lang_text("inc_all_ex");

                                ui.checkbox(&mut self.letters, t_letters);
                                ui.checkbox(&mut self.u_letters, t_u_letters);
                                ui.checkbox(&mut self.numbs, t_numbs);
                                ui.checkbox(&mut self.spec_symbs, t_spec_symbs);
                                ui.checkbox(&mut self.let_num_drc_free, t_let_num_drc_free);
                            });

                            // Gen button
                            row.col(|ui| {
                                ui.horizontal_centered(|ui| {
                                    let style = Style::default();
                                    let mut layout_job = LayoutJob::default();

                                    egui::widget_text::RichText::new(self.get_lang_text("but_gen_gen"))
                                        .font(egui::FontId::monospace(16.0))
                                        .color(egui::Color32::LIGHT_BLUE)
                                        .append_to(
                                            &mut layout_job,
                                            &style,
                                            FontSelection::Default,
                                            Center,
                                        );
                                    egui::widget_text::RichText::new(self.get_lang_text("but_gen_press"))
                                        .font(egui::FontId::monospace(10.0))
                                        .append_to(
                                            &mut layout_job,
                                            &style,
                                            FontSelection::Default,
                                            Center,
                                        );

                                    if ui
                                        .add_sized(
                                            [110.0, 45.0],
                                            egui::Button::new(layout_job).rounding(
                                                egui::Rounding {
                                                    nw: 12.0,
                                                    ne: 3.0,
                                                    sw: 3.0,
                                                    se: 3.0,
                                                },
                                            ),
                                        )
                                        .on_hover_text(self.get_lang_text("but_ttip"))
                                        .clicked()
                                    {
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
                    ui.vertical_centered(|ui| {
                        ui.add(egui::Label::new(
                            egui::RichText::new(self.get_lang_text("to_clip"))
                                .font(egui::FontId::monospace(10.0))
                                .color(egui::Color32::LIGHT_GRAY),
                        ));
                    });
                    ui.separator();

                    ui.with_layout(
                        egui::Layout::centered_and_justified(Direction::LeftToRight),
                        |ui| {
                            ui.add(egui::Label::new(
                                egui::RichText::new(pwd)
                                    .font(egui::FontId::monospace(32.0))
                                    .color(egui::Color32::GREEN)
                                    .strong(),
                            ));
                        },
                    );
                } else {
                    ui.with_layout(egui::Layout::bottom_up(Center), |ui| {
                        ui.add(egui::Label::new(
                            egui::RichText::new(self.get_lang_text("made_rust"))
                                .font(egui::FontId::monospace(9.0))
                                .color(egui::Color32::DARK_GRAY),
                        ));
                    });
                }

                // Keyboard events
                if ctx.input(|i| i.key_pressed(Key::Enter)) {
                    self.submit_to_pwd();
                }
                if ctx.input(|i| i.key_pressed(Key::Escape)) {
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
        }
    }
}
