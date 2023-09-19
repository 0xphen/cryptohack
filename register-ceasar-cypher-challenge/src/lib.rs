pub struct CeasarCipher;

#[derive(Debug)]
pub struct DecodedText(String, u32);

impl DecodedText {
    pub fn new(text: String, key: u32) -> Self {
        DecodedText(text, key)
    }
}

impl CeasarCipher {
    pub fn find_letter_by_index(index: u32) -> Option<char> {
        if index < 26 {
            Some((b'A' + index as u8) as char)
        } else {
            return None;
        }
    }

    pub fn find_index_by_letter(letter: char) -> Option<usize> {
        if letter.is_ascii_alphabetic() {
            let lowercase_letter = letter.to_ascii_lowercase();
            if lowercase_letter >= 'a' && lowercase_letter <= 'z' {
                Some((lowercase_letter as u8 - b'a') as usize)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn decode_cipher_text(cipher_text: &str, key: u32) -> String {
        let mut decoded_text = String::new();
        for c in cipher_text.chars() {
            let c_index = match CeasarCipher::find_index_by_letter(c) {
                Some(i) => i as u32,
                None => panic!("Unknown character `{:?}`", c),
            };

            let decoded_c_index = ((c_index as i32 - key as i32) % 26 + 26) % 26;
            let decoded_c = CeasarCipher::find_letter_by_index(decoded_c_index as u32).unwrap();

            decoded_text.push_str(decoded_c.to_string().as_str());
        }

        return decoded_text;
    }

    pub fn decode_brute_force(cipher_text: &str) -> Vec<DecodedText> {
        let mut decoded_texts: Vec<DecodedText> = vec![];
        for i in 0..=25 {
            let decoded_text = CeasarCipher::decode_cipher_text(cipher_text, i);
            decoded_texts.push(DecodedText::new(decoded_text, i));
        }

        return decoded_texts;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_cipher_text() {
        let decoded_text = CeasarCipher::decode_cipher_text("QHVRDFDGHPB", 3);

        assert_eq!(decoded_text, "NESOACADEMY");
    }

    #[test]
    fn solve_register_challenge() {
        let decoded_texts = CeasarCipher::decode_brute_force("BZQITBZIDMTOMVQCANIJZQK");
        println!("DECODED TEXTS: {:?}", decoded_texts);

        // [DecodedText("BZQITBZIDMTOMVQCANIJZQK", 0), DecodedText("AYPHSAYHCLSNLUPBZMHIYPJ", 1), DecodedText("ZXOGRZXGBKRMKTOAYLGHXOI", 2), DecodedText("YWNFQYWFAJQLJSNZXKFGWNH", 3), DecodedText("XVMEPXVEZIPKIRMYWJEFVMG", 4), DecodedText("WULDOWUDYHOJHQLXVIDEULF", 5), DecodedText("VTKCNVTCXGNIGPKWUHCDTKE", 6), DecodedText("USJBMUSBWFMHFOJVTGBCSJD", 7), DecodedText("TRIALTRAVELGENIUSFABRIC", 8), DecodedText("SQHZKSQZUDKFDMHTREZAQHB", 9), DecodedText("RPGYJRPYTCJECLGSQDYZPGA", 10), DecodedText("QOFXIQOXSBIDBKFRPCXYOFZ", 11), DecodedText("PNEWHPNWRAHCAJEQOBWXNEY", 12), DecodedText("OMDVGOMVQZGBZIDPNAVWMDX", 13), DecodedText("NLCUFNLUPYFAYHCOMZUVLCW", 14), DecodedText("MKBTEMKTOXEZXGBNLYTUKBV", 15), DecodedText("LJASDLJSNWDYWFAMKXSTJAU", 16), DecodedText("KIZRCKIRMVCXVEZLJWRSIZT", 17), DecodedText("JHYQBJHQLUBWUDYKIVQRHYS", 18), DecodedText("IGXPAIGPKTAVTCXJHUPQGXR", 19), DecodedText("HFWOZHFOJSZUSBWIGTOPFWQ", 20), DecodedText("GEVNYGENIRYTRAVHFSNOEVP", 21), DecodedText("FDUMXFDMHQXSQZUGERMNDUO", 22), DecodedText("ECTLWECLGPWRPYTFDQLMCTN", 23), DecodedText("DBSKVDBKFOVQOXSECPKLBSM", 24), DecodedText("CARJUCAJENUPNWRDBOJKARL", 25)]

        // Solution to challenge is:  DecodedText("TRIALTRAVELGENIUSFABRIC", 8)
    }
}
