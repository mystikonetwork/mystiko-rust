mod database;
mod error;
mod handler;
mod mystiko;

pub use database::*;
pub use error::*;
pub use handler::*;
pub use mystiko::*;

pub type Result<T> = anyhow::Result<T, MystikoError>;
