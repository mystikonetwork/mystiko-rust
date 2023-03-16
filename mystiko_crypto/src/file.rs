use crate::error::FileError;
use anyhow::Result;
use std::path::{Path, PathBuf};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::io::BufReader;

async fn create_file_reader(file_path: &Path) -> Result<BufReader<File>, FileError> {
    let file_handle = File::open(file_path).await.map_err(|why| {
        FileError::OpenFileError(file_path.to_string_lossy().to_string(), why.to_string())
    })?;
    Ok(BufReader::new(file_handle))
}

pub async fn load_file(file_path_str: &str) -> Result<Vec<u8>, FileError> {
    let mut buffer: Vec<u8> = Vec::new();
    let file_path = PathBuf::from(file_path_str);
    let mut reader = create_file_reader(&file_path).await?;

    reader
        .read_to_end(&mut buffer)
        .await
        .map_err(|why| FileError::ReadFileError(file_path_str.to_string(), why.to_string()))?;
    Ok(buffer)
}
