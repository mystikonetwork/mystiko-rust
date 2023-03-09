use std::any::Any;
use std::collections::HashSet;
use std::hash::Hash;
use regex::Regex;
use validator::{ValidationError};
use crate::raw::base::Validator;

pub fn is_ethereum_address(address: &str) -> Result<(), ValidationError> {
    let eth = Regex::new(r"^(0x)[0-9a-fA-F]{40}$").unwrap();
    if eth.is_match(address) {
        return Ok(());
    }
    Err(ValidationError::new("ethereum address error"))
}

pub fn array_unique<T>(array: &[T]) -> Result<(), ValidationError>
    where T: Hash + PartialEq + Eq
{
    let mut seen = HashSet::new();
    for item in array {
        if seen.contains(item) {
            return Err(ValidationError::new("array is not unique"));
        }
        seen.insert(item);
    }
    Ok(())
}

pub fn is_number_string<const NO_SYMBOLS: bool, const EACH: bool>(
    object: &dyn Any,
) -> Result<(), ValidationError> {
    match object.downcast_ref::<String>() {
        Some(s) => {
            if !is_numeric(s, NO_SYMBOLS) {
                return Err(ValidationError::new("is number string error"));
            }
        }
        None => {
            if let Some(v) = object.downcast_ref::<Vec<String>>() {
                if EACH {
                    let is_number =
                        v.iter().all(|s| is_numeric(s, NO_SYMBOLS));
                    if !is_number {
                        return Err(ValidationError::new("is number string error"));
                    }
                } else {
                    return Err(ValidationError::new("is number string error"));
                }
            } else {
                return Err(ValidationError::new("is number string error"));
            }
        }
    }
    Ok(())
}

pub fn validate_nested_vec<T>(v: &Vec<T>) -> Result<(), ValidationError>
    where T: Validator
{
    for x in v {
        let result = x.validation();
        if result.is_err() {
            return Err(ValidationError::new("validate nested vec error"));
        }
    }

    Ok(())
}

pub fn string_vec_each_not_empty(v: &Vec<String>) -> Result<(), ValidationError> {
    for s in v {
        if s.is_empty() {
            return Err(ValidationError::new("vec element cannot be empty"));
        }
    }

    Ok(())
}

pub fn is_sem_ver(s: &str) -> Result<(), ValidationError> {
    let re = Regex::new(r"^(\d+)\.(\d+)\.(\d+)(?:-([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?(?:\+([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?$").unwrap();
    if re.is_match(s) {
        return Ok(());
    }
    Err(ValidationError::new("SemVer is error"))
}

pub fn is_numeric(s: &String, no_symbol: bool) -> bool {
    let re = if no_symbol {
        Regex::new(r"^[0-9]+$").unwrap()
    } else {
        Regex::new(r"^[+-]?([0-9]*[.])?[0-9]+$").unwrap()
    };
    re.is_match(s)
}
