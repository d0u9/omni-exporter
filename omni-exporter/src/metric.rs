use std::borrow::Cow;
use std::time::Duration;

use crate::error::Error;

use serde::{Deserialize, Serialize};

// This is a simple implementation of the Prometheus OpenMetrics Specification.
// https://github.com/prometheus/OpenMetrics/blob/main/specification/OpenMetrics.md

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
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

    pub fn is_none(&self) -> bool {
        *self == Self::None
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

impl std::fmt::Debug for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(s) = self.into_u64() {
            write!(f, "{:?}", s)
        } else {
            write!(f, "null")
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MetricValue {
    U64(u64),
    F64(f64),
    String(String),
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

impl From<u64> for MetricValue {
    fn from(value: u64) -> Self {
        MetricValue::U64(value)
    }
}

impl From<Option<f64>> for MetricValue {
    fn from(value: Option<f64>) -> Self {
        match value {
            Some(v) => MetricValue::F64(v),
            None => MetricValue::None,
        }
    }
}

impl From<f64> for MetricValue {
    fn from(value: f64) -> Self {
        MetricValue::F64(value)
    }
}

impl From<usize> for MetricValue {
    fn from(value: usize) -> Self {
        MetricValue::U64(value as u64)
    }
}

impl From<bool> for MetricValue {
    fn from(value: bool) -> Self {
        MetricValue::U64(value as u64)
    }
}

impl From<String> for MetricValue {
    fn from(value: String) -> Self {
        MetricValue::String(value)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Metric {
    family: Option<MetricFamily>,
    labels: Vec<Label>,

    pub name: Cow<'static, str>,
    pub value: MetricValue,
    pub timestamp: Timestamp,
}

impl Metric {
    pub fn new<N>(name: N, value: MetricValue) -> Self
    where
        N: Into<Cow<'static, str>>,
    {
        Self {
            name: name.into(),
            value,
            timestamp: Timestamp::None,
            family: None,
            labels: vec![],
        }
    }

    pub fn new_with_labels<N, L>(name: N, value: MetricValue, labels: L) -> Self
    where
        N: Into<Cow<'static, str>>,
        L: Into<Vec<Label>>,
    {
        Self {
            name: name.into(),
            value,
            timestamp: Timestamp::None,
            family: None,
            labels: labels.into(),
        }
    }

    pub fn set_timestamp(&mut self, timestamp: Timestamp) {
        self.timestamp = timestamp;
    }

    pub fn add_label<K, V>(&mut self, key: K, value: V)
    where
        K: Into<LabelKey>,
        V: Into<LabelValue>,
    {
        self.labels.push(Label {
            key: key.into(),
            value: value.into(),
        });
    }
}

impl std::fmt::Debug for Metric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[ ")?;

        write!(
            f,
            "T({:?}), {:?}, {:?}, ",
            self.timestamp, self.name, self.value,
        )?;

        if !self.labels.is_empty() {
            write!(f, "{:?}, ", self.labels)?;
        }

        if let Some(ref family) = self.family {
            write!(f, "{:?}, ", family)?;
        }

        write!(f, "]")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum MetricType {
    Counter,
    Gauge,
    Histogram,
    Summary,
    Untyped,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LabelKey {
    inner: Cow<'static, str>,
}

impl From<String> for LabelKey {
    fn from(value: String) -> Self {
        Self {
            inner: Cow::Owned(value),
        }
    }
}

impl From<&'static str> for LabelKey {
    fn from(value: &'static str) -> Self {
        Self {
            inner: Cow::Borrowed(value),
        }
    }
}

impl std::fmt::Debug for LabelKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

impl From<LabelKey> for String {
    fn from(value: LabelKey) -> Self {
        value.inner.into()
    }
}

impl AsRef<str> for LabelKey {
    fn as_ref(&self) -> &str {
        self.inner.as_ref()
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LabelValue {
    inner: Cow<'static, str>,
}

impl From<String> for LabelValue {
    fn from(value: String) -> Self {
        Self {
            inner: Cow::Owned(value),
        }
    }
}

impl From<&'static str> for LabelValue {
    fn from(value: &'static str) -> Self {
        Self {
            inner: Cow::Borrowed(value),
        }
    }
}

impl From<LabelValue> for String {
    fn from(value: LabelValue) -> Self {
        value.inner.into()
    }
}

impl std::fmt::Debug for LabelValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

impl AsRef<str> for LabelValue {
    fn as_ref(&self) -> &str {
        self.inner.as_ref()
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
// I want this can be used in no_std environment
#[serde(transparent)]
pub struct LabelKeySet {
    inner: Vec<LabelKey>,
}

impl LabelKeySet {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn push(mut self, key: LabelKey) -> Result<Self, Error> {
        if self.inner.contains(&key) {
            return Err(Error::LabelKeyAlreadyExists(key.into()));
        }

        self.inner.push(key);

        Ok(self)
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.len() == 0
    }

    pub fn contains(&self, key: &str) -> bool {
        self.inner.iter().any(|k| k.as_ref() == key)
    }
}

impl Default for LabelKeySet {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> FromIterator<T> for LabelKeySet
where
    T: Into<LabelKey>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            inner: iter.into_iter().map(|s| s.into()).collect(),
        }
    }
}

impl<'a> From<&'a [&'static str]> for LabelKeySet {
    fn from(value: &'a [&'static str]) -> Self {
        Self {
            inner: value.iter().map(|s| (*s).into()).collect(),
        }
    }
}

impl From<Vec<&'static str>> for LabelKeySet {
    fn from(value: Vec<&'static str>) -> Self {
        Self {
            inner: value.into_iter().map(|s| s.into()).collect(),
        }
    }
}

impl std::fmt::Debug for LabelKeySet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ ")?;
        self.inner
            .iter()
            .enumerate()
            .try_for_each(|(i, label_key)| {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:?}", label_key)
            })?;
        write!(f, "}}")
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Label {
    pub key: LabelKey,
    pub value: LabelValue,
}

impl Label {
    pub fn new<K, V>(key: K, value: V) -> Self
    where
        K: Into<LabelKey>,
        V: Into<LabelValue>,
    {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }
}

impl std::fmt::Debug for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?} : {:?})", self.key, self.value)
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct MetricFamily {
    namespace: Option<Cow<'static, str>>,
    name: Cow<'static, str>,
    help: Cow<'static, str>,
    metric_type: MetricType,
    label_set: Cow<'static, LabelKeySet>,
}

impl MetricFamily {
    pub fn new_with_namespace<S, N, L>(
        namespace: S,
        name: N,
        help: &'static str,
        metric_type: MetricType,
        label_set: L,
    ) -> Self
    where
        S: Into<String>,
        N: Into<String>,
        L: Into<LabelKeySet>,
    {
        Self {
            namespace: Some(Cow::Owned(namespace.into())),
            name: Cow::Owned(name.into()),
            help: Cow::Borrowed(help),
            metric_type,
            label_set: Cow::Owned(label_set.into()),
        }
    }

    pub fn new<N, H, L>(name: N, help: H, metric_type: MetricType, label_set: L) -> Self
    where
        N: Into<Cow<'static, str>>,
        H: Into<Cow<'static, str>>,
        L: Into<LabelKeySet>,
    {
        Self {
            namespace: None,
            name: name.into(),
            help: help.into(),
            metric_type,
            label_set: Cow::Owned(label_set.into()),
        }
    }

    pub fn dup_with_help<H>(&self, help: H) -> Self
    where
        H: Into<Cow<'static, str>>,
    {
        Self {
            namespace: self.namespace.clone(),
            name: self.name.clone(),
            help: help.into(),
            metric_type: self.metric_type,
            label_set: self.label_set.clone(),
        }
    }

    pub fn tag_metric(&self, metric: &mut Metric) {
        metric.family = Some(self.clone());

        // Remove all unknown labels from the metric
        metric
            .labels
            .retain(|label| self.label_set.contains(label.key.as_ref()));
    }
}

impl std::fmt::Debug for MetricFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MetricFamily{{ ns: {:?}, name: {:?}, help: {:?}, type: {:?}: label_set: {:?} }}",
            self.namespace, self.name, self.help, self.metric_type, self.label_set,
        )
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MetricBatch {
    pub metrics: Vec<Metric>,
}

impl MetricBatch {
    pub fn iter_metrics(&self) -> impl Iterator<Item = &Metric> {
        self.metrics.iter()
    }

    pub fn families(&self) -> impl Iterator<Item = &MetricFamily> {
        let mut families = self
            .metrics
            .iter()
            .filter_map(|m| m.family.as_ref())
            .collect::<Vec<_>>();

        families.sort_by_key(|f| f.name.as_ref());
        families.dedup_by_key(|f| f.name.as_ref());

        families.into_iter()
    }

    pub fn metrics_by_family(&self, family: &MetricFamily) -> impl Iterator<Item = &Metric> {
        self.metrics
            .iter()
            .filter(|m| m.family.as_ref() == Some(family))
    }

    pub fn metrics_orphan(&self) -> impl Iterator<Item = &Metric> {
        self.metrics.iter().filter(|m| m.family.is_none())
    }
}

impl std::fmt::Debug for MetricBatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "MetricBatch: {{")?;

        for family in self.families() {
            writeln!(f, "Family: {:?}", family)?;

            for metric in self.metrics_by_family(family) {
                write!(f, "\t[ ")?;

                write!(
                    f,
                    "T({:?}), {:?}, {:?}, ",
                    metric.timestamp, metric.name, metric.value,
                )?;

                if !metric.labels.is_empty() {
                    write!(f, "{:?}, ", metric.labels)?;
                }

                writeln!(f, "]")?;
            }
        }

        writeln!(f, "}}")
    }
}

impl From<Metrics> for MetricBatch {
    fn from(metrics: Metrics) -> Self {
        Self { metrics: metrics.0 }
    }
}

#[derive(Clone, Serialize, Deserialize)]
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

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
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
            writeln!(f, "{:?}", metric)?;
        }
        writeln!(f, "}}")
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
