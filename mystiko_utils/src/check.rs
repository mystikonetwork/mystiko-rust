pub fn check(condition: bool, message: &str) {
    if !condition {
        // todo throw error?
        panic!("{}", message);
    }
}
