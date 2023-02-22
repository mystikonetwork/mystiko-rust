use super::Repository;
use crate::models::nullifier::Nullifier;

pub trait NullifierRepository: Repository<Nullifier> {}