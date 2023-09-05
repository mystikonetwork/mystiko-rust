mod error;
mod handler;
mod mystiko;

pub use error::*;
pub use handler::*;
pub use mystiko::*;

pub type Result<T> = anyhow::Result<T, MystikoError>;
