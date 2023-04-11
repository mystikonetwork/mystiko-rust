use crate::error::MystikoError;

pub type Result<T> = anyhow::Result<T, MystikoError>;
