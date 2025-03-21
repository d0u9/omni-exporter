use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use crate::error::{Err, Result};

mod consts {
    pub const SYS_FILE_BUFFER_SIZE: usize = 128;
}

pub async fn sys_read_file<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
    let mut file = File::open(path).await?;
    let mut buffer = vec![0; consts::SYS_FILE_BUFFER_SIZE];
    let n = file.read(&mut buffer).await?;
    buffer.truncate(n);

    Ok(buffer)
}

pub async fn sys_read_file_string<P: AsRef<Path>>(path: P) -> Result<String> {
    let bytes = sys_read_file(path).await?;
    String::from_utf8(bytes)
        .map_err(|e| Err::ParseString(e.to_string()))
        .map(|s| s.trim().to_string())
}
