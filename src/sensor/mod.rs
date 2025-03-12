mod traits;
pub use traits::ExtSensorReader;
pub use traits::SensorReader;

mod impls;
pub use impls::MockReader;

mod types;
pub use types::PlainTextAdapter;
