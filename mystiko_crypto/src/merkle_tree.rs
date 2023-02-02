use crate::error::MerkleTreeError;
use crate::utils::poseidon_hash;
use ethers::core::utils::keccak256;
use ff::*;
use mystiko_utils::constants::FIELD_SIZE;
use num_bigint::{BigInt, Sign};
use num_integer::Integer;
use poseidon_rs::Fr;

type CompareFn = dyn Fn(&BigInt, &BigInt) -> bool;

fn calc_default_zero_element() -> BigInt {
    let input = b"Welcome To Mystiko's Magic World!";
    let seed_hash = keccak256(input);
    let seed_bigint = BigInt::from_bytes_be(Sign::Plus, &seed_hash);
    seed_bigint.mod_floor(&FIELD_SIZE)
}

fn hash2(first: &BigInt, second: &BigInt) -> BigInt {
    let b1: Fr = Fr::from_str(&first.to_string()).unwrap();
    let b2: Fr = Fr::from_str(&second.to_string()).unwrap();
    let arr: Vec<Fr> = vec![b1, b2];

    poseidon_hash(arr)
}

fn calc_zeros(first_zero: BigInt, levels: &u32) -> Vec<BigInt> {
    let mut z: Vec<BigInt> = vec![];
    z.push(first_zero);
    for i in 1..(levels + 1) as usize {
        z.push(hash2(&z[i - 1], &z[i - 1]));
    }
    z
}

pub struct MerkleTree {
    max_levels: u32,
    capacity: u32,
    zeros: Vec<BigInt>,
    layers: Vec<Vec<BigInt>>,
}

impl MerkleTree {
    pub fn new(
        in_initial_elements: Option<Vec<BigInt>>,
        in_max_levels: Option<u32>,
        in_zero_element: Option<BigInt>,
    ) -> Result<Self, MerkleTreeError> {
        let initial_elements = match in_initial_elements {
            Some(a) => a,
            _ => vec![],
        };

        let max_levels = match in_max_levels {
            Some(a) => a,
            _ => 20,
        };

        let zero_element = match in_zero_element {
            Some(a) => a,
            _ => calc_default_zero_element(),
        };

        let capacity = 2u32.pow(max_levels);
        if capacity < initial_elements.len() as u32 {
            return Err(MerkleTreeError::MerkleTreeIsFull);
        }

        let zeros = calc_zeros(zero_element, &max_levels);
        let layers: Vec<Vec<BigInt>> = vec![initial_elements];

        let mut s = Self {
            max_levels,
            capacity,
            zeros,
            layers,
        };
        s.rebuild();
        Ok(s)
    }

    fn rebuild(&mut self) {
        for level in 1..(self.max_levels + 1) as usize {
            self.layers.push(vec![]);
            let len = self.layers[level - 1].len();
            if len == 0 {
                continue;
            }

            let count: usize = (len + 1) / 2;
            for i in 0..count {
                let first = self.layers[level - 1][i * 2].clone();
                let second = if i * 2 + 1 < len {
                    self.layers[level - 1][i * 2 + 1].clone()
                } else {
                    self.zeros[level - 1].clone()
                };

                self.layers[level].push(hash2(&first, &second));
            }
        }
    }

    pub fn root(&self) -> BigInt {
        let levels = self.max_levels as usize;
        if self.layers[levels].is_empty() {
            self.zeros[levels].clone()
        } else {
            self.layers[levels][0].clone()
        }
    }

    pub fn insert(&mut self, element: BigInt) -> Result<(), MerkleTreeError> {
        if self.layers[0].len() >= self.capacity as usize {
            return Err(MerkleTreeError::MerkleTreeIsFull);
        }

        self.update(self.layers[0].len(), element)
    }

    pub fn bulk_insert(&mut self, elements: Vec<BigInt>) -> Result<(), MerkleTreeError> {
        if self.layers[0].len() + elements.len() > self.capacity as usize {
            return Err(MerkleTreeError::MerkleTreeIsFull);
        }

        for element in elements.iter().take(elements.len() - 1) {
            self.layers[0].push(element.clone());
            let mut level = 0;
            let mut index = self.layers[0].len() - 1;
            let mut i = index;
            while i % 2 == 1 {
                level += 1;
                index >>= 1;
                let ph = hash2(
                    &self.layers[level - 1][index * 2],
                    &self.layers[level - 1][index * 2 + 1],
                );

                self.update_layers(level, index, ph);
                i = index;
            }
        }

        self.insert(elements[elements.len() - 1].clone())
    }

    fn update_layers(&mut self, level: usize, index: usize, element: BigInt) {
        assert!(index <= self.layers[level].len());
        if index < self.layers[level].len() {
            self.layers[level][index] = element;
        } else {
            self.layers[level].push(element);
        }
    }

    pub fn update(&mut self, index: usize, element: BigInt) -> Result<(), MerkleTreeError> {
        if index > self.layers[0].len() || index >= self.capacity as usize {
            return Err(MerkleTreeError::IndexOutOfBounds);
        }

        self.update_layers(0, index, element);
        let mut current_index = index;
        for level in 1..(self.max_levels + 1) as usize {
            current_index >>= 1;
            let first = &self.layers[level - 1][current_index * 2];
            let second = if current_index * 2 + 1 < self.layers[level - 1].len() {
                &self.layers[level - 1][current_index * 2 + 1]
            } else {
                &self.zeros[level - 1]
            };

            let ph = hash2(first, second);
            self.update_layers(level, current_index, ph);
        }
        Ok(())
    }

    pub fn path(&self, index: usize) -> Result<(Vec<BigInt>, Vec<usize>), MerkleTreeError> {
        if index > self.layers[0].len() {
            return Err(MerkleTreeError::IndexOutOfBounds);
        }

        let mut path_elements: Vec<BigInt> = vec![];
        let mut path_indices: Vec<usize> = vec![];
        let mut current_index = index;
        for level in 0..self.max_levels as usize {
            path_indices.push(current_index % 2);
            if (current_index ^ 1) < self.layers[level].len() {
                path_elements.push(self.layers[level][current_index ^ 1].clone());
            } else {
                path_elements.push(self.zeros[level].clone());
            }

            current_index >>= 1;
        }

        Ok((path_elements, path_indices))
    }

    pub fn elements(&self) -> Vec<BigInt> {
        self.layers[0].clone()
    }

    pub fn index_of(&self, element: &BigInt, comparator: Option<&CompareFn>) -> Option<usize> {
        match comparator {
            Some(cmp) => self.layers[0].iter().position(|el| cmp(element, el)),
            None => self.layers[0].iter().position(|value| value.eq(element)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_zeros() {
        let fist_zero = calc_default_zero_element();
        assert_eq!(
            fist_zero,
            BigInt::parse_bytes(
                b"4506069241680023110764189603658664710592327039412547147745745078424755206435",
                10
            )
            .unwrap()
        );

        let zeros = calc_zeros(fist_zero, &(32 as u32));
        assert_eq!(
            zeros[31],
            BigInt::parse_bytes(
                b"13202030544264649816737469308990869537826379298057211734249690002947353708909",
                10
            )
            .unwrap()
        );
    }

    #[test]
    fn test_new_merkle_tree() {
        let tree1 = MerkleTree::new(None, None, None).unwrap();
        assert_eq!(
            tree1.root(),
            BigInt::parse_bytes(
                b"17749238747541177922260023106539184144732198174810064796938596694265936155259",
                10
            )
            .unwrap()
        );

        assert_eq!(tree1.elements(), vec![]);

        let e1 = BigInt::parse_bytes(
            b"12d7aafbf3d4c1852ad3634d69607fc9ea8028f2d5724fcf3b917e71fd2dbff6",
            16,
        )
        .unwrap();
        let e2 = BigInt::parse_bytes(
            b"062c3655c709b4b58142b9b270f5a5b06b8df8921cbbb261a7729eae759e7ec3",
            16,
        )
        .unwrap();

        let elements = vec![e1.clone(), e2.clone()];
        let tree2 = MerkleTree::new(Some(elements.clone()), None, None).unwrap();
        assert_eq!(
            tree2.root(),
            BigInt::parse_bytes(
                b"21205178834650720622262399337497375208854240907281368468056255721030220387133",
                10
            )
            .unwrap()
        );

        fn compare_big_int(a: &BigInt, b: &BigInt) -> bool {
            a == b
        }

        assert_eq!(tree2.elements(), elements);
        assert_eq!(tree2.index_of(&e1, None).unwrap(), 0);
        assert_eq!(tree2.index_of(&e2, Some(&compare_big_int)).unwrap(), 1);

        let zero_element = BigInt::from(0 as u32);
        let tree3 = MerkleTree::new(None, Some(1), Some(zero_element.clone())).unwrap();
        assert_eq!(
            tree3.root(),
            hash2(&zero_element.clone(), &zero_element.clone())
        );

        let tree4 = MerkleTree::new(Some(elements.clone()), Some(0), Some(zero_element.clone()));
        assert_eq!(tree4.err().unwrap(), MerkleTreeError::MerkleTreeIsFull);
    }

    #[test]
    fn test_insert() {
        let e1 = BigInt::parse_bytes(
            b"12d7aafbf3d4c1852ad3634d69607fc9ea8028f2d5724fcf3b917e71fd2dbff6",
            16,
        )
        .unwrap();
        let e2 = BigInt::parse_bytes(
            b"062c3655c709b4b58142b9b270f5a5b06b8df8921cbbb261a7729eae759e7ec3",
            16,
        )
        .unwrap();
        let elements = vec![e1];
        let mut tree = MerkleTree::new(Some(elements), Some(0), None).unwrap();
        let result = tree.insert(e2);
        assert_eq!(result.err().unwrap(), MerkleTreeError::MerkleTreeIsFull);

        let mut tree = MerkleTree::new(None, None, None).unwrap();
        let e1 = BigInt::parse_bytes(
            b"12d7aafbf3d4c1852ad3634d69607fc9ea8028f2d5724fcf3b917e71fd2dbff6",
            16,
        )
        .unwrap();
        let e2 = BigInt::parse_bytes(
            b"062c3655c709b4b58142b9b270f5a5b06b8df8921cbbb261a7729eae759e7ec3",
            16,
        )
        .unwrap();

        tree.insert(e1).unwrap();
        tree.insert(e2).unwrap();
        assert_eq!(
            tree.root(),
            BigInt::parse_bytes(
                b"21205178834650720622262399337497375208854240907281368468056255721030220387133",
                10
            )
            .unwrap()
        );
    }

    #[test]
    fn test_bulk_insert() {
        let e1 = BigInt::parse_bytes(
            b"12d7aafbf3d4c1852ad3634d69607fc9ea8028f2d5724fcf3b917e71fd2dbff6",
            16,
        )
        .unwrap();
        let e2 = BigInt::parse_bytes(
            b"062c3655c709b4b58142b9b270f5a5b06b8df8921cbbb261a7729eae759e7ec3",
            16,
        )
        .unwrap();
        let elements = vec![e1];
        let mut tree = MerkleTree::new(Some(elements), Some(0), None).unwrap();
        let result = tree.bulk_insert(vec![e2]);
        assert_eq!(result.err().unwrap(), MerkleTreeError::MerkleTreeIsFull);

        let mut tree = MerkleTree::new(None, None, None).unwrap();
        let e1 = BigInt::parse_bytes(
            b"12d7aafbf3d4c1852ad3634d69607fc9ea8028f2d5724fcf3b917e71fd2dbff6",
            16,
        )
        .unwrap();
        let e2 = BigInt::parse_bytes(
            b"062c3655c709b4b58142b9b270f5a5b06b8df8921cbbb261a7729eae759e7ec3",
            16,
        )
        .unwrap();
        let e3 = BigInt::parse_bytes(
            b"02d18bd99c2ce3d70411809537b64bfbbac5f51a7b7e2eeb8d84346162f9c707",
            16,
        )
        .unwrap();

        tree.bulk_insert(vec![e1, e2, e3]).unwrap();
        assert_eq!(
            tree.root(),
            BigInt::parse_bytes(
                b"10254041194642220426314275741279894727412053938657566062675343387806484605596",
                10
            )
            .unwrap()
        );
    }

    #[test]
    fn test_update() {
        let e1 = BigInt::parse_bytes(
            b"12d7aafbf3d4c1852ad3634d69607fc9ea8028f2d5724fcf3b917e71fd2dbff6",
            16,
        )
        .unwrap();
        let e2 = BigInt::parse_bytes(
            b"02d18bd99c2ce3d70411809537b64bfbbac5f51a7b7e2eeb8d84346162f9c707",
            16,
        )
        .unwrap();

        let mut tree = MerkleTree::new(Some(vec![e1]), None, None).unwrap();
        tree.update(0, e2.clone()).unwrap();
        assert_eq!(
            tree.root(),
            BigInt::parse_bytes(
                b"5919354211942147568484662594760486300826527524526112436647850872711338828514",
                10
            )
            .unwrap()
        );

        let result = tree.update(2, e2);
        assert_eq!(result.err().unwrap(), MerkleTreeError::IndexOutOfBounds);
    }

    #[test]
    fn test_path() {
        let e1 = BigInt::parse_bytes(
            b"12d7aafbf3d4c1852ad3634d69607fc9ea8028f2d5724fcf3b917e71fd2dbff6",
            16,
        )
        .unwrap();
        let e2 = BigInt::parse_bytes(
            b"062c3655c709b4b58142b9b270f5a5b06b8df8921cbbb261a7729eae759e7ec3",
            16,
        )
        .unwrap();
        let e3 = BigInt::parse_bytes(
            b"02d18bd99c2ce3d70411809537b64bfbbac5f51a7b7e2eeb8d84346162f9c707",
            16,
        )
        .unwrap();

        let tree = MerkleTree::new(
            Some(vec![e1.clone(), e2.clone(), e3.clone()]),
            Some(2),
            None,
        )
        .unwrap();
        let default_zero = calc_default_zero_element();
        let result1 = tree.path(0).unwrap();
        assert_eq!(result1.1, vec![0, 0]);
        assert_eq!(result1.0, vec![e2.clone(), hash2(&e3, &default_zero)]);

        let result2 = tree.path(1).unwrap();
        assert_eq!(result2.1, vec![1, 0]);
        assert_eq!(result2.0, vec![e1.clone(), hash2(&e3, &default_zero)]);

        let result3 = tree.path(2).unwrap();
        assert_eq!(result3.1, vec![0, 1]);
        assert_eq!(result3.0, vec![default_zero.clone(), hash2(&e1, &e2)]);

        let result4 = tree.path(4);
        assert_eq!(result4.err().unwrap(), MerkleTreeError::IndexOutOfBounds);
    }
}
