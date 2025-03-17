use crate::error::Result;

use super::MeminfoMetric;
use super::MeminfoInner;

pub struct Meminfo {
    inner: MeminfoInner,
}

impl Meminfo {
    pub fn new() -> Self {
        Self {
            inner: MeminfoInner::new(),
        }
    }

    pub async fn get_meminfo(&self) -> Result<Vec<MeminfoMetric>> {
        self.inner.get_meminfo().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_meminfo() {
        let meminfo = Meminfo::new();

        let metrics = meminfo.get_meminfo().await.unwrap();
        assert!(metrics.len() > 0);

        #[cfg(target_os = "macos")]
        assert!(metrics.iter().any(|m| m.name == MetricNames::FreeBytes));

        println!("{:?}", metrics);
    }
}
