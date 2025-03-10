use async_trait::async_trait;
use std::fmt::Debug;

use crate::protocol::ProtocolGetter;

pub trait Sensor: Sync + Send {
    fn name(&self) -> &str;
}

#[async_trait]
pub trait SensorReader: Sync + Send + 'static {
    type Data: SensorData;

    fn name(&self) -> &str;
    fn id(&self) -> &str;

    async fn read(&self) -> Self::Data;
}

pub trait SensorData: Sync + Send + Debug + 'static {
    fn name(&self) -> &str;
    fn from_proto<P>(proto: P) -> Self
    where
        P: ProtocolGetter + Send + Sync + 'static;
}
