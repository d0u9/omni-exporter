use async_trait::async_trait;

use omni_exporter::error::Result;
use omni_exporter::exporter::Collector;
use omni_exporter::exporter::ExtCollector;
use omni_exporter::exporter::SimpleExporter;
use omni_exporter::sensor::ExtSensorReader;
use omni_exporter::sensor::MockReader;
use omni_exporter::sensor::SensorReader;
use omni_exporter::storage::ExtStorageReader;
use omni_exporter::storage::PlainText;
use omni_exporter::storage::PlainTextSlot;
use omni_exporter::storage::SlotGetter;
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

#[derive(Clone)]
pub struct TestSlot {
    metric_name: String,
}

impl<'a> SlotGetter for &'a TestSlot {
    fn get_metric_name(&self) -> &str {
        &self.metric_name
    }
}

pub struct TestStorage {
    slots: Vec<TestSlot>,
}

impl TestStorage {
    pub fn new() -> Self {
        Self {
            slots: vec![
                TestSlot {
                    metric_name: "test_test-1".to_string(),
                },
                TestSlot {
                    metric_name: "test_test-2".to_string(),
                },
            ],
        }
    }
}

struct TestIter<'a> {
    pos: usize,
    slots: &'a Vec<TestSlot>,
}

impl<'a> Iterator for TestIter<'a> {
    type Item = Box<dyn SlotGetter + 'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.slots.get(self.pos).map(|slot| {
            self.pos += 1;
            Box::new(slot) as Box<dyn SlotGetter + 'a>
        })
    }
}

impl ExtStorageReader for TestStorage {
    fn slots<'a>(&'a self) -> Box<dyn Iterator<Item = Box<dyn SlotGetter + 'a>> + 'a> {
        Box::new(TestIter {
            pos: 0,
            slots: &self.slots,
        })
    }
}

/////////

pub struct TestSensorReader {}

impl TestSensorReader {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl SensorReader for TestSensorReader {
    type Slot = PlainTextSlot;
    type Storage = PlainText;

    async fn read(&self) -> Result<Self::Storage> {
        Ok(PlainText::new())
    }
}

#[async_trait]
impl ExtSensorReader for TestSensorReader {
    async fn read(&self) -> Result<Box<dyn ExtStorageReader>> {
        Ok(Box::new(TestStorage::new()))
    }
}
