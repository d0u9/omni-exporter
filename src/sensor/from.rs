use std::collections::HashMap;
use std::vec;

use crate::fetcher::MeminfoMetrics;
use crate::metric::{Metric, MetricType, MetricValue};

impl From<MeminfoMetrics> for Vec<Metric<'static>> {
    fn from(meminfo: MeminfoMetrics) -> Self {
        vec![Metric::new(
            MetricValue::U64(meminfo.mem_free_bytes.unwrap_or(0)),
            None,
            None,
        )]
    }
} 