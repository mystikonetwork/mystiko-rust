use std::error::Error;

pub fn check(condition: bool, message: &str) -> Result<(), Box<dyn Error>> {
    if !condition {
        return Err(message)?;
    }
    Ok(())
}