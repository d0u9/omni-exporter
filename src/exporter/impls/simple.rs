use super::super::Collector;
use crate::error::Result;
use crate::sensor::SensorReader;
use crate::storage::InternalSlot;
use crate::storage::InternalStorage;
use crate::storage::PlainText;
use crate::storage::PlainTextSlot;
use crate::storage::SlotGetter;
use crate::storage::StorageReader;

pub struct Simple {
    sensors: Vec<Box<dyn SensorReader<Slot = InternalSlot, Storage = InternalStorage>>>,
}

impl Simple {
    pub fn new() -> Self {
        Self { sensors: vec![] }
    }
}

impl Simple {
    pub async fn scrape(&self) -> Result<String> {
        Ok("".to_string())
    }
}

impl Collector<InternalSlot, InternalStorage> for Simple {
    fn add_sensor(
        &mut self,
        sensor: Box<dyn SensorReader<Slot = InternalSlot, Storage = InternalStorage>>,
    ) {
        self.sensors.push(sensor);
    }
}
