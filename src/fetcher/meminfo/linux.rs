use crate::error::Result;
use std::borrow::Cow;

use super::super::FetcherMetric;
use super::super::FetcherMetricName;
use crate::procfs;

pub type MeminfoMetric = FetcherMetric<MeminfoMetricNames>;

pub struct MeminfoInner {
    procfs: procfs::ProcFs,
}

impl MeminfoInner {
    pub fn new() -> Self {
        let procfs = procfs::ProcFs::default();
        Self { procfs }
    }

    pub async fn get_meminfo(&self) -> Result<Vec<MeminfoMetric>> {
        let m = self.procfs.meminfo().await?;
        let metrics = vec![
            MeminfoMetric::new(MeminfoMetricNames::ActiveAnonBytes, m.active_anon),
            MeminfoMetric::new(MeminfoMetricNames::ActiveBytes, m.active),
            MeminfoMetric::new(MeminfoMetricNames::ActiveFileBytes, m.active_file),
            MeminfoMetric::new(MeminfoMetricNames::AnonHugePagesBytes, m.anon_huge_pages),
            MeminfoMetric::new(MeminfoMetricNames::AnonPagesBytes, m.anon_pages),
            MeminfoMetric::new(MeminfoMetricNames::BounceBytes, m.bounce),
            MeminfoMetric::new(MeminfoMetricNames::BuffersBytes, m.buffers),
            MeminfoMetric::new(MeminfoMetricNames::CachedBytes, m.cached),
            MeminfoMetric::new(MeminfoMetricNames::CmaFreeBytes, m.cma_free),
            MeminfoMetric::new(MeminfoMetricNames::CmaTotalBytes, m.cma_total),
            MeminfoMetric::new(MeminfoMetricNames::CommitLimitBytes, m.commit_limit),
            MeminfoMetric::new(MeminfoMetricNames::CommittedASBytes, m.committed_as),
            MeminfoMetric::new(MeminfoMetricNames::DirectMap1GBytes, m.direct_map_1g),
            MeminfoMetric::new(MeminfoMetricNames::DirectMap2MBytes, m.direct_map_2m),
            MeminfoMetric::new(MeminfoMetricNames::DirectMap4kBytes, m.direct_map_4k),
            MeminfoMetric::new(MeminfoMetricNames::DirtyBytes, m.dirty),
            MeminfoMetric::new(
                MeminfoMetricNames::HardwareCorruptedBytes,
                m.hardware_corrupted,
            ),
            MeminfoMetric::new(MeminfoMetricNames::HugePagesFree, m.huge_pages_free),
            MeminfoMetric::new(MeminfoMetricNames::HugePagesRsvd, m.huge_pages_rsvd),
            MeminfoMetric::new(MeminfoMetricNames::HugepagesizeBytes, m.hugepagesize),
            MeminfoMetric::new(MeminfoMetricNames::InactiveAnonBytes, m.inactive_anon),
            MeminfoMetric::new(MeminfoMetricNames::InactiveBytes, m.inactive),
            MeminfoMetric::new(MeminfoMetricNames::InactiveFileBytes, m.inactive_file),
            MeminfoMetric::new(MeminfoMetricNames::KernelStackBytes, m.kernel_stack),
            MeminfoMetric::new(MeminfoMetricNames::MappedBytes, m.mapped),
            MeminfoMetric::new(MeminfoMetricNames::MemAvailableBytes, m.mem_available),
            MeminfoMetric::new(MeminfoMetricNames::MemFreeBytes, m.mem_free),
            MeminfoMetric::new(MeminfoMetricNames::MlockedBytes, m.mlocked),
            MeminfoMetric::new(MeminfoMetricNames::NFSUnstableBytes, m.nfs_unstable),
            MeminfoMetric::new(MeminfoMetricNames::PageTablesBytes, m.page_tables),
            MeminfoMetric::new(MeminfoMetricNames::PercpuBytes, m.percpu),
            MeminfoMetric::new(MeminfoMetricNames::SReclaimableBytes, m.sreclaimable),
            MeminfoMetric::new(MeminfoMetricNames::SUnreclaimBytes, m.sunreclaim),
            MeminfoMetric::new(MeminfoMetricNames::ShmemBytes, m.shmem),
            MeminfoMetric::new(MeminfoMetricNames::ShmemHugePagesBytes, m.shmem_huge_pages),
            MeminfoMetric::new(MeminfoMetricNames::ShmemPmdMappedBytes, m.shmem_pmd_mapped),
            MeminfoMetric::new(MeminfoMetricNames::SlabBytes, m.slab),
            MeminfoMetric::new(MeminfoMetricNames::SwapCachedBytes, m.swap_cached),
            MeminfoMetric::new(MeminfoMetricNames::SwapFreeBytes, m.swap_free),
            MeminfoMetric::new(MeminfoMetricNames::SwapTotalBytes, m.swap_total),
            MeminfoMetric::new(MeminfoMetricNames::UnevictableBytes, m.unevictable),
            MeminfoMetric::new(MeminfoMetricNames::VmallocChunkBytes, m.vmalloc_chunk),
            MeminfoMetric::new(MeminfoMetricNames::VmallocTotalBytes, m.vmalloc_total),
            MeminfoMetric::new(MeminfoMetricNames::VmallocUsedBytes, m.vmalloc_used),
            MeminfoMetric::new(MeminfoMetricNames::WritebackBytes, m.writeback),
            MeminfoMetric::new(MeminfoMetricNames::WritebackTmpBytes, m.writeback_tmp),
        ];

        Ok(metrics)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum MeminfoMetricNames {
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

impl MeminfoMetricNames {
    pub fn as_str(&self) -> &'static str {
        match self {
            MeminfoMetricNames::ActiveAnonBytes => "active_anon_bytes",
            MeminfoMetricNames::ActiveBytes => "active_bytes",
            MeminfoMetricNames::ActiveFileBytes => "active_file_bytes",
            MeminfoMetricNames::AnonHugePagesBytes => "anon_hugepages_bytes",
            MeminfoMetricNames::AnonPagesBytes => "anon_pages_bytes",
            MeminfoMetricNames::BounceBytes => "bounce_bytes",
            MeminfoMetricNames::BuffersBytes => "buffers_bytes",
            MeminfoMetricNames::CachedBytes => "cached_bytes",
            MeminfoMetricNames::CmaFreeBytes => "cma_free_bytes",
            MeminfoMetricNames::CmaTotalBytes => "cma_total_bytes",
            MeminfoMetricNames::CommitLimitBytes => "commit_limit_bytes",
            MeminfoMetricNames::CommittedASBytes => "committed_as_bytes",
            MeminfoMetricNames::DirectMap1GBytes => "direct_map_1g_bytes",
            MeminfoMetricNames::DirectMap2MBytes => "direct_map_2m_bytes",
            MeminfoMetricNames::DirectMap4kBytes => "direct_map_4k_bytes",
            MeminfoMetricNames::DirtyBytes => "dirty_bytes",
            MeminfoMetricNames::HardwareCorruptedBytes => "hardware_corrupted_bytes",
            MeminfoMetricNames::HugePagesFree => "hugepages_free",
            MeminfoMetricNames::HugePagesRsvd => "hugepages_rsvd",
            MeminfoMetricNames::HugepagesizeBytes => "hugepagesize_bytes",
            MeminfoMetricNames::InactiveAnonBytes => "inactive_anon_bytes",
            MeminfoMetricNames::InactiveBytes => "inactive_bytes",
            MeminfoMetricNames::InactiveFileBytes => "inactive_file_bytes",
            MeminfoMetricNames::KernelStackBytes => "kernel_stack_bytes",
            MeminfoMetricNames::MappedBytes => "mapped_bytes",
            MeminfoMetricNames::MemAvailableBytes => "mem_available_bytes",
            MeminfoMetricNames::MemFreeBytes => "mem_free_bytes",
            MeminfoMetricNames::MemTotalBytes => "mem_total_bytes",
            MeminfoMetricNames::MlockedBytes => "mlocked_bytes",
            MeminfoMetricNames::NFSUnstableBytes => "nfs_unstable_bytes",
            MeminfoMetricNames::PageTablesBytes => "page_tables_bytes",
            MeminfoMetricNames::PercpuBytes => "percpu_bytes",
            MeminfoMetricNames::SReclaimableBytes => "s_reclaimable_bytes",
            MeminfoMetricNames::ShmemBytes => "shmem_bytes",
            MeminfoMetricNames::ShmemHugePagesBytes => "shmem_hugepages_bytes",
            MeminfoMetricNames::ShmemPmdMappedBytes => "shmem_pmd_mapped_bytes",
            MeminfoMetricNames::SlabBytes => "slab_bytes",
            MeminfoMetricNames::SUnreclaimBytes => "s_unreclaim_bytes",
            MeminfoMetricNames::SwapCachedBytes => "swap_cached_bytes",
            MeminfoMetricNames::SwapFreeBytes => "swap_free_bytes",
            MeminfoMetricNames::SwapTotalBytes => "swap_total_bytes",
            MeminfoMetricNames::UnevictableBytes => "unevictable_bytes",
            MeminfoMetricNames::VmallocChunkBytes => "vmalloc_chunk_bytes",
            MeminfoMetricNames::VmallocTotalBytes => "vmalloc_total_bytes",
            MeminfoMetricNames::VmallocUsedBytes => "vmalloc_used_bytes",
            MeminfoMetricNames::WritebackBytes => "writeback_bytes",
            MeminfoMetricNames::WritebackTmpBytes => "writeback_tmp_bytes",
        }
    }
}

impl FetcherMetricName for MeminfoMetricNames {
    fn as_str(&self) -> &'static str {
        self.as_str()
    }
}

impl From<MeminfoMetricNames> for Cow<'static, str> {
    fn from(name: MeminfoMetricNames) -> Self {
        Cow::Borrowed(name.as_str())
    }
}
