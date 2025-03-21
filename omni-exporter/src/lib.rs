pub mod collector;
pub mod error;
pub mod fetcher;
pub mod metric;
pub mod sensor;

// mod exotic;
mod ffi;

#[cfg(target_os = "linux")]
mod procfs;
