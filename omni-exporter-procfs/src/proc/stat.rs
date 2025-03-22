// https://github.com/prometheus/procfs/blob/master/stat.go
// https://github.com/torvalds/linux/blob/master/Documentation/filesystems/proc.rst#17-miscellaneous-kernel-statistics-in-procstat

use std::collections::HashMap;
use std::io::Cursor;
use std::path::Path;

use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};

use crate::internal::utils;
use crate::{Err, Result};

use super::fs::ProcFs;
use super::proc_stat::USER_HZ;

// CPUStat shows how much time the cpu spend in various stages.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CPUStat {
    pub user: f64,
    pub nice: f64,
    pub system: f64,
    pub idle: f64,
    pub iowait: f64,
    pub irq: f64,
    pub softirq: f64,
    pub steal: f64,
    pub guest: f64,
    pub guest_nice: f64,
}

// SoftIRQStat represent the softirq statistics as exported in the procfs stat file.
// A nice introduction can be found at https://0xax.gitbooks.io/linux-insides/content/interrupts/interrupts-9.html
// It is possible to get per-cpu stats by reading `/proc/softirqs`.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SoftIRQStat {
    pub hi: u64,
    pub timer: u64,
    pub net_tx: u64,
    pub net_rx: u64,
    pub block: u64,
    pub block_io_poll: u64,
    pub tasklet: u64,
    pub sched: u64,
    pub hrtimer: u64,
    pub rcu: u64,
}

// Stat represents kernel/system statistics.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Stat {
    // Boot time in seconds since the Epoch.
    pub boot_time: u64,
    // Summed up cpu statistics.
    pub cpu_total: CPUStat,
    // Per-CPU statistics.
    pub cpu: HashMap<usize, CPUStat>,
    // Number of times interrupts were handled, which contains numbered and unnumbered IRQs.
    pub irq_total: u64,
    // Number of times a numbered IRQ was triggered.
    pub irq: Vec<u64>,
    // Number of times a context switch happened.
    pub context_switches: u64,
    // Number of times a process was created.
    pub process_created: u64,
    // Number of processes currently running.
    pub processes_running: u64,
    // Number of processes currently blocked (waiting for IO).
    pub processes_blocked: u64,
    // Number of times a softirq was scheduled.
    pub softirq_total: u64,
    // Detailed softirq statistics.
    pub softirq: SoftIRQStat,
}

impl ProcFs {
    pub async fn stat(&self) -> Result<Stat> {
        let file = self.proc.path(Path::new("stat"));
        let content = utils::read_file_no_stat(file).await?;

        let mut reader = Cursor::new(content);
        Self::parse_stat(&mut reader).await
    }

    async fn parse_stat<R: AsyncReadExt + Unpin>(r: R) -> Result<Stat> {
        let mut stat = Stat::default();

        let reader = BufReader::new(r);
        let mut lines = reader.lines();

        while let Some(line) = lines.next_line().await? {
            let parts: Vec<&str> = line.split_whitespace().collect();

            if parts.len() < 2 {
                continue;
            }

            match parts[0] {
                "btime" => stat.boot_time = parts[1].parse::<u64>()?,
                "intr" => {
                    stat.irq_total = parts[1].parse::<u64>()?;
                    stat.irq = parts[2..]
                        .iter()
                        .map(|s| s.parse::<u64>())
                        .collect::<std::result::Result<Vec<u64>, _>>()?;
                }
                "ctxt" => stat.context_switches = parts[1].parse::<u64>()?,
                "processes" => stat.process_created = parts[1].parse::<u64>()?,
                "procs_running" => stat.processes_running = parts[1].parse::<u64>()?,
                "procs_blocked" => stat.processes_blocked = parts[1].parse::<u64>()?,
                "softirq" => {
                    let (softirq, total) = Self::parse_softirq(&line).await?;
                    stat.softirq_total = total;
                    stat.softirq = softirq;
                }
                s if s.starts_with("cpu") => {
                    let (cpu_stat, cpu) = Self::parse_cpu_stat(&line).await?;
                    match cpu {
                        Some(cpu) => _ = stat.cpu.insert(cpu, cpu_stat),
                        None => stat.cpu_total = cpu_stat,
                    }
                }
                _ => {}
            }
        }

        Ok(stat)
    }

    async fn parse_cpu_stat(line: &str) -> Result<(CPUStat, Option<usize>)> {
        let parts: Vec<&str> = line.split_whitespace().collect();

        let cpu = parts[0];
        let values: Vec<f64> = parts[1..]
            .iter()
            .map(|s| s.parse::<f64>())
            .collect::<std::result::Result<Vec<f64>, _>>()?;

        if values.len() < 10 {
            return Err(Err::InvalidIndex(format!(
                "Not enough cpu values: {}",
                values.len()
            )));
        }

        let stat = CPUStat {
            user: values[0] / USER_HZ as f64,
            nice: values[1] / USER_HZ as f64,
            system: values[2] / USER_HZ as f64,
            idle: values[3] / USER_HZ as f64,
            iowait: values[4] / USER_HZ as f64,
            irq: values[5] / USER_HZ as f64,
            softirq: values[6] / USER_HZ as f64,
            steal: values[7] / USER_HZ as f64,
            guest: values[8] / USER_HZ as f64,
            guest_nice: values[9] / USER_HZ as f64,
        };

        if cpu == "cpu" {
            Ok((stat, None))
        } else {
            Ok((stat, Some(cpu[3..].parse::<usize>()?)))
        }
    }

    async fn parse_softirq(line: &str) -> Result<(SoftIRQStat, u64)> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let values: Vec<u64> = parts[1..]
            .iter()
            .map(|s| s.parse::<u64>())
            .collect::<std::result::Result<Vec<u64>, _>>()?;

        if values.len() < 11 {
            return Err(Err::InvalidIndex(format!(
                "Not enough softirq values: {}",
                values.len()
            )));
        }

        Ok((
            SoftIRQStat {
                hi: values[1],
                timer: values[2],
                net_tx: values[3],
                net_rx: values[4],
                block: values[5],
                block_io_poll: values[6],
                tasklet: values[7],
                sched: values[8],
                hrtimer: values[9],
                rcu: values[10],
            },
            values[0],
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_stat() {
        let fs = ProcFs::default();
        let stat = fs.stat().await.unwrap();
        println!("{:?}", stat.cpu);
        println!("irq len: {:?}", stat.irq.len());
    }
}
