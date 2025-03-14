use std::collections::HashMap;
use std::vec;

use crate::fetcher::MeminfoMetrics;
use crate::types::{Metric, MetricType};

impl From<MeminfoMetrics> for Vec<Metric<HashMap<String, String>>> {
    fn from(meminfo: MeminfoMetrics) -> Self {
        vec![Metric::new_with_no_labels(
            MetricType::Gauge,
            meminfo.mem_free_bytes.into(),
        )]
    }
}
