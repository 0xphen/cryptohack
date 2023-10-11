use utils::{bytes_to_base64, hex_to_bytes};

pub fn hex_to_ascii(hex: &str) -> String {
    String::from_utf8(hex_to_bytes(hex)).expect("Failed to convert bytes to ASCII")
}

pub fn hex_to_base64(hex: &str) -> String {
    bytes_to_base64(hex_to_bytes(hex))
}

pub fn bytes_to_ascii(bytes: Vec<u8>) -> String {
  String::from_utf8_lossy(&bytes).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_convert_hex_to_ascii_string() {
        let ascii_string = hex_to_ascii("63727970746f7b596f755f77696c6c5f62655f776f726b696e675f776974685f6865785f737472696e67735f615f6c6f747d");

        assert_eq!(
            ascii_string,
            "crypto{You_will_be_working_with_hex_strings_a_lot}"
        )
    }

    #[test]
    fn can_convert_hex_to_base64() {
        let base64_string = hex_to_base64("72bca9b68fc16ac7beeb8f849dca1d8a783e8acf9679bf9269f7bf");

        assert_eq!(base64_string, "crypto/Base+64+Encoding+is+Web+Safe/")
    }

    #[test]
    fn can_convert_bytes_to_ascii() {
        let result = bytes_to_ascii(vec![65, 67, 69]);
        assert_eq!(result, "ACE");
    }
}
