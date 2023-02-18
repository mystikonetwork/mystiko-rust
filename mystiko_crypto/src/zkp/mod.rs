mod compute_witness;
mod generate_proof;
mod utils;

pub mod prove;
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

        let proof_json: serde_json::Value = serde_json::from_str(proof.as_str()).unwrap();
        let inputs = proof_json.get("inputs").unwrap();
        let mut inputs: Vec<_> = inputs
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap())
            .collect();
        inputs[2] = "0x0000000000000000000000000000000000000000000000000000000000000004";

        let mut proof_json: serde_json::Value = serde_json::from_str(proof.as_str()).unwrap();
        if let Some(map) = proof_json.as_object_mut() {
            map.insert(
                "inputs".to_owned(),
                serde_json::Value::Array(
                    inputs.iter().map(|s| serde_json::Value::from(*s)).collect(),
                ),
            );
        }

        let result = verify_by_file(
            proof_json.to_string(),
            "./src/zkp/tests/files/verification.key",
        )
        .unwrap();
        assert!(!result);
    }
}
