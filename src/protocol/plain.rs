use super::Protocol;

pub struct Plain {
    inner: String,
}

impl Plain {
    pub fn new() -> Self {
        Self {
            inner: String::new(),
        }
    }
}

impl Protocol for Plain {
    fn add_item(&mut self, v: &str) {
        self.inner = v.to_string();
    }

    fn get_item(&self) -> &str {
        &self.inner
    }
}
