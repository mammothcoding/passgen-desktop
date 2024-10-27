pub mod text_processor {
    use crate::generator::generator::Generator;

    pub(crate) const EN: [(&str, &str); 14] = [
        ("pass_len", "Password length:"),
        ("inc_lcase", "include lowercase letters"),
        ("inc_cap", "include capital letters"),
        ("inc_num", "include numbers"),
        ("inc_ss", "include special symbols"),
        ("inc_all_ex", "all nums & letters exclude \"0oOiIlL1\""),
        ("but_gen_gen", "GENERATE\n"),
        ("but_gen_press", "(press Enter)"),
        ("but_ttip", "Generate a password"),
        ("made_rust", "Made with Rust"),
        ("to_clip", "this password was copied to clipboard"),
        ("homepage", "Homepage: https://github.com/mammothcoding"),
        ("lang_ttip", "Interface language switcher"),
        ("but_about", "About program"),
    ];

    pub(crate) const RU: [(&str, &str); 14] = [
        ("pass_len", "Длина пароля:"),
        ("inc_lcase", "включая маленькие буквы"),
        ("inc_cap", "включая заглавные буквы"),
        ("inc_num", "включая цифры"),
        ("inc_ss", "включая спец. символы"),
        ("inc_all_ex", "все буквы и цифры кроме \"0oOiIlL1\""),
        ("but_gen_gen", "ГЕНЕРИТЬ\n"),
        ("but_gen_press", "(нажмите Enter)"),
        ("but_ttip", "Сгенерировать пароль"),
        ("made_rust", "Создано на языке RUST"),
        ("to_clip", "пароль был скопирован в буфер обмена"),
        ("homepage", "Домашняя страница: https://github.com/mammothcoding"),
        ("lang_ttip", "Переключатель языка интерфейса"),
        ("but_about", "О программе"),
    ];

    impl Generator {
        pub fn get_lang_text(&self, text_id: &str) -> String
        {
            let lang_name: String = self.lang.clone().to_owned();
            let lang_name_slice: &str = &lang_name[..];

            let res = match lang_name_slice {
                "ru" => self.ru_texts.get(text_id),
                _ => self.en_texts.get(text_id),
            };

            res.unwrap().to_string()
        }
    }
}
