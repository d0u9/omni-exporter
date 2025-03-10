use std::sync::Arc;

use sysinfo;
use tokio::sync::RwLock;

pub struct SystemInner {
    sys: sysinfo::System,
}

impl SystemInner {
    fn new() -> Self {
        let mut sys = sysinfo::System::new_all();
        sys.refresh_all();
        Self { sys }
    }

    fn refresh(&mut self) {
        self.sys.refresh_all();
    }

    fn available_memory(&self) -> u64 {
        self.sys.available_memory()
    }
}

pub struct System {
    inner: Arc<RwLock<SystemInner>>,
}

impl System {
    pub fn new() -> Self {
        let sys = SystemInner::new();
        Self {
            inner: Arc::new(RwLock::new(sys)),
        }
    }

    pub async fn refresh(&self) {
        let mut inner = self.inner.write().await;
        inner.refresh();
    }

    pub async fn available_memory(&self) -> u64 {
        let inner = self.inner.read().await;
        inner.available_memory()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_available_memory() {
        let sys = System::new();
        let available_memory = sys.available_memory().await;
        dbg!(&available_memory);
        assert!(available_memory > 0);
    }
}
