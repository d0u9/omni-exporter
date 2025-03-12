use crate::error::Result;
use crate::sensor::ExtSensorReader;
use crate::sensor::PlainTextAdapter;
use crate::sensor::SensorReader;
use crate::storage::InternalSlot;
use crate::storage::InternalStorage;
use crate::storage::PlainText;
use crate::storage::PlainTextSlot;
use crate::storage::SlotGetter;
use crate::storage::StorageReader;

use super::super::traits::Collector;
use super::super::traits::ExtCollector;

pub struct Simple {
    sensors: Vec<Box<dyn SensorReader<Slot = InternalSlot, Storage = InternalStorage>>>,
    ext_sensors: Vec<Box<dyn ExtSensorReader>>,
}

impl Simple {
    pub fn new() -> Self {
        Self {
            sensors: vec![],
            ext_sensors: vec![],
        }
    }
}

impl Simple {
    async fn scrape_internal(&self) -> Result<String> {
        for sensor in self.sensors.iter() {
            let storage = sensor.read().await?;
            let slots = storage.into_slots();
            for slot in slots {
                println!("slot: {}", slot.get_metric_name());
            }
        }
        Ok("".to_string())
    }

    async fn scrape_ext(&self) -> Result<String> {
        for sensor in self.ext_sensors.iter() {
            let storage = sensor.read().await?;
            let slots = storage.slots();
            for slot in slots {
                println!("slot: {}", slot.get_metric_name());
            }
        }
        Ok("".to_string())
    }

    pub async fn scrape(&self) -> Result<String> {
        let internal_result = self.scrape_internal().await?;
        let ext_result = self.scrape_ext().await?;
        Ok(format!("{} {}", internal_result, ext_result))
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

impl Collector<PlainTextSlot, PlainText> for Simple {
    fn add_sensor(
        &mut self,
        sensor: Box<dyn SensorReader<Slot = PlainTextSlot, Storage = PlainText>>,
    ) {
        self.sensors.push(Box::new(PlainTextAdapter::new(sensor)));
    }
}

impl ExtCollector for Simple {
    fn add_ext_sensor(&mut self, sensor: Box<dyn ExtSensorReader>) {
        self.ext_sensors.push(sensor);
    }
}
