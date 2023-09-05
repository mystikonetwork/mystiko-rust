use mystiko_storage::{
    comparable_string_to_i128, comparable_string_to_i64, comparable_string_to_isize, comparable_string_to_u128,
    comparable_string_to_u64, comparable_string_to_usize, i128_to_comparable_string, i64_to_comparable_string,
    isize_to_comparable_string, u128_to_comparable_string, u64_to_comparable_string, usize_to_comparable_string,
};

#[test]
fn test_i64_comparable_string() {
    let numbers = vec![-100i64, -2i64, 0i64, 3i64, 50i64];
    let mut numbers_string = numbers.iter().map(|n| i64_to_comparable_string(*n)).collect::<Vec<_>>();
    numbers_string.sort();
    let converted_numbers = numbers_string
        .into_iter()
        .map(|s| comparable_string_to_i64(&s).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(numbers, converted_numbers);
}

#[test]
fn test_u64_comparable_string() {
    let numbers = vec![0u64, 3u64, 50u64, 100u64];
    let mut numbers_string = numbers.iter().map(|n| u64_to_comparable_string(*n)).collect::<Vec<_>>();
    numbers_string.sort();
    let converted_numbers = numbers_string
        .into_iter()
        .map(|s| comparable_string_to_u64(&s).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(numbers, converted_numbers);
}

#[test]
fn test_isize_comparable_string() {
    let numbers = vec![-100isize, -2isize, 0isize, 3isize, 50isize];
    let mut numbers_string = numbers
        .iter()
        .map(|n| isize_to_comparable_string(*n))
        .collect::<Vec<_>>();
    numbers_string.sort();
    let converted_numbers = numbers_string
        .into_iter()
        .map(|s| comparable_string_to_isize(&s).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(numbers, converted_numbers);
}

#[test]
fn test_usize_comparable_string() {
    let numbers = vec![0usize, 3usize, 50usize, 100usize];
    let mut numbers_string = numbers
        .iter()
        .map(|n| usize_to_comparable_string(*n))
        .collect::<Vec<_>>();
    numbers_string.sort();
    let converted_numbers = numbers_string
        .into_iter()
        .map(|s| comparable_string_to_usize(&s).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(numbers, converted_numbers);
}

#[test]
fn test_i128_comparable_string() {
    let numbers = vec![-100i128, -2i128, 0i128, 3i128, 50i128];
    let mut numbers_string = numbers
        .iter()
        .map(|n| i128_to_comparable_string(*n))
        .collect::<Vec<_>>();
    numbers_string.sort();
    let converted_numbers = numbers_string
        .into_iter()
        .map(|s| comparable_string_to_i128(&s).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(numbers, converted_numbers);
}

#[test]
fn test_u128_comparable_string() {
    let numbers = vec![0u128, 3u128, 50u128, 100u128];
    let mut numbers_string = numbers
        .iter()
        .map(|n| u128_to_comparable_string(*n))
        .collect::<Vec<_>>();
    numbers_string.sort();
    let converted_numbers = numbers_string
        .into_iter()
        .map(|s| comparable_string_to_u128(&s).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(numbers, converted_numbers);
}
