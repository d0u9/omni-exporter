use omni_exporter::error::Result;
use omni_exporter::exporter::Collector;
use omni_exporter::exporter::ExtCollector;
use omni_exporter::exporter::SimpleExporter;
use omni_exporter::sensor::ExtSensorReader;
use omni_exporter::sensor::MockReader;
use omni_exporter::sensor::SensorReader;
use omni_exporter::storage::ExtSlotGetter;
use omni_exporter::storage::ExtStorageReader;
use omni_exporter::storage::PlainText;
use omni_exporter::storage::PlainTextSlot;
use std::sync::Arc;

fn env_setup() {
    env_logger::init();
}

#[tokio::test]
async fn exporter_simple_test() {
    env_setup();
    log::info!("exporter_simple_test");

    let mock_reader = MockReader::new();
    let test_reader = TestSensorReader::new();

    let mut exporter = SimpleExporter::new();
    exporter.add_sensor(Box::new(mock_reader));
    //exporter.add_sensor(Box::new(test_reader));
    exporter.add_ext_sensor(Box::new(test_reader));

    let result = exporter.scrape().await;

    log::info!("result: {:?}", result);
}

////////////////////////////////////////////////////////////
pub struct TestSlotInner {
    metric_name: String,
}

#[derive(Clone)]
pub struct TestSlot {
    inner: Arc<TestSlotInner>,
}

impl ExtSlotGetter for TestSlot {
    fn get_metric_name(&self) -> &str {
        &self.inner.metric_name
    }
}

pub struct TestStorage {
    slots: Vec<TestSlot>,
}

impl TestStorage {
    pub fn new() -> Self {
        Self {
            slots: vec![TestSlot {
                inner: Arc::new(TestSlotInner {
                    metric_name: "test_test".to_string(),
                }),
            }],
        }
    }
}

impl ExtStorageReader for TestStorage {
    fn slots<'a>(&'a self) -> Box<dyn Iterator<Item = Box<dyn ExtSlotGetter + 'a>>> {
        let iter = self.slots.clone();
        Box::new(
            iter.into_iter()
                .map(|slot| Box::new(slot) as Box<dyn ExtSlotGetter + 'a>),
        )
    }
}

/////////

pub struct TestSensorReader {}

impl TestSensorReader {
    pub fn new() -> Self {
        Self {}
    }
}

impl SensorReader for TestSensorReader {
    type Slot = PlainTextSlot;
    type Storage = PlainText;

    fn read(&self) -> Result<Self::Storage> {
        Ok(PlainText::new())
    }
}

impl ExtSensorReader for TestSensorReader {
    fn read(&self) -> Result<Box<dyn ExtStorageReader>> {
        Ok(Box::new(TestStorage::new()))
    }
}
