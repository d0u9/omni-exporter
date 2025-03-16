use std::path::Path;

use super::internal::fs;

pub const DEFAULT_MOUNT_POINT: &str = fs::consts::DEFAULT_PROC_MOUNT_POINT;
pub const SECTOR_SIZE: usize = 512;

pub struct FS {
    pub(super) proc: fs::FS,
}

impl FS {
    pub fn new_default() -> Self {
        Self::new(DEFAULT_MOUNT_POINT)
    }

    pub fn new<T: AsRef<Path>>(mount_point: T) -> Self {
        Self {
            proc: fs::FS::new(mount_point),
        }
    }
}
