use crate::error::ZkpError;
use crate::zkp::compute_witness::compute_witness;
use crate::zkp::generate_proof::generate_proof;
use crate::zkp::utils::{create_file_reader, load_file};
use zokrates_ast::ir::{self, ProgEnum};
use zokrates_ast::typed::abi::Abi;
use zokrates_bellman::Bellman;
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

    let p = if let ProgEnum::Bn128Program(p) = prog {
        p
    } else {
        return Err(ZkpError::NotSupport);
    };

    let witness = compute_witness(p.clone(), &abi, json_args_str)?;
    let proof = generate_proof::<_, G16, Bellman>(p, witness, proving_key.as_slice())?;
    Ok(proof)
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

    let p = if let ProgEnum::Bn128Program(p) = prog {
        p
    } else {
        return Err(ZkpError::NotSupport);
    };

    let witness = compute_witness(p.clone(), &abi, json_args_str)?;
    let proof = generate_proof::<_, G16, Bellman>(p, witness, proving_key)?;
    Ok(proof)
}

#[cfg(test)]
mod tests {
    use crate::error::ZkpError;
    use crate::zkp::prove::{prove, prove_by_file};
    use crate::zkp::utils::load_file;

    #[test]
    fn test_prove_by_file() {
        let arr = ("1", "0", "1");
        let args = serde_json::to_string(&arr).unwrap();
        let witness = prove_by_file(
            "./xxx/program",
            "./src/zkp/tests/files/abi.json",
            "./src/zkp/tests/files/proving.key",
            &args,
        );
        assert_eq!(
            witness.err().unwrap().name(),
            ZkpError::ReadFileError(String::from(""), String::from(""))
        );

        let witness = prove_by_file(
            "./src/zkp/tests/files/program",
            "./xxx/abi.json",
            "./src/zkp/tests/files/proving.key",
            &args,
        );
        assert_eq!(
            witness.err().unwrap().name(),
            ZkpError::ReadFileError(String::from(""), String::from(""))
        );

        let witness = prove_by_file(
            "./src/zkp/tests/files/program",
            "./src/zkp/tests/files/abi.json",
            "./xxx/proving.key",
            &args,
        );
        assert_eq!(
            witness.err().unwrap().name(),
            ZkpError::ReadFileError(String::from(""), String::from(""))
        );

        let witness = prove_by_file(
            "./src/zkp/tests/files/wrong/program",
            "./src/zkp/tests/files/abi.json",
            "./src/zkp/tests/files/proving.key",
            &args,
        );
        assert_eq!(
            witness.err().unwrap().name(),
            ZkpError::DeserializeProgramError(String::from(""))
        );
    }

    #[test]
    fn test_prove() {
        let arr = ("1", "0", "1");
        let args = serde_json::to_string(&arr).unwrap();
        let prog = load_file("./src/zkp/tests/files/wrong/program").unwrap();
        let abi_spec = load_file("./src/zkp/tests/files/abi.json").unwrap();
        let pk = load_file("./src/zkp/tests/files/proving.key").unwrap();

        let witness = prove(prog.as_slice(), abi_spec.as_slice(), pk.as_slice(), &args);
        assert_eq!(
            witness.err().unwrap().name(),
            ZkpError::DeserializeProgramError(String::from(""))
        );
    }
}
