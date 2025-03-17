use std::collections::HashMap;

use tokio::sync::RwLock;

use crate::error::Result;
use crate::metric::Label;
use crate::procfs::fs;
use crate::procfs::stat::CPUStat;

use super::super::FetcherMetric;
use super::super::FetcherMetricName;

pub type CpuMetric = FetcherMetric<CpuMetricNames>;

pub struct CPUInner {
    cpu_status: RwLock<HashMap<usize, CPUStat>>,
    procfs: fs::FS,
}

impl CPUInner {
    pub fn new() -> Self {
        let procfs = fs::FS::new_default();
        Self {
            procfs,
            cpu_status: RwLock::new(HashMap::new()),
        }
    }

    pub async fn update(&self) -> Result<Vec<CpuMetric>> {
        let metrics = vec![self.update_stat().await?];
        Ok(metrics.into_iter().flatten().collect())
    }

    async fn update_stat(&self) -> Result<Vec<CpuMetric>> {
        let m = self.procfs.stat().await?;
        self.update_cpu_stats(m.cpu).await;

        let stats = self.cpu_status.read().await;

        fn metric(n: &usize, stat: &CPUStat) -> Vec<CpuMetric> {
            vec![CpuMetric {
                name: CpuMetricNames::CpuTotal,
                value: stat.user.into(),
                labels: Some(vec![
                    Label::new("cpu", n.to_string()),
                    Label::new("mode", "user".to_string()),
                ]),
            }]
        }

        let metrics = stats.iter().flat_map(|(i, n)| metric(i, n)).collect();

        Ok(metrics)
    }

    async fn update_cpu_stats(&self, new_status: HashMap<usize, CPUStat>) {
        let mut old_stats = self.cpu_status.write().await;

        for (i, n) in new_status.iter() {
            let cpu_stats = old_stats.entry(*i).or_insert_with(|| n.clone());

            if cpu_stats.idle >= n.idle {
                log::debug!(
                    "jump_back_debug_message: cpu = {}, old_val = {}, new_val = {}",
                    i,
                    cpu_stats.idle,
                    n.idle
                );
                *cpu_stats = CPUStat::default();
            }

            if n.idle >= cpu_stats.idle {
                cpu_stats.idle = n.idle
            } else {
                log::debug!(
                    "CPU idle counter jumped backwards: cpu = {}, old_value = {}, new_value = {}",
                    i,
                    cpu_stats.idle,
                    n.idle
                );
            }

            if n.user >= cpu_stats.user {
                cpu_stats.user = n.user
            } else {
                log::debug!(
                    "CPU user counter jumped backwards: cpu = {}, old_value = {}, new_value = {}",
                    i,
                    cpu_stats.user,
                    n.user
                );
            }

            if n.nice >= cpu_stats.nice {
                cpu_stats.nice = n.nice
            } else {
                log::debug!(
                    "CPU nice counter jumped backwards: cpu = {}, old_value = {}, new_value = {}",
                    i,
                    cpu_stats.nice,
                    n.nice
                )
            }

            if n.system >= cpu_stats.system {
                cpu_stats.system = n.system
            } else {
                log::debug!(
                    "CPU system counter jumped backwards: cpu = {}, old_value = {}, new_value = {}",
                    i,
                    cpu_stats.system,
                    n.system
                )
            }

            if n.iowait >= cpu_stats.iowait {
                cpu_stats.iowait = n.iowait
            } else {
                log::debug!(
                    "CPU iowait counter jumped backwards: cpu = {}, old_value = {}, new_value = {}",
                    i,
                    cpu_stats.iowait,
                    n.iowait
                )
            }

            if n.irq >= cpu_stats.irq {
                cpu_stats.irq = n.irq
            } else {
                log::debug!(
                    "CPU irq counter jumped backwards: cpu = {}, old_value = {}, new_value = {}",
                    i,
                    cpu_stats.irq,
                    n.irq
                )
            }

            if n.softirq >= cpu_stats.softirq {
                cpu_stats.softirq = n.softirq
            } else {
                log::debug!(
                    "CPU softirq counter jumped backwards: cpu = {}, old_value = {}, new_value = {}",
                    i,
                    cpu_stats.softirq,
                    n.softirq
                )
            }

            if n.steal >= cpu_stats.steal {
                cpu_stats.steal = n.steal
            } else {
                log::debug!(
                    "CPU steal counter jumped backwards: cpu = {}, old_value = {}, new_value = {}",
                    i,
                    cpu_stats.steal,
                    n.steal
                )
            }

            if n.guest >= cpu_stats.guest {
                cpu_stats.guest = n.guest
            } else {
                log::debug!(
                    "CPU guest counter jumped backwards: cpu = {}, old_value = {}, new_value = {}",
                    i,
                    cpu_stats.guest,
                    n.guest
                )
            }

            if n.guest_nice >= cpu_stats.guest_nice {
                cpu_stats.guest_nice = n.guest_nice
            } else {
                log::debug!(
                    "CPU guest nice counter jumped backwards: cpu = {}, old_value = {}, new_value = {}",
                    i,
                    cpu_stats.guest_nice,
                    n.guest_nice
                )
            }
        }

        // Remove offline CPUs
        if new_status.len() != old_stats.len() {
            old_stats.retain(|i, _| new_status.contains_key(i));
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CpuMetricNames {
    CpuTotal,
}

impl FetcherMetricName for CpuMetricNames {
    fn to_str(&self) -> &'static str {
        match self {
            CpuMetricNames::CpuTotal => "cpu_total",
        }
    }
}

impl AsRef<str> for CpuMetricNames {
    fn as_ref(&self) -> &str {
        self.to_str()
    }
}

impl From<CpuMetricNames> for &'static str {
    fn from(name: CpuMetricNames) -> Self {
        name.to_str()
    }
}

#[tokio::test]
async fn test_get_cpu_usage() {
    let cpu = CPUInner::new();
    let metrics = cpu.update().await.unwrap();
    println!("{:?}", metrics);
}
