use crate::error::ZkpError;
use anyhow::Result;
use zokrates_ast::ir::{self, Witness};
use zokrates_field::Field;
use zokrates_proof_systems::{Backend, Proof, Scheme};

pub fn generate_proof<T: Field, S: Scheme<T>, B: Backend<T, S>>(
    ir_prog: ir::Prog<T>,
    witness: Witness<T>,
    proving_key: &[u8],
) -> Result<Proof<T, S>, ZkpError> {
    let proof = B::generate_proof(ir_prog, witness, proving_key.to_vec());
    Ok(proof)
}
