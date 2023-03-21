use crate::raw::base::Validator;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::hash::Hash;
use validator::ValidationError;

lazy_static! {
    static ref ETHEREUM_ADDRESS_REGEX: Regex = Regex::new(r"^(0x)[0-9a-fA-F]{40}$").unwrap();
    static ref IS_SEM_VER: Regex = Regex::new(r"^(\d+)\.(\d+)\.(\d+)(?:-([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?(?:\+([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?$").unwrap();
    static ref NO_SYMBOL_NUMERIC: Regex = Regex::new(r"^[0-9]+$").unwrap();
    static ref NUMERIC_WITH_SYMBOL: Regex = Regex::new(r"^[+-]?([0-9]*[.])?[0-9]+$").unwrap();
}

pub fn is_ethereum_address(address: &str) -> Result<(), ValidationError> {
    if ETHEREUM_ADDRESS_REGEX.is_match(address) {
        return Ok(());
    }
    Err(ValidationError::new("ethereum address error"))
}

pub fn array_unique<T>(array: &[T]) -> Result<(), ValidationError>
    where
        T: Hash + PartialEq + Eq,
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

pub fn is_number_string<const NO_SYMBOLS: bool>(s: &String) -> Result<(), ValidationError> {
    if !is_numeric(s, NO_SYMBOLS) {
        return Err(ValidationError::new("is number string error"));
    }
    Ok(())
}

pub fn is_number_string_vec<const NO_SYMBOLS: bool>(v: &Vec<String>) -> Result<(), ValidationError> {
    let is_number = v.iter().all(|s| is_numeric(s, NO_SYMBOLS));
    if !is_number {
        return Err(ValidationError::new("is number string error"));
    }
    Ok(())
}

pub fn validate_nested_vec<T>(v: &Vec<T>) -> Result<(), ValidationError>
    where
        T: Validator,
{
    if v.iter().all(|x| x.validation().is_ok()) {
        Ok(())
    } else {
        Err(ValidationError::new("validate nested vec error"))
    }
}

pub fn string_vec_each_not_empty(v: &Vec<String>) -> Result<(), ValidationError> {
    if v.iter().all(|s| !s.is_empty()) {
        Ok(())
    } else {
        Err(ValidationError::new("vec element cannot be empty"))
    }
}

pub fn is_sem_ver(s: &str) -> Result<(), ValidationError> {
    if IS_SEM_VER.is_match(s) {
        return Ok(());
    }
    Err(ValidationError::new("SemVer is error"))
}

pub fn is_numeric(s: &str, no_symbol: bool) -> bool {
    return if no_symbol {
        NO_SYMBOL_NUMERIC.is_match(s)
    } else {
        NUMERIC_WITH_SYMBOL.is_match(s)
    };
}
