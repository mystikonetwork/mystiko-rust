use crate::error::ZkpError;
use anyhow::Result;
use zokrates_abi::Encode;
use zokrates_ast::ir::{self, Witness};
use zokrates_ast::typed::abi::Abi;
use zokrates_field::Field;

fn convert_args<T: Field>(abi_spec: &Abi, json_args_str: &str) -> Result<Vec<T>, ZkpError> {
    let signature = abi_spec.signature();

    let args_abi: zokrates_abi::Inputs<T> = zokrates_abi::parse_strict(json_args_str, signature.inputs)
        .map(zokrates_abi::Inputs::Abi)
        .map_err(|why| ZkpError::AbiParseError(why.to_string()))?;

    Ok(args_abi.encode())
}

pub fn compute_witness<T: Field>(
    ir_prog: ir::Prog<T>,
    abi_spec: &Abi,
    json_args_str: &str,
) -> Result<Witness<T>, ZkpError> {
    let args = convert_args(abi_spec, json_args_str)?;
    let interpreter = zokrates_interpreter::Interpreter::default();
    interpreter
        .execute(
            &args,
            ir_prog.statements.into_iter(),
            &ir_prog.arguments,
            &ir_prog.solvers,
        )
        .map_err(|e| ZkpError::ComputeWitnessError(e.to_string()))
}
