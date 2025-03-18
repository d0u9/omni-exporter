use crate::metric::{Label, MetricValue};

pub trait FetcherMetricName:
    AsRef<str> + Into<&'static str> + std::fmt::Debug + PartialEq + Clone
{
    fn to_str(&self) -> &'static str;
}

#[derive(Debug, PartialEq, Clone)]
pub struct Metric<N>
where
    N: FetcherMetricName,
{
    pub name: N,
    pub value: MetricValue,
    pub labels: Option<Vec<Label>>,
}

impl<N> Metric<N>
where
    N: FetcherMetricName,
{
    pub fn new<V: Into<MetricValue>>(name: N, value: V) -> Self {
        Self {
            name,
            value: value.into(),
            labels: None,
        }
    }

    pub fn new_with_labels<V: Into<MetricValue>>(name: N, value: V, labels: Vec<Label>) -> Self {
        Self {
            name,
            value: value.into(),
            labels: Some(labels),
        }
    }
}
