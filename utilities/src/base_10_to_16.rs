pub mod base_10_to_16 {
    use std::ops::Div;

    use num_bigint::BigUint;
    use num_traits::Zero;

    pub fn into_bn(bn: BigUint) -> String {
        let is_zero = |a: &BigUint, b: &BigUint| -> bool { a.div(b) == Zero::zero() };

        let mut hex = String::new();
        let mut a = bn;
        let base_size: BigUint = BigUint::from(16_u32);

        loop {
            let val = a.modpow(&BigUint::from(1_u32), &base_size);
            let hex_val = format!("{:X}", val);
            hex = format!("{}{}", hex_val, hex);

            if is_zero(&a, &base_size) {
                break;
            }

            a = a.div(base_size.clone());
        }
        hex
    }

    pub fn into(dec: u32) -> String {
        into_bn(BigUint::from(dec as u32))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use num_bigint::BigUint;
    use num_traits::Num;

    #[test]
    fn base10_to_16() {
        let result = base_10_to_16::into(79);
        assert_eq!(result, "4F");

        let bn = BigUint::from_str_radix(
            "11515195063862318899931685488813747395775516287289682636499965282714637259206269",
            10,
        )
        .expect("Failed to convert to BigUint");

        let result = base_10_to_16::into_bn(bn);
        assert_eq!(
            result,
            "63727970746F7B336E633064316E365F346C6C5F3768335F7734795F6430776E7D"
        );
    }
}
