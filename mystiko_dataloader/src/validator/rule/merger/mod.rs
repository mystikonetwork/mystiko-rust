mod data;
mod error;

pub use data::*;
pub use error::*;

pub type DataMergeResult<T> = anyhow::Result<T, DataMergeError>;
