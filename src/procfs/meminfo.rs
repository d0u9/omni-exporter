// https://github.com/prometheus/procfs/blob/master/meminfo.go

use std::io::Cursor;
use std::path::Path;

use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};

use crate::procfs::internal::utils;

use super::error::Result;
use super::fs::FS;

#[derive(Debug, Default)]
pub struct Meminfo {
    // Total usable ram (i.e. physical ram minus a few reserved
    // bits and the kernel binary code)
    pub mem_total: Option<u64>,
    pub mem_free: Option<u64>,
    pub mem_available: Option<u64>,
    pub buffers: Option<u64>,
    pub cached: Option<u64>,
    pub swap_cached: Option<u64>,
    pub active: Option<u64>,
    pub inactive: Option<u64>,
    pub active_anon: Option<u64>,
    pub inactive_anon: Option<u64>,
    pub active_file: Option<u64>,
    pub inactive_file: Option<u64>,
    pub unevictable: Option<u64>,
    pub mlocked: Option<u64>,
    pub swap_total: Option<u64>,
    pub swap_free: Option<u64>,
    pub dirty: Option<u64>,
    pub writeback: Option<u64>,
    pub writeback_tmp: Option<u64>,
    pub anon_pages: Option<u64>,
    pub mapped: Option<u64>,
    pub shmem: Option<u64>,
    pub kreclaimable: Option<u64>,
    pub slab: Option<u64>,
    pub sreclaimable: Option<u64>,
    pub sunreclaim: Option<u64>,
    pub kernel_stack: Option<u64>,
    pub page_tables: Option<u64>,
    pub nfs_unstable: Option<u64>,
    pub bounce: Option<u64>,
    pub commit_limit: Option<u64>,
    pub committed_as: Option<u64>,
    pub vmalloc_total: Option<u64>,
    pub vmalloc_used: Option<u64>,
    pub vmalloc_chunk: Option<u64>,
    pub percpu: Option<u64>,
    pub hardware_corrupted: Option<u64>,
    pub anon_huge_pages: Option<u64>,
    pub shmem_huge_pages: Option<u64>,
    pub shmem_pmd_mapped: Option<u64>,
    pub file_huge_pages: Option<u64>,
    pub file_pmd_mapped: Option<u64>,
    pub huge_pages_total: Option<u64>,
    pub huge_pages_free: Option<u64>,
    pub huge_pages_rsvd: Option<u64>,
    pub huge_pages_surp: Option<u64>,
    pub hugepagesize: Option<u64>,
    pub hugetlb: Option<u64>,
    pub direct_map_4k: Option<u64>,
    pub direct_map_2m: Option<u64>,
    pub direct_map_1g: Option<u64>,
    pub cma_total: Option<u64>,
    pub cma_free: Option<u64>,
}

impl FS {
    pub async fn meminfo(&self) -> Result<Meminfo> {
        let proc = self.proc.path(Path::new("meminfo"));
        let content = utils::read_file_no_stat(proc).await?;

        let mut reader = Cursor::new(content);
        Self::parse_meminfo(&mut reader).await
    }

    async fn parse_meminfo<R: AsyncReadExt + Unpin>(r: R) -> Result<Meminfo> {
        let mut meminfo = Meminfo::default();

        let reader = BufReader::new(r);
        let mut lines = reader.lines();

        while let Some(line) = lines.next_line().await? {
            let parts: Vec<&str> = line.split_whitespace().collect();

            let val = parts[1].parse::<u64>()?;
            let val = match parts.get(2).copied() {
                Some("kB") => val * 1024,
                Some("MB") => val * 1024 * 1024,
                Some("GB") => val * 1024 * 1024 * 1024,
                _ => val,
            };

            match parts[0] {
                "MemTotal:" => meminfo.mem_total = Some(val),
                "MemFree:" => meminfo.mem_free = Some(val),
                "MemAvailable:" => meminfo.mem_available = Some(val),
                "Buffers:" => meminfo.buffers = Some(val),
                "Cached:" => meminfo.cached = Some(val),
                "SwapCached:" => meminfo.swap_cached = Some(val),
                "Active:" => meminfo.active = Some(val),
                "Inactive:" => meminfo.inactive = Some(val),
                "Active(anon):" => meminfo.active_anon = Some(val),
                "Inactive(anon):" => meminfo.inactive_anon = Some(val),
                "Active(file):" => meminfo.active_file = Some(val),
                "Inactive(file):" => meminfo.inactive_file = Some(val),
                "Unevictable:" => meminfo.unevictable = Some(val),
                "Mlocked:" => meminfo.mlocked = Some(val),
                "SwapTotal:" => meminfo.swap_total = Some(val),
                "SwapFree:" => meminfo.swap_free = Some(val),
                "Dirty:" => meminfo.dirty = Some(val),
                "Writeback:" => meminfo.writeback = Some(val),
                "AnonPages:" => meminfo.anon_pages = Some(val),
                "Mapped:" => meminfo.mapped = Some(val),
                "Shmem:" => meminfo.shmem = Some(val),
                "KReclaimable:" => meminfo.kreclaimable = Some(val),
                "Slab:" => meminfo.slab = Some(val),
                "SReclaimable:" => meminfo.sreclaimable = Some(val),
                "SUnreclaim:" => meminfo.sunreclaim = Some(val),
                "KernelStack:" => meminfo.kernel_stack = Some(val),
                "PageTables:" => meminfo.page_tables = Some(val),
                "NFS_Unstable:" => meminfo.nfs_unstable = Some(val),
                "Bounce:" => meminfo.bounce = Some(val),
                "WritebackTmp:" => meminfo.writeback_tmp = Some(val),
                "CommitLimit:" => meminfo.commit_limit = Some(val),
                "Committed_AS:" => meminfo.committed_as = Some(val),
                "VmallocTotal:" => meminfo.vmalloc_total = Some(val),
                "VmallocUsed:" => meminfo.vmalloc_used = Some(val),
                "VmallocChunk:" => meminfo.vmalloc_chunk = Some(val),
                "Percpu:" => meminfo.percpu = Some(val),
                "HardwareCorrupted:" => meminfo.hardware_corrupted = Some(val),
                "AnonHugePages:" => meminfo.anon_huge_pages = Some(val),
                "ShmemHugePages:" => meminfo.shmem_huge_pages = Some(val),
                "ShmemPmdMapped:" => meminfo.shmem_pmd_mapped = Some(val),
                "FileHugePages:" => meminfo.file_huge_pages = Some(val),
                "FilePmdMapped:" => meminfo.file_pmd_mapped = Some(val),
                "HugePages_Total:" => meminfo.huge_pages_total = Some(val),
                "HugePages_Free:" => meminfo.huge_pages_free = Some(val),
                "HugePages_Rsvd:" => meminfo.huge_pages_rsvd = Some(val),
                "HugePages_Surp:" => meminfo.huge_pages_surp = Some(val),
                "Hugepagesize:" => meminfo.hugepagesize = Some(val),
                "Hugetlb:" => meminfo.hugetlb = Some(val),
                "DirectMap4k:" => meminfo.direct_map_4k = Some(val),
                "DirectMap2M:" => meminfo.direct_map_2m = Some(val),
                "DirectMap1G:" => meminfo.direct_map_1g = Some(val),
                "cma_total:" => meminfo.cma_total = Some(val),
                "cma_free:" => meminfo.cma_free = Some(val),
                _ => (),
            }
        }

        Ok(meminfo)
    }
}

#[tokio::test]
async fn test_meminfo() {
    let fs = FS::new_default();
    let meminfo = fs.meminfo().await.unwrap();
    println!("{:?}", meminfo);
}
