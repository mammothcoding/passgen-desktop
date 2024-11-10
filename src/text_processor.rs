pub mod text_processor {
    use crate::generator::generator::Generator;

    pub(crate) const EN: [(&str, &str); 20] = [
        ("pass_len", "Password length:"),
        ("inc_lcase", "include lowercase letters"),
        ("inc_cap", "include capital letters"),
        ("inc_num", "include numbers"),
        ("inc_ss", "include special symbols"),
        ("inc_conven", "strong & usability password"),
        ("inc_conven_ttip", "The first position in the password is a capital or small letter.\nThe last position is the symbol.\nExcluded ambiguous characters \"0oOiIlL1\"."),
        ("but_gen_gen", "GENERATE\n"),
        ("but_gen_press", "(press Enter)"),
        ("but_ttip", "Generate a password"),
        ("made_rust", "Dev on Rust"),
        ("to_clip", "this password was copied to clipboard"),
        ("lang_ttip", "Interface language switcher"),
        ("but_about", "About program"),
        ("cls", "Clear password"),
        ("abo_win_name", "About programm"),
        ("abo_desc", "A simple password generator"),
        ("abo_version", "Version"),
        ("abo_license", "License"),
        ("abo_homepage", "Homepage"),
    ];

    pub(crate) const RU: [(&str, &str); 20] = [
        ("pass_len", "Длина пароля:"),
        ("inc_lcase", "включая маленькие буквы"),
        ("inc_cap", "включая заглавные буквы"),
        ("inc_num", "включая цифры"),
        ("inc_ss", "включая спец. символы"),
        ("inc_conven", "сильный и удобный пароль"),
        ("inc_conven_ttip", "Первая позиция в пароле - большая или малая буква.\nПоследняя позиция - символ.\nИсключены двояко читаемые символы \"0oOiIlL1\"."),
        ("but_gen_gen", "ГЕНЕРИТЬ\n"),
        ("but_gen_press", "(нажмите Enter)"),
        ("but_ttip", "Сгенерировать пароль"),
        ("made_rust", "Создано на языке Rust"),
        ("to_clip", "пароль был скопирован в буфер обмена"),
        ("lang_ttip", "Переключатель языка интерфейса"),
        ("but_about", "О программе"),
        ("cls", "Очистить пароль"),
        ("abo_win_name", "О программе"),
        ("abo_desc", "Простой генератор паролей"),
        ("abo_version", "Версия"),
        ("abo_license", "Лицензия"),
        ("abo_homepage", "Домашняя страница"),
    ];

    impl Generator {
        pub fn get_lang_text(&self, text_id: &str) -> String {
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
