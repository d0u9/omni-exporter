use std::path::Path;

use crate::internal::fs;

pub const DEFAULT_MOUNT_POINT: &str = fs::consts::DEFAULT_PROC_MOUNT_POINT;

pub struct ProcFs {
    pub(super) proc: fs::FS,
}

impl Default for ProcFs {
    fn default() -> Self {
        Self::new(DEFAULT_MOUNT_POINT)
    }
}

impl ProcFs {
    pub fn new<T: AsRef<Path>>(mount_point: T) -> Self {
        Self {
            proc: fs::FS::new(mount_point),
        }
    }
}
