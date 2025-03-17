// https://github.com/prometheus/procfs/blob/master/sysfs/system_cpu.go

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use tokio::fs;

use super::super::error::{Err, IOError, Result};
use super::super::internal::sysreadfile;
use super::fs::FS;

// CPU represents a path to a CPU located in `/sys/devices/system/cpu/cpu[0-9]*`.
#[derive(Debug)]
pub struct Cpu {
    sys: PathBuf,
}

impl Cpu {
    pub fn new(path: PathBuf) -> Self {
        Self { sys: path }
    }

    pub fn number(&self) -> Option<usize> {
        self.sys
            .file_name()
            .and_then(|name| name.to_str())
            .and_then(|s| s.strip_prefix("cpu"))
            .and_then(|s| s.parse().ok())
    }

    pub async fn online(&self) -> Result<bool> {
        let cpu_path = self.sys.join("online");
        println!("{:?}", cpu_path);
        let online = sysreadfile::sys_read_file(cpu_path).await?;
        let online_str = String::from_utf8_lossy(&online);
        Ok(online_str.chars().next() == Some('1'))
    }
}

#[derive(Debug, Default)]
struct CpuTopology {
    core_id: usize,
    core_siblings_list: String,
    physical_package_id: usize,
    thread_siblings_list: String,
}

impl Cpu {
    // Topology gets the topology information for a single CPU from `/sys/devices/system/cpu/cpuN/topology`.
    pub async fn topology(&self) -> Result<CpuTopology> {
        // cpu_topology_path is a directory
        let cpu_topology_path = self.sys.join("topology");
        if !cpu_topology_path.is_dir() {
            return Err(Err::IOErr(IOError::NotDir(
                cpu_topology_path.to_string_lossy().to_string(),
            )));
        };

        Self::parse_cpu_topology(cpu_topology_path).await
    }

    async fn parse_cpu_topology<T: AsRef<Path>>(path: T) -> Result<CpuTopology> {
        let core_id = sysreadfile::sys_read_file_string(path.as_ref().join("core_id")).await?;
        let core_siblings_list =
            sysreadfile::sys_read_file_string(path.as_ref().join("core_siblings_list")).await?;
        let physical_package_id =
            sysreadfile::sys_read_file_string(path.as_ref().join("physical_package_id")).await?;
        let thread_siblings_list =
            sysreadfile::sys_read_file_string(path.as_ref().join("thread_siblings_list")).await?;

        Ok(CpuTopology {
            core_id: core_id.parse()?,
            core_siblings_list,
            physical_package_id: physical_package_id.parse()?,
            thread_siblings_list,
        })
    }
}

#[derive(Debug, Default)]
struct CpuThermalThrottle {
    core_throttle_count: usize,
    package_throttle_count: usize,
}

impl Cpu {
    // ThermalThrottle gets the cpu throttle count information for a single CPU from `/sys/devices/system/cpu/cpuN/thermal_throttle`.
    pub async fn thermal_throttle(&self) -> Result<CpuThermalThrottle> {
        // themal_path is a directory
        let thermal_path = self.sys.join("thermal_throttle");
        if !thermal_path.is_dir() {
            return Err(Err::IOErr(IOError::NotDir(
                thermal_path.to_string_lossy().to_string(),
            )));
        };

        Self::parse_cpu_thermal_throttle(thermal_path).await
    }

    async fn parse_cpu_thermal_throttle<T: AsRef<Path>>(path: T) -> Result<CpuThermalThrottle> {
        let core_throttle_count =
            sysreadfile::sys_read_file_string(path.as_ref().join("core_throttle_count")).await?;
        let package_throttle_count =
            sysreadfile::sys_read_file_string(path.as_ref().join("package_throttle_count")).await?;
        Ok(CpuThermalThrottle {
            core_throttle_count: core_throttle_count.parse()?,
            package_throttle_count: package_throttle_count.parse()?,
        })
    }
}

impl FS {
    pub async fn cpus(&self) -> Result<Vec<Cpu>> {
        // Find files match this pattern: /sys/devices/system/cpu/cpu[0-9]*
        let sys_path = "/sys/devices/system/cpu/";
        let mut dir = fs::read_dir(sys_path).await?;

        let mut cpus: Vec<Cpu> = Vec::new();
        while let Some(entry) = dir.next_entry().await? {
            let path = entry.path();
            if path.is_dir() {
                if let Some(name) = path.file_name() {
                    let name = name.to_string_lossy();
                    if name.starts_with("cpu") && name[3..].chars().all(|c| c.is_ascii_digit()) {
                        cpus.push(Cpu::new(path));
                    }
                }
            }
        }

        let mut cpus = cpus
            .into_iter()
            .filter_map(|cpu| cpu.number().map(|n| (n, cpu)))
            .collect::<Vec<(usize, Cpu)>>();

        cpus.sort_by_key(|(n, _)| *n);
        let cpus = cpus.into_iter().map(|(_, cpu)| cpu).collect::<Vec<Cpu>>();

        Ok(cpus)
    }
}

#[derive(Debug, Default)]
struct SystemCpuFreqStats {
    name: String,
    cpuinfo_current_frequency: u64,
    cpuinfo_minimum_frequency: u64,
    cpuinfo_maximum_frequency: u64,
    cpuinfo_transition_latency: u64,
    scaling_current_frequency: u64,
    scaling_minimum_frequency: u64,
    scaling_maximum_frequency: u64,
    available_governors: String,
    driver: String,
    governor: String,
    related_cpus: String,
    set_speed: String,
    // Refer `CONFIG_CPU_FREQ_STAT`: https://www.kernel.org/doc/html/latest/cpu-freq/cpufreq-stats.html#configuring-cpufreq-stats
    cpuinfo_frequency_duration: HashMap<u64, u64>,
    cpuinfo_frequency_transitions_total: u64,
    cpuinfo_transition_table: Vec<Vec<u64>>,
}

impl FS {
    // SystemCpufreq returns CPU frequency metrics for all CPUs.
    pub async fn system_cpufreq(&self) -> Result<Vec<SystemCpuFreqStats>> {
        Err(Err::NotImplemented)
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::error::{Err, IOError};
    use super::*;

    #[tokio::test]
    async fn test_cpu_number() {
        let fs = FS::default();
        let cpus = fs.cpus().await.unwrap();
        println!("{:?}", cpus);
    }

    #[tokio::test]
    async fn test_cpu_online() {
        let fs = FS::default();
        let cpus = fs.cpus().await.unwrap();
        let onlline = match cpus[0].online().await {
            Ok(online) => online,
            Err(e) => match e {
                Err::IOErr(IOError::NotFound(path)) => {
                    println!("{}", path);
                    false
                }
                _ => false,
            },
        };
        println!("{:?}", onlline);
    }

    #[tokio::test]
    async fn test_cpu_topology() {
        let fs = FS::default();
        let cpus = fs.cpus().await.unwrap();
        for cpu in cpus {
            let topology = cpu.topology().await.unwrap();
            println!("{:?}", topology);
        }
    }

    #[tokio::test]
    async fn test_cpu_thermal_throttle() {
        let fs = FS::default();
        let cpus = fs.cpus().await.unwrap();
        for cpu in cpus {
            let thermal_throttle = cpu.thermal_throttle().await;
            println!("{:?}", thermal_throttle);
        }
    }
}
