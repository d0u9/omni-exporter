#[cfg(target_os = "linux")]
mod internal;

#[cfg(target_os = "linux")]
mod fs;
#[cfg(target_os = "linux")]
pub use fs::ProcFs;
#[cfg(target_os = "linux")]
pub mod sysfs;

#[cfg(target_os = "linux")]
pub mod meminfo;
#[cfg(target_os = "linux")]
pub mod proc_stat;
#[cfg(target_os = "linux")]
pub mod stat;

mod error;
pub use error::*;
