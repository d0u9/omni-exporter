
fn env_setup() {
    env_logger::init();
}

#[tokio::test]
async fn collector_with_ext_sensor_test() {
    env_setup();
    log::info!("collector_with_ext_sensor_test");
}

////////////////////////////////////////////////////////////
