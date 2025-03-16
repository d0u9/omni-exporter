// https://github.com/prometheus/procfs/blob/master/internal/fs/fs.go
use std::path::{Path, PathBuf};

pub mod consts {
    // DEFAULT_PROC_MOUNT_POINT is the common mount point of the proc filesystem.
    pub const DEFAULT_PROC_MOUNT_POINT: &str = "/proc";

    // DEFAULT_SYS_MOUNT_POINT is the common mount point of the sys filesystem.
    pub const DEFAULT_SYS_MOUNT_POINT: &str = "/sys";

    // DEFAULT_CONFIGFS_MOUNT_POINT is the common mount point of the configfs.
    pub const DEFAULT_CONFIGFS_MOUNT_POINT: &str = "/sys/kernel/config";

    // DEFAULT_SELINUX_MOUNT_POINT is the common mount point of the selinuxfs.
    pub const DEFAULT_SELINUX_MOUNT_POINT: &str = "/sys/fs/selinux";
}

// FS represents a pseudo-filesystem, normally /proc or /sys, which provides an
// interface to kernel data structures.
pub struct FS {
    inner: PathBuf,
}

impl FS {
    pub fn new<T: AsRef<Path>>(mount_point: T) -> Self {
        let inner = mount_point.as_ref().to_owned();
        Self { inner }
    }

    pub fn path<P: AsRef<Path>>(&self, p: P) -> PathBuf {
        self.inner.clone().join(p.as_ref())
    }
}
