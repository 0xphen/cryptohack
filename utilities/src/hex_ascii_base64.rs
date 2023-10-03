pub mod hex_to_ascii {
    use hex_to_base64::HexToBase64;

    /// Converts a hexadecimal character to it's corresponding
    /// hex value.
    pub fn hex_char_to_decimal(hex_char: char) -> Option<u8> {
        match hex_char {
            '0'..='9' => Some(hex_char as u8 - '0' as u8),
            'A'..='F' => Some(hex_char as u8 - 'A' as u8 + 10),
            'a'..='f' => Some(hex_char as u8 - 'a' as u8 + 10),
            _ => None, // Invalid hex character
        }
    }

    pub fn hex_pair_to_decimal(hex_pair: &str) -> Option<u8> {
        if let Ok(decimal_value) = u8::from_str_radix(hex_pair, 16) {
            Some(decimal_value)
        } else {
            None // Invalid hex pair
        }
    }

    pub fn to_ascii(hex: &str) -> String {
        let binary = HexToBase64::hex_to_binary(hex);

        let ascii_string: String = binary
            .chars()
            .collect::<Vec<_>>()
            .chunks(8)
            .map(|chunk| chunk.iter().collect::<String>())
            .map(|b| HexToBase64::binary_to_decimal(&b) as u8)
            .map(|dec| char::from(dec))
            .collect();

        ascii_string
    }

    pub fn hex_to_base64(hex: &str) -> String {
        HexToBase64::hex_to_base64(hex)
    }
}

pub mod binary_to_ascii {
    use hex_to_base64::HexToBase64;

    pub fn to_ascii(b: &str) -> String {
        let ascii: String = b
            .chars()
            .collect::<Vec<_>>()
            .chunks(8)
            .map(|chunk| chunk.iter().collect::<String>())
            .map(|b| {
                let dec = HexToBase64::binary_to_decimal(&b);
                (dec as u8) as char
            })
            .collect();

        ascii
    }
}

pub mod str_to_binary {
    pub fn to_binary(s: &str) -> String {
        s.chars()
            .collect::<Vec<_>>()
            .into_iter()
            .map(|c| format!("{:08b}", c as u8))
            .collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_convert_hex_to_decimal() {
        let mut result = hex_to_ascii::hex_pair_to_decimal("AF");
        assert_eq!(result.unwrap(), 175);

        result = hex_to_ascii::hex_char_to_decimal('A');
        assert_eq!(result.unwrap(), 10);
    }

    #[test]
    fn can_convert_bytes_to_ascii_string() {
        let ascii_string = hex_to_ascii::to_ascii("63727970746f7b596f755f77696c6c5f62655f776f726b696e675f776974685f6865785f737472696e67735f615f6c6f747d");

        assert_eq!(
            ascii_string,
            "crypto{You_will_be_working_with_hex_strings_a_lot}"
        )
    }

    #[test]
    fn can_convert_hex_to_base64() {
        let base64_string =
            hex_to_ascii::hex_to_base64("72bca9b68fc16ac7beeb8f849dca1d8a783e8acf9679bf9269f7bf");

        assert_eq!(base64_string, "crypto/Base+64+Encoding+is+Web+Safe/")
    }

    #[test]
    fn can_convert_binary_to_ascii() {
        let result = binary_to_ascii::to_ascii("010000010100001101000101");
        assert_eq!(result, "ACE");
    }

    #[test]
    fn can_convert_str_to_binary() {
        let result = str_to_binary::to_binary("hello world");

        assert_eq!(result, "0110100001100101011011000110110001101111001000000111011101101111011100100110110001100100")
    }
}
