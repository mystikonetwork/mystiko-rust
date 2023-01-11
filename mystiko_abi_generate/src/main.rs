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

    let dst = format!("{}/{}.rs", dst, file_name_snake_case);
    Abigen::new(file_name, src.to_str().unwrap())?
        .generate()?
        .write_to_file(dst)?;

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
                println!("generate file {}", file_name);
                let result = abi_file_generation(&path, dst);
                if result.is_err() {
                    panic!("generate meet error 11 {:?}", result.err());
                }
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
