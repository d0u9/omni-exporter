use std::ffi::OsStr;

use tokio::fs;

use super::super::error::Result;
use super::fs::FS;

// CPU represents a path to a CPU located in `/sys/devices/system/cpu/cpu[0-9]*`.
#[derive(Debug)]
pub struct Cpu(usize);

impl Cpu {
    pub fn number_from_name<T: AsRef<OsStr>>(name: T) -> Option<Self> {
        let name = name.as_ref().to_string_lossy();
        if !name.starts_with("cpu") {
            return None;
        }
        name[3..].parse::<usize>().ok().map(Self)
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
            if let Some(number) = entry
                .path()
                .is_dir()
                .then(|| path.file_name())
                .flatten()
                .and_then(Cpu::number_from_name)
            {
                cpus.push(number);
            }
        }

        Ok(cpus)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cpu_number() {
        let fs = FS::default();
        println!("{:?}", fs.cpus().await.unwrap());
    }
}
