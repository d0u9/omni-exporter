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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metric() {
        let metric = Metric {
            value: MetricValue::U64(1),
            timestamp: Some(1),
            family: None,
        };
        assert_eq!(metric.value, MetricValue::U64(1));
        assert_eq!(metric.timestamp, Some(1));
        assert!(metric.family.is_none());

        let family = MetricFamily::new(
            "test_family".to_string(),
            "test_help".to_string(),
            MetricType::Counter,
            HashSet::from_iter(vec![]),
        );
        let mut metric = metric;
        family.tag_metric(&mut metric);
        assert!(metric.family.is_some());
        assert!(metric.family.unwrap() == family.inner);
    }

    #[test]
    fn test_metric_family() {
        let metric_family = MetricFamily::new(
            "test_family".to_string(),
            "test_help".to_string(),
            MetricType::Counter,
            HashSet::from_iter(vec![]),
        );
        assert_eq!(metric_family.inner.label_set.len(), 0);
        assert_eq!(metric_family.inner.help, "test_help");
        assert_eq!(metric_family.inner.metric_type, MetricType::Counter);
    }

    #[test]
    fn test_label_set() {
        let key1 = "key1";
        let key2 = "key2";
        let keys = [Cow::Borrowed(key1), Cow::Borrowed(key2)];

        let label_set = LabelSet::from_vec(vec![keys[0].clone(), keys[1].clone()]);
        assert_eq!(label_set.inner.len(), 2);
        assert_eq!(label_set.inner.contains(&keys[0]), true);
        assert_eq!(label_set.inner.contains(&keys[1]), true);
    }
}
