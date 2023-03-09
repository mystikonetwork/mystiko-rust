use std::fmt::{Debug};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct ValidationError {
    pub errors: Vec<String>,
}

impl ValidationError {
    pub fn new(errors: Vec<String>) -> ValidationError {
        Self { errors }
    }
}
