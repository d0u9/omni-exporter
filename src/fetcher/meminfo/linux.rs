use crate::error::Result;
use crate::exotic::system as syslib;
use crate::metric::MetricValue;

pub struct MeminfoInner {
    inner: syslib::System,
}

impl MeminfoInner {
    pub fn new() -> Self {
        let sys = syslib::System::new();
        Self::new_with_system(sys)
    }

    pub fn new_with_system(sys: syslib::System) -> Self {
        Self { inner: sys }
    }

    pub async fn get_meminfo(&self) -> Result<Vec<Metric>> {
        let mut metrics = Vec::new();

        let m = self.inner.free_memory().await;
        metrics.push(Metric {
            name: MetricNames::MemFreeBytes,
            value: MetricValue::U64(m),
        });

        Ok(metrics)
    }
}

#[derive(Debug, PartialEq)]
pub enum MetricNames {
    // Common for all OS
    ActiveAnonBytes,
    ActiveBytes,
    ActiveFileBytes,
    AnonHugePagesBytes,
    AnonPagesBytes,
    BounceBytes,
    BuffersBytes,
    CachedBytes,
    CmaFreeBytes,
    CmaTotalBytes,
    CommitLimitBytes,
    CommittedASBytes,
    DirectMap1GBytes,
    DirectMap2MBytes,
    DirectMap4kBytes,
    DirtyBytes,
    HardwareCorruptedBytes,
    HugePagesFree,
    HugePagesRsvd,
    HugepagesizeBytes,
    InactiveAnonBytes,
    InactiveBytes,
    InactiveFileBytes,
    KernelStackBytes,
    MappedBytes,
    MemAvailableBytes,
    MemFreeBytes,
    MemTotalBytes,
    MlockedBytes,
    NFSUnstableBytes,
    PageTablesBytes,
    PercpuBytes,
    SReclaimableBytes,
    SUnreclaimBytes,
    ShmemBytes,
    ShmemHugePagesBytes,
    ShmemPmdMappedBytes,
    SlabBytes,
    SwapCachedBytes,
    SwapFreeBytes,
    SwapTotalBytes,
    UnevictableBytes,
    VmallocChunkBytes,
    VmallocTotalBytes,
    VmallocUsedBytes,
    WritebackBytes,
    WritebackTmpBytes,
}

impl MetricNames {
    pub fn to_str(&self) -> &'static str {
        match self {
            MetricNames::ActiveAnonBytes => "active_anon_bytes",
            MetricNames::ActiveBytes => "active_bytes",
            MetricNames::ActiveFileBytes => "active_file_bytes",
            MetricNames::AnonHugePagesBytes => "anon_hugepages_bytes",
            MetricNames::AnonPagesBytes => "anon_pages_bytes",
            MetricNames::BounceBytes => "bounce_bytes",
            MetricNames::BuffersBytes => "buffers_bytes",
            MetricNames::CachedBytes => "cached_bytes",
            MetricNames::CmaFreeBytes => "cma_free_bytes",
            MetricNames::CmaTotalBytes => "cma_total_bytes",
            MetricNames::CommitLimitBytes => "commit_limit_bytes",
            MetricNames::CommittedASBytes => "committed_as_bytes",
            MetricNames::DirectMap1GBytes => "direct_map_1g_bytes",
            MetricNames::DirectMap2MBytes => "direct_map_2m_bytes",
            MetricNames::DirectMap4kBytes => "direct_map_4k_bytes",
            MetricNames::DirtyBytes => "dirty_bytes",
            MetricNames::HardwareCorruptedBytes => "hardware_corrupted_bytes",
            MetricNames::HugePagesFree => "hugepages_free",
            MetricNames::HugePagesRsvd => "hugepages_rsvd",
            MetricNames::HugepagesizeBytes => "hugepagesize_bytes",
            MetricNames::InactiveAnonBytes => "inactive_anon_bytes",
            MetricNames::InactiveBytes => "inactive_bytes",
            MetricNames::InactiveFileBytes => "inactive_file_bytes",
            MetricNames::KernelStackBytes => "kernel_stack_bytes",
            MetricNames::MappedBytes => "mapped_bytes",
            MetricNames::MemAvailableBytes => "mem_available_bytes",
            MetricNames::MemFreeBytes => "mem_free_bytes",
            MetricNames::MemTotalBytes => "mem_total_bytes",
            MetricNames::MlockedBytes => "mlocked_bytes",
            MetricNames::NFSUnstableBytes => "nfs_unstable_bytes",
            MetricNames::PageTablesBytes => "page_tables_bytes",
            MetricNames::PercpuBytes => "percpu_bytes",
            MetricNames::SReclaimableBytes => "s_reclaimable_bytes",
            MetricNames::ShmemBytes => "shmem_bytes",
            MetricNames::ShmemHugePagesBytes => "shmem_hugepages_bytes",
            MetricNames::ShmemPmdMappedBytes => "shmem_pmd_mapped_bytes",
            MetricNames::SlabBytes => "slab_bytes",
            MetricNames::SUnreclaimBytes => "s_unreclaim_bytes",
            MetricNames::SwapCachedBytes => "swap_cached_bytes",
            MetricNames::SwapFreeBytes => "swap_free_bytes",
            MetricNames::SwapTotalBytes => "swap_total_bytes",
            MetricNames::UnevictableBytes => "unevictable_bytes",
            MetricNames::VmallocChunkBytes => "vmalloc_chunk_bytes",
            MetricNames::VmallocTotalBytes => "vmalloc_total_bytes",
            MetricNames::VmallocUsedBytes => "vmalloc_used_bytes",
            MetricNames::WritebackBytes => "writeback_bytes",
            MetricNames::WritebackTmpBytes => "writeback_tmp_bytes",
        }
    }
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

#[derive(Debug, PartialEq)]
pub struct Metric {
    pub name: MetricNames,
    pub value: MetricValue,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_meminfo() {
        let sys = syslib::System::new();
        let meminfo = Meminfo::new_with_system(sys);

        let metrics = meminfo.get_meminfo().await.unwrap();
        assert!(metrics.len() > 0);
        assert!(metrics.iter().any(|m| m.name == MetricNames::MemFreeBytes));

        println!("{:?}", metrics);
    }
}
