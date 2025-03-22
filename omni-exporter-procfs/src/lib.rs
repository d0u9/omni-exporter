#[cfg(target_os = "linux")]
mod internal;

#[cfg(target_os = "linux")]
pub mod proc;

#[cfg(target_os = "linux")]
pub mod sys;

mod error;
pub use error::*;
