use num_bigint::BigUint;

lazy_static! {
    pub static ref FIELD_SIZE: BigUint = BigUint::parse_bytes(
        b"21888242871839275222246405745257275088548364400416034343698204186575808495617",
        10,
    )
    .unwrap();
}
