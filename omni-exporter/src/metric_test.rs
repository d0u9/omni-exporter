#![cfg(test)]

use super::*;

#[test]
fn test_metric() {
    let metric = Metric {
        name: "test_metric".into(),
        value: MetricValue::U64(1),
        timestamp: 1_u64.into(),
        family: None,
        labels: vec![],
    };
    assert_eq!(metric.value, MetricValue::U64(1));
    assert_eq!(metric.timestamp, 1_u64.into());
    assert!(metric.family.is_none());

    let family = MetricFamily::new(
        "test_family",
        "test_help",
        MetricType::Counter,
        vec!["a", "b"],
    );
    let mut metric = metric;
    family.tag_metric(&mut metric);
    assert!(metric.family.is_some());
    assert!(metric.family.unwrap() == family);
}

#[test]
fn test_metric_family() {
    let metric_family = MetricFamily::new(
        "test_family",
        "test_help",
        MetricType::Counter,
        vec!["a", "b"],
    );
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
    assert_eq!(metric.labels[0].key, "test_key".into());
    assert_eq!(metric.labels[0].value, "test_value".into());
}

#[test]
fn test_metric_serialize() {
    let metric = Metric::new("test_metric", MetricValue::U64(1));
    let serialized = serde_json::to_string(&metric).unwrap();
    assert_eq!(
        serialized,
        "{\"family\":null,\"labels\":[],\"name\":\"test_metric\",\"value\":{\"U64\":1},\"timestamp\":\"None\"}"
    );
}

#[test]
fn test_metric_family_serialize() {
    let metric_family = MetricFamily::new(
        "test_family",
        "test_help",
        MetricType::Counter,
        vec!["a", "b"],
    );
    let serialized = serde_json::to_string(&metric_family).unwrap();
    assert_eq!(
        serialized,
        r#"{
            "namespace": null,
            "name": "test_family",
            "help": "test_help",
            "metric_type": "Counter",
            "label_set": ["a", "b"]
        }"#
        .replace(" ", "")
        .replace("\n", "")
    );
}

#[test]
fn test_metric_labels_serialize() {
    let label = Label::new("test_key", "test_value".to_string());
    let serialized = serde_json::to_string(&label).unwrap();
    println!("serialized: {}", serialized);
    assert_eq!(
        serialized,
        r#"{
            "key":"test_key",
            "value":"test_value"
        }"#
        .replace(" ", "")
        .replace("\n", "")
    );
}

#[test]
fn test_metric_serialize_with_timestamp() {
    let mut metric = Metric::new("test_metric", MetricValue::U64(10010));
    metric.set_timestamp(10086_u64.into());
    let serialized = serde_json::to_string(&metric).unwrap();
    assert_eq!(
        serialized,
        r#"{
            "family": null,
            "labels": [],
            "name": "test_metric", 
            "value": {"U64": 10010},
            "timestamp": {"Duration": {"secs": 10086, "nanos": 0}}
        }"#
        .replace(" ", "")
        .replace("\n", "")
    );
}

#[test]
fn test_metric_with_family() {
    let metric_family = MetricFamily::new(
        "test_family",
        "test_help",
        MetricType::Counter,
        vec!["a", "b"],
    );
    let mut metric = Metric::new("test_metric", MetricValue::U64(10010));
    metric.set_timestamp(10086_u64.into());
    metric_family.tag_metric(&mut metric);
    let serialized = serde_json::to_string(&metric).unwrap();
    assert_eq!(
        serialized,
        r#"{
            "family": {
                "namespace": null,
                "name": "test_family", 
                "help": "test_help",
                "metric_type": "Counter",
                "label_set": ["a", "b"]
            },
            "labels": [],
            "name": "test_metric",
            "value": {"U64": 10010},
            "timestamp": {"Duration": {"secs": 10086, "nanos": 0}}
        }"#
        .replace(" ", "")
        .replace("\n", "")
    );
}
