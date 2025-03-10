use crate::error::Result;

// https://prometheus.io/docs/instrumenting/exposition_formats/#comments-help-text-and-type-information

#[derive(Debug, Clone, Copy)]
pub enum ProtocolValue {
    None,
    F64(f64),
    U64(u64),
}

pub trait ProtocolGetter {
    fn get_metric_name(&self) -> &str;
    fn get_labels(&self) -> impl Iterator<Item = (&str, &str)>;
    fn get_value(&self) -> Result<ProtocolValue>;
}

pub trait ProtocolSetter {
    fn set_metric_name(&mut self, name: &str);
    fn set_labels(&mut self, key: &str, value: &str);
    fn set_value(&mut self, value: ProtocolValue);
}
