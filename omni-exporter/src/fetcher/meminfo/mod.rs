mod meminfo_impl;
pub use meminfo_impl::Meminfo;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
use macos::MeminfoInner;
#[cfg(target_os = "macos")]
pub use macos::{MeminfoMetric, MeminfoMetricNames};

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
use linux::MeminfoInner;
#[cfg(target_os = "linux")]
pub use linux::{MeminfoMetric, MeminfoMetricNames};
