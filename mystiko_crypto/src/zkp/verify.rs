use crate::error::ZkpError;
use crate::zkp::utils::load_file;
use zokrates_bellman::Bellman;
use zokrates_common::helpers::*;
use zokrates_field::{Bn128Field, Field};
use zokrates_proof_systems::*;

pub fn verify_by_file(proof: String, verification_key_path_str: &str) -> Result<bool, ZkpError> {
    let verification_key = load_file(verification_key_path_str)?;
    verify(proof, verification_key.as_slice())
}

pub fn verify(proof: String, verification_key: &[u8]) -> Result<bool, ZkpError> {
    let vk: serde_json::Value = serde_json::from_slice(verification_key)
        .map_err(|why| ZkpError::ParseError("verification key".to_string(), why.to_string()))?;

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
        return Err(ZkpError::Mismatch(
            "curve of the proof and the verification mismatch".to_string(),
        ));
    }

    if proof_scheme != vk_scheme {
        return Err(ZkpError::Mismatch(
            "scheme of the proof and the verification mismatch".to_string(),
        ));
    }

    let parameters = match Parameters::try_from((
        BackendParameter::Bellman.to_string().as_str(),
        vk_curve,
        vk_scheme,
    )) {
        Ok(param) => param,
        Err(why) => return Err(ZkpError::Mismatch(why)),
    };

    match parameters {
        Parameters(BackendParameter::Bellman, CurveParameter::Bn128, SchemeParameter::G16) => {
            do_verify::<Bn128Field, G16, Bellman>(vk, proof_json)
        }
        _ => unreachable!(),
    }
}

fn do_verify<T: Field, S: Scheme<T>, B: Backend<T, S>>(
    vk: serde_json::Value,
    proof: serde_json::Value,
) -> Result<bool, ZkpError> {
    let vk = serde_json::from_value(vk).map_err(|why| ZkpError::ProofError(why.to_string()))?;
    let proof: Proof<T, S> =
        serde_json::from_value(proof).map_err(|why| ZkpError::VKError(why.to_string()))?;

    Ok(B::verify(vk, proof))
}
