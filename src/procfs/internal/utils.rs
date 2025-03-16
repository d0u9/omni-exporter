use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use crate::error::Result;

// ReadFileNoStat uses io.ReadAll to read contents of entire file.
// This is similar to os.ReadFile but without the call to os.Stat, because
// many files in /proc and /sys report incorrect file sizes (either 0 or 4096).
// Reads a max file size of 1024kB.  For files larger than this, a scanner
// should be used.
pub async fn read_file_no_stat<P: AsRef<Path>>(filename: P) -> Result<Vec<u8>> {
    let mut file = File::open(filename).await.unwrap();
    let mut buffer = vec![0; 1024 * 1024];
    let bytes_read = file.read(&mut buffer).await.unwrap();
    buffer.truncate(bytes_read);

    Ok(buffer)
}
