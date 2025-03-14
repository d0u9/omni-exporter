use async_trait::async_trait;

use omni_exporter::collector::Sensor;
use omni_exporter::collector::SimpleCollector;
use omni_exporter::error::Result;
use omni_exporter::sensor::SystemReader;
use omni_exporter::sensor::{Labels, SensorReader};
use omni_exporter::types::{Metric, MetricType, MetricValue};

fn env_setup() {
    env_logger::init();
}

#[tokio::test]
async fn collector_with_ext_sensor_test() {
    env_setup();
    log::info!("collector_with_ext_sensor_test");

    let mut collector = SimpleCollector::new();

    let sensor = ExtSensorReader::new();
    collector.add_sensor(Sensor::External(Box::new(sensor)));

    let sensor = SystemReader::new();
    collector.add_sensor(Sensor::System(sensor));

    let metrics = collector.collect().await.unwrap();

    log::info!("result: {:?}", metrics);
}

////////////////////////////////////////////////////////////

struct ExtSensorReader {
    fetcher: MockFetcher,
}

impl ExtSensorReader {
    fn new() -> Self {
        Self {
            fetcher: MockFetcher::new(),
        }
    }
}

#[async_trait]
impl SensorReader for ExtSensorReader {
    async fn update(&self) -> Result<Vec<Metric<Labels>>> {
        let metrics = self.fetcher.fetch().await?;
        let ret = metrics
            .into_iter()
            .map(|m| Metric::new_with_no_labels(MetricType::Gauge, MetricValue::U64(m)))
            .collect();
        Ok(ret)
    }
}

struct MockFetcher;

type MockMetrics = Vec<u64>;

impl MockFetcher {
    fn new() -> Self {
        Self {}
    }

    async fn fetch(&self) -> Result<MockMetrics> {
        Ok(vec![1, 2, 3])
    }
}
