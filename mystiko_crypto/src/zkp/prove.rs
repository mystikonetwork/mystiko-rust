use crate::error::ZkpError;
use crate::zkp::compute_witness::compute_witness;
use crate::zkp::generate_proof::generate_proof;
use crate::zkp::utils::load_file;
use zokrates_ast::ir::{self, ProgEnum};
use zokrates_bellman::Bellman;
use zokrates_common::helpers::*;
use zokrates_proof_systems::*;

pub fn prove_by_file(
    program_path_str: &str,
    abi_spec_path_str: &str,
    proving_key_path_str: &str,
    json_args_str: &str,
) -> Result<String, ZkpError> {
    let program = load_file(program_path_str)?;
    let abi_spec = load_file(abi_spec_path_str)?;
    let proving_key = load_file(proving_key_path_str)?;

    prove(
        program.as_slice(),
        abi_spec.as_slice(),
        proving_key.as_slice(),
        json_args_str,
    )
}

pub fn prove(
    program: &[u8],
    abi_spec: &[u8],
    proving_key: &[u8],
    json_args_str: &str,
) -> Result<String, ZkpError> {
    let prog1 = match ir::ProgEnum::deserialize(program) {
        Ok(deserialized) => deserialized,
        Err(err) => return Err(ZkpError::DeserializeProgramError(err)),
    };

    let witness = match prog1 {
        ProgEnum::Bn128Program(p) => compute_witness(p, abi_spec, json_args_str)?,
        _ => unreachable!(),
    };

    let prog2 = match ir::ProgEnum::deserialize(program) {
        Ok(deserialized) => deserialized,
        Err(err) => return Err(ZkpError::DeserializeProgramError(err)),
    };

    let curve_parameter = CurveParameter::try_from(prog2.curve()).unwrap();
    let parameters = Parameters(
        BackendParameter::Bellman,
        curve_parameter,
        SchemeParameter::G16,
    );

    match parameters {
        Parameters(BackendParameter::Bellman, _, SchemeParameter::G16) => match prog2 {
            ProgEnum::Bn128Program(p) => {
                let proof = generate_proof::<_, _, G16, Bellman>(p, witness, proving_key)?;
                Ok(proof)
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}
