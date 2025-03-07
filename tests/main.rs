use omni_exporter::exporter::Exporter;
use omni_exporter::exporter::SimpleExporter;

fn env_setup() {
    env_logger::init();
}

#[tokio::test]
async fn main_test() {
    env_setup();

    let exporter = SimpleExporter::new();
    let result = exporter.scrape().await;

    log::info!("result: {:?}", result);
}
