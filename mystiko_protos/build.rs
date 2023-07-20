use std::error::Error;
use std::io::{self, Write};
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    let output = Command::new("buf")
        .arg("generate")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("failed to execute process");
    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;
    if output.status.success() {
        Ok(())
    } else {
        Err("protobuf build failed".into())
    }
}
