mod traits;
pub use traits::Sensor;
pub use traits::SensorReader;

mod impl_mock;
pub use impl_mock::MockSensor;
