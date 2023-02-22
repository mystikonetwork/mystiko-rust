pub mod account;
pub mod chain;
pub mod commitment;
pub mod contract;
pub mod deposit;
pub mod nullifier;
pub mod wallet;
pub mod transaction;
use crate::filter::Filter;

pub trait Repository<T> {
    fn create(&self, t: T) -> T;
    fn create_all(&self, t: Vec<T>) -> Vec<T>;
    fn find(&self, filter: Filter) -> Option<Vec<T>>;
    fn find_one(&self, filter: Filter) -> Option<T>;
    fn update(&self, t: T) -> T;
    fn delete(&self, t: T) -> T;
}
