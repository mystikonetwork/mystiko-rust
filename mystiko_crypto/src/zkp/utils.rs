use crate::error::ZkpError;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;

pub fn create_file_reader(file_path_str: &str) -> Result<BufReader<File>, ZkpError> {
    let file_path = Path::new(file_path_str);
    let file_handle = File::open(file_path)
        .map_err(|why| ZkpError::ReadFileError(file_path_str.parse().unwrap(), why.to_string()))?;
    Ok(BufReader::new(file_handle))
}

pub fn load_file(file_path_str: &str) -> Result<Vec<u8>, ZkpError> {
    let mut buffer: Vec<u8> = Vec::new();
    let mut reader = create_file_reader(file_path_str)?;

    reader
        .read_to_end(&mut buffer)
        .map_err(|why| ZkpError::ReadFileError(file_path_str.parse().unwrap(), why.to_string()))?;
    Ok(buffer)
}
