use std::any::Any;
use std::collections::HashSet;
use std::hash::Hash;
use regex::Regex;
use validator::{Validate, ValidationError};
use crate::raw::base::RawConfigTrait;

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
            if !is_valid_number_string(s, NO_SYMBOLS) {
                return Err(ValidationError::new("is number string error"));
            }
        }
        None => {
            if let Some(v) = object.downcast_ref::<Vec<String>>() {
                if EACH {
                    let is_number =
                        v.iter().all(|s| is_valid_number_string(s, NO_SYMBOLS));
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
    where T: RawConfigTrait
{
    for x in v {
        x.validate()
    }

    Ok(())
}

fn is_valid_number_string(s: &String, no_symbol: bool) -> bool {
    let mut seen_decimal = false;
    let mut seen_digit = false;
    let mut start = 0;
    if no_symbol && s.starts_with("+") || s.starts_with("-") {
        return false;
    }
    if !no_symbol && s.starts_with("+") || s.starts_with("-") {
        start = 1;
    }
    for (i, c) in s[start..].chars().enumerate() {
        if c == '.' {
            if seen_decimal || i == 0 || i == s.len() - 1 - start {
                return false;
            }
            seen_decimal = true;
        } else if !c.is_digit(10) {
            return false;
        } else {
            seen_digit = true;
        }
    }
    seen_digit
}

#[cfg(test)]
mod tests {
    use crate::raw::validator::is_number_string;

    #[test]
    fn test_is_number_string() {
        let number_vec = vec![
            String::from("10000000000000000"),
            String::from("100000000000000000"),
        ];
        is_number_string::<true, true>(&number_vec).expect("error");
    }
}