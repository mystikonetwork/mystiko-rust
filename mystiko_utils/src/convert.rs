use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use rust_decimal::Decimal;
use std::ops::Div;
use std::str::FromStr;

pub fn from_decimals<T>(bn: T, decimals: Option<u32>) -> f64
where
    T: ToString,
{
    let decimals = decimals.unwrap_or(18);
    let factor = Decimal::from_u64(10u64.pow(decimals)).unwrap();
    let result = Decimal::from_str(&bn.to_string()).unwrap();
    result.div(factor).to_f64().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::convert::from_decimals;
    use num_bigint::BigInt;
    use std::str::FromStr;

    #[test]
    fn test_from_decimals() {
        assert_eq!(
            from_decimals(BigInt::from_str("1").unwrap(), Some(4),),
            0.0001
        );
        assert_eq!(
            from_decimals(BigInt::from_str("1000000000000000000").unwrap(), None,),
            1f64
        );
        assert_eq!(from_decimals("1000000000000000000", None,), 1f64);
    }
}
