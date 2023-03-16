use crate::error::ProtocolError;
use crate::mystiko_crypto::zkp::proof::ZKProof;
use mystiko_crypto::constants::FIELD_SIZE;
use mystiko_crypto::hash::keccak256;
use mystiko_crypto::merkle_tree::MerkleTree;
use mystiko_crypto::utils::{bigint_to_be_32_bytes, calc_mod};
use num_bigint::{BigInt, Sign};

fn is_power_of_two(a_number: usize) -> bool {
    a_number != 0 && (a_number & (a_number - 1)) == 0
}

fn path_indices_number(path_indices: &[usize]) -> BigInt {
    let binary_string = path_indices
        .iter()
        .rev()
        .map(|x| format!("{:b}", x))
        .collect::<String>();
    BigInt::parse_bytes(binary_string.as_bytes(), 2).unwrap()
}

fn calc_leave_hash(leaves: &[BigInt]) -> BigInt {
    let leaf_buffer: Vec<u8> = leaves.iter().flat_map(bigint_to_be_32_bytes).collect();
    let hash = keccak256(leaf_buffer.as_slice());
    let hash = BigInt::from_bytes_be(Sign::Plus, &hash);
    calc_mod(&hash, &FIELD_SIZE)
}

#[derive(Debug, Clone)]
pub struct Rollup {
    tree: MerkleTree,
    new_leaves: Vec<BigInt>,
    program_file: String,
    abi_file: String,
    proving_key_file: String,
}

impl Rollup {
    pub fn new(
        tree: MerkleTree,
        new_leaves: Vec<BigInt>,
        program_file: String,
        abi_file: String,
        proving_key_file: String,
    ) -> Self {
        Self {
            tree,
            new_leaves,
            program_file,
            abi_file,
            proving_key_file,
        }
    }

    pub async fn prove(&self) -> Result<ZKProof, ProtocolError> {
        let new_leaves = self.new_leaves.clone();
        let rollup_size = new_leaves.len();
        assert!(is_power_of_two(rollup_size));
        let rollup_height = (rollup_size as f64).log2().round() as usize;
        let current_leaf_count = self.tree.count();
        assert_eq!(current_leaf_count % rollup_size, 0);
        let current_root = self.tree.root();

        // todo check insert result
        let mut new_tree = self.tree.clone();
        new_tree.bulk_insert(new_leaves.clone()).unwrap();
        let new_root = new_tree.root();
        let leaf_path = new_tree.path(current_leaf_count).unwrap();
        let (_, path_indices) = leaf_path.1.split_at(rollup_height);
        let path_indices = path_indices_number(path_indices);
        let (_, path_elements) = leaf_path.0.split_at(rollup_height);
        let path_elements: Vec<String> = path_elements.iter().map(|n| n.to_string()).collect();
        let leaves_hash = calc_leave_hash(new_leaves.as_slice());
        let new_leaves: Vec<String> = new_leaves.iter().map(|n| n.to_string()).collect();

        let mut array: Vec<serde_json::Value> = vec![serde_json::json!(current_root.to_string())];
        array.push(serde_json::json!(new_root.to_string()));
        array.push(serde_json::json!(leaves_hash.to_string()));
        array.push(serde_json::json!(path_indices.to_string()));
        array.push(serde_json::json!(path_elements));
        array.push(serde_json::json!(new_leaves));
        let input = serde_json::Value::Array(array).to_string();

        let proof = ZKProof::generate_with_file(
            &self.program_file,
            &self.abi_file,
            &self.proving_key_file,
            &input,
        )
        .await
        .unwrap();

        Ok(proof)
    }
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
        let expect_indices = BigInt::from(1373u32);
        let indices = path_indices_number(&path);
        assert_eq!(indices, expect_indices);

        let path = [0, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1];
        let expect_indices = BigInt::from(1492u32);
        let indices = path_indices_number(&path);
        assert_eq!(indices, expect_indices);
    }

    #[test]
    fn test_calc_leave_hash() {
        let r1 = BigInt::from(66051u32);
        let r2 = BigInt::from(197121u32);
        let r3 = BigInt::parse_bytes(
            b"5999809398626971894156481321441750001229812699285374901473004231265197659290",
            10,
        )
        .unwrap();
        let leaves = [r1, r2, r3];
        let expect_hash = BigInt::parse_bytes(
            b"6310518973517441342440727149209914865806190787755638376161673961442084637476",
            10,
        )
        .unwrap();
        let leaves_hash = calc_leave_hash(&leaves);
        assert_eq!(expect_hash, leaves_hash);
    }
}
