pub fn check(condition: bool, message: &str) {
    if !condition {
        panic!("{}", message);
    }
}