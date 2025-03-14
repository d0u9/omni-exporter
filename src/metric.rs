use std::borrow::Cow;


// This is a simple implementation of the Prometheus OpenMetrics Specification.
// https://github.com/prometheus/OpenMetrics/blob/main/specification/OpenMetrics.md

type Timestamp = u64;

#[derive(Debug, PartialEq)]
enum MetricValue {
    U64(u64),
    F64(f64),
}

struct Metric<'a> {
    family: Option<&'a MetricFamily<'a>>,

    pub value: MetricValue,
    pub timestamp: Option<Timestamp>,
}

impl<'a> Metric<'a> {
    fn new(
        value: MetricValue,
        timestamp: Option<Timestamp>,
        family: Option<&'a MetricFamily<'a>>,
    ) -> Self {
        Self {
            family,
            value,
            timestamp,
        }
    }
}

#[derive(PartialEq, Debug)]
enum MetricType {
    Counter,
    Gauge,
    Histogram,
    Summary,
    Untyped,
}

type LabelKey<'a> = Cow<'a, str>;
type LabelValue<'a> = Cow<'a, str>;

#[derive(PartialEq)]
struct Label<'a> {
    key: LabelKey<'a>,
    value: LabelValue<'a>,
}

#[derive(PartialEq)]
struct LabelSet<'a> {
    labels: Vec<Label<'a>>,
}

impl<'a> LabelSet<'a> {
    fn new(labels: Vec<Label<'a>>) -> Self {
        Self { labels }
    }
}

#[derive(PartialEq)]
struct MetricFamily<'a> {
    pub label_set: LabelSet<'a>,
    pub help: Cow<'a, str>,
    pub metric_type: MetricType,
}

impl<'a> MetricFamily<'a> {
    fn new(label_set: LabelSet<'a>, help: Cow<'a, str>, metric_type: MetricType) -> Self {
        Self {
            label_set,
            help,
            metric_type,
        }
    }

    fn tag_metric<'b>(&'b self, metric: &mut Metric<'b>) {
        metric.family = Some(self);
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
            LabelSet::new(vec![]),
            Cow::Borrowed("test"),
            MetricType::Counter,
        );
        let mut metric = metric;
        family.tag_metric(&mut metric);
        assert!(metric.family.is_some());
        assert!(metric.family.unwrap() == &family);
    }

    #[test]
    fn test_metric_family() {
        let label_set = LabelSet::new(vec![]);
        let metric_family =
            MetricFamily::new(label_set, Cow::Borrowed("test"), MetricType::Counter);
        assert_eq!(metric_family.label_set.labels.len(), 0);
        assert_eq!(metric_family.help, "test");
        assert_eq!(metric_family.metric_type, MetricType::Counter);
    }

    #[test]
    fn test_label_set() {
        let key1 = "key1";
        let key2 = "key2";
        let keys = [Cow::Borrowed(key1), Cow::Borrowed(key2)];

        let values = vec![
            Label {
                key: keys[0].clone(),
                value: Cow::Owned("value1".to_string()),
            },
            Label {
                key: keys[1].clone(),
                value: Cow::Borrowed("value2"),
            },
        ];

        let label_set = LabelSet::new(values);
        assert_eq!(label_set.labels.len(), 2);
        assert_eq!(label_set.labels[0].key, "key1");
        assert_eq!(label_set.labels[0].value, "value1");
        assert_eq!(label_set.labels[1].key, "key2");
        assert_eq!(label_set.labels[1].value, "value2");
    }
}
