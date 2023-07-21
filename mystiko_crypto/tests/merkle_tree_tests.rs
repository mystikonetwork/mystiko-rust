extern crate mystiko_crypto;
extern crate num_bigint;

use num_bigint::BigUint;

use mystiko_crypto::error::MerkleTreeError;
use mystiko_crypto::merkle_tree::{calc_default_zero_element, calc_zeros, hash_two, MerkleTree};

#[tokio::test]
async fn test_calc_zeros() {
    let fist_zero = calc_default_zero_element();
    assert_eq!(
        fist_zero,
        BigUint::parse_bytes(
            b"4506069241680023110764189603658664710592327039412547147745745078424755206435",
            10,
        )
        .unwrap()
    );

    let zeros = calc_zeros(fist_zero, &32u32);
    assert_eq!(
        zeros[31],
        BigUint::parse_bytes(
            b"13202030544264649816737469308990869537826379298057211734249690002947353708909",
            10,
        )
        .unwrap()
    );
}

#[tokio::test]
async fn test_new_merkle_tree() {
    let tree1 = MerkleTree::new(None, None, None).unwrap();
    assert_eq!(
        tree1.root(),
        BigUint::parse_bytes(
            b"17749238747541177922260023106539184144732198174810064796938596694265936155259",
            10,
        )
        .unwrap()
    );

    assert_eq!(tree1.elements(), vec![]);

    let e1 = BigUint::parse_bytes(b"12d7aafbf3d4c1852ad3634d69607fc9ea8028f2d5724fcf3b917e71fd2dbff6", 16).unwrap();
    let e2 = BigUint::parse_bytes(b"062c3655c709b4b58142b9b270f5a5b06b8df8921cbbb261a7729eae759e7ec3", 16).unwrap();

    let elements = vec![e1.clone(), e2.clone()];
    let tree2 = MerkleTree::new(Some(elements.clone()), None, None).unwrap();
    assert_eq!(
        tree2.root(),
        BigUint::parse_bytes(
            b"21205178834650720622262399337497375208854240907281368468056255721030220387133",
            10,
        )
        .unwrap()
    );

    fn compare_big_int(a: &BigUint, b: &BigUint) -> bool {
        a == b
    }

    assert_eq!(tree2.elements(), elements);
    assert_eq!(tree2.index_of(&e1, None).unwrap(), 0);
    assert_eq!(tree2.index_of(&e2, Some(&compare_big_int)).unwrap(), 1);

    let zero_element = BigUint::from(0u32);
    let tree3 = MerkleTree::new(None, Some(1), Some(zero_element.clone())).unwrap();
    assert_eq!(tree3.root(), hash_two(&zero_element, &zero_element));

    let tree4 = MerkleTree::new(Some(elements), Some(0), Some(zero_element));
    assert_eq!(tree4.err().unwrap(), MerkleTreeError::MerkleTreeIsFull);
}

#[tokio::test]
async fn test_insert() {
    let e1 = BigUint::parse_bytes(b"12d7aafbf3d4c1852ad3634d69607fc9ea8028f2d5724fcf3b917e71fd2dbff6", 16).unwrap();
    let e2 = BigUint::parse_bytes(b"062c3655c709b4b58142b9b270f5a5b06b8df8921cbbb261a7729eae759e7ec3", 16).unwrap();
    let elements = vec![e1.clone()];
    let mut tree = MerkleTree::new(Some(elements), Some(0), None).unwrap();
    let result = tree.insert(e2.clone());
    assert_eq!(result.err().unwrap(), MerkleTreeError::MerkleTreeIsFull);

    let mut tree = MerkleTree::new(None, None, None).unwrap();
    tree.insert(e1).unwrap();
    tree.insert(e2).unwrap();
    assert_eq!(
        tree.root(),
        BigUint::parse_bytes(
            b"21205178834650720622262399337497375208854240907281368468056255721030220387133",
            10,
        )
        .unwrap()
    );
}

#[tokio::test]
async fn test_bulk_insert() {
    let e1 = BigUint::parse_bytes(b"12d7aafbf3d4c1852ad3634d69607fc9ea8028f2d5724fcf3b917e71fd2dbff6", 16).unwrap();
    let e2 = BigUint::parse_bytes(b"062c3655c709b4b58142b9b270f5a5b06b8df8921cbbb261a7729eae759e7ec3", 16).unwrap();
    let elements = vec![e1];
    let mut tree = MerkleTree::new(Some(elements), Some(0), None).unwrap();
    let result = tree.bulk_insert(vec![e2]);
    assert_eq!(result.err().unwrap(), MerkleTreeError::MerkleTreeIsFull);

    let mut tree = MerkleTree::new(None, None, None).unwrap();
    let e1 = BigUint::parse_bytes(b"12d7aafbf3d4c1852ad3634d69607fc9ea8028f2d5724fcf3b917e71fd2dbff6", 16).unwrap();
    let e2 = BigUint::parse_bytes(b"062c3655c709b4b58142b9b270f5a5b06b8df8921cbbb261a7729eae759e7ec3", 16).unwrap();
    let e3 = BigUint::parse_bytes(b"02d18bd99c2ce3d70411809537b64bfbbac5f51a7b7e2eeb8d84346162f9c707", 16).unwrap();

    tree.bulk_insert(vec![e1, e2, e3]).unwrap();
    assert_eq!(
        tree.root(),
        BigUint::parse_bytes(
            b"10254041194642220426314275741279894727412053938657566062675343387806484605596",
            10,
        )
        .unwrap()
    );
}

#[tokio::test]
async fn test_update() {
    let e1 = BigUint::parse_bytes(b"12d7aafbf3d4c1852ad3634d69607fc9ea8028f2d5724fcf3b917e71fd2dbff6", 16).unwrap();
    let e2 = BigUint::parse_bytes(b"02d18bd99c2ce3d70411809537b64bfbbac5f51a7b7e2eeb8d84346162f9c707", 16).unwrap();

    let mut tree = MerkleTree::new(Some(vec![e1]), None, None).unwrap();
    tree.update(0, e2.clone()).unwrap();
    assert_eq!(
        tree.root(),
        BigUint::parse_bytes(
            b"5919354211942147568484662594760486300826527524526112436647850872711338828514",
            10,
        )
        .unwrap()
    );

    let result = tree.update(2, e2);
    assert_eq!(result.err().unwrap(), MerkleTreeError::IndexOutOfBounds);
}

#[tokio::test]
async fn test_reverse_error() {
    let mut tree = MerkleTree::new(None, None, None).unwrap();
    let result = tree.revert(1);
    assert!(matches!(result.err().unwrap(), MerkleTreeError::IndexOutOfBounds));

    let result = tree.revert(0);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_tree_reverse_level_change() {
    for level in [3, 4, 5, 6, 20] {
        let e1 = BigUint::parse_bytes(b"12d7aafbf3d4c1852ad3634d69607fc9ea8028f2d5724fcf3b917e71fd2dbff6", 16).unwrap();
        let e2 = BigUint::parse_bytes(b"02d18bd99c2ce3d70411809537b64bfbbac5f51a7b7e2eeb8d84346162f9c707", 16).unwrap();
        let e3 = BigUint::parse_bytes(b"062c3655c709b4b58142b9b270f5a5b06b8df8921cbbb261a7729eae759e7ec3", 16).unwrap();
        let e4 = BigUint::parse_bytes(b"1910433764d42a31380011ba6b129d61db48c3f48a281ae277f8d5e6e8e92210", 16).unwrap();
        let e5 = BigUint::parse_bytes(b"2c4c7b8f5a000a30f4f4cffdcbe2ebb5cb8c377acaa3a6f27d137db5d0bcf108", 16).unwrap();
        let e6 = BigUint::parse_bytes(b"160461ed9f689597820c9b695469232b9c52425e227084e398dc539ff173bf60", 16).unwrap();

        let tree0 = MerkleTree::new(None, Some(level), None).unwrap();
        let tree1 = MerkleTree::new(Some(vec![e1.clone()]), Some(level), None).unwrap();
        let tree2 = MerkleTree::new(Some(vec![e1.clone(), e2.clone()]), Some(level), None).unwrap();
        let tree3 = MerkleTree::new(Some(vec![e1.clone(), e2.clone(), e3.clone()]), Some(level), None).unwrap();
        let tree4 = MerkleTree::new(
            Some(vec![e1.clone(), e2.clone(), e3.clone(), e4.clone()]),
            Some(level),
            None,
        )
        .unwrap();
        let tree5 = MerkleTree::new(
            Some(vec![e1.clone(), e2.clone(), e3.clone(), e4.clone(), e5.clone()]),
            Some(level),
            None,
        )
        .unwrap();
        let mut tree6 = MerkleTree::new(
            Some(vec![
                e1.clone(),
                e2.clone(),
                e3.clone(),
                e4.clone(),
                e5.clone(),
                e6.clone(),
            ]),
            Some(level),
            None,
        )
        .unwrap();
        let result = tree6.revert(5);
        assert!(result.is_ok());
        assert_eq!(tree6.root(), tree5.root());
        let result = tree6.revert(4);
        assert!(result.is_ok());
        assert_eq!(tree6.root(), tree4.root());
        let result = tree6.revert(3);
        assert!(result.is_ok());
        assert_eq!(tree6.root(), tree3.root());
        let result = tree6.revert(2);
        assert!(result.is_ok());
        assert_eq!(tree6.root(), tree2.root());
        let result = tree6.revert(1);
        assert!(result.is_ok());
        assert_eq!(tree6.root(), tree1.root());
        let result = tree6.revert(0);
        assert!(result.is_ok());
        assert_eq!(tree6.root(), tree0.root());
    }
}

#[tokio::test]
async fn test_tree_reverse_element_change() {
    for count in 0..33 {
        for step in 0..count + 1 {
            let e1 =
                BigUint::parse_bytes(b"12d7aafbf3d4c1852ad3634d69607fc9ea8028f2d5724fcf3b917e71fd2dbff6", 16).unwrap();

            let tree_base = MerkleTree::new(Some(vec![e1.clone(); step]), Some(5), None).unwrap();
            let mut tree = MerkleTree::new(Some(vec![e1.clone(); count]), Some(5), None).unwrap();
            tree.revert(step).unwrap();
            assert_eq!(tree.root(), tree_base.root());
        }
    }
}

#[tokio::test]
async fn test_path() {
    let e1 = BigUint::parse_bytes(b"12d7aafbf3d4c1852ad3634d69607fc9ea8028f2d5724fcf3b917e71fd2dbff6", 16).unwrap();
    let e2 = BigUint::parse_bytes(b"062c3655c709b4b58142b9b270f5a5b06b8df8921cbbb261a7729eae759e7ec3", 16).unwrap();
    let e3 = BigUint::parse_bytes(b"02d18bd99c2ce3d70411809537b64bfbbac5f51a7b7e2eeb8d84346162f9c707", 16).unwrap();

    let tree = MerkleTree::new(Some(vec![e1.clone(), e2.clone(), e3.clone()]), Some(2), None).unwrap();
    let default_zero = calc_default_zero_element();
    let result1 = tree.path(0).unwrap();
    assert_eq!(result1.1, vec![0, 0]);
    assert_eq!(result1.0, vec![e2.clone(), hash_two(&e3, &default_zero)]);

    let result2 = tree.path(1).unwrap();
    assert_eq!(result2.1, vec![1, 0]);
    assert_eq!(result2.0, vec![e1.clone(), hash_two(&e3, &default_zero)]);

    let result3 = tree.path(2).unwrap();
    assert_eq!(result3.1, vec![0, 1]);
    assert_eq!(result3.0, vec![default_zero, hash_two(&e1, &e2)]);

    let result4 = tree.path(4);
    assert!(matches!(result4.err().unwrap(), MerkleTreeError::IndexOutOfBounds));
}
