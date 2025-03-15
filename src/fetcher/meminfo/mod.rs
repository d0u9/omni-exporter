mod meminfo;
pub use meminfo::{Meminfo, Metric};

#[cfg(target_os = "macos")]
mod macos;
pub use macos::{MeminfoInner, MetricNames};

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::{MeminfoInner, MetricNames};