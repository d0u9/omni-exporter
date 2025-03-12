use omni_exporter::exporter::Collector;
use omni_exporter::exporter::Exporter;
use omni_exporter::exporter::SimpleExporter;
use omni_exporter::sensor::MockSensor;
use omni_exporter::sensor::data::OwnedData;

fn env_setup() {
    env_logger::init();
}

#[tokio::test]
async fn exporter_simple_test() {
    env_setup();

    let mock_sensor = MockSensor::new();
    let mock_sensor_reader = mock_sensor.get_reader::<OwnedData>();

    let mut exporter = SimpleExporter::new();
    exporter.add_sensor(mock_sensor_reader);

    let result = exporter.scrape().await;

    log::info!("result: {:?}", result);
}
