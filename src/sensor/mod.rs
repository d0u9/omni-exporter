mod traits;
pub use traits::Sensor;
pub use traits::SensorData;
pub use traits::SensorReader;

mod impls;
pub use impls::MockSensor;
pub use impls::MockSensorReader;

pub mod data;
