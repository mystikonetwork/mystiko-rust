#![forbid(unsafe_code)]
extern crate anyhow;
extern crate async_compression;
extern crate tokio;

use anyhow::Result;
use async_compression::tokio::bufread::GzipDecoder;
use std::path::PathBuf;
use tokio::fs::{read, File};
use tokio::io::{AsyncReadExt, BufReader};

pub async fn read_file_bytes(path: &str) -> Result<Vec<u8>> {
    Ok(read(PathBuf::from(path)).await?)
}

pub async fn read_gzip_file_bytes(path: &str) -> Result<Vec<u8>> {
    let gzip_file = File::open(PathBuf::from(path)).await?;
    let mut decoder = GzipDecoder::new(BufReader::new(gzip_file));
    let mut bytes = Vec::<u8>::new();
    decoder.read_to_end(&mut bytes).await?;
    Ok(bytes)
}
