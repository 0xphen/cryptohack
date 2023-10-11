#[allow(dead_code)]
pub fn solution(a: &str, b: u32) -> String {
    let a_as_bytes = a.as_bytes();
    let b_as_bytes = b.to_be_bytes();

    a_as_bytes
        .iter()
        .map(|&byte| {
            // XOR each byte of the string with each byte of the u32 value
            let x = byte ^ b_as_bytes[0] ^ b_as_bytes[1] ^ b_as_bytes[2] ^ b_as_bytes[3];
            (x as char).to_string()
        })
        .collect()
}

#[allow(dead_code)]
pub fn xor(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {
    assert_eq!(
        a.len(),
        b.len(),
        "Both byte vectors should have equal length"
    );
    a.iter().zip(b.iter()).map(|(&x, &y)| x ^ y).collect()
}

pub fn xor_u32(a: u8, b: u8) -> u8 {
    a ^ b
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hex_ascii_base64::bytes_to_ascii;
    use utils::hex_to_bytes;

    #[test]
    fn xor_a_b() {
        let result = solution("label", 13);
        assert_eq!(result, "aloha")
    }

    #[test]
    fn xor_properties() {
        let k1 = "a6c8b6733c9b22de7bc0253266a3867df55acde8635e19c73313";
        let _k2_xor_k1 = "37dcb292030faa90d07eec17e3b1c6d8daf94c35d4c9191a5e1e";

        let k2_xor_k3 = "c1545756687e7573db23aa1c3452a098b71a7fbf0fddddde5fc1";
        let flag_xor_k1_xor_k3_xor_k2 = "04ee9855208a2cd59091d04767ae47963170d1660df7f56f5faf";

        let k1_bytes = hex_to_bytes(k1);
        let k2_xor_k3_bytes = hex_to_bytes(k2_xor_k3);
        let flag_xor_k1_xor_k3_xor_k2_bytes = hex_to_bytes(flag_xor_k1_xor_k3_xor_k2);

        // We know from the self inverse property of xor, a xor a = 0.
        // Hence,  we can find unknown keys by xoring the already known keys.
        // f ^ k1 ^ k2 ^ k3 = x  <=>  f ^ k1 ^ k2_xor_k3 = x
        // We extract the unknown `f`: f = x ^ k1 ^ k2_xor_k3

        let flag = xor(
            xor(k1_bytes, k2_xor_k3_bytes),
            flag_xor_k1_xor_k3_xor_k2_bytes,
        );

        assert_eq!(bytes_to_ascii(flag), "crypto{x0r_i5_ass0c1at1v3}");
    }
}
