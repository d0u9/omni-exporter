use super::*;

#[test]
fn test_metric() {
    let metric = Metric {
        name: "test_metric",
        value: MetricValue::U64(1),
        timestamp: Timestamp::U64(1),
        family: None,
        labels: vec![],
    };
    assert_eq!(metric.value, MetricValue::U64(1));
    assert_eq!(metric.timestamp, Timestamp::U64(1));
    assert!(metric.family.is_none());

    let family = MetricFamily::new("test_family", "test_help", MetricType::Counter, &["a", "b"]);
    let mut metric = metric;
    family.tag_metric(&mut metric);
    assert!(metric.family.is_some());
    assert!(metric.family.unwrap() == family);
}

#[test]
fn test_metric_family() {
    let metric_family =
        MetricFamily::new("test_family", "test_help", MetricType::Counter, &["a", "b"]);
    assert_eq!(metric_family.label_set.len(), 2);
    assert_eq!(metric_family.help, "test_help");
    assert_eq!(metric_family.metric_type, MetricType::Counter);
}

#[test]
fn test_metric_labels() {
    let mut metric = Metric::new("test_metric", MetricValue::U64(1));
    assert_eq!(metric.labels.len(), 0);

    metric.add_label("test_key", "test_value".to_string());
    assert_eq!(metric.labels.len(), 1);
    assert_eq!(metric.labels[0].key, "test_key");
    assert_eq!(metric.labels[0].value, "test_value");
}
