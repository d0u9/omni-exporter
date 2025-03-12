mod traits;
pub use traits::Data as SensorData;
pub use traits::DataFromProtoRef as SensorDataFromRef;
pub use traits::Reader as SensorReader;
pub use traits::Sensor;

mod impls;
pub use impls::MockSensor;
pub use impls::MockSensorReader;

pub mod data;
