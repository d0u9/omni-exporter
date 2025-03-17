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
