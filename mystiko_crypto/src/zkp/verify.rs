use crate::error::ZkpError;
use crate::zkp::utils::create_file_reader;
use zokrates_bellman::Bellman;
use zokrates_common::helpers::*;
use zokrates_field::{Bn128Field, Field};
use zokrates_proof_systems::*;

pub fn verify_by_file(proof: String, verification_key_path_str: &str) -> Result<bool, ZkpError> {
    let vk_reader = create_file_reader(verification_key_path_str)?;
    let vk = serde_json::from_reader(vk_reader)
        .map_err(|why| ZkpError::ParseError("verification key".to_string(), why.to_string()))?;
    do_verify(proof, vk)
}

pub fn verify(proof: String, verification_key: &[u8]) -> Result<bool, ZkpError> {
    let vk: serde_json::Value = serde_json::from_slice(verification_key)
        .map_err(|why| ZkpError::ParseError("verification key".to_string(), why.to_string()))?;

    do_verify(proof, vk)
}

fn do_verify(proof: String, vk: serde_json::Value) -> Result<bool, ZkpError> {
    let proof_json: serde_json::Value = serde_json::from_str(proof.as_str())
        .map_err(|why| ZkpError::ParseError("proof".to_string(), why.to_string()))?;

    let proof_curve = proof_json
        .get("curve")
        .ok_or_else(|| ZkpError::ProofError("Field `curve` not found in proof".to_string()))?
        .as_str()
        .ok_or_else(|| ZkpError::ProofError("`curve` should be a string".to_string()))?;
    let proof_scheme = proof_json
        .get("scheme")
        .ok_or_else(|| ZkpError::ProofError("Field `scheme` not found in proof".to_string()))?
        .as_str()
        .ok_or_else(|| ZkpError::ProofError("`scheme` should be a string".to_string()))?;
    let vk_curve = vk
        .get("curve")
        .ok_or_else(|| {
            ZkpError::VKError("Field `curve` not found in verification key".to_string())
        })?
        .as_str()
        .ok_or_else(|| ZkpError::VKError("`curve` should be a string".to_string()))?;
    let vk_scheme = vk
        .get("scheme")
        .ok_or_else(|| {
            ZkpError::VKError("Field `scheme` not found in verification key".to_string())
        })?
        .as_str()
        .ok_or_else(|| ZkpError::VKError("`scheme` should be a string".to_string()))?;

    if proof_curve != vk_curve {
        return Err(ZkpError::MismatchError(
            "curve of the proof and the verification mismatch".to_string(),
        ));
    }

    if proof_scheme != vk_scheme {
        return Err(ZkpError::MismatchError(
            "scheme of the proof and the verification mismatch".to_string(),
        ));
    }

    let parameters = match Parameters::try_from((
        BackendParameter::Bellman.to_string().as_str(),
        vk_curve,
        vk_scheme,
    )) {
        Ok(param) => param,
        Err(why) => return Err(ZkpError::MismatchError(why)),
    };

    match parameters {
        Parameters(BackendParameter::Bellman, CurveParameter::Bn128, SchemeParameter::G16) => {
            call_verify::<Bn128Field, G16, Bellman>(vk, proof_json)
        }
        _ => Err(ZkpError::NotSupport),
    }
}

fn call_verify<T: Field, S: Scheme<T>, B: Backend<T, S>>(
    vk: serde_json::Value,
    proof: serde_json::Value,
) -> Result<bool, ZkpError> {
    let vk = serde_json::from_value(vk).map_err(|why| ZkpError::ProofError(why.to_string()))?;
    let proof: Proof<T, S> =
        serde_json::from_value(proof).map_err(|why| ZkpError::VerifyError(why.to_string()))?;

    Ok(B::verify(vk, proof))
}

#[cfg(test)]
mod tests {
    use crate::error::ZkpError;
    use crate::zkp::utils::create_file_reader;
    use crate::zkp::utils::load_file;
    use crate::zkp::verify::{verify, verify_by_file};

    #[test]
    fn test_verify_by_file() {
        let proof =
            create_file_reader("./src/zkp/tests/files/wrong/proof_missing_curve.json").unwrap();
        let proof: serde_json::Value = serde_json::from_reader(proof).unwrap();
        let result = verify_by_file(proof.to_string(), "./src/zkp/tests/files/verification.key");
        assert_eq!(
            result.err().unwrap().name(),
            ZkpError::ProofError(String::from(""))
        );

        let proof =
            create_file_reader("./src/zkp/tests/files/wrong/proof_missing_scheme.json").unwrap();
        let proof: serde_json::Value = serde_json::from_reader(proof).unwrap();
        let result = verify_by_file(proof.to_string(), "./src/zkp/tests/files/verification.key");
        assert_eq!(
            result.err().unwrap().name(),
            ZkpError::ProofError(String::from(""))
        );

        let proof = create_file_reader("./src/zkp/tests/files/proof.json").unwrap();
        let proof: serde_json::Value = serde_json::from_reader(proof).unwrap();
        let result = verify_by_file(
            proof.to_string(),
            "./src/zkp/tests/files/wrong/verification_error.key",
        );
        assert_eq!(
            result.err().unwrap().name(),
            ZkpError::ParseError(String::from(""), String::from(""))
        );

        let proof = create_file_reader("./src/zkp/tests/files/proof.json").unwrap();
        let proof: serde_json::Value = serde_json::from_reader(proof).unwrap();
        let result = verify_by_file(
            proof.to_string(),
            "./src/zkp/tests/files/wrong/verification_missing_curve.key",
        );
        assert_eq!(
            result.err().unwrap().name(),
            ZkpError::VKError(String::from(""))
        );

        let proof = create_file_reader("./src/zkp/tests/files/proof.json").unwrap();
        let proof: serde_json::Value = serde_json::from_reader(proof).unwrap();
        let result = verify_by_file(
            proof.to_string(),
            "./src/zkp/tests/files/wrong/verification_missing_scheme.key",
        );
        assert_eq!(
            result.err().unwrap().name(),
            ZkpError::VKError(String::from(""))
        );

        let proof = create_file_reader("./src/zkp/tests/files/proof.json").unwrap();
        let proof: serde_json::Value = serde_json::from_reader(proof).unwrap();
        let result = verify_by_file(
            proof.to_string(),
            "./src/zkp/tests/files/wrong/verification_gm17.key",
        );
        assert_eq!(
            result.err().unwrap().name(),
            ZkpError::MismatchError(String::from(""))
        );

        let proof = create_file_reader("./src/zkp/tests/files/proof.json").unwrap();
        let proof: serde_json::Value = serde_json::from_reader(proof).unwrap();
        let result = verify_by_file(
            proof.to_string(),
            "./src/zkp/tests/files/wrong/verification_bls12_381.key",
        );
        assert_eq!(
            result.err().unwrap().name(),
            ZkpError::MismatchError(String::from(""))
        );

        let proof = create_file_reader("./src/zkp/tests/files/wrong/proof_gm17.json").unwrap();
        let proof: serde_json::Value = serde_json::from_reader(proof).unwrap();
        let result = verify_by_file(
            proof.to_string(),
            "./src/zkp/tests/files/wrong/verification_gm17.key",
        );
        assert_eq!(
            result.err().unwrap().name(),
            ZkpError::MismatchError(String::from(""))
        );

        let proof = create_file_reader("./src/zkp/tests/files/wrong/proof_bls12_381.json").unwrap();
        let proof: serde_json::Value = serde_json::from_reader(proof).unwrap();
        let result = verify_by_file(
            proof.to_string(),
            "./src/zkp/tests/files/wrong/verification_bls12_381.key",
        );
        assert_eq!(result.err().unwrap().name(), ZkpError::NotSupport);
    }

    #[test]
    fn test_verify() {
        let proof = create_file_reader("./src/zkp/tests/files/proof.json").unwrap();
        let proof: serde_json::Value = serde_json::from_reader(proof).unwrap();
        let vk = load_file("./src/zkp/tests/files/wrong/verification_error.key").unwrap();

        let result = verify(proof.to_string(), vk.as_slice());
        assert_eq!(
            result.err().unwrap().name(),
            ZkpError::ParseError(String::from(""), String::from(""))
        );
    }
}
