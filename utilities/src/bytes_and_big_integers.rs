pub mod int_to_msg {
    use num_bigint::BigUint;

    use crate::{base_10_to_16::base_10_to_16, hex_ascii_base64::hex_to_ascii};

    // Steps to convert an integer to a message.
    // 1. Convert integer (BigNumber) to Hexadecimal.
    // 2. Split hex into pairs of 2 digits/characters and convert each pair to its ASCII represemtation.
    // 3. Convert ASCII to string.
    pub fn into(value: BigUint) -> String {
        let hex = base_10_to_16::into_bn(value);
        hex_to_ascii(&hex)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        use num_bigint::BigUint;
        use num_traits::Num;

        #[test]
        fn convert_int_to_message() {
            let bn = BigUint::from_str_radix(
                "11515195063862318899931685488813747395775516287289682636499965282714637259206269",
                10,
            )
            .expect("Failed to convert to BigUint");

            let result = into(bn);
            assert_eq!(result, "crypto{3nc0d1n6_4ll_7h3_w4y_d0wn}")
        }
    }
}
