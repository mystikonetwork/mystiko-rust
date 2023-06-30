#![forbid(unsafe_code)]
extern crate anyhow;
extern crate async_compression;
extern crate blake2;
extern crate hex;
extern crate reqwest;
extern crate tokio;
extern crate tokio_stream;
extern crate tokio_util;

use anyhow::{Error, Result};
use async_compression::tokio::bufread::GzipDecoder;
use blake2::{Blake2s256, Digest};
use digest::DynDigest;
use reqwest::Client;
use std::env::temp_dir;
use std::path::PathBuf;
use tokio::fs::{create_dir_all, read, remove_file, try_exists, File};
use tokio::io::{copy, AsyncWriteExt, BufReader, BufWriter};
use tokio_stream::StreamExt;
use tokio_util::io::StreamReader;

pub struct Downloader {
    client: Client,
    pub folder: PathBuf,
}

#[derive(Clone)]
pub struct DownloadOptions {
    pub skip_cache: bool,
    pub skip_decompression: bool,
}

#[derive(Default)]
pub struct DownloaderBuilder {
    client: Client,
    folder: Option<String>,
}

impl Downloader {
    pub async fn download(&mut self, url: &str, download_options: Option<DownloadOptions>) -> Result<PathBuf> {
        let options = download_options.unwrap_or(DownloadOptions::default());
        let is_compressed = url.ends_with(".gz") || url.ends_with(".tgz") || url.ends_with(".tar.gz");
        let file_path = self.download_raw(url, options.clone()).await?;
        if is_compressed && !options.skip_decompression {
            let decompressed_file_path = PathBuf::from(format!("{}_decompressed", file_path.to_str().unwrap()));
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
            file_writer.shutdown().await?;
            Ok(decompressed_file_path)
        } else {
            Ok(file_path)
        }
    }

    pub async fn download_failover(&mut self, urls: &Vec<String>, options: Option<DownloadOptions>) -> Result<PathBuf> {
        for (index, url) in urls.iter().enumerate() {
            let result = self.download(url, options.clone()).await;
            if result.is_err() && index < urls.len() - 1 {
                continue;
            } else {
                return result;
            }
        }
        Err(Error::msg("urls cannot be empty"))
    }

    pub async fn read_bytes(&mut self, url: &str, options: Option<DownloadOptions>) -> Result<Vec<u8>> {
        Ok(read(self.download(url, options).await?).await?)
    }

    pub async fn read_bytes_failover(
        &mut self,
        urls: &Vec<String>,
        options: Option<DownloadOptions>,
    ) -> Result<Vec<u8>> {
        Ok(read(self.download_failover(urls, options).await?).await?)
    }

    async fn download_raw(&mut self, url: &str, options: DownloadOptions) -> Result<PathBuf> {
        let mut hasher = Blake2s256::new();
        DynDigest::update(&mut hasher, url.as_bytes());
        let hash = hex::encode(hasher.finalize());
        let file_path = self.folder.join(PathBuf::from(&hash));
        let file_exists = try_exists(&file_path).await?;
        if file_exists && !options.skip_cache {
            Ok(file_path)
        } else {
            let response = self.client.get(url).send().await?;
            if response.status().is_success() {
                if file_exists {
                    remove_file(&file_path).await?;
                }
                let file = File::create(&file_path).await?;
                let stream = response
                    .bytes_stream()
                    .map(|result| result.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)));
                let mut file_reader = StreamReader::new(stream);
                let mut file_writer = BufWriter::new(file);
                copy(&mut file_reader, &mut file_writer).await?;
                file_writer.shutdown().await?;
                Ok(file_path)
            } else {
                Err(Error::msg(format!(
                    "failed to fetch {}, status code {}",
                    url,
                    response.status()
                )))
            }
        }
    }
}

impl DownloadOptions {
    pub fn new() -> Self {
        DownloadOptions {
            skip_cache: false,
            skip_decompression: false,
        }
    }
}

impl Default for DownloadOptions {
    fn default() -> Self {
        Self::new()
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

    pub async fn build(self) -> Result<Downloader> {
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
