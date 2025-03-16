mod meminfo_impl;
pub use meminfo_impl::{Meminfo, Metric};

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::{MeminfoInner, MetricNames};

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::{MeminfoInner, MetricNames};
