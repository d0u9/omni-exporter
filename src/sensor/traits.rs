pub trait Sensor: Sync + Send {
    fn name(&self) -> &str;
}

pub trait SensorReader: Sync + Send + 'static {
    fn name(&self) -> &str;
    fn id(&self) -> &str;
    fn read(&self) -> String;
}
