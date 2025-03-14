use std::borrow::Cow;
use std::collections::HashSet;
use std::sync::Arc;

// This is a simple implementation of the Prometheus OpenMetrics Specification.
// https://github.com/prometheus/OpenMetrics/blob/main/specification/OpenMetrics.md

type Timestamp = u64;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MetricValue {
    U64(u64),
    F64(f64),
    None,
}

impl From<Option<u64>> for MetricValue {
    fn from(value: Option<u64>) -> Self {
        match value {
            Some(v) => MetricValue::U64(v),
            None => MetricValue::None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Metric {
    family: Option<Arc<MetricFamilyInner>>,

    pub value: MetricValue,
    pub timestamp: Option<Timestamp>,
}

impl Metric {
    pub fn new(value: MetricValue) -> Self {
        Self {
            value,
            timestamp: None,
            family: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MetricType {
    Counter,
    Gauge,
    Histogram,
    Summary,
    Untyped,
}

type LabelKey<'a> = Cow<'a, str>;
type LabelValue = String;

#[derive(Debug, Clone, PartialEq)]
pub struct Label<'a> {
    key: LabelKey<'a>,
    value: LabelValue,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LabelSet<'a> {
    inner: HashSet<LabelKey<'a>>,
}

impl<'a> LabelSet<'a> {
    pub fn from_vec(label_keys: Vec<LabelKey<'a>>) -> Self {
        Self {
            inner: label_keys.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct MetricFamilyInner {
    name: String,
    help: String,
    metric_type: MetricType,
    label_set: HashSet<String>,
}

impl MetricFamilyInner {
    pub fn new(
        name: String,
        help: String,
        metric_type: MetricType,
        label_set: HashSet<String>,
    ) -> Self {
        Self {
            name,
            help,
            metric_type,
            label_set,
        }
    }
}

pub struct MetricFamily {
    inner: Arc<MetricFamilyInner>,
}

impl MetricFamily {
    pub fn new(
        name: String,
        help: String,
        metric_type: MetricType,
        label_set: HashSet<String>,
    ) -> Self {
        Self {
            inner: Arc::new(MetricFamilyInner::new(name, help, metric_type, label_set)),
        }
    }

    pub fn tag_metric(&self, metric: &mut Metric) {
        log::info!("tag_metric: {:?}", self.inner);
        metric.family = Some(self.inner.clone());
    }
}

#[derive(Clone)]
pub struct Metrics(Vec<Metric>);

impl Metrics {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn from_vec(metrics: Vec<Metric>) -> Self {
        Self(metrics)
    }

    pub fn extend(&mut self, metrics: Metrics) {
        self.0.extend(metrics.0);
    }

    pub fn extend_with_vec(&mut self, metrics: Vec<Metric>) {
        self.0.extend(metrics);
    }
}

impl AsRef<Vec<Metric>> for Metrics {
    fn as_ref(&self) -> &Vec<Metric> {
        &self.0
    }
}

impl AsMut<Vec<Metric>> for Metrics {
    fn as_mut(&mut self) -> &mut Vec<Metric> {
        &mut self.0
    }
}

impl std::fmt::Debug for Metrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Metrics")
    }
}

impl IntoIterator for Metrics {
    type Item = Metric;
    type IntoIter = std::vec::IntoIter<Metric>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

////////////////////////////////////////////////////////////
/// Unit Test
////////////////////////////////////////////////////////////

#[cfg(test)]
#[path = "metric_test.rs"]
mod tests;
