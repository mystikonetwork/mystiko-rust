#![forbid(unsafe_code)]
extern crate async_compression;
extern crate blake2;
extern crate hex;
extern crate reqwest;
extern crate tokio;
extern crate tokio_stream;
extern crate tokio_util;

use async_compression::tokio::bufread::GzipDecoder;
use blake2::{Blake2s256, Digest};
use digest::DynDigest;
use reqwest::Client;
use std::env::temp_dir;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use tokio::fs::{create_dir_all, remove_file, try_exists, File};
use tokio::io::{copy, AsyncWriteExt, BufReader, BufWriter};
use tokio_stream::StreamExt;
use tokio_util::io::StreamReader;

pub struct Downloader {
    client: Client,
    folder: PathBuf,
}

#[derive(Clone)]
pub struct DownloadOptions {
    pub skip_cache: bool,
    pub skip_decompression: bool,
    pub hasher: Box<dyn DynDigest>,
}

pub struct DownloaderBuilder {
    client: Client,
    folder: Option<String>,
}

impl Downloader {
    pub async fn download(
        &mut self,
        url: &str,
        download_options: Option<DownloadOptions>,
    ) -> Result<PathBuf, Error> {
        let options = download_options.unwrap_or(DownloadOptions::default());
        let is_compressed =
            url.ends_with(".gz") || url.ends_with(".tgz") || url.ends_with(".tar.gz");
        let file_path = self.download_raw(url, options.clone()).await?;
        if is_compressed && !options.skip_decompression {
            let decompressed_file_path =
                PathBuf::from(format!("{}_uncompressed", file_path.to_str().unwrap()));
            if try_exists(&decompressed_file_path).await? {
                if options.skip_cache {
                    remove_file(&decompressed_file_path).await?;
                } else {
                    return Ok(decompressed_file_path);
                }
            }
            let compressed_file = File::open(&file_path).await?;
            let decompressed_file = File::create(&decompressed_file_path).await?;
            let mut file_reader = GzipDecoder::new(BufReader::new(compressed_file));
            let mut file_writer = BufWriter::new(decompressed_file);
            copy(&mut file_reader, &mut file_writer).await?;
            file_writer.flush().await?;
            Ok(decompressed_file_path)
        } else {
            Ok(file_path)
        }
    }

    async fn download_raw(
        &mut self,
        url: &str,
        mut options: DownloadOptions,
    ) -> Result<PathBuf, Error> {
        options.hasher.update(url.as_bytes());
        let hash = hex::encode(options.hasher.finalize().to_vec());
        let file_path = self.folder.join(PathBuf::from(&hash));
        let file_exists = try_exists(&file_path).await?;
        if file_exists && !options.skip_cache {
            Ok(file_path)
        } else {
            if file_exists {
                remove_file(&file_path).await?;
            }
            let file = File::create(&file_path).await?;
            let response = self
                .client
                .get(url)
                .send()
                .await
                .map_err(|e| Error::new(ErrorKind::Other, format!("reqwest error: {}", e)))?;
            let stream = response
                .bytes_stream()
                .map(|result| result.map_err(|e| Error::new(ErrorKind::Other, e)));
            let mut file_reader = StreamReader::new(stream);
            let mut file_writer = BufWriter::new(file);
            copy(&mut file_reader, &mut file_writer).await?;
            file_writer.flush().await?;
            Ok(file_path)
        }
    }
}

impl DownloadOptions {
    pub fn default() -> Self {
        DownloadOptions {
            skip_cache: false,
            skip_decompression: false,
            hasher: Box::new(Blake2s256::new()),
        }
    }
}

impl DownloaderBuilder {
    pub fn new() -> Self {
        DownloaderBuilder {
            client: Client::new(),
            folder: None,
        }
    }

    pub fn folder(mut self, path: &str) -> Self {
        self.folder = Some(String::from(path));
        self
    }

    pub async fn build(self) -> Result<Downloader, Error> {
        let folder: PathBuf = match self.folder {
            Some(path) => PathBuf::from(&path),
            None => temp_dir().join(PathBuf::from("mystiko_downloader")),
        };
        if !try_exists(&folder).await? {
            create_dir_all(&folder).await?;
        }
        Ok(Downloader {
            client: self.client,
            folder,
        })
    }
}
