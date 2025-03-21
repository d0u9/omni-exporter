use std::path::Path;

use super::super::internal::fs;

const DEFAULT_MOUNT_POINT: &str = fs::consts::DEFAULT_SYS_MOUNT_POINT;

#[derive(Debug)]
pub struct SysFs {
    pub(super) sysfs: fs::FS,
}

impl Default for SysFs {
    fn default() -> Self {
        Self::new()
    }
}

impl SysFs {
    pub fn new() -> Self {
        Self {
            sysfs: fs::FS::new(DEFAULT_MOUNT_POINT),
        }
    }
}

impl AsRef<Path> for SysFs {
    fn as_ref(&self) -> &Path {
        self.sysfs.as_ref()
    }
}
