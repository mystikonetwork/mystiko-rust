use std::fmt::{Debug};
use serde::{Serialize};

#[derive(Debug, Serialize)]
pub struct ValidationError {
    pub errors: Vec<String>,
}

impl ValidationError {
    pub fn new(errors: Vec<String>) -> ValidationError {
        Self { errors }
    }
}

#[derive(Debug, Serialize)]
pub struct AuxDataError {
    pub message: String,
}

impl AuxDataError {
    pub fn new(message: String) -> AuxDataError { Self { message } }
}