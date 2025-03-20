use std::path::Path;

use super::super::internal::fs;

const DEFAULT_MOUNT_POINT: &str = fs::consts::DEFAULT_SYS_MOUNT_POINT;

#[derive(Debug)]
pub struct SysFs {
    fs: fs::FS,
}

impl Default for SysFs {
    fn default() -> Self {
        Self::new()
    }
}

impl SysFs {
    pub fn new() -> Self {
        Self {
            fs: fs::FS::new(DEFAULT_MOUNT_POINT),
        }
    }

    pub fn join<T: AsRef<Path>>(&self, p: T) -> Self {
        Self {
            fs: self.fs.join(p),
        }
    }
}

impl AsRef<Path> for SysFs {
    fn as_ref(&self) -> &Path {
        self.fs.as_ref()
    }
}
