#[cfg(target_os = "linux")]
mod meminfo;
#[cfg(target_os = "linux")]
pub use meminfo::Meminfo;

#[cfg(target_os = "linux")]
mod proc_stat;
#[cfg(target_os = "linux")]
pub use proc_stat::*;

#[cfg(target_os = "linux")]
mod stat;
#[cfg(target_os = "linux")]
pub use stat::CPUStat;

#[cfg(target_os = "linux")]
mod fs;
#[cfg(target_os = "linux")]
pub use fs::ProcFs;
