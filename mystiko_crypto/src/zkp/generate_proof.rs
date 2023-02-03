use crate::error::ZkpError;
use zokrates_ast::ir::{self, Witness};
use zokrates_field::Field;
use zokrates_proof_systems::*;

pub fn generate_proof<T: Field, S: Scheme<T>, B: Backend<T, S>>(
    ir_prog: ir::Prog<T>,
    witness: Witness<T>,
    proving_key: &[u8],
) -> Result<String, ZkpError> {
    let proof = B::generate_proof(ir_prog, witness, proving_key.to_vec());
    let proof =
        serde_json::to_string_pretty(&TaggedProof::<T, S>::new(proof.proof, proof.inputs)).unwrap();
    Ok(proof)
}
