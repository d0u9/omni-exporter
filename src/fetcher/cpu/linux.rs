use std::borrow::Cow;
use std::collections::HashMap;

use tokio::sync::RwLock;

use crate::error::Result;
use crate::metric::Label;
use crate::metric::MetricValue;
use crate::procfs;
use crate::procfs::stat::CPUStat;
use crate::procfs::sys;
use crate::procfs::{Err as ProcFsErr, IOErr as ProcFsIOErr};

use super::super::FetcherMetric;
use super::super::FetcherMetricName;

pub type CpuMetric = FetcherMetric<CpuMetricNames>;

pub struct CPUInner {
    cpu_status: RwLock<HashMap<usize, CPUStat>>,
    procfs: procfs::ProcFs,
    sys: sys::SysFs,
}

impl CPUInner {
    pub fn new() -> Self {
        let procfs = procfs::ProcFs::default();
        let sys = sys::SysFs::default();
        Self {
            procfs,
            sys,
            cpu_status: RwLock::new(HashMap::new()),
        }
    }

    pub async fn update(&self) -> Result<Vec<CpuMetric>> {
        let metrics = vec![
            self.update_stat().await?,
            self.update_online().await?,
            self.update_topology().await?,
        ];
        Ok(metrics.into_iter().flatten().collect())
    }
}

impl Default for CPUInner {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CpuMetricNames {
    // CPU stat
    User,
    Nice,
    System,
    Idle,
    Iowait,
    Irq,
    Softirq,
    Steal,

    // CPU online
    Online,

    // CPU topology
    CoreId,
    CoreSiblingsList,
    PhysicalPackageId,
    ThreadSiblingsList,
}

impl CpuMetricNames {
    fn as_str(&self) -> &'static str {
        match self {
            // CPU stat
            CpuMetricNames::User => "user",
            CpuMetricNames::Nice => "nice",
            CpuMetricNames::System => "system",
            CpuMetricNames::Idle => "idle",
            CpuMetricNames::Iowait => "iowait",
            CpuMetricNames::Irq => "irq",
            CpuMetricNames::Softirq => "softirq",
            CpuMetricNames::Steal => "steal",

            // CPU online
            CpuMetricNames::Online => "online",

            // CPU topology
            CpuMetricNames::CoreId => "core_id",
            CpuMetricNames::CoreSiblingsList => "core_siblings_list",
            CpuMetricNames::PhysicalPackageId => "physical_package_id",
            CpuMetricNames::ThreadSiblingsList => "thread_siblings_list",
        }
    }
}

impl FetcherMetricName for CpuMetricNames {
    fn as_str(&self) -> &'static str {
        self.as_str()
    }
}

impl From<CpuMetricNames> for Cow<'static, str> {
    fn from(name: CpuMetricNames) -> Self {
        Cow::Borrowed(name.as_str())
    }
}

////////////////////////////////////////////////////////////
/// Stat
////////////////////////////////////////////////////////////
impl CPUInner {
    async fn update_stat(&self) -> Result<Vec<CpuMetric>> {
        let m = self.procfs.stat().await?;
        self.update_cpu_stats(m.cpu).await;

        let stats = self.cpu_status.read().await;

        let metrics = stats
            .iter()
            .flat_map(|(i, n)| Self::metric_stat(i, n))
            .collect();

        Ok(metrics)
    }

    fn metric_stat(n: &usize, stat: &CPUStat) -> Vec<CpuMetric> {
        vec![
            CpuMetric::new_with_labels(
                CpuMetricNames::User,
                stat.user,
                vec![
                    Label::new("cpu", n.to_string()),
                    Label::new("mode", "user".to_string()),
                ],
            ),
            CpuMetric::new_with_labels(
                CpuMetricNames::Nice,
                stat.nice,
                vec![
                    Label::new("cpu", n.to_string()),
                    Label::new("mode", "nice".to_string()),
                ],
            ),
            CpuMetric::new_with_labels(
                CpuMetricNames::System,
                stat.system,
                vec![
                    Label::new("cpu", n.to_string()),
                    Label::new("mode", "system".to_string()),
                ],
            ),
            CpuMetric::new_with_labels(
                CpuMetricNames::Idle,
                stat.idle,
                vec![
                    Label::new("cpu", n.to_string()),
                    Label::new("mode", "idle".to_string()),
                ],
            ),
            CpuMetric::new_with_labels(
                CpuMetricNames::Iowait,
                stat.iowait,
                vec![
                    Label::new("cpu", n.to_string()),
                    Label::new("mode", "iowait".to_string()),
                ],
            ),
            CpuMetric::new_with_labels(
                CpuMetricNames::Irq,
                stat.irq,
                vec![
                    Label::new("cpu", n.to_string()),
                    Label::new("mode", "irq".to_string()),
                ],
            ),
            CpuMetric::new_with_labels(
                CpuMetricNames::Softirq,
                stat.softirq,
                vec![
                    Label::new("cpu", n.to_string()),
                    Label::new("mode", "softirq".to_string()),
                ],
            ),
            CpuMetric::new_with_labels(
                CpuMetricNames::Steal,
                stat.steal,
                vec![
                    Label::new("cpu", n.to_string()),
                    Label::new("mode", "steal".to_string()),
                ],
            ),
        ]
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

////////////////////////////////////////////////////////////
/// Online
////////////////////////////////////////////////////////////
impl CPUInner {
    pub async fn update_online(&self) -> Result<Vec<CpuMetric>> {
        let cpus = self.sys.cpus().await?;

        // No-op if the system does not support CPU online stats.
        if let Err(ProcFsErr::IO(ProcFsIOErr::NotFound(_))) = cpus[0].online().await {
            return Ok(vec![]);
        }

        let mut metrics = vec![];
        for cpu in cpus {
            let online = cpu.online().await?;
            metrics.push(CpuMetric::new_with_labels(
                CpuMetricNames::Online,
                MetricValue::from(online),
                vec![Label::new("cpu", cpu.number().unwrap_or(0).to_string())],
            ));
        }

        Ok(metrics)
    }
}

////////////////////////////////////////////////////////////
/// Topology
////////////////////////////////////////////////////////////
impl CPUInner {
    pub async fn update_topology(&self) -> Result<Vec<CpuMetric>> {
        let cpus = self.sys.cpus().await?;

        let mut metrics = vec![];
        for cpu in cpus {
            let topology = cpu.topology().await?;
            metrics.extend(vec![
                CpuMetric::new_with_labels(
                    CpuMetricNames::CoreId,
                    MetricValue::from(topology.core_id),
                    vec![Label::new("cpu", cpu.number().unwrap_or(0).to_string())],
                ),
                CpuMetric::new_with_labels(
                    CpuMetricNames::CoreSiblingsList,
                    MetricValue::from(topology.core_siblings_list),
                    vec![Label::new("cpu", cpu.number().unwrap_or(0).to_string())],
                ),
                CpuMetric::new_with_labels(
                    CpuMetricNames::PhysicalPackageId,
                    MetricValue::from(topology.physical_package_id),
                    vec![Label::new("cpu", cpu.number().unwrap_or(0).to_string())],
                ),
                CpuMetric::new_with_labels(
                    CpuMetricNames::ThreadSiblingsList,
                    MetricValue::from(topology.thread_siblings_list),
                    vec![Label::new("cpu", cpu.number().unwrap_or(0).to_string())],
                ),
            ]);
        }

        Ok(metrics)
    }
}

#[tokio::test]
async fn test_get_cpu_usage() {
    let cpu = CPUInner::new();
    let metrics = cpu.update().await.unwrap();
    println!("{:?}", metrics);
}
