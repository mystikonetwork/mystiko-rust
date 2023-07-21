use std::error::Error;
use std::io::{self, Write};
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    let output = Command::new("buf")
        .arg("generate")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output();
    let success = match output {
        Ok(output) => {
            io::stdout().write_all(&output.stdout)?;
            io::stderr().write_all(&output.stderr)?;
            output.status.success()
        }
        Err(e) => {
            println!("cargo:warning={}", e);
            false
        }
    };
    if !success {
        print!("cargo:warning=Protobuf code generate failed, use previous generated files");
    }
    Ok(())
}
