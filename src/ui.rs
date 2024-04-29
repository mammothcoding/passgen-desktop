pub mod ui {
    use std::any::Any;
    use crate::generator::generator::Generator;
    use eframe::{egui, epaint};
    use eframe::egui::text::LayoutJob;
    use eframe::egui::WidgetText::RichText;
    use eframe::egui::{Align, Direction, FontSelection, Style, TextureOptions, Widget};
    use eframe::epaint::text::Row;
    use egui_extras::{Column, TableBuilder};
    use std::sync::Arc;
    use eframe::egui::Align::Center;
    use eframe::egui::load::SizedTexture;
    use eframe::epaint::{ColorImage, TextureId, Vec2};
    use image::RgbImage;
    use crate::ico::ico::ICO_PNG_PXL_DATA;

    impl eframe::App for Generator {
        fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
            // Title
            egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
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

            // Footer
            egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
                TableBuilder::new(ui)
                    .column(Column::initial(24.0))
                    .column(Column::remainder())
                    .column(Column::initial(10.0))
                    .body(|mut body| {
                        body.row(25.0, |mut row| {
                            // Dark-light switcher
                            row.col(|ui| {
                                /*ui.with_layout(
                                    egui::Layout::left_to_right(egui::Align::Center),
                                    |ui| {
                                        egui::widgets::global_dark_light_mode_switch(ui);
                                    },
                                );*/

                                let mut imgbuf = image::ImageBuffer::new(24, 24);
                                for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
                                    let r = ICO_PNG_PXL_DATA[((x + y * 24) * 4) as usize] as u8;
                                    let g = ICO_PNG_PXL_DATA[((x + y * 24) * 4 + 1) as usize] as u8;
                                    let b = ICO_PNG_PXL_DATA[((x + y * 24) * 4 + 2) as usize] as u8;
                                    let a = ICO_PNG_PXL_DATA[((x + y * 24) * 4 + 3) as usize] as u8;
                                    *pixel = image::Rgba([r, g, b, a]);
                                }
                                //let rgba = imgbuf.into_raw();
                                let mut img = ColorImage::from_rgba_premultiplied(
                                    [24, 24],
                                    imgbuf.as_flat_samples().as_slice(),
                                );
                                let img_data = epaint::image::ImageData::Color(Arc::from(img));
                                let mut texture_man = epaint::textures::TextureManager::default();
                                let texture_id = texture_man.alloc("mcp".parse().unwrap(), img_data, TextureOptions::default());
                                let sized_texture = SizedTexture {
                                    id: texture_id,
                                    size: Vec2 {
                                        x: 24.0,
                                        y: 24.0,
                                    },
                                };

                                ui.add(
                                    egui::Image::from_texture(sized_texture)
                                );

                            });

                            // Central bottom text
                            row.col(|ui| {
                                let footer_text = if self.lang.as_str() == "en" {
                                    "Homepage: https://github.com/mammothcoding"
                                } else {
                                    "Домашняя страница: https://github.com/mammothcoding"
                                };
                                ui.with_layout(
                                    egui::Layout::centered_and_justified(Direction::LeftToRight),
                                    |ui| {
                                        //let lab = ui.label(egui::RichText::new(footer_text).color(egui::Color32::LIGHT_BLUE));
                                        let hyp = egui::Hyperlink::from_label_and_url(
                                            footer_text,
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
                                        let ind_btn = ui.add_sized(
                                            [20.0, 20.0],
                                            egui::Button::new(ind_text)
                                                .small()
                                                .rounding(egui::Rounding::same(60.0))
                                                .fill(egui::Color32::from_rgb(0, 115, 153)),
                                        );
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
                                    let pass_len_label = ui.label("Password length:");
                                    let sli = egui::Slider::new(&mut self.pwd_len, 4..=10000)
                                        .logarithmic(true);
                                    ui.add(sli).labelled_by(pass_len_label.id);
                                });

                                // Rules
                                ui.checkbox(&mut self.letters, "include lowercase letters");
                                ui.checkbox(&mut self.u_letters, "include capital letters");
                                ui.checkbox(&mut self.numbs, "include numbers");
                                ui.checkbox(&mut self.spec_symbs, "include special symbols");
                                ui.checkbox(
                                    &mut self.let_num_drc_free,
                                    "all nums & letters exclude \"0oOiIlL1\"",
                                );
                            });

                            // Gen button
                            row.col(|ui| {
                                ui.horizontal_centered(|ui| {
                                    let style = Style::default();
                                    let mut layout_job = LayoutJob::default();

                                    egui::widget_text::RichText::new("GENERATE\n")
                                        .font(egui::FontId::monospace(17.0))
                                        .color(egui::Color32::LIGHT_BLUE)
                                        .append_to(
                                            &mut layout_job,
                                            &style,
                                            FontSelection::Default,
                                            Align::Center,
                                        );
                                    egui::widget_text::RichText::new("       (press Enter)")
                                        .font(egui::FontId::monospace(10.0))
                                        .append_to(
                                            &mut layout_job,
                                            &style,
                                            FontSelection::Default,
                                            Align::Center,
                                        );

                                    if ui
                                        .add_sized(
                                            [100.0, 45.0],
                                            egui::Button::new(layout_job).rounding(
                                                egui::Rounding {
                                                    nw: 12.0,
                                                    ne: 3.0,
                                                    sw: 3.0,
                                                    se: 3.0,
                                                },
                                            ),
                                        )
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
                                    .color(egui::Color32::LIGHT_YELLOW)
                                    .strong(),
                            ));
                        },
                    );
                } else {
                    ui.with_layout(egui::Layout::bottom_up(Center), |ui| {
                        ui.add(egui::Label::new(
                            egui::RichText::new("Made with Rust")
                                .font(egui::FontId::monospace(9.0))
                                .color(egui::Color32::DARK_GRAY),
                        ));
                    });
                }
            });
        }
    }
}
