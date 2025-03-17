mod cpu_impl;
pub use cpu_impl::CPU;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::{CPUInner, CpuMetric, CpuMetricNames};

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::{CPUInner, CpuMetric, CpuMetricNames};
