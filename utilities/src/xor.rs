mod solution {
    use crate::to_ascii;
    use hex_to_base64::HexToBase64;

    #[allow(dead_code)]
    pub fn solution(a: &str, b: u32) -> String {
        // Steps to xor a and b.
        // 1. Convert each character in `a` to its ASCII or unicode representation.
        // 2 Convert each unicode value to decimal and concatenate the result.
        // 3. Xor a ^ b

        // If size of binary is less than 8 (bits), pad 0's to its MSBs.
        let pad_binary = |b: String| -> String {
            let rem = 8 - b.len();
            return format!("{}{}", "0".repeat(rem), &b);
        };

        let a_binary: Vec<String> = a
            .chars()
            .collect::<Vec<_>>()
            .into_iter()
            .map(|e| e as u8)
            .map(|e_u8| HexToBase64::decimal_to_binary(e_u8 as u32))
            .map(|b| pad_binary(b))
            .collect();

        let b_binary = pad_binary(HexToBase64::decimal_to_binary(b));

        let xor_result = a_binary
            .into_iter()
            .map(|a| xor(&a, &b_binary))
            .collect::<String>();

        to_ascii::binary_to_ascii::to_ascii(&xor_result)
    }

    #[allow(dead_code)]
    pub fn xor(a: &str, b: &str) -> String {
        a.chars()
            .zip(b.chars())
            .map(|(bit_a, bit_b)| {
                if bit_a != bit_b {
                    '1'.to_string()
                } else {
                    '0'.to_string()
                }
            })
            .collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::to_ascii::binary_to_ascii;
    use hex_to_base64::HexToBase64;

    #[test]
    fn xor_a_b() {
        let result = solution::solution("label", 13);
        assert_eq!(result, "aloha")
    }

    #[test]
    fn xor_properties() {
        let key1 = "a6c8b6733c9b22de7bc0253266a3867df55acde8635e19c73313";
        let key2_xor_key1 = "37dcb292030faa90d07eec17e3b1c6d8daf94c35d4c9191a5e1e";
        
        let key2_xor_key3 = "c1545756687e7573db23aa1c3452a098b71a7fbf0fddddde5fc1";
        let flag_xor_key1_xor_key3_xor_key2 =
            "04ee9855208a2cd59091d04767ae47963170d1660df7f56f5faf";

        let key1_binary = HexToBase64::hex_to_binary(key1);
        let key2_xor_key3_binary = HexToBase64::hex_to_binary(key2_xor_key3);
        let flag_xor_key1_xor_key3_xor_key2_binary =
            HexToBase64::hex_to_binary(flag_xor_key1_xor_key3_xor_key2);

        let flag = solution::xor(
            &solution::xor(&key1_binary, &key2_xor_key3_binary),
            &flag_xor_key1_xor_key3_xor_key2_binary,
        );

        assert_eq!(
            binary_to_ascii::to_ascii(&flag),
            "crypto{x0r_i5_ass0c1at1v3}"
        );
    }
}
