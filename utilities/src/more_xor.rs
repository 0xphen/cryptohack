// https://cryptohack.org/courses/intro/xorkey1/
pub mod solution {
    use hex_to_base64::HexToBase64;

    use crate::hex_ascii_base64::{binary_to_ascii, str_to_binary};
    use crate::xor_starter::solution::xor;

    const FLAG_FORMAT: &str = "crypto{y0ur_f1rst_fl4g}";
    const CIPHER_TEXT: &str =
        "0e0b213f26041e480b26217f27342e175d0e070a3c5b103e2526217f27342e175d0e077e263451150104";

    pub fn solution() {
        let crypto_binary = str_to_binary::to_binary("crypto{");
        let ct_binary = HexToBase64::hex_to_binary(CIPHER_TEXT);

        let mut key = crypto_binary
            .chars()
            .collect::<Vec<_>>()
            .chunks(8)
            .zip(
                (&ct_binary[0..crypto_binary.len()])
                    .chars()
                    .collect::<Vec<_>>()
                    .chunks(8),
            )
            .map(|(a, b)| {
                let a = a.iter().collect::<String>();
                let b = b.iter().collect::<String>();
                xor(&a, &b)
            })
            .collect::<String>();

        let mut ascii_key = binary_to_ascii::to_ascii(&key);
        ascii_key = format!("{}y", ascii_key);
    }
}
