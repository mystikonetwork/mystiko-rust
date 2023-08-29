#[cfg(target_os = "android")]
extern crate android_logger;
#[cfg(not(target_os = "android"))]
extern crate env_logger;
extern crate log;

pub mod config;
pub mod core;
pub mod handler;
