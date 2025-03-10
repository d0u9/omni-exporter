use std::marker::PhantomData;

use super::SensorData;

use crate::protocol::Protocol;
use crate::protocol::plain;

#[derive(Debug)]
pub struct OwnedSensorData {
    data: String,
}

impl OwnedSensorData {
    pub fn new() -> Self {
        Self {
            data: "xxx".to_string(),
        }
    }
}

impl SensorData for OwnedSensorData {
    fn name(&self) -> &str {
        "OwnedSensorData"
    }

    fn from_proto<P>(proto: P) -> Self
    where
        P: Protocol + Send + Sync + 'static,
    {
        Self {
            data: proto.get_item().to_string(),
        }
    }
}
