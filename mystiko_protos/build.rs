use std::error::Error;
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=schema/");
    let output = Command::new("bash")
        .arg("scripts/build-protos.sh")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output();
    let success = match output {
        Ok(output) => {
            if !output.stderr.is_empty() {
                println!("cargo:warning={}", String::from_utf8_lossy(&output.stderr));
            }
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
