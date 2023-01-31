use crate::error::ZkpError;
use crate::zkp::compute_witness::compute_witness;
use crate::zkp::generate_proof::generate_proof;
use crate::zkp::utils::{create_file_reader, load_file};
use zokrates_ast::ir::{self, ProgEnum};
use zokrates_ast::typed::abi::Abi;
use zokrates_bellman::Bellman;
use zokrates_common::helpers::*;
use zokrates_proof_systems::*;

pub fn prove_by_file(
    program_path_str: &str,
    abi_spec_path_str: &str,
    proving_key_path_str: &str,
    json_args_str: &str,
) -> Result<String, ZkpError> {
    let program_reader = create_file_reader(program_path_str)?;
    let abi_reader = create_file_reader(abi_spec_path_str)?;

    let proving_key = load_file(proving_key_path_str)?;
    let abi: Abi = serde_json::from_reader(abi_reader)
        .map_err(|why| ZkpError::ParseError("abi".to_string(), why.to_string()))?;
    let prog = match ir::ProgEnum::deserialize(program_reader) {
        Ok(p) => p.collect(),
        Err(err) => return Err(ZkpError::DeserializeProgramError(err)),
    };

    let curve_parameter = CurveParameter::try_from(prog.curve()).unwrap();
    let parameters = Parameters(
        BackendParameter::Bellman,
        curve_parameter,
        SchemeParameter::G16,
    );

    match parameters {
        Parameters(BackendParameter::Bellman, _, SchemeParameter::G16) => match prog {
            ProgEnum::Bn128Program(p) => {
                let witness = compute_witness(p.clone(), &abi, json_args_str)?;
                let proof = generate_proof::<_, G16, Bellman>(p, witness, proving_key.as_slice())?;
                Ok(proof)
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

pub fn prove(
    program: &[u8],
    abi_spec: &[u8],
    proving_key: &[u8],
    json_args_str: &str,
) -> Result<String, ZkpError> {
    let abi: Abi = serde_json::from_slice(abi_spec)
        .map_err(|why| ZkpError::ParseError("abi".to_string(), why.to_string()))?;
    let prog = match ir::ProgEnum::deserialize(program) {
        Ok(p) => p.collect(),
        Err(err) => return Err(ZkpError::DeserializeProgramError(err)),
    };

    let curve_parameter = CurveParameter::try_from(prog.curve()).unwrap();
    let parameters = Parameters(
        BackendParameter::Bellman,
        curve_parameter,
        SchemeParameter::G16,
    );

    match parameters {
        Parameters(BackendParameter::Bellman, _, SchemeParameter::G16) => match prog {
            ProgEnum::Bn128Program(p) => {
                let witness = compute_witness(p.clone(), &abi, json_args_str)?;
                let proof = generate_proof::<_, G16, Bellman>(p, witness, proving_key)?;
                Ok(proof)
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}
