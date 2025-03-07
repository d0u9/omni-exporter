fn env_setup() {
    env_logger::init();
}

#[tokio::test]
async fn main_test() {
    env_setup();

    let result = omni_exporter::say_hello("World");
    assert_eq!(result, "Hello, World!");
}
