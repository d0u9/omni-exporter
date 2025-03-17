use crate::error::Result;

use super::CPUInner;
use super::CpuMetric;

pub struct CPU {
    inner: CPUInner,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            inner: CPUInner::new(),
        }
    }

    pub async fn update(&self) -> Result<Vec<CpuMetric>> {
        self.inner.update().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cpu() {
        let cpu = CPU::new();
        let metrics = cpu.update().await.unwrap();
        println!("{:?}", metrics);
    }
}
