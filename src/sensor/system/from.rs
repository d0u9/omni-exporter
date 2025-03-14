use std::vec;

use crate::fetcher::MeminfoMetrics;
use crate::metric::{Metric, Metrics};

impl From<MeminfoMetrics> for Metrics {
    fn from(meminfo: MeminfoMetrics) -> Self {
        Metrics::from_vec(vec![Metric::new(meminfo.mem_free_bytes.into())])
    }
}
