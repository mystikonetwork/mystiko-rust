use std::any::Any;
use std::collections::HashSet;
use std::hash::Hash;
use regex::Regex;
use validator::ValidationError;

pub fn is_ethereum_address(address: &str) -> Result<(), ValidationError> {
    let eth = Regex::new(r"^(0x)[0-9a-f]{40}$").unwrap();
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

pub fn is_number_string(
    object: &dyn Any,
    no_symbols: Option<bool>,
    each: Option<bool>,
) -> Result<(), ValidationError> {
    let no_symbols = match no_symbols {
        None => { true }
        Some(value) => { value }
    };
    let each = match each {
        None => { false }
        Some(value) => { value }
    };
    match object.downcast_ref::<&str>() {
        Some(s) => {
            if !is_valid_number_string(s, no_symbols) {
                return Err(ValidationError::new("is number string error"));
            }
        }
        None => {
            if let Some(v) = object.downcast_ref::<Vec<&str>>() {
                if each {
                    let is_number =
                        v.iter().all(|s| is_valid_number_string(s, no_symbols));
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

fn is_valid_number_string(s: &str, no_symbol: bool) -> bool {
    println!("{}", no_symbol);
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
        let s: &str = "-f";
        is_number_string(&s, Some(true), None).expect("error");
    }
}