extern crate regex;

use regex::Regex;
use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;

use ethers::prelude::Abigen;

fn abi_file_generation(src: &Path, dst: &String) -> Result<(), Box<dyn Error>> {
    let file_name = src.file_stem().unwrap().to_str().unwrap();
    let file_name_snake_case = file_name.replace("ERC", "_erc");
    let file_name_snake_case = file_name_snake_case.replace("V2TBridge", "_v2t_bridge");

    let re = Regex::new(r"([A-Z])").unwrap();
    let file_name_snake_case = re.replace_all(&file_name_snake_case, "_$1").to_lowercase();
    let file_name_snake_case = file_name_snake_case.trim_start_matches('_');

    let dst_file = format!("{dst}/{file_name_snake_case}.rs");
    Abigen::new(file_name, src.to_str().unwrap())?
        .generate()?
        .write_to_file(dst_file)?;

    Ok(())
}

fn list_files(src: &Path, dst: &String) {
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

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("please input src dir and dst dir");
        return;
    }

    env::set_var("CARGO_MANIFEST_DIR", &args[1]);
    list_files(Path::new(&args[1]), &args[2]);
}

#[cfg(test)]
mod tests {
    extern crate assert_cli;

    use super::*;
    use std::env;

    #[test]
    fn test_main() {
        assert_cli::Assert::main_binary().succeeds().unwrap();

        let dir = env::current_dir().unwrap();
        let binding = dir.join(Path::new("tests"));
        let path = binding.to_str().unwrap();
        let args = vec![path.clone(), path.clone()];
        assert_cli::Assert::main_binary()
            .with_args(&args)
            .succeeds()
            .unwrap();
    }
}
