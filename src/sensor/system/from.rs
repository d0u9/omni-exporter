use std::vec;

use crate::fetcher::MeminfoMetrics;
use crate::metric;

impl From<MeminfoMetrics> for Vec<metric::Metric> {
    fn from(meminfo: MeminfoMetrics) -> Self {
        vec![metric::Metric::new(meminfo.mem_free_bytes.into())]
    }
}
