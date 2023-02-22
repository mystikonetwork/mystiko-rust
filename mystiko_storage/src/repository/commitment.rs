use super::Repository;
use crate::models::commitment::Commitment;

pub trait CommitmentRepository: Repository<Commitment> {}