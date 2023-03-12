use crate::error::ZkpError;
use serde::{Deserialize, Serialize};
use zokrates_field::Bn128Field;
use zokrates_proof_systems::groth16::ProofPoints;
use zokrates_proof_systems::{G1Affine, G2Affine, G2AffineFq2, Proof as ZksProof, G16};

type ZokratesSystemProof = ZksProof<Bn128Field, G16>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct G1Point {
    pub x: String,
    pub y: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct G2Point {
    pub x: [String; 2],
    pub y: [String; 2],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proof {
    pub a: G1Point,
    pub b: G2Point,
    pub c: G1Point,
}

fn g1_point_to_affine(point: G1Point) -> G1Affine {
    G1Affine(point.x, point.y)
}

fn g1_affine_to_point(point: G1Affine) -> G1Point {
    G1Point {
        x: point.0,
        y: point.1,
    }
}

fn g2_point_to_affine(point: G2Point) -> G2Affine {
    G2Affine::Fq2(G2AffineFq2(
        (point.x[0].clone(), point.y[0].clone()),
        (point.x[1].clone(), point.y[1].clone()),
    ))
}

fn g2_affine_to_point(point: G2Affine) -> G2Point {
    if let G2Affine::Fq2(a) = point {
        G2Point {
            x: [a.0 .0, a.1 .0],
            y: [a.0 .1, a.1 .1],
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
