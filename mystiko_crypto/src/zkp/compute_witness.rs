use crate::error::ZkpError;
use zokrates_abi::Encode;
use zokrates_ast::ir::{self, Witness};
use zokrates_ast::typed::abi::Abi;
use zokrates_field::Field;

fn convert_args<T: Field>(abi_spec: &[u8], json_args_str: &str) -> Result<Vec<T>, ZkpError> {
    let abi: Abi = serde_json::from_slice(abi_spec)
        .map_err(|why| ZkpError::ParseError("abi".to_string(), why.to_string()))?;

    let signature = abi.signature();
    let args_abi: zokrates_abi::Inputs<T> =
        zokrates_abi::parse_strict(json_args_str, signature.inputs)
            .map(zokrates_abi::Inputs::Abi)
            .map_err(|why| why.to_string())
            .unwrap();

    Ok(args_abi.encode())
}

pub fn compute_witness<T: Field, I: Iterator<Item = ir::Statement<T>>>(
    ir_prog: ir::ProgIterator<T, I>,
    abi_spec: &[u8],
    json_args_str: &str,
) -> Result<Witness<T>, ZkpError> {
    let args = convert_args(abi_spec, json_args_str)?;
    let interpreter = zokrates_interpreter::Interpreter::default();
    interpreter
        .execute(ir_prog, &args)
        .map_err(|e| ZkpError::ComputeWitnessError(e.to_string()))
}
