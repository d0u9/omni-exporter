use std::sync::OnceLock;

use crate::error::Result;
use crate::fetcher::Meminfo as Fetcher;
use crate::fetcher::Metric as MeminfoMetric;
use crate::metric::{Metric, MetricFamily, MetricType, Metrics, Timestamp};

pub fn family_guage() -> &'static MetricFamily {
    static METRIC_FAMILY: OnceLock<MetricFamily> = OnceLock::new();
    METRIC_FAMILY.get_or_init(|| {
        MetricFamily::new(
            "meminfo",
            "Memory information",
            MetricType::Gauge,
            LabelKeys::list_all(),
        )
    })
}

pub fn family_counter() -> &'static MetricFamily {
    static METRIC_FAMILY: OnceLock<MetricFamily> = OnceLock::new();
    METRIC_FAMILY.get_or_init(|| {
        MetricFamily::new(
            "meminfo",
            "Memory information",
            MetricType::Counter,
            LabelKeys::list_all(),
        )
    })
}

#[allow(dead_code)]
pub enum LabelKeys {
    Label1,
    Label2,
}

impl AsRef<str> for LabelKeys {
    fn as_ref(&self) -> &str {
        self.to_str()
    }
}

impl LabelKeys {
    const LABEL1: &str = "label1";
    const LABEL2: &str = "label2";

    pub fn to_str(&self) -> &'static str {
        match self {
            LabelKeys::Label1 => Self::LABEL1,
            LabelKeys::Label2 => Self::LABEL2,
        }
    }

    pub fn list_all() -> &'static [&'static str; 2] {
        &[Self::LABEL1, Self::LABEL2]
    }
}

impl From<Vec<MeminfoMetric>> for Metrics {
    fn from(meminfo: Vec<MeminfoMetric>) -> Self {
        Metrics::from_vec(
            meminfo
                .into_iter()
                .map(|m| Metric::new(m.name.into(), m.value.into()))
                .collect(),
        )
    }
}

pub struct Meminfo;

impl Meminfo {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn get_meminfo(&self) -> Result<Metrics> {
        let fetcher = Self::fetcher()?;
        let mut metrics: Metrics = fetcher.get_meminfo().await?.into();
        metrics.iter_mut().for_each(|m| {
            Self::tag_metric(m);
            if m.timestamp == Timestamp::None {
                m.timestamp = Timestamp::now();
            }
        });
        Ok(metrics)
    }

    fn tag_metric(metric: &mut Metric) {
        if metric.name.ends_with("_total") {
            family_counter().tag_metric(metric);
        } else {
            family_guage().tag_metric(metric);
        }
    }

    fn fetcher() -> Result<&'static Fetcher> {
        static METRIC_FAMILY: OnceLock<Fetcher> = OnceLock::new();
        let fetcher = METRIC_FAMILY.get_or_init(|| Fetcher::new());
        Ok(fetcher)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_meminfo() {
        let meminfo = Meminfo::new();
        let metrics = meminfo.get_meminfo().await.unwrap();
        println!("{:?}", metrics);
    }
}
