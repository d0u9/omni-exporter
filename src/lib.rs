use log::info;

pub fn say_hello(name: &str) -> String {
    info!("say_hello");
    format!("Hello, {}!", name)
}
