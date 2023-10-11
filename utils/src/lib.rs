pub fn hex_to_bytes(hex: &str) -> Vec<u8> {
    hex.as_bytes()
        .chunks(2)
        .map(|chunk| {
            let src = std::str::from_utf8(chunk).unwrap();
            u8::from_str_radix(src, 16).expect("Failed to convert hex to bytes")
        })
        .collect()
}

/// Converts a hexadecimal string to its Base64 representation.
///
/// # Arguments
///
/// * `hex` - A string slice that holds the hexadecimal value.
///
/// # Returns
///
/// Returns a `String` containing the Base64 representation of the input hexadecimal.
pub fn bytes_to_base64(bytes: Vec<u8>) -> String {
    const CHARSET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let mut base64 = String::new();

    for chunk in bytes.chunks(3) {
        let chunk = &chunk.iter().map(|&b| b as u32).collect::<Vec<u32>>();

        let n = match chunk.len() {
            3 => chunk[0] << 16 | chunk[1] << 8 | chunk[2],
            2 => chunk[0] << 16 | chunk[1] << 8,
            1 => chunk[0] << 16,
            _ => continue,
        };

        base64.push(CHARSET[((n >> 18) & 63) as usize] as char);
        base64.push(CHARSET[((n >> 12) & 63) as usize] as char);

        if chunk.len() > 1 {
            base64.push(CHARSET[((n >> 6) & 63) as usize] as char);
        } else {
            base64.push('=');
        }

        if chunk.len() > 2 {
            base64.push(CHARSET[(n & 63) as usize] as char);
        } else {
            base64.push('=');
        }
    }

    base64
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn convert_hex_to_bytes() {
        let result = hex_to_bytes("ff021ea");
        assert_eq!(result, vec!(255, 2, 30, 10));
    }

    #[test]
    fn test_convert_hex_to_base64() {
        let base64 = bytes_to_base64(hex_to_bytes("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"));

        assert_eq!(
            base64,
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
        );
    }
}
