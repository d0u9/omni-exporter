use crate::error::Result;
use crate::procfs::fs;

use super::Metric;

pub struct MeminfoInner {
    procfs: fs::FS,
}

impl MeminfoInner {
    pub fn new() -> Self {
        let procfs = fs::FS::new_default();
        Self { procfs }
    }

    pub async fn get_meminfo(&self) -> Result<Vec<Metric>> {
        let m = self.procfs.meminfo().await?;
        let metrics = vec![
            Metric {
                name: MetricNames::ActiveAnonBytes,
                value: m.active_anon.into(),
            },
            Metric {
                name: MetricNames::ActiveBytes,
                value: m.active.into(),
            },
            Metric {
                name: MetricNames::ActiveFileBytes,
                value: m.active_file.into(),
            },
            Metric {
                name: MetricNames::AnonHugePagesBytes,
                value: m.anon_huge_pages.into(),
            },
            Metric {
                name: MetricNames::AnonPagesBytes,
                value: m.anon_pages.into(),
            },
            Metric {
                name: MetricNames::BounceBytes,
                value: m.bounce.into(),
            },
            Metric {
                name: MetricNames::BuffersBytes,
                value: m.buffers.into(),
            },
            Metric {
                name: MetricNames::CachedBytes,
                value: m.cached.into(),
            },
            Metric {
                name: MetricNames::CmaFreeBytes,
                value: m.cma_free.into(),
            },
            Metric {
                name: MetricNames::CmaTotalBytes,
                value: m.cma_total.into(),
            },
            Metric {
                name: MetricNames::CommitLimitBytes,
                value: m.commit_limit.into(),
            },
            Metric {
                name: MetricNames::CommittedASBytes,
                value: m.committed_as.into(),
            },
            Metric {
                name: MetricNames::DirectMap1GBytes,
                value: m.direct_map_1g.into(),
            },
            Metric {
                name: MetricNames::DirectMap2MBytes,
                value: m.direct_map_2m.into(),
            },
            Metric {
                name: MetricNames::DirectMap4kBytes,
                value: m.direct_map_4k.into(),
            },
            Metric {
                name: MetricNames::DirtyBytes,
                value: m.dirty.into(),
            },
            Metric {
                name: MetricNames::HardwareCorruptedBytes,
                value: m.hardware_corrupted.into(),
            },
            Metric {
                name: MetricNames::HugePagesFree,
                value: m.huge_pages_free.into(),
            },
            Metric {
                name: MetricNames::HugePagesRsvd,
                value: m.huge_pages_rsvd.into(),
            },
            Metric {
                name: MetricNames::HugepagesizeBytes,
                value: m.hugepagesize.into(),
            },
            Metric {
                name: MetricNames::InactiveAnonBytes,
                value: m.inactive_anon.into(),
            },
            Metric {
                name: MetricNames::InactiveBytes,
                value: m.inactive.into(),
            },
            Metric {
                name: MetricNames::InactiveFileBytes,
                value: m.inactive_file.into(),
            },
            Metric {
                name: MetricNames::KernelStackBytes,
                value: m.kernel_stack.into(),
            },
            Metric {
                name: MetricNames::MappedBytes,
                value: m.mapped.into(),
            },
            Metric {
                name: MetricNames::MemAvailableBytes,
                value: m.mem_available.into(),
            },
            Metric {
                name: MetricNames::MemFreeBytes,
                value: m.mem_free.into(),
            },
            Metric {
                name: MetricNames::MlockedBytes,
                value: m.mlocked.into(),
            },
            Metric {
                name: MetricNames::NFSUnstableBytes,
                value: m.nfs_unstable.into(),
            },
            Metric {
                name: MetricNames::PageTablesBytes,
                value: m.page_tables.into(),
            },
            Metric {
                name: MetricNames::PercpuBytes,
                value: m.percpu.into(),
            },
            Metric {
                name: MetricNames::SReclaimableBytes,
                value: m.sreclaimable.into(),
            },
            Metric {
                name: MetricNames::SUnreclaimBytes,
                value: m.sunreclaim.into(),
            },
            Metric {
                name: MetricNames::ShmemBytes,
                value: m.shmem.into(),
            },
            Metric {
                name: MetricNames::ShmemHugePagesBytes,
                value: m.shmem_huge_pages.into(),
            },
            Metric {
                name: MetricNames::ShmemPmdMappedBytes,
                value: m.shmem_pmd_mapped.into(),
            },
            Metric {
                name: MetricNames::SlabBytes,
                value: m.slab.into(),
            },
            Metric {
                name: MetricNames::SwapCachedBytes,
                value: m.swap_cached.into(),
            },
            Metric {
                name: MetricNames::SwapFreeBytes,
                value: m.swap_free.into(),
            },
            Metric {
                name: MetricNames::SwapTotalBytes,
                value: m.swap_total.into(),
            },
            Metric {
                name: MetricNames::UnevictableBytes,
                value: m.unevictable.into(),
            },
            Metric {
                name: MetricNames::VmallocChunkBytes,
                value: m.vmalloc_chunk.into(),
            },
            Metric {
                name: MetricNames::VmallocTotalBytes,
                value: m.vmalloc_total.into(),
            },
            Metric {
                name: MetricNames::VmallocUsedBytes,
                value: m.vmalloc_used.into(),
            },
            Metric {
                name: MetricNames::WritebackBytes,
                value: m.writeback.into(),
            },
            Metric {
                name: MetricNames::WritebackTmpBytes,
                value: m.writeback_tmp.into(),
            },
        ];

        Ok(metrics)
    }
}

#[derive(Debug, PartialEq)]
pub enum MetricNames {
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
