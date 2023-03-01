use crate::error::ProtocolError;
use crate::mystiko_crypto::zkp::types::ZKProof;
use crate::utils::u256_to_big_int;
use crate::utils::{big_int_to_u256, u256_to_big_endian_fixed_bytes};
use ethers::core::types::U256;
use mystiko_abi::zk_rollup::rollup::RollupCall;
use mystiko_crypto::constants::FIELD_SIZE;
use mystiko_crypto::merkle_tree::MerkleTree;
use mystiko_crypto::utils::calc_mod;
use mystiko_crypto::zkp::prove::prove_by_file;
use num_bigint::{BigInt, Sign};
use sha3::{Digest, Keccak256};

pub struct Rollup {
    tree: MerkleTree,
    new_leaves: Vec<U256>,
    program_file: String,
    abi_file: String,
    proving_key_file: String,
}

fn is_power_of_two(a_number: usize) -> bool {
    a_number != 0 && (a_number & (a_number - 1)) == 0
}

fn path_indices_number(path_indices: &[usize]) -> U256 {
    let binary_string = path_indices
        .iter()
        .rev()
        .map(|x| format!("{:b}", x))
        .collect::<String>();
    let n = BigInt::parse_bytes(binary_string.as_bytes(), 2).unwrap();
    big_int_to_u256(&n)
}

fn calc_leave_hash(leaves: &[U256]) -> U256 {
    let leaf_buffer = leaves
        .iter()
        .flat_map(u256_to_big_endian_fixed_bytes)
        .collect::<Vec<u8>>();
    let result = Keccak256::digest(leaf_buffer);
    let hash = BigInt::from_bytes_be(Sign::Plus, &result);
    let hash = calc_mod(hash, &FIELD_SIZE);
    big_int_to_u256(&hash)
}

pub fn zk_prove_rollup(rollup: &mut Rollup) -> Result<ZKProof, ProtocolError> {
    let new_leaves = rollup.new_leaves.clone();
    let rollup_size = new_leaves.len();
    assert!(is_power_of_two(rollup_size));
    let rollup_height = (rollup_size as f64).log2().round() as usize;
    let current_leaf_count = rollup.tree.count();
    assert_eq!(current_leaf_count % rollup_size, 0);
    let current_root = rollup.tree.root();
    let leaves = new_leaves.iter().map(u256_to_big_int).collect();
    // todo check insert result
    rollup.tree.bulk_insert(leaves).unwrap();
    let new_root = rollup.tree.root();
    let leaf_path = rollup.tree.path(current_leaf_count).unwrap();
    let (_, path_indices) = leaf_path.1.split_at(rollup_height);
    let path_indices = path_indices_number(path_indices);
    let (_, path_elements) = leaf_path.0.split_at(rollup_height);
    let path_elements = path_elements.iter().map(big_int_to_u256).collect();
    let leaves_hash = calc_leave_hash(new_leaves.as_slice());

    let call = RollupCall {
        old_root: big_int_to_u256(&current_root),
        new_root: big_int_to_u256(&new_root),
        leaves_hash,
        path_indices,
        path_elements,
        leaves: new_leaves,
    };

    let mut array: Vec<serde_json::Value> = vec![serde_json::json!(call.old_root)];
    array.push(serde_json::json!(call.new_root));
    array.push(serde_json::json!(call.leaves_hash));
    array.push(serde_json::json!(call.path_indices));
    array.push(serde_json::json!(call.path_elements));
    array.push(serde_json::json!(call.leaves));
    let input = serde_json::Value::Array(array).to_string();

    let proof = prove_by_file(
        &rollup.program_file,
        &rollup.abi_file,
        &rollup.proving_key_file,
        &input,
    )
    .unwrap();

    Ok(proof)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::verify::zk_verify;

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
        let expect_indices = U256::from(1373);
        let indices = path_indices_number(&path);
        assert_eq!(indices, expect_indices);

        let path = [0, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1];
        let expect_indices = U256::from(1492);
        let indices = path_indices_number(&path);
        assert_eq!(indices, expect_indices);
    }

    #[test]
    fn test_calc_leave_hash() {
        let r = BigInt::parse_bytes(
            b"5999809398626971894156481321441750001229812699285374901473004231265197659290",
            10,
        )
        .unwrap();
        let r = big_int_to_u256(&r);
        let leaves = [U256::from(0x1234), U256::from(0x4321), r];
        let expect_hash = BigInt::parse_bytes(
            b"14284422019051201358174527051990313509342483281784737317742104747058327943770",
            10,
        )
        .unwrap();
        let expect_hash = big_int_to_u256(&expect_hash);
        let leaves_hash = calc_leave_hash(&leaves);
        assert_eq!(expect_hash, leaves_hash);
    }

    const FILE_PATH: &str = "./../mystiko-circuits/dist/zokrates/dev";

    #[test]
    fn test_rollup1() {
        let in_initial_elements =
            [BigInt::from(100), BigInt::from(200), BigInt::from(300)].to_vec();
        let tree = MerkleTree::new(Some(in_initial_elements), None, None).unwrap();

        let mut rollup = Rollup {
            tree,
            new_leaves: vec![U256::from(1)],
            program_file: (FILE_PATH.to_owned() + "/Rollup1.program").to_string(),
            abi_file: (FILE_PATH.to_owned() + "/Rollup1.abi.json").to_string(),
            proving_key_file: (FILE_PATH.to_owned() + "/Rollup1.pkey").to_string(),
        };
        let proof = zk_prove_rollup(&mut rollup).unwrap();
        let verify = zk_verify(proof, &(FILE_PATH.to_owned() + "/Rollup1.vkey")).unwrap();
        assert!(verify);
    }

    #[test]
    fn test_rollup2() {
        let in_initial_elements = [BigInt::from(100), BigInt::from(200)].to_vec();
        let tree = MerkleTree::new(Some(in_initial_elements), None, None).unwrap();

        let mut rollup = Rollup {
            tree,
            new_leaves: vec![U256::from(1), U256::from(2)],
            program_file: (FILE_PATH.to_owned() + "/Rollup2.program").to_string(),
            abi_file: (FILE_PATH.to_owned() + "/Rollup2.abi.json").to_string(),
            proving_key_file: (FILE_PATH.to_owned() + "/Rollup2.pkey").to_string(),
        };
        let proof = zk_prove_rollup(&mut rollup).unwrap();
        let verify = zk_verify(proof, &(FILE_PATH.to_owned() + "/Rollup2.vkey")).unwrap();
        assert!(verify);
    }

    #[test]
    fn test_rollup4() {
        let in_initial_elements = [
            BigInt::from(100),
            BigInt::from(200),
            BigInt::from(300),
            BigInt::from(400),
        ]
        .to_vec();
        let tree = MerkleTree::new(Some(in_initial_elements), None, None).unwrap();

        let mut rollup = Rollup {
            tree,
            new_leaves: vec![U256::from(1), U256::from(2), U256::from(3), U256::from(4)],
            program_file: (FILE_PATH.to_owned() + "/Rollup4.program").to_string(),
            abi_file: (FILE_PATH.to_owned() + "/Rollup4.abi.json").to_string(),
            proving_key_file: (FILE_PATH.to_owned() + "/Rollup4.pkey").to_string(),
        };
        let proof = zk_prove_rollup(&mut rollup).unwrap();
        let verify = zk_verify(proof, &(FILE_PATH.to_owned() + "/Rollup4.vkey")).unwrap();
        assert!(verify);
    }

    #[test]
    fn test_rollup8() {
        let in_initial_elements = [
            BigInt::from(100),
            BigInt::from(200),
            BigInt::from(300),
            BigInt::from(400),
            BigInt::from(500),
            BigInt::from(600),
            BigInt::from(700),
            BigInt::from(800),
        ]
        .to_vec();
        let tree = MerkleTree::new(Some(in_initial_elements), None, None).unwrap();

        let mut rollup = Rollup {
            tree,
            new_leaves: vec![
                U256::from(1),
                U256::from(2),
                U256::from(3),
                U256::from(4),
                U256::from(5),
                U256::from(6),
                U256::from(7),
                U256::from(8),
            ],
            program_file: (FILE_PATH.to_owned() + "/Rollup8.program").to_string(),
            abi_file: (FILE_PATH.to_owned() + "/Rollup8.abi.json").to_string(),
            proving_key_file: (FILE_PATH.to_owned() + "/Rollup8.pkey").to_string(),
        };
        let proof = zk_prove_rollup(&mut rollup).unwrap();
        let verify = zk_verify(proof, &(FILE_PATH.to_owned() + "/Rollup8.vkey")).unwrap();
        assert!(verify);
    }

    #[test]
    fn test_rollup16() {
        let in_initial_elements = [
            BigInt::from(100),
            BigInt::from(200),
            BigInt::from(300),
            BigInt::from(400),
            BigInt::from(500),
            BigInt::from(600),
            BigInt::from(700),
            BigInt::from(800),
            BigInt::from(900),
            BigInt::from(1000),
            BigInt::from(1100),
            BigInt::from(1200),
            BigInt::from(1300),
            BigInt::from(1400),
            BigInt::from(1500),
            BigInt::from(1600),
        ]
        .to_vec();
        let tree = MerkleTree::new(Some(in_initial_elements), None, None).unwrap();

        let mut rollup = Rollup {
            tree,
            new_leaves: vec![
                U256::from(1),
                U256::from(2),
                U256::from(3),
                U256::from(4),
                U256::from(5),
                U256::from(6),
                U256::from(7),
                U256::from(8),
                U256::from(9),
                U256::from(10),
                U256::from(11),
                U256::from(12),
                U256::from(13),
                U256::from(14),
                U256::from(15),
                U256::from(16),
            ],
            program_file: (FILE_PATH.to_owned() + "/Rollup16.program").to_string(),
            abi_file: (FILE_PATH.to_owned() + "/Rollup16.abi.json").to_string(),
            proving_key_file: (FILE_PATH.to_owned() + "/Rollup16.pkey").to_string(),
        };
        let proof = zk_prove_rollup(&mut rollup).unwrap();
        let verify = zk_verify(proof, &(FILE_PATH.to_owned() + "/Rollup16.vkey")).unwrap();
        assert!(verify);
    }
}
