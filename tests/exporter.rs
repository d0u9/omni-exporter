use omni_exporter::error::Result;
use omni_exporter::exporter::Collector;
use omni_exporter::exporter::SimpleExporter;
use omni_exporter::sensor::MockReader;
use omni_exporter::sensor::SensorReader;
use omni_exporter::storage::InternalSlot;
use omni_exporter::storage::InternalStorage;
use omni_exporter::storage::PlainText;
use omni_exporter::storage::PlainTextSlot;
use omni_exporter::storage::SlotGetter;
use omni_exporter::storage::StorageReader;

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
    exporter.add_sensor(Box::new(test_reader));

    let result = exporter.scrape().await;

    log::info!("result: {:?}", result);
}

////////////////////////////////////////////////////////////

pub struct TestSlot {
    metric_name: String,
}

impl SlotGetter for TestSlot {
    fn get_metric_name(&self) -> &str {
        &self.metric_name
    }
}

pub struct TestStorage {
    slots: Vec<TestSlot>,
}

impl TestStorage {
    pub fn new() -> Self {
        Self { slots: vec![] }
    }
}

impl StorageReader for TestStorage {
    type Slot = TestSlot;

    fn into_slots(self) -> impl Iterator<Item = Self::Slot> {
        self.slots.into_iter()
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
    //type Slot = PlainTextSlot;
    //type Storage = PlainText;
    type Slot = InternalSlot;
    type Storage = InternalStorage;

    fn read(&self) -> Result<Self::Storage> {
        //Ok(Box::new(TestStorage::new()))
        Ok(InternalStorage::PlainText(PlainText::new()))
    }
}
