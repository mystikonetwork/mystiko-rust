use crate::error::ZkpError;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::io::BufReader;

async fn create_file_reader(file_path_str: &str) -> Result<BufReader<File>, ZkpError> {
    let file_path = Path::new(file_path_str);
    let file_handle = File::open(file_path)
        .await
        .map_err(|why| ZkpError::ReadFileError(file_path_str.parse().unwrap(), why.to_string()))?;
    Ok(BufReader::new(file_handle))
}

pub async fn load_file(file_path_str: &str) -> Result<Vec<u8>, ZkpError> {
    let mut buffer: Vec<u8> = Vec::new();
    let mut reader = create_file_reader(file_path_str).await?;

    reader
        .read_to_end(&mut buffer)
        .await
        .map_err(|why| ZkpError::ReadFileError(file_path_str.to_string(), why.to_string()))?;
    Ok(buffer)
}
