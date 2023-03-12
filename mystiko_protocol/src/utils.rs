use num_bigint::BigInt;

pub fn to_string_slice(v: &[BigInt]) -> Vec<String> {
    v.iter().map(|n| n.to_string()).collect()
}
