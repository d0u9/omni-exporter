mod system;
pub use system::SystemReader;

mod sysctl;
pub use sysctl::SysctlReader;

mod traits;
pub use traits::SensorReader;
