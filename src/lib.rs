pub fn say_hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = say_hello("World");
        assert_eq!(result, "Hello, World!");
    }
}
