use mystiko_config::raw::validator::{array_unique, is_ethereum_address, is_number_string, is_number_string_vec, is_numeric, is_sem_ver, string_vec_each_not_empty};

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
    let is_number = is_number_string_vec::<true>(&number_vec);
    assert_eq!(is_number.is_err(), false);

    number_vec = vec!["abc".to_string()];
    let is_number = is_number_string_vec::<true>(&number_vec);
    assert_eq!(is_number.is_err(), true);

    number_vec = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let is_number = is_number_string_vec::<false>(&number_vec);
    assert_eq!(is_number.is_err(), true);

    assert_eq!(is_number_string::<true>(&"123".to_string()).is_err(), false);
}

#[test]
fn test_string_vec_each_not_empty() {
    let mut v = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let not_empty = string_vec_each_not_empty(&v);
    assert_eq!(not_empty.is_err(), false);
    v = vec!["a".to_string(), "b".to_string(), "".to_string()];
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
