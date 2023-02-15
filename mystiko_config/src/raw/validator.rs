use std::any::Any;
use std::collections::HashSet;
use std::fmt::Debug;
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
        x.validation()
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
    use crate::raw::validator::{array_unique, is_ethereum_address, is_number_string, is_numeric, is_sem_ver, string_vec_each_not_empty};

    #[test]
    fn test_is_ethereum_address() {
        let mut address = "0x0000000";
        assert_eq!(is_ethereum_address(&address).is_err(), true);
        address = "0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d";
        assert_eq!(is_ethereum_address(&address).is_err(), false);
    }

    #[test]
    fn test_array_unique() {
        let mut arr = vec!["a", "a", "b", "c"];
        assert_eq!(array_unique::<&str>(&arr).is_err(), true);
        arr = vec!["1", "2", "3"];
        assert_eq!(array_unique::<Vec<&str>>(&[arr]).is_err(), false);
    }

    #[test]
    fn test_is_number_string() {
        let mut number_vec = vec![
            String::from("10000000000000000"),
            String::from("100000000000000000"),
        ];
        let is_number = is_number_string::<true, true>(&number_vec);
        assert_eq!(is_number.is_err(), false);

        number_vec = vec!["abc".to_string()];
        let is_number = is_number_string::<true, true>(&number_vec);
        assert_eq!(is_number.is_err(), true);

        number_vec = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let is_number = is_number_string::<false, false>(&number_vec);
        assert_eq!(is_number.is_err(), true);

        let nums = vec![1, 2, 3];
        let is_number = is_number_string::<true, false>(&nums);
        assert_eq!(is_number.is_err(), true);
    }

    #[test]
    fn test_string_vec_each_not_empty() {
        let mut v = vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
        ];
        let not_empty = string_vec_each_not_empty(&v);
        assert_eq!(not_empty.is_err(), false);
        v = vec![
            "a".to_string(),
            "b".to_string(),
            "".to_string(),
        ];
        let not_empty = string_vec_each_not_empty(&v);
        assert_eq!(not_empty.is_err(), true);
    }

    #[test]
    fn test_is_sem_ver() {
        let mut v = String::from("1.2.3");
        assert_eq!(is_sem_ver(&v).is_err(), false);
        v = String::from("0.1.0");
        assert_eq!(is_sem_ver(&v).is_err(), false);
        v = String::from("2.0.0-alpha.1");
        assert_eq!(is_sem_ver(&v).is_err(), false);
        v = String::from("3.4.5-beta+20181012");
        assert_eq!(is_sem_ver(&v).is_err(), false);
        v = String::from("0");
        assert_eq!(is_sem_ver(&v).is_err(), true);
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