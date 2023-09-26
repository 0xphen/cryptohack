mod xor {
    use crate::to_ascii;
    use hex_to_base64::HexToBase64;

    pub fn xor_str(a: &str, b: u32) -> String {
        // Steps to xor a and b.
        // 1. Convert each character in `a` to its ASCII or unioode representation.
        // 2 Convert each unicode value to decimal and concatenate the result.
        // 3. Xor a ^ b

        // If size of binary `b` is less than 8 (bits), pad 0's to its MSBs.
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

        let b_binary = HexToBase64::decimal_to_binary(b);
        let get_b_binary = |size_a: isize| -> String {
            let rem = size_a - b_binary.len() as isize;
            if rem > 0 {
                return format!("{}{}", "0".repeat(rem as usize), b_binary);
            }

            return b_binary.clone();
        };

        let xor_result: String = a_binary
            .into_iter()
            .map(|a| {
                a.as_str()
                    .chars()
                    .zip(get_b_binary(a.len() as isize).as_str().chars())
                    .map(|(bit_a, bit_b)| {
                        if bit_a != bit_b {
                            '1'.to_string()
                        } else {
                            '0'.to_string()
                        }
                    })
                    .collect::<String>()
            })
            .collect();

        to_ascii::binary_to_ascii::to_ascii(&xor_result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xor_a_b() {
        let result = xor::xor_str("label", 13);
        assert_eq!(result, "aloha")
    }
}
