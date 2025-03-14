fn env_setup() {
    env_logger::init();
}

#[tokio::test]
async fn exporter_simple_test() {
    env_setup();
    log::info!("exporter_simple_test");

    log::info!("result: {:?}", ());
}
