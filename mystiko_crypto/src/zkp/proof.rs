use crate::error::ZkpError;
use crate::file::load_file;
use crate::zkp::compute_witness::compute_witness;
use crate::zkp::generate_proof::generate_proof;
use serde::{Deserialize, Serialize};
use zokrates_ast::ir::{self, ProgEnum};
use zokrates_ast::typed::abi::Abi;
use zokrates_bellman::Bellman;
use zokrates_common::helpers::*;
use zokrates_field::{Bn128Field, Field};
use zokrates_proof_systems::groth16::ProofPoints;
use zokrates_proof_systems::{Backend, G1Affine, G2Affine, G2AffineFq2, Scheme, G16};

type ZokratesSystemProof = zokrates_proof_systems::Proof<Bn128Field, G16>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct G1Point {
    pub x: String,
    pub y: String,
}

impl G1Point {
    fn to_affine(&self) -> G1Affine {
        G1Affine(self.x.clone(), self.y.clone())
    }

    fn from_affine(point: &G1Affine) -> Self {
        G1Point {
            x: point.0.clone(),
            y: point.1.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct G2Point {
    pub x: [String; 2],
    pub y: [String; 2],
}

impl G2Point {
    fn to_affine(&self) -> G2Affine {
        G2Affine::Fq2(G2AffineFq2(
            (self.x[0].clone(), self.y[0].clone()),
            (self.x[1].clone(), self.y[1].clone()),
        ))
    }

    fn from_affine(point: &G2Affine) -> Self {
        if let G2Affine::Fq2(a) = point {
            G2Point {
                x: [a.0 .0.clone(), a.1 .0.clone()],
                y: [a.0 .1.clone(), a.1 .1.clone()],
            }
        } else {
            // todo return Result error
            panic!("Unexpected G2Affine type");
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proof {
    pub a: G1Point,
    pub b: G2Point,
    pub c: G1Point,
}

//
// pub fn json_to_zks_proof(proof: String) -> Result<ZokratesSystemProof, ZkpError> {
//     let proof_json: serde_json::Value = serde_json::from_str(proof.as_str())
//         .map_err(|why| ZkpError::ParseError("proof".to_string(), why.to_string()))?;
//
//     let proof: ZokratesSystemProof =
//         serde_json::from_value(proof_json).map_err(|why| ZkpError::ProofError(why.to_string()))?;
//     Ok(proof)
// }

fn call_verify<T: Field, S: Scheme<T>, B: Backend<T, S>>(
    vk: serde_json::Value,
    proof: zokrates_proof_systems::Proof<T, S>,
) -> Result<bool, ZkpError> {
    let vk = serde_json::from_value(vk).map_err(|why| ZkpError::VKError(why.to_string()))?;
    Ok(B::verify(vk, proof))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZKProof {
    pub proof: Proof,
    pub inputs: Vec<String>,
}

impl ZKProof {
    pub fn to_json_string(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }

    pub fn from_json_string(proof: &str) -> Result<Self, ZkpError> {
        let proof_json: serde_json::Value = serde_json::from_str(proof)
            .map_err(|why| ZkpError::ParseError("proof".to_string(), why.to_string()))?;

        let proof: ZKProof = serde_json::from_value(proof_json)
            .map_err(|why| ZkpError::ProofError(why.to_string()))?;
        Ok(proof)
    }

    fn to_tagged_proof(&self) -> ZokratesSystemProof {
        let proof = self.proof.clone();
        let inputs = self.inputs.clone();
        let point = ProofPoints {
            a: proof.a.to_affine(),
            b: proof.b.to_affine(),
            c: proof.c.to_affine(),
        };
        ZokratesSystemProof::new(point, inputs)
    }

    fn from_tagged_proof(zok: &ZokratesSystemProof) -> Self {
        let proof = Proof {
            a: G1Point::from_affine(&zok.proof.a),
            b: G2Point::from_affine(&zok.proof.b),
            c: G1Point::from_affine(&zok.proof.c),
        };

        ZKProof {
            proof,
            inputs: zok.inputs.clone(),
        }
    }

    pub async fn generate_with_file(
        program_path_str: &str,
        abi_spec_path_str: &str,
        proving_key_path_str: &str,
        json_args_str: &str,
    ) -> Result<Self, ZkpError> {
        let program = load_file(program_path_str).await?;
        let abi_spec = load_file(abi_spec_path_str).await?;
        let proving_key = load_file(proving_key_path_str).await?;

        let abi: Abi = serde_json::from_slice(abi_spec.as_slice())
            .map_err(|why| ZkpError::ParseError("abi".to_string(), why.to_string()))?;
        let prog = match ir::ProgEnum::deserialize(program.as_slice()) {
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
        Ok(ZKProof::from_tagged_proof(&proof))
    }

    pub fn generate(
        program: &[u8],
        abi_spec: &[u8],
        proving_key: &[u8],
        json_args_str: &str,
    ) -> Result<Self, ZkpError> {
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
        Ok(ZKProof::from_tagged_proof(&proof))
    }

    pub async fn verify_with_file(
        &self,
        verification_key_path_str: &str,
    ) -> Result<bool, ZkpError> {
        let vk = load_file(verification_key_path_str).await?;
        let vk = serde_json::from_reader(vk.as_slice())
            .map_err(|why| ZkpError::ParseError("verification key".to_string(), why.to_string()))?;
        self.do_verify(vk)
    }

    pub fn verify(&self, verification_key: &[u8]) -> Result<bool, ZkpError> {
        let vk: serde_json::Value = serde_json::from_slice(verification_key)
            .map_err(|why| ZkpError::ParseError("verification key".to_string(), why.to_string()))?;

        self.do_verify(vk)
    }

    fn do_verify(&self, vk: serde_json::Value) -> Result<bool, ZkpError> {
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

        if vk_curve != "bn128" {
            return Err(ZkpError::MismatchError(
                "curve of the proof and the verification mismatch".to_string(),
            ));
        }

        if vk_scheme != "g16" {
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

        let zk_system_proof = self.to_tagged_proof();
        match parameters {
            Parameters(BackendParameter::Bellman, CurveParameter::Bn128, SchemeParameter::G16) => {
                call_verify::<Bn128Field, G16, Bellman>(vk, zk_system_proof)
            }
            _ => Err(ZkpError::NotSupport),
        }
    }
}
