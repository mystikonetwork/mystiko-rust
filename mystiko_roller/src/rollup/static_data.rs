use crate::data::handler::ProofInfo;
use lazy_static::lazy_static;
use mystiko_crypto::zkp::proof::{G1Point, G2Point, Proof, ZKProof};
use mystiko_protocol::rollup::RollupProof;
use num_bigint::BigUint;
use std::str::FromStr;

lazy_static! {
    pub static ref STATIC_ERROR_INVALID_ROLLUP_SIZE: String = "0x53a2556c0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000a726f6c6c757053697a6500000000000000000000000000000000000000000000".to_string();
    pub static ref STATIC_ERROR_INVALID_LEAF_HASH: String = "0x53a2556c000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000086c65616648617368000000000000000000000000000000000000000000000000".to_string();

    pub static ref STATIC_PROOF_DATA: ProofInfo = ProofInfo {
        r: RollupProof {
            zk_proof: ZKProof {
                proof: Proof {
                    a: G1Point {
                        x: "0x271397ad6b5e3a041bf3119c2f7b6c9fdca59662b83e289c0af4614499ff6515".to_string(),
                        y: "0x09a0f85c79382c658e5e72d9a8f0d25928a648da3cf7c7f257b279bf2e66e949".to_string(),
                    },
                    b: G2Point {
                        x: [
                            "0x076bb476c7b072d22aaf3c59612dddca6346c5ef4ea8d34710f9aec1074fb7c1".to_string(),
                            "0x0cd8f58ad56afba35893f6af128c660fa7b7dced3e78cb1fa6a41f1ff81a64f5".to_string(),
                        ],
                        y: [
                            "0x25c4e8f40bd09f92ad44ee8970c602947ebb64cb95aaf515ba2112034f7d54ef".to_string(),
                            "0x2e1b357f0f3895a12f224078c2d836878bb0c1b9210cf79436f1bbe911410e98".to_string(),
                        ],
                    },
                    c: G1Point {
                        x: "0x0635d910c454726f910e6300673c708294b6c5019258dcdaf7d4d0f1b13ae25b".to_string(),
                        y: "0x1af2613735a2e9f243b15c8f802f3c9cbc9000174e2bf77ce7a0c0e43cde2bea".to_string(),
                    },
                },
                inputs: vec![
                    "0x2c351d63154c9a48d03d539a81f8f95a8e09d7ad9e04c8d4c1a15ce6657bb47a".to_string(),
                    "0x1eb5c83507ee19fd149178134855b3aa449332253b8376af5f97f25a32936e17".to_string(),
                    "0x020332c716e689d9a6e2e9731cdd71e51fc2269d36d8bc1efdc50510d2df6d0f".to_string(),
                    "0x0000000000000000000000000000000000000000000000000000000000000022".to_string(),
                ],
            },
            new_root: BigUint::from_str("13890566555204546011498387055273076146663528550252880433766037483427158257175")
                .unwrap(),
            leaves_hash: BigUint::from_str(
                "910276693119302666391498604708974964643344764052861980510623737984921333007",
            )
            .unwrap(),
        },
    };
}
