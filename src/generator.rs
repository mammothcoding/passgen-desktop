pub mod generator {

    const CIRCUITED_FIELDS: [&str; 6] = [
        "pwd_len",
        "letters",
        "u_letters",
        "numbs",
        "spec_symbs",
        "let_num_drc_free",
    ];

    pub struct Generator {
        pub letters: bool,
        pub u_letters: bool,
        pub numbs: bool,
        pub spec_symbs: bool,
        pub let_num_drc_free: bool,
        pub cursor_position: usize,
        pub field_position: String,
        pub pwd_len: String,
        pub min_pwd_len: u32,
        pub max_pwd_len: u32,
        pub pwd: String,
        pub lang: String,
        pub errors: (String, String),
    }

    impl Generator {
        pub fn default() -> Generator {
            Generator {
                letters: false,
                u_letters: false,
                numbs: false,
                spec_symbs: true,
                let_num_drc_free: true,
                cursor_position: 1,
                field_position: "pwd_len".to_string(),
                pwd_len: "8".to_string(),
                min_pwd_len: 4,
                max_pwd_len: 10000,
                pwd: "".to_string(),
                lang: "en".to_string(),
                errors: ("".to_string(), "".to_string()),
            }
        }

        pub fn get(&self, field_string: &str) -> bool {
            match field_string {
                "letters" => self.letters.clone(),
                "u_letters" => self.u_letters.clone(),
                "numbs" => self.numbs.clone(),
                "spec_symbs" => self.spec_symbs.clone(),
                "let_num_drc_free" => self.let_num_drc_free.clone(),
                _ => true,
            }
        }

        pub fn set(&mut self, field_string: &str, new_val: bool) {
            match field_string {
                "letters" => self.letters = new_val,
                "u_letters" => self.u_letters = new_val,
                "numbs" => self.numbs = new_val,
                "spec_symbs" => self.spec_symbs = new_val,
                "let_num_drc_free" => self.let_num_drc_free = new_val,
                _ => {}
            }
        }

        pub fn switch_lang(&mut self) {
            if self.lang == "en" {
                self.lang = "ru".to_string();
            } else {
                self.lang = "en".to_string();
            }
        }



        /*pub fn submit_to_pwd(&mut self) {
            if self.is_valid_user_input() {
                let mut pwd = self.generate_pass();
                while !self.is_valid_pwd_by_consist(pwd.clone()) {
                    pwd = self.generate_pass();
                }
                self.pwd = pwd;

                if cfg!(unix) {
                    let pipe = Command::new("echo")
                        .arg("-n")
                        .arg(self.pwd.clone())
                        .stdout(Stdio::piped())
                        .spawn();
                    if let Err(_err) = &pipe {
                        self.errors = (
                            "echo error by copy to clipbord!".to_string(),
                            "При вставке в буфер обмена произошла ошибка echo!".to_string(),
                        );
                    } else {
                        let pipe_out = pipe.unwrap().stdout.take().expect("Failed to take pipe stdout!");
                        let out = Command::new("xclip")
                            .arg("-selection")
                            .arg("clipboard")
                            .stdin(pipe_out)
                            .spawn();
                        if let Err(_err) = &out {
                            self.errors = (
                                "\'xclip\' packet needed for copy to clipbord!".to_string(),
                                "Для вставки в буфер обмена установите пакет \'xclip\'!".to_string(),
                            );
                        } else {
                            let owait = out.unwrap().wait();
                            if let Err(_err) = &owait {
                                self.errors = (
                                    "Failed to run xclip!".to_string(),
                                    "Failed to run xclip!".to_string(),
                                );
                                owait.unwrap();
                            }
                        }
                    }
                } else {
                    let clipboard = Clipboard::new();
                    if let Err(_err) = &clipboard {
                        self.errors = (
                            "Copy to clipboard error!".to_string(),
                            "Ошибка копирования в буфер обмена!".to_string(),
                        );
                    } else {
                        let clip = clipboard
                            .unwrap()
                            .set_text(self.pwd.clone());
                        if let Err(_err) = &clip {
                            self.errors = (
                                "Copy to clipboard error!".to_string(),
                                "Ошибка копирования в буфер обмена!".to_string(),
                            );
                        } else {
                            clip.unwrap();
                        }
                    }
                }
            } else {
                self.cursor_position = 1;
                self.field_position = "pwd_len".parse().unwrap();
                self.pwd_len = "8".parse().unwrap();
            }
        }*/

        fn is_valid_user_input(&self) -> bool {
            let parse_res = self.pwd_len.parse::<u32>();
            match parse_res {
                Ok(val) => {
                    if val < self.min_pwd_len || val > self.max_pwd_len {
                        false
                    } else {
                        true
                    }
                }
                Err(_err) => false,
            }
        }
    }
}
