use crate::error::ProtocolError;
use crate::mystiko_crypto::zkp::proof::ZKProof;
use anyhow::Result;
use mystiko_crypto::constants::FIELD_SIZE;
use mystiko_crypto::hash::keccak256;
use mystiko_crypto::merkle_tree::MerkleTree;
use mystiko_crypto::utils::{biguint_to_be_32_bytes, mod_floor};
use num_bigint::BigUint;

#[derive(Debug)]
pub struct Rollup<'a> {
    tree: &'a mut MerkleTree,
    new_leaves: Vec<BigUint>,
    program: Vec<u8>,
    abi: Vec<u8>,
    proving_key: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct RollupProof {
    pub zk_proof: ZKProof,
    pub new_root: BigUint,
    pub leaves_hash: BigUint,
}

impl<'a> Rollup<'a> {
    pub fn new(
        tree: &'a mut MerkleTree,
        new_leaves: Vec<BigUint>,
        program: Vec<u8>,
        abi: Vec<u8>,
        proving_key: Vec<u8>,
    ) -> Rollup<'a> {
        Self {
            tree,
            new_leaves,
            program,
            abi,
            proving_key,
        }
    }

    pub fn prove(&mut self) -> Result<RollupProof, ProtocolError> {
        let new_leaves = self.new_leaves.clone();
        let rollup_size = new_leaves.len();
        assert!(is_power_of_two(rollup_size));
        let rollup_height = (rollup_size as f64).log2().round() as usize;
        let current_leaf_count = self.tree.count();
        assert_eq!(current_leaf_count % rollup_size, 0);
        let current_root = self.tree.root();

        self.tree.bulk_insert(new_leaves.clone())?;
        let new_root = self.tree.root();
        let leaf_path = self.tree.path(current_leaf_count)?;
        let (_, path_indices) = leaf_path.1.split_at(rollup_height);
        let path_indices = path_indices_number(path_indices);
        let (_, path_elements) = leaf_path.0.split_at(rollup_height);
        let path_elements: Vec<String> = path_elements.iter().map(|n| n.to_string()).collect();
        let leaves_hash = calc_leaves_hash(new_leaves.as_slice());
        let new_leaves: Vec<String> = new_leaves.iter().map(|n| n.to_string()).collect();

        let array: Vec<serde_json::Value> = vec![
            serde_json::to_value(current_root.to_string())?,
            serde_json::to_value(new_root.to_string())?,
            serde_json::to_value(leaves_hash.to_string())?,
            serde_json::to_value(path_indices.to_string())?,
            serde_json::to_value(path_elements)?,
            serde_json::to_value(new_leaves)?,
        ];

        let input = serde_json::Value::Array(array).to_string();
        let zk_proof = ZKProof::generate(
            self.program.as_slice(),
            self.abi.as_slice(),
            self.proving_key.as_slice(),
            &input,
        )?;

        Ok(RollupProof {
            zk_proof,
            new_root,
            leaves_hash,
        })
    }
}

fn is_power_of_two(a_number: usize) -> bool {
    a_number != 0 && (a_number & (a_number - 1)) == 0
}

fn path_indices_number(path_indices: &[usize]) -> BigUint {
    let binary_string = path_indices
        .iter()
        .rev()
        .map(|x| format!("{:b}", x))
        .collect::<String>();
    BigUint::parse_bytes(binary_string.as_bytes(), 2).unwrap()
}

fn calc_leaves_hash(leaves: &[BigUint]) -> BigUint {
    let leaf_buffer: Vec<u8> = leaves.iter().flat_map(biguint_to_be_32_bytes).collect();
    let hash = keccak256(leaf_buffer.as_slice());
    let hash = BigUint::from_bytes_be(&hash);
    mod_floor(&hash, &FIELD_SIZE)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_power_of_two() {
        assert!(!is_power_of_two(0usize));

        assert!(is_power_of_two(1usize));
        assert!(is_power_of_two(2usize));
        assert!(is_power_of_two(4usize));
        assert!(is_power_of_two(64usize));
    }

    #[test]
    fn test_path_indices_number() {
        let path = [1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1];
        let expect_indices = BigUint::from(1373u32);
        let indices = path_indices_number(&path);
        assert_eq!(indices, expect_indices);

        let path = [0, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1];
        let expect_indices = BigUint::from(1492u32);
        let indices = path_indices_number(&path);
        assert_eq!(indices, expect_indices);
    }

    #[test]
    fn test_calc_leave_hash() {
        let r1 = BigUint::from(66051u32);
        let r2 = BigUint::from(197121u32);
        let r3 = BigUint::parse_bytes(
            b"5999809398626971894156481321441750001229812699285374901473004231265197659290",
            10,
        )
        .unwrap();
        let leaves = [r1, r2, r3];
        let expect_hash = BigUint::parse_bytes(
            b"6310518973517441342440727149209914865806190787755638376161673961442084637476",
            10,
        )
        .unwrap();
        let leaves_hash = calc_leaves_hash(&leaves);
        assert_eq!(expect_hash, leaves_hash);
    }
}
