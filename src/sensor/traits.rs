pub trait Sensor: Sync + Send {
    fn name(&self) -> &str;
}

pub trait SensorReader: Sync + Send + 'static {
    type Data: SensorData;

    fn name(&self) -> &str;
    fn id(&self) -> &str;
    fn read(&self) -> Self::Data;
}

pub trait SensorData: Sync + Send + 'static {
    fn name(&self) -> &str;
    fn from_str(s: &str) -> Self;
}
