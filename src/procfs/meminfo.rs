// https://github.com/prometheus/procfs/blob/master/meminfo.go
use crate::error::Result;
use crate::procfs::internal::utils;
use std::path::Path;

use super::fs::FS;

#[derive(Debug)]
struct Meminfo {
    // Total usable ram (i.e. physical ram minus a few reserved
    // bits and the kernel binary code)
    pub mem_total: u64,
}

impl Meminfo {
    pub fn meminfo() -> Self {
        Self { mem_total: 0 }
    }
}

impl FS {
    pub async fn meminfo(&self) -> Result<Meminfo> {
        let proc = self.proc.path(Path::new("meminfo"));
        let content = utils::read_file_no_stat(proc).await?;

        let content = String::from_utf8(content).unwrap();
        println!("{:?}", content);

        Ok(Meminfo { mem_total: 0 })
    }
}

#[tokio::test]
async fn test_meminfo() {
    let fs = FS::new_default().unwrap();
    let meminfo = fs.meminfo().await.unwrap();
    println!("{:?}", meminfo);
}
