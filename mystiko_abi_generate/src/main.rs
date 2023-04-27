extern crate regex;

use regex::Regex;
use std::env;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::Path;

use ethers_contract_abigen::Abigen;

fn abi_file_replace_package(input_filename: &str) -> Result<(), Box<dyn Error>> {
    // Read the contents of the input Rust file
    let contents = fs::read_to_string(input_filename)?;

    let replaced_contents = contents.replace("::ethers::core::", "::ethers_core::");
    let replaced_contents =
        replaced_contents.replace("::ethers::contract::", "::ethers_contract::");
    let replaced_contents =
        replaced_contents.replace("::ethers::providers::", "::ethers_providers::");

    // Write the replaced contents to the output Rust file
    let mut file = fs::File::create(input_filename)?;
    file.write_all(replaced_contents.as_bytes())?;

    Ok(())
}

fn abi_file_generation(src: &Path, dst: &str) -> Result<(), Box<dyn Error>> {
    let file_name = src.file_stem().unwrap().to_str().unwrap();
    let file_name_snake_case = file_name.replace("ERC", "_erc");
    let file_name_snake_case = file_name_snake_case.replace("V2TBridge", "_v2t_bridge");

    let re = Regex::new(r"([A-Z])").unwrap();
    let file_name_snake_case = re.replace_all(&file_name_snake_case, "_$1").to_lowercase();
    let file_name_snake_case = file_name_snake_case.trim_start_matches('_');

    let dst_file = format!("{dst}/{file_name_snake_case}.rs");
    Abigen::new(file_name, src.to_str().unwrap())?
        .add_derive("serde::Serialize")?
        .add_derive("serde::Deserialize")?
        .generate()?
        .write_to_file(dst_file.clone())?;

    abi_file_replace_package(&dst_file)?;

    Ok(())
}

fn list_files(src: &Path, dst: &str) {
    let entries = fs::read_dir(src).unwrap();

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            list_files(&path, dst);
        } else {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            if file_name.ends_with(".json") && !file_name.ends_with(".dbg.json") {
                println!("generate file {file_name}");
                abi_file_generation(&path, dst).unwrap();
            }
        }
    }
}

fn generate(args: Vec<String>) {
    if args.len() != 3 {
        println!("please input src dir and dst dir");
        return;
    }

    let src = Path::new(&args[1]);
    let dst = &args[2];

    let dir = env::current_dir().unwrap();
    let binding = dir.join(src);
    let path = binding.to_str().unwrap();
    env::set_var("CARGO_MANIFEST_DIR", path);

    let src_dir = fs::read_dir(src);
    if src_dir.is_err() {
        println!("src dir error {:?}", src_dir.err().unwrap());
        return;
    }

    let dst_dir = fs::read_dir(Path::new(dst));
    if dst_dir.is_err() {
        println!("dst dir error {:?}", dst_dir.err().unwrap());
        return;
    }

    list_files(src, dst);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    generate(args);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }

    #[test]
    fn test_wrong_path() {
        let args = vec![
            String::from("abi-generate"),
            String::from("./xxx"),
            String::from("./tests"),
        ];
        generate(args);

        let args = vec![
            String::from("abi-generate"),
            String::from("./tests"),
            String::from("./xxx"),
        ];
        generate(args);
    }

    #[test]
    fn test_generate() {
        let args = vec![
            String::from("abi-generate"),
            String::from("./tests"),
            String::from("./tests"),
        ];
        generate(args);

        for entry in fs::read_dir("./tests").unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() && path.extension() == Some(std::ffi::OsStr::new("rs")) {
                fs::remove_file(&path).unwrap();
                println!("Deleted {:?}", path);
            }
        }
    }
}
