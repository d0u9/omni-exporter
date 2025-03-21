use async_trait::async_trait;

use crate::collector::Sensor;
use crate::error::Result;
use crate::metric::Metrics;
use crate::sensor::SensorReader;

use super::cpu;
use super::meminfo;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CoreFetcher {
    Cpu,
    Meminfo,
}

pub struct CoreFetcherSelector {
    fetchers: [Option<CoreFetcher>; 2],
}

impl CoreFetcherSelector {
    pub fn new() -> Self {
        Self {
            fetchers: [Some(CoreFetcher::Cpu), Some(CoreFetcher::Meminfo)],
        }
    }

    pub fn new_disable_all() -> Self {
        Self {
            fetchers: [None, None],
        }
    }

    pub fn status(&self) -> &[Option<CoreFetcher>] {
        &self.fetchers
    }

    pub fn enable(mut self, fetcher: CoreFetcher) -> Self {
        self.fetchers[fetcher as usize] = Some(fetcher);
        self
    }

    pub fn disable(mut self, fetcher: CoreFetcher) -> Self {
        self.fetchers[fetcher as usize] = None;
        self
    }
}

impl Default for CoreFetcherSelector {
    fn default() -> Self {
        Self::new()
    }
}

pub struct CoreReader {
    selector: CoreFetcherSelector,
    meminfo: meminfo::Meminfo,
    cpu: cpu::Cpu,
}

impl CoreReader {
    pub fn new() -> Self {
        Self::new_with_selector(CoreFetcherSelector::default())
    }

    pub fn new_with_selector(selector: CoreFetcherSelector) -> Self {
        Self {
            selector,
            meminfo: meminfo::Meminfo::new(),
            cpu: cpu::Cpu::new(),
        }
    }

    pub fn set_selector(&mut self, selector: CoreFetcherSelector) {
        self.selector = selector;
    }

    pub fn get_selector(&self) -> &CoreFetcherSelector {
        &self.selector
    }

    pub fn new_sensor() -> Sensor {
        Sensor::Core(Self::new())
    }

    async fn read_by_selector(&self, selector: &CoreFetcherSelector) -> Result<Metrics> {
        let mut metrics = Vec::new();
        for fetcher in selector.status().iter().filter_map(|f| f.as_ref()) {
            match fetcher {
                CoreFetcher::Cpu => metrics.extend(self.cpu.update().await?),
                CoreFetcher::Meminfo => metrics.extend(self.meminfo.get_meminfo().await?),
            }
        }

        Ok(Metrics::from_vec(metrics))
    }

    async fn read_all(&self) -> Result<Metrics> {
        self.read_by_selector(&self.selector).await
    }
}

impl Default for CoreReader {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SensorReader for CoreReader {
    type Metrics = Metrics;

    async fn read(&self) -> Result<Self::Metrics> {
        self.read_all().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_system_sensor() {
        let s = CoreReader::new();
        let metrics = s.read().await.unwrap();
        println!("{:?}", metrics);
    }
}
