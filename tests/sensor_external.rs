use async_trait::async_trait;

use omni_exporter::error::Result;
use omni_exporter::sensor::SensorReader;
use omni_exporter::types::{Metric, MetricType, MetricValue};

use std::collections::HashMap;

fn env_setup() {
    env_logger::init();
}

#[tokio::test]
async fn exporter_simple_test() {
    env_setup();
    log::info!("test custom sensor");

    let sensor = ExtSensorReader::new();
    let metrics = sensor.update().await.unwrap();

    log::info!("metrics: {:?}", metrics);
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
    type Labels = HashMap<String, String>;
    type Metrics = Vec<Metric<Self::Labels>>;

    async fn update(&self) -> Result<Self::Metrics> {
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
