use crate::error::ZkpError;
use ethers::core::types::U256;
use ff::hex;
use mystiko_abi::commitment_pool::commitment_pool::{G1Point, G2Point, Proof};
use serde::{Deserialize, Serialize};
use zokrates_field::Bn128Field;
use zokrates_proof_systems::groth16::ProofPoints;
use zokrates_proof_systems::{G1Affine, G2Affine, G2AffineFq2, Proof as ZksProof, G16};

type ZokratesSystemProof = ZksProof<Bn128Field, G16>;

fn u256_to_bytes(num: U256) -> [u8; 32] {
    let mut x = [0u8; 32];
    num.to_big_endian(&mut x);
    x
}

fn g1_point_to_affine(point: G1Point) -> G1Affine {
    G1Affine(
        format!("0x{}", hex::encode(u256_to_bytes(point.x))),
        format!("0x{}", hex::encode(u256_to_bytes(point.y))),
    )
}

fn g1_affine_to_point(point: G1Affine) -> G1Point {
    G1Point {
        x: U256::from_str_radix(&point.0, 16).unwrap(),
        y: U256::from_str_radix(&point.1, 16).unwrap(),
    }
}

fn g2_point_to_affine(point: G2Point) -> G2Affine {
    G2Affine::Fq2(G2AffineFq2(
        (
            format!("0x{}", hex::encode(u256_to_bytes(point.x[0]))),
            format!("0x{}", hex::encode(u256_to_bytes(point.y[0]))),
        ),
        (
            format!("0x{}", hex::encode(u256_to_bytes(point.x[1]))),
            format!("0x{}", hex::encode(u256_to_bytes(point.y[1]))),
        ),
    ))
}

fn g2_affine_to_point(point: G2Affine) -> G2Point {
    if let G2Affine::Fq2(a) = point {
        let (x0, y0) = a.0;
        let (x1, y1) = a.1;
        G2Point {
            x: [
                U256::from_str_radix(&x0, 16).unwrap(),
                U256::from_str_radix(&x1, 16).unwrap(),
            ],
            y: [
                U256::from_str_radix(&y0, 16).unwrap(),
                U256::from_str_radix(&y1, 16).unwrap(),
            ],
        }
    } else {
        // todo return Result error
        panic!("Unexpected G2Affine type");
    }
}

pub fn json_to_zks_proof(proof: String) -> Result<ZokratesSystemProof, ZkpError> {
    let proof_json: serde_json::Value = serde_json::from_str(proof.as_str())
        .map_err(|why| ZkpError::ParseError("proof".to_string(), why.to_string()))?;

    let proof: ZokratesSystemProof =
        serde_json::from_value(proof_json).map_err(|why| ZkpError::ProofError(why.to_string()))?;
    Ok(proof)
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

    pub fn from_json_string(proof: String) -> Result<Self, ZkpError> {
        let proof_json: serde_json::Value = serde_json::from_str(proof.as_str())
            .map_err(|why| ZkpError::ParseError("proof".to_string(), why.to_string()))?;

        let proof: ZKProof = serde_json::from_value(proof_json)
            .map_err(|why| ZkpError::ProofError(why.to_string()))?;
        Ok(proof)
    }

    pub fn from_tagged_proof(zok: ZokratesSystemProof) -> Self {
        let proof = Proof {
            a: g1_affine_to_point(zok.proof.a),
            b: g2_affine_to_point(zok.proof.b),
            c: g1_affine_to_point(zok.proof.c),
        };

        ZKProof {
            proof,
            inputs: zok.inputs,
        }
    }

    pub fn to_tagged_proof(&self) -> ZokratesSystemProof {
        let proof = self.proof.clone();
        let inputs = self.inputs.clone();
        let point = ProofPoints {
            a: g1_point_to_affine(proof.a),
            b: g2_point_to_affine(proof.b),
            c: g1_point_to_affine(proof.c),
        };
        ZokratesSystemProof::new(point, inputs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zkp::utils::create_file_reader;
    use zokrates_proof_systems::TaggedProof;

    type ZokTaggedProof = TaggedProof<Bn128Field, G16>;

    fn zks_proof_to_json(proof: ZokratesSystemProof) -> String {
        serde_json::to_string_pretty(&ZokTaggedProof::new(proof.proof, proof.inputs)).unwrap()
    }

    #[test]
    fn test_zks_serialize() {
        let proof = create_file_reader("./src/zkp/tests/files/proof.zokrates.json").unwrap();
        let proof: serde_json::Value = serde_json::from_reader(proof).unwrap();
        let proof_zk = json_to_zks_proof(proof.to_string()).unwrap();
        zks_proof_to_json(proof_zk);
    }
}
