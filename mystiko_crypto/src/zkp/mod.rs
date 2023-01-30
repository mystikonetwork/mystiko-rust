mod compute_witness;
mod generate_proof;
mod utils;

pub mod prove;
pub mod verify;

#[cfg(test)]
mod tests {
    use crate::zkp::prove::prove_by_file;
    use crate::zkp::verify::verify_by_file;

    #[test]
    fn test_prove() {
        let arr = ("1", "0", "1");
        let args = serde_json::to_string(&arr).unwrap();
        let witness = prove_by_file(
            "./abc/abc/tests/files/program",
            "./src/zkp/tests/files/abi.json",
            "./src/zkp/tests/files/proving.key",
            &args,
        );
        assert!(witness.is_err());

        let witness = prove_by_file(
            "./src/zkp/tests/files/program",
            "./abc/abc/tests/files/abi.json",
            "./src/zkp/tests/files/proving.key",
            &args,
        );
        assert!(witness.is_err());
    }

    #[test]
    fn test_prove_and_verify() {
        // let arr = ('1', json!([[[false]]]), "1");
        let arr = ("1", "0", "1");
        let args = serde_json::to_string(&arr).unwrap();
        let proof = prove_by_file(
            "./src/zkp/tests/files/program",
            "./src/zkp/tests/files/abi.json",
            "./src/zkp/tests/files/proving.key",
            &args,
        )
        .unwrap();

        let result = verify_by_file(proof, "./src/zkp/tests/files/verification.key").unwrap();
        assert!(result);
    }
}
