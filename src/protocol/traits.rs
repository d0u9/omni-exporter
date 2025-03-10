use std::convert::From;
use std::convert::Into;

// https://prometheus.io/docs/instrumenting/exposition_formats/#comments-help-text-and-type-information

pub trait ProtocolGetter {
    type Cell;

    fn metric_name(&self) -> &str;

    fn label<T: From<Self::Cell>>(&self, key: &str) -> Option<T>;
    fn value<T: From<Self::Cell>>(&self) -> Option<T>;
}

pub trait ProtocolSetter {
    type Cell;

    fn metric_name(&mut self, name: &str);
    fn label<T: Into<Self::Cell>>(&mut self, key: &str, value: T);
    fn value<T: Into<Self::Cell>>(&mut self, value: T);
}
