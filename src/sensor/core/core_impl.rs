use async_trait::async_trait;

use crate::error::Result;
use crate::metric::Metrics;
use crate::sensor::SensorReader;

use super::cpu;
use super::meminfo;

pub struct CoreReader {
    meminfo: meminfo::Meminfo,
    cpu: cpu::Cpu,
}

impl CoreReader {
    pub fn new() -> Self {
        CoreReader {
            meminfo: meminfo::Meminfo::new(),
            cpu: cpu::Cpu::new(),
        }
    }

    async fn read_all(&self) -> Result<Metrics> {
        let mut metrics = Vec::new();
        metrics.extend(self.meminfo.get_meminfo().await?);
        metrics.extend(self.cpu.update().await?);
        Ok(Metrics::from_vec(metrics))
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
