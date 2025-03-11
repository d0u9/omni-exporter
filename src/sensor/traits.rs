use std::fmt::Debug;

use async_trait::async_trait;

use crate::error::Result;
use crate::protocol::ProtoValue;
use crate::protocol::ProtocolGetter;

pub trait Sensor: Sync + Send {
    fn name(&self) -> &str;
}

#[async_trait]
pub trait SensorReader: Sync + Send + 'static {
    type Data: SensorData;

    fn name(&self) -> &str;
    fn id(&self) -> &str;

    async fn read(&self) -> Result<Vec<Self::Data>>;
}

pub trait SensorData: Sync + Send + Debug + Sized + 'static {
    fn metric_name(&self) -> &str;

    fn value(&self) -> ProtoValue;

    fn labels(&self) -> impl Iterator<Item = (&str, &str)>;

    fn from_proto<P>(proto: P) -> Result<Self>
    where
        P: ProtocolGetter + Send + Sync + 'static;
}
