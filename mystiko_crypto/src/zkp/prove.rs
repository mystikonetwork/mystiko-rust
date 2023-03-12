use crate::error::ZkpError;
use crate::zkp::compute_witness::compute_witness;
use crate::zkp::generate_proof::generate_proof;
use crate::zkp::types::ZKProof;
use crate::zkp::utils::{create_file_reader, load_file};
use zokrates_ast::ir::{self, ProgEnum};
use zokrates_ast::typed::abi::Abi;
use zokrates_bellman::Bellman;
use zokrates_field::Bn128Field;
use zokrates_proof_systems::G16;

pub fn prove_by_file(
    program_path_str: &str,
    abi_spec_path_str: &str,
    proving_key_path_str: &str,
    json_args_str: &str,
) -> Result<ZKProof, ZkpError> {
    let program_reader = create_file_reader(program_path_str)?;
    let abi_reader = create_file_reader(abi_spec_path_str)?;

    let proving_key = load_file(proving_key_path_str)?;
    let abi: Abi = serde_json::from_reader(abi_reader)
        .map_err(|why| ZkpError::ParseError("abi".to_string(), why.to_string()))?;
    let prog = match ir::ProgEnum::deserialize(program_reader) {
        Ok(p) => p.collect(),
        Err(err) => return Err(ZkpError::DeserializeProgramError(err)),
    };

    let p = if let ProgEnum::Bn128Program(p) = prog {
        p
    } else {
        return Err(ZkpError::NotSupport);
    };

    let witness = compute_witness(p.clone(), &abi, json_args_str)?;
    let proof = generate_proof::<Bn128Field, G16, Bellman>(p, witness, proving_key.as_slice())?;
    Ok(ZKProof::from_tagged_proof(proof))
}

pub fn prove(
    program: &[u8],
    abi_spec: &[u8],
    proving_key: &[u8],
    json_args_str: &str,
) -> Result<ZKProof, ZkpError> {
    let abi: Abi = serde_json::from_slice(abi_spec)
        .map_err(|why| ZkpError::ParseError("abi".to_string(), why.to_string()))?;
    let prog = match ir::ProgEnum::deserialize(program) {
        Ok(p) => p.collect(),
        Err(err) => return Err(ZkpError::DeserializeProgramError(err)),
    };

    let p = if let ProgEnum::Bn128Program(p) = prog {
        p
    } else {
        return Err(ZkpError::NotSupport);
    };

    let witness = compute_witness(p.clone(), &abi, json_args_str)?;
    let proof = generate_proof::<Bn128Field, G16, Bellman>(p, witness, proving_key)?;
    Ok(ZKProof::from_tagged_proof(proof))
}
