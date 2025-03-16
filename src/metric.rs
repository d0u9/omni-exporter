use std::borrow::Cow;
use std::time::Duration;

// This is a simple implementation of the Prometheus OpenMetrics Specification.
// https://github.com/prometheus/OpenMetrics/blob/main/specification/OpenMetrics.md

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Timestamp {
    U64(u64),
    Duration(Duration),
    None,
}

impl Timestamp {
    pub fn now() -> Self {
        let now = std::time::SystemTime::now();
        if let Ok(t) = now.duration_since(std::time::UNIX_EPOCH) {
            Self::Duration(t)
        } else {
            Self::None
        }
    }

    pub fn into_u64(self) -> Option<u64> {
        match self {
            Self::U64(u64) => Some(u64),
            Self::Duration(d) => Some(d.as_secs()),
            Self::None => None,
        }
    }
}

impl From<Duration> for Timestamp {
    fn from(duration: Duration) -> Self {
        Self::Duration(duration)
    }
}

impl From<u64> for Timestamp {
    fn from(u64: u64) -> Self {
        Self::U64(u64)
    }
}

impl From<Timestamp> for Option<u64> {
    fn from(timestamp: Timestamp) -> Self {
        timestamp.into_u64()
    }
}

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
    family: Option<MetricFamily>,
    labels: Vec<Label>,

    pub name: &'static str,
    pub value: MetricValue,
    pub timestamp: Timestamp,
}

impl Metric {
    pub fn new(name: &'static str, value: MetricValue) -> Self {
        Self {
            name,
            value,
            timestamp: Timestamp::None,
            family: None,
            labels: vec![],
        }
    }

    pub fn set_timestamp(&mut self, timestamp: Timestamp) {
        self.timestamp = timestamp;
    }

    pub fn add_label(&mut self, key: &'static str, value: String) {
        self.labels.push(Label { key, value });
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MetricType {
    Counter,
    Gauge,
    Histogram,
    Summary,
    Untyped,
}

type LabelKey = &'static str;
type LabelValue = String;

#[derive(Debug, Clone, PartialEq)]
pub struct Label {
    key: LabelKey,
    value: LabelValue,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MetricFamily {
    namespace: Option<&'static str>,
    name: &'static str,
    help: Cow<'static, str>,
    metric_type: MetricType,
    label_set: &'static [&'static str],
}

impl MetricFamily {
    pub fn new_with_namespace(
        namespace: &'static str,
        name: &'static str,
        help: &'static str,
        metric_type: MetricType,
        label_set: &'static [&'static str],
    ) -> Self {
        Self {
            namespace: Some(namespace),
            name,
            help: Cow::Borrowed(help),
            metric_type,
            label_set,
        }
    }

    pub fn new(
        name: &'static str,
        help: &'static str,
        metric_type: MetricType,
        label_set: &'static [&'static str],
    ) -> Self {
        Self {
            namespace: None,
            name,
            help: Cow::Borrowed(help),
            metric_type,
            label_set,
        }
    }

    pub fn dup_with_help<T: ToString>(&self, help: T) -> Self {
        Self {
            namespace: self.namespace,
            name: self.name,
            help: Cow::Owned(help.to_string()),
            metric_type: self.metric_type,
            label_set: self.label_set,
        }
    }

    pub fn tag_metric(&self, metric: &mut Metric) {
        log::info!("tag_metric: {:?}", self);
        metric.family = Some(self.clone());
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

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Metric> {
        self.0.iter_mut()
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
        writeln!(f, "Metrics: {{")?;
        for metric in &self.0 {
            write!(f, "    {:?}, {:?}", metric.name, metric.value)?;
            if let Some(ts) = metric.timestamp.into_u64() {
                write!(f, ", {:?}", ts)?;
            } else {
                write!(f, ", NoTimestamp")?;
            }
            if !metric.labels.is_empty() {
                write!(f, ", Labels: [")?;
                for label in &metric.labels {
                    write!(f, "{:?}: {:?}, ", label.key, label.value)?;
                }
                write!(f, "]")?;
            }
            if let Some(family) = &metric.family {
                write!(
                    f,
                    ", Family: [{:?} {:?}, {:?}, {:?}]",
                    family.namespace, family.name, family.help, family.metric_type
                )?;
            }
            writeln!(f)?;
        }
        writeln!(f, "}}")?;
        Ok(())
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
