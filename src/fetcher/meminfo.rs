use crate::error::Result;
use crate::exotic::system as syslib;

pub struct Meminfo {
    inner: syslib::System,
}

impl Meminfo {
    pub fn new() -> Self {
        let sys = syslib::System::new();
        Self::new_with_system(sys)
    }

    pub fn new_with_system(sys: syslib::System) -> Self {
        Self { inner: sys }
    }

    pub async fn get_meminfo(&self) -> Result<Metrics> {
        let mut metrics = Metrics::default();

        let m = self.inner.free_memory().await;
        metrics.mem_free_bytes = Some(m);

        Ok(metrics)
    }
}

#[derive(Default)]
pub struct Metrics {
    pub mem_free_bytes: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_meminfo() {
        let sys = syslib::System::new();
        let meminfo = Meminfo::new_with_system(sys);

        let metrics = meminfo.get_meminfo().await.unwrap();
        assert!(metrics.mem_free_bytes.is_some());
        assert!(metrics.mem_free_bytes.unwrap() > 0);
    }
}
