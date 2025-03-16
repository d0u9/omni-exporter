use std::path::Path;

use crate::error::Result;

use super::internal::fs;

const DEFAULT_MOUNT_POINT: &str = fs::consts::DEFAULT_PROC_MOUNT_POINT;
const SECTOR_SIZE: usize = 512;

pub struct FS {
    pub(super) proc: fs::FS,
    pub(super) is_real: bool,
}

impl FS {
    pub fn new_default() -> Result<Self> {
        Self::new(DEFAULT_MOUNT_POINT)
    }

    pub fn new<T: AsRef<Path>>(mount_point: T) -> Result<Self> {
        Ok(Self {
            proc: fs::FS::new(mount_point)?,
            is_real: true,
        })
    }
}
