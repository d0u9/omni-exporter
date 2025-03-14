use std::vec;

use crate::fetcher::MeminfoMetrics;
use crate::metric::Metric;

impl From<MeminfoMetrics> for Vec<Metric> {
    fn from(meminfo: MeminfoMetrics) -> Self {
        vec![Metric::new(meminfo.mem_free_bytes.into())]
    }
}
