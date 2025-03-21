mod internal;

mod fs;
pub use fs::ProcFs;
pub mod sys;

pub mod meminfo;
pub mod proc_stat;
pub mod stat;

mod error;
#[allow(unused_imports)]
pub use error::Result;
#[allow(unused_imports)]
pub use error::{Err, IOErr};
