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
    where T: RawConfigTrait
{
    for x in v {
        x.validate()
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

fn is_numeric(s: &String, no_symbol: bool) -> bool {
    let re = if no_symbol {
        Regex::new(r"^[0-9]+$").unwrap()
    } else {
        Regex::new(r"^[+-]?([0-9]*[.])?[0-9]+$").unwrap()
    };
    re.is_match(s)
}

#[cfg(test)]
mod tests {
    use crate::raw::validator::{is_number_string, is_numeric};

    #[test]
    fn test_is_number_string() {
        let number_vec = vec![
            String::from("10000000000000000"),
            String::from("100000000000000000"),
        ];
        is_number_string::<true, true>(&number_vec).expect("error");
    }

    #[test]
    fn test_is_numeric() {
        let mut s = String::from("100");
        assert_eq!(is_numeric(&s, true), true);
        s = String::from("+100");
        assert_eq!(is_numeric(&s, true), false);
        assert_eq!(is_numeric(&s, false), true);
        s = String::from("-10");
        assert_eq!(is_numeric(&s, true), false);
        assert_eq!(is_numeric(&s, false), true);
        s = String::from("1.2");
        assert_eq!(is_numeric(&s, true), false);
    }
}