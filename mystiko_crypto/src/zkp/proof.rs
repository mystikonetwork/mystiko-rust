use crate::error::ZkpError;
use crate::zkp::compute_witness::compute_witness;
use crate::zkp::generate_proof::generate_proof;
use anyhow::Result;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use zokrates_ast::ir::{self, ProgEnum};
use zokrates_ast::typed::abi::Abi;
use zokrates_bellman::Bellman;
use zokrates_field::{Bn128Field, Field};
use zokrates_proof_systems::groth16::ProofPoints;
use zokrates_proof_systems::{Backend, G1Affine, G2Affine, G2AffineFq2, Scheme, G16};

type ZokratesSystemProof = zokrates_proof_systems::Proof<Bn128Field, G16>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct G2Point {
    pub x: [String; 2],
    pub y: [String; 2],
}

impl G2Point {
    fn to_affine(&self) -> G2Affine {
        G2Affine::Fq2(G2AffineFq2(
            (self.x[0].clone(), self.x[1].clone()),
            (self.y[0].clone(), self.y[1].clone()),
        ))
    }

    fn from_affine(point: &G2Affine) -> Self {
        if let G2Affine::Fq2(a) = point {
            G2Point {
                x: [a.0 .0.clone(), a.0 .1.clone()],
                y: [a.1 .0.clone(), a.1 .1.clone()],
            }
        } else {
            panic!("Unexpected G2Affine type");
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Proof {
    pub a: G1Point,
    pub b: G2Point,
    pub c: G1Point,
}

impl Proof {
    pub fn convert_to<T: DeserializeOwned>(&self) -> Result<T> {
        let serialized = serde_json::to_string(self)?;
        Ok(serde_json::from_str(&serialized)?)
    }
}

fn call_verify<T: Field, S: Scheme<T>, B: Backend<T, S>>(
    vk: serde_json::Value,
    proof: zokrates_proof_systems::Proof<T, S>,
) -> Result<bool, ZkpError> {
    let vk = serde_json::from_value(vk).map_err(|why| ZkpError::VKError(why.to_string()))?;
    Ok(B::verify(vk, proof))
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ZKProof {
    pub proof: Proof,
    pub inputs: Vec<String>,
}

impl ZKProof {
    pub fn to_json_string(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }

    pub fn from_json_string(proof: &str) -> Result<Self, ZkpError> {
        let proof_json: serde_json::Value = serde_json::from_str(proof)?;

        let proof: ZKProof = serde_json::from_value(proof_json).map_err(|why| ZkpError::ProofError(why.to_string()))?;
        Ok(proof)
    }

    pub fn to_tagged_proof(&self) -> ZokratesSystemProof {
        let proof = self.proof.clone();
        let inputs = self.inputs.clone();
        let point = ProofPoints {
            a: proof.a.to_affine(),
            b: proof.b.to_affine(),
            c: proof.c.to_affine(),
        };
        ZokratesSystemProof::new(point, inputs)
    }

    pub fn from_tagged_proof(zok: &ZokratesSystemProof) -> Self {
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

    pub fn generate(
        program: &[u8],
        abi_spec: &[u8],
        proving_key: &[u8],
        json_args_str: &str,
    ) -> Result<Self, ZkpError> {
        let abi: Abi = serde_json::from_slice(abi_spec)?;
        let cursor = Cursor::new(program);
        let prog = match ir::ProgEnum::deserialize(cursor) {
            Ok(p) => p.collect(),
            Err(err) => return Err(ZkpError::DeserializeProgramError(err)),
        };

        let p = match prog {
            ProgEnum::Bn128Program(p) => p,
            _ => return Err(ZkpError::NotSupport),
        };

        let witness = compute_witness(p.clone(), &abi, json_args_str)?;
        let proof = generate_proof::<Bn128Field, G16, Bellman>(p, witness, proving_key)?;
        Ok(ZKProof::from_tagged_proof(&proof))
    }

    pub fn verify(&self, verification_key: &[u8]) -> Result<bool, ZkpError> {
        let vk: serde_json::Value = serde_json::from_slice(verification_key)?;
        self.do_verify(vk)
    }

    fn do_verify(&self, vk: serde_json::Value) -> Result<bool, ZkpError> {
        let vk_curve = vk
            .get("curve")
            .ok_or_else(|| ZkpError::VKError("Field `curve` not found in verification key".to_string()))?
            .as_str()
            .ok_or_else(|| ZkpError::VKError("`curve` should be a string".to_string()))?;
        let vk_scheme = vk
            .get("scheme")
            .ok_or_else(|| ZkpError::VKError("Field `scheme` not found in verification key".to_string()))?
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

        let zk_system_proof = self.to_tagged_proof();
        call_verify::<Bn128Field, G16, Bellman>(vk, zk_system_proof)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zokrates_proof_systems::G2AffineFq;

    #[test]
    #[should_panic(expected = "Unexpected G2Affine type")]
    fn test_g2_point_from_affine() {
        let point = G2Affine::Fq2(G2AffineFq2(
            ("1".to_string(), "2".to_string()),
            ("3".to_string(), "4".to_string()),
        ));
        let _ = G2Point::from_affine(&point);

        let point = G2Affine::Fq(G2AffineFq("0".to_string(), "1".to_string()));
        let _ = G2Point::from_affine(&point);
    }
}
