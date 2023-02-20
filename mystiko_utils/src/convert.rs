use std::ops::Div;
use std::str::FromStr;
use rust_decimal::Decimal;
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};

pub fn from_decimals<T>(bn: T, decimals: Option<u32>) -> f64
    where T: ToString
{
    let decimals = match decimals {
        None => { 18 }
        Some(value) => { value }
    };
    let factor = Decimal::from_u64(10u64.pow(decimals)).unwrap();
    let result = Decimal::from_str(&bn.to_string()).unwrap();
    result.div(factor).to_f64().unwrap()
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use num_bigint::BigInt;
    use crate::convert::from_decimals;

    #[test]
    fn test_from_decimals() {
        assert_eq!(
            from_decimals(
                BigInt::from_str("1").unwrap(),
                Some(4),
            ), 0.0001
        );
        assert_eq!(
            from_decimals(
                BigInt::from_str("1000000000000000000").unwrap(),
                None,
            ), 1f64
        );
        assert_eq!(
            from_decimals(
                "1000000000000000000",
                None,
            ), 1f64
        );
    }
}