mod compute_witness;
mod generate_proof;
mod utils;

pub mod prove;
pub mod types;
pub mod verify;

#[cfg(test)]
mod tests {
    use crate::zkp::prove::{prove, prove_by_file};
    use crate::zkp::utils::load_file;
    use crate::zkp::verify::{verify, verify_by_file};

    #[test]
    fn test_prove_and_verify() {
        let arr = ("1", "0", "1");
        let args = serde_json::to_string(&arr).unwrap();

        let prog = load_file("./src/zkp/tests/files/program").unwrap();
        let abi_spec = load_file("./src/zkp/tests/files/abi.json").unwrap();
        let pk = load_file("./src/zkp/tests/files/proving.key").unwrap();
        let vk = load_file("./src/zkp/tests/files/verification.key").unwrap();

        let proof = prove(prog.as_slice(), abi_spec.as_slice(), pk.as_slice(), &args).unwrap();
        let result = verify(proof, vk.as_slice()).unwrap();
        assert!(result);
    }

    #[test]
    fn test_prove_and_verify_by_file() {
        // let arr = ('1', json!([[[false]]]), "1");
        let arr = ("3", "2", "5");
        let args = serde_json::to_string(&arr).unwrap();
        println!("args {:?}", arr);
        let proof = prove_by_file(
            "./src/zkp/tests/files/program",
            "./src/zkp/tests/files/abi.json",
            "./src/zkp/tests/files/proving.key",
            &args,
        )
        .unwrap();

        let result =
            verify_by_file(proof.clone(), "./src/zkp/tests/files/verification.key").unwrap();
        assert!(result);

        let mut modify_proof = proof.clone();
        modify_proof.inputs[2] =
            "0x0000000000000000000000000000000000000000000000000000000000000004".to_string();
        let result =
            verify_by_file(modify_proof, "./src/zkp/tests/files/verification.key").unwrap();
        assert!(!result);
    }
}
