mod core;
pub use core::CoreFetcher;
pub use core::CoreFetcherSelector;
pub use core::CoreReader;

mod sysctl;
pub use sysctl::SysctlReader;

mod traits;
pub use traits::SensorReader;
