pub mod gen_engine {
    use crate::generator::generator::Generator;
    //use rand::{Rng, SeedableRng};
    //use rand_isaac;

    const LETTERS_CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    const U_LETTERS_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const NUMBERS_CHARSET: &[u8] = b"0123456789";
    const SPEC_SYMB_CHARSET: &[u8] = b")([]{}*&^%$#@!~";
    const LET_NUM_DRC_FREE: &[u8] = b"ABCDEFGHJKMNPQRSTUVWXYZabcdefghjkmnpqrstuvwxyz\
                            23456789"; // double readable characters free / без двоякочитаемых символов

    impl Generator {
        pub fn generate_pass(&self) -> String {
            //let mut rng = rand::thread_rng();
            let mut pass_assembly: Vec<&[u8]> = Vec::new();

            if self.let_num_drc_free {
                pass_assembly.push(LET_NUM_DRC_FREE);
            } else {
                if self.letters {
                    pass_assembly.push(LETTERS_CHARSET);
                }
                if self.u_letters {
                    pass_assembly.push(U_LETTERS_CHARSET);
                }
                if self.numbs {
                    pass_assembly.push(NUMBERS_CHARSET);
                }
            }
            if self.spec_symbs {
                pass_assembly.push(SPEC_SYMB_CHARSET);
            }

            let pass_charset: Vec<u8> = pass_assembly.into_iter().flatten().cloned().collect();
            /*(0..self.pwd_len)
                .map(|_| pass_charset[rng.gen_range(0..pass_charset.len())] as char)
                .collect()*/
            //let range: Vec<f64> = (0..pass_charset.len()).map(|num| num as f64).collect();
            /*(0..self.pwd_len)
                .map(|_| pass_charset[SeedableRng::from_seed(&range)])
                .collect()*/

            //let a= get_random_buf().unwrap();
            String::from("bla")
            //pass_charset[a[0]]
        }

        pub fn is_valid_pwd_by_consist(&self, pass: String) -> bool {
            let pwd_in_bytes = pass.clone().into_bytes();

            let check_to_available_for = |symbols: &[u8]| -> bool {
                let mut res = false;
                for ch in &pwd_in_bytes {
                    if symbols.contains(&ch) {
                        res = true;
                        break;
                    }
                }
                res
            };

            if self.letters || self.let_num_drc_free {
                if !check_to_available_for(LETTERS_CHARSET) {
                    return false;
                }
            }
            if self.u_letters || self.let_num_drc_free {
                if !check_to_available_for(U_LETTERS_CHARSET) {
                    return false;
                }
            }
            if self.numbs || self.let_num_drc_free {
                if !check_to_available_for(NUMBERS_CHARSET) {
                    return false;
                }
            }
            if self.spec_symbs {
                if !check_to_available_for(SPEC_SYMB_CHARSET) {
                    return false;
                }
            }
            true
        }
    }
}
