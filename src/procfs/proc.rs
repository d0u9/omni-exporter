use super::fs::FS;

pub struct Proc {
    pub(super) fs: FS,
    pub(super) pid: usize,
}

impl Proc {
    pub fn new(fs: FS, pid: usize) -> Self {
        Self { fs, pid }
    }
}
