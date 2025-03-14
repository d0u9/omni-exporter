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
