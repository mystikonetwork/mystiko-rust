use crate::constants::FIELD_SIZE;
use crate::error::MerkleTreeError;
use crate::hash::{keccak256, poseidon_fr};
use anyhow::Result;
use ff::*;
use num_bigint::BigUint;
use num_integer::Integer;
use poseidon_rs::Fr;

type CompareFn = dyn Fn(&BigUint, &BigUint) -> bool;

#[derive(Debug, Clone)]
pub struct MerkleTree {
    max_levels: u32,
    capacity: u32,
    zeros: Vec<BigUint>,
    layers: Vec<Vec<BigUint>>,
}

impl MerkleTree {
    pub fn new(
        in_initial_elements: Option<Vec<BigUint>>,
        in_max_levels: Option<u32>,
        in_zero_element: Option<BigUint>,
    ) -> Result<Self, MerkleTreeError> {
        let initial_elements = in_initial_elements.unwrap_or_default();
        let max_levels = in_max_levels.unwrap_or(20);
        let zero_element = in_zero_element.unwrap_or_else(calc_default_zero_element);

        let capacity = 2u32.pow(max_levels);
        if capacity < initial_elements.len() as u32 {
            return Err(MerkleTreeError::MerkleTreeIsFull);
        }

        let zeros = calc_zeros(zero_element, &max_levels);
        let layers: Vec<Vec<BigUint>> = vec![initial_elements];

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
        self.layers.reserve(self.max_levels as usize);
        for level in 1..(self.max_levels + 1) as usize {
            let mut new_layer = vec![];
            let len = self.layers[level - 1].len();
            if len != 0 {
                let count: usize = (len + 1) / 2;
                new_layer.reserve(count);
                for i in 0..count {
                    let first = &self.layers[level - 1][i * 2];
                    let second = if i * 2 + 1 < len {
                        &self.layers[level - 1][i * 2 + 1]
                    } else {
                        &self.zeros[level - 1]
                    };

                    new_layer.push(hash_two(first, second));
                }
            }
            self.layers.push(new_layer);
        }
    }

    pub fn root(&self) -> BigUint {
        let levels = self.max_levels as usize;
        if self.layers[levels].is_empty() {
            self.zeros[levels].clone()
        } else {
            self.layers[levels][0].clone()
        }
    }

    pub fn insert(&mut self, element: BigUint) -> Result<(), MerkleTreeError> {
        if self.layers[0].len() >= self.capacity as usize {
            return Err(MerkleTreeError::MerkleTreeIsFull);
        }

        self.update(self.layers[0].len(), element)
    }

    pub fn bulk_insert(&mut self, elements: Vec<BigUint>) -> Result<(), MerkleTreeError> {
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
                let ph = hash_two(
                    &self.layers[level - 1][index * 2],
                    &self.layers[level - 1][index * 2 + 1],
                );

                self.update_layers(level, index, ph);
                i = index;
            }
        }

        self.insert(elements[elements.len() - 1].clone())
    }

    pub fn revert(&mut self, count: usize) -> Result<(), MerkleTreeError> {
        let layer_size = self.layers[0].len();
        if count > layer_size {
            return Err(MerkleTreeError::IndexOutOfBounds);
        }

        if count == layer_size {
            return Ok(());
        }

        self.layers[0].truncate(count);
        let mut new_layer_size = count;
        for level in 1..(self.max_levels + 1) as usize {
            if new_layer_size == 0 {
                self.layers[level] = vec![];
            } else {
                new_layer_size >>= 1;
                assert!(self.layers[level].len() >= new_layer_size);
                self.layers[level].truncate(new_layer_size);
            }
        }

        if !self.layers[0].is_empty() {
            let index = self.layers[0].len() - 1;
            let elem = self.layers[0][index].clone();
            self.update(index, elem).unwrap();
        }

        Ok(())
    }

    fn update_layers(&mut self, level: usize, index: usize, element: BigUint) {
        assert!(index <= self.layers[level].len());
        if index < self.layers[level].len() {
            self.layers[level][index] = element;
        } else {
            self.layers[level].push(element);
        }
    }

    pub fn update(&mut self, index: usize, element: BigUint) -> Result<(), MerkleTreeError> {
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

            let ph = hash_two(first, second);
            self.update_layers(level, current_index, ph);
        }
        Ok(())
    }

    pub fn path(&self, index: usize) -> Result<(Vec<BigUint>, Vec<usize>), MerkleTreeError> {
        if index > self.layers[0].len() {
            return Err(MerkleTreeError::IndexOutOfBounds);
        }

        let mut path_elements: Vec<BigUint> = vec![];
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

    pub fn elements(&self) -> Vec<BigUint> {
        self.layers[0].clone()
    }

    pub fn count(&self) -> usize {
        self.layers[0].len()
    }

    pub fn index_of(&self, element: &BigUint, comparator: Option<&CompareFn>) -> Option<usize> {
        match comparator {
            Some(cmp) => self.layers[0].iter().position(|el| cmp(element, el)),
            None => self.layers[0].iter().position(|value| value.eq(element)),
        }
    }
}

pub fn calc_default_zero_element() -> BigUint {
    let input = b"Welcome To Mystiko's Magic World!";
    let seed_hash = keccak256(input);
    let seed_bigint = BigUint::from_bytes_be(&seed_hash);
    seed_bigint.mod_floor(&FIELD_SIZE)
}

pub fn hash_two(first: &BigUint, second: &BigUint) -> BigUint {
    let b1: Fr = Fr::from_str(&first.to_string()).unwrap();
    let b2: Fr = Fr::from_str(&second.to_string()).unwrap();
    poseidon_fr(&[b1, b2])
}

pub fn calc_zeros(first_zero: BigUint, levels: &u32) -> Vec<BigUint> {
    let mut z: Vec<BigUint> = vec![];
    z.push(first_zero);
    for i in 1..(levels + 1) as usize {
        z.push(hash_two(&z[i - 1], &z[i - 1]));
    }
    z
}
