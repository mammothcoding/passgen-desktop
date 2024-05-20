pub mod text_processor {
    use std::collections::HashMap;
    use crate::generator::generator::Generator;

    /*pub struct TextProcessor {
        pub pass_len: str,
        pub inc_lcase: str,
        pub inc_cap: str,
    }
*/


    let en_text: HashMap<&str, &str> = HashMap::from([
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
    ]);

    const RU: HashMap<&str, &str> = HashMap::from([
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
    ]);

    impl Generator {
        pub fn get_text(&self, text_id: &str) -> String
        {
            let en = "en".to_string();
            let ru = "ru".to_string();

            let res = match &self.lang {
                en => EN.get(text_id),
                ru => RU.get(text_id),
            };
            return res.unwrap().to_string();
        }
    }
}
