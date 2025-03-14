use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum MetricValue {
    U64(u64),
    None,
}

impl From<Option<u64>> for MetricValue {
    fn from(value: Option<u64>) -> Self {
        match value {
            Some(value) => MetricValue::U64(value),
            None => MetricValue::None,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum MetricType {
    Counter,
    Gauge,
    Histogram,
    Summary,
    Untyped,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Metric<L>
where
    L: MetricLabels,
{
    pub mtype: MetricType,
    pub value: MetricValue,
    pub labels: L,
}

impl<L> Metric<L>
where
    L: MetricLabels + Default,
{
    pub fn new_with_no_labels(mtype: MetricType, value: MetricValue) -> Self {
        Metric {
            mtype,
            value,
            labels: L::default(),
        }
    }
}

pub trait MetricLabels {
    fn labels_ref(&self) -> impl Iterator<Item = (&str, &str)>;

    fn labels(self) -> impl Iterator<Item = (String, String)>;
}

impl MetricLabels for Vec<(String, String)> {
    fn labels_ref<'a>(&'a self) -> impl Iterator<Item = (&'a str, &'a str)> {
        self.iter().map(|(k, v)| (k.as_str(), v.as_str()))
    }

    fn labels(self) -> impl Iterator<Item = (String, String)> {
        self.into_iter().map(|(k, v)| (k, v))
    }
}

impl MetricLabels for HashMap<String, String> {
    fn labels_ref(&self) -> impl Iterator<Item = (&str, &str)> {
        self.iter().map(|(k, v)| (k.as_str(), v.as_str()))
    }

    fn labels(self) -> impl Iterator<Item = (String, String)> {
        self.into_iter().map(|(k, v)| (k, v))
    }
}
