use super::super::internal::fs;

const DEFAULT_MOUNT_POINT: &str = fs::consts::DEFAULT_SYS_MOUNT_POINT;

pub struct FS {
    sys: fs::FS,
}

impl Default for FS {
    fn default() -> Self {
        Self::new()
    }
}

impl FS {
    pub fn new() -> Self {
        Self {
            sys: fs::FS::new(DEFAULT_MOUNT_POINT),
        }
    }
}
