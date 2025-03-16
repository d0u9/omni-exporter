use crate::error::Result;
use crate::metric::MetricValue;

use super::MeminfoInner;
use super::MetricNames;

pub struct Meminfo {
    inner: MeminfoInner,
}

impl Meminfo {
    pub fn new() -> Self {
        Self {
            inner: MeminfoInner::new(),
        }
    }

    pub async fn get_meminfo(&self) -> Result<Vec<Metric>> {
        self.inner.get_meminfo().await
    }
}

#[derive(Debug, PartialEq)]
pub struct Metric {
    pub name: MetricNames,
    pub value: MetricValue,
}

impl AsRef<str> for MetricNames {
    fn as_ref(&self) -> &'static str {
        self.to_str()
    }
}

impl From<MetricNames> for &'static str {
    fn from(name: MetricNames) -> Self {
        name.to_str()
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
        assert!(metrics.iter().any(|m| m.name == MetricNames::FreeBytes));

        println!("{:?}", metrics);
    }
}
