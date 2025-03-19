use async_trait::async_trait;

use omni_exporter::collector::Sensor;
use omni_exporter::collector::SimpleCollector;
use omni_exporter::error::Result;
use omni_exporter::metric::Metric;
use omni_exporter::metric::MetricFamily;
use omni_exporter::metric::MetricType;
use omni_exporter::metric::MetricValue;
use omni_exporter::metric::Metrics;
use omni_exporter::metric::Timestamp;
use omni_exporter::sensor::CoreReader;
use omni_exporter::sensor::SensorReader;
use omni_exporter::sensor::SysctlReader;

fn env_setup() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
}

#[tokio::test]
async fn collector_with_ext_sensor_test() {
    env_setup();

    log::info!("collector_with_ext_sensor_test");

    let mut collector = SimpleCollector::new();

    let sensor = ExtSensorReader::new();
    collector.add_sensor(Sensor::External(Box::new(sensor)));

    let sensor = CoreReader::new();
    collector.add_sensor(Sensor::Core(sensor));

    let sensor = SysctlReader::new();
    collector.add_sensor(Sensor::Sysctl(sensor));

    let metrics = collector.collect().await.unwrap();

    log::info!("result: {:?}", metrics);
}

////////////////////////////////////////////////////////////

struct ExtSensorReader {
    family: MetricFamily,
    fetcher: MockFetcher,
}

const TEST_FAMILY: &str = "test_family";
const TEST_HELP: &str = "test_help";
const TEST_LABEL1: &str = "test_label1";
const TEST_LABEL2: &str = "test_label2";

impl ExtSensorReader {
    fn new() -> Self {
        Self {
            family: MetricFamily::new(
                TEST_FAMILY,
                TEST_HELP,
                MetricType::Counter,
                [TEST_LABEL1, TEST_LABEL2].iter().map(|s| *s),
            ),
            fetcher: MockFetcher::new(),
        }
    }
}

#[async_trait]
impl SensorReader for ExtSensorReader {
    type Metrics = Metrics;

    async fn read(&self) -> Result<Self::Metrics> {
        let metrics = self.fetcher.fetch().await?;
        let ret = metrics
            .into_iter()
            .map(|m| {
                let mut metric = Metric::new("test_metric", MetricValue::U64(m));
                self.family.tag_metric(&mut metric);
                metric
            })
            .map(|mut m| {
                m.set_timestamp(Timestamp::now());
                m.add_label(TEST_LABEL1, "test_value1".to_string());
                m.add_label(TEST_LABEL2, "test_value2".to_string());
                m
            })
            .collect();
        Ok(Metrics::from_vec(ret))
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
