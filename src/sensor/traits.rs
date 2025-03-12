use std::fmt::Debug;

use async_trait::async_trait;

use crate::error::Result;
use crate::protocol::ItemGetter as ProtoItem;
use crate::protocol::Value as ProtoValue;

pub trait Sensor: Sync + Send {
    fn name(&self) -> &str;
}

#[async_trait]
pub trait Reader: Sync + Send + 'static {
    type Data: Data;

    fn name(&self) -> &str;
    fn id(&self) -> &str;

    async fn read(&self) -> Result<Vec<Self::Data>>;
}

pub trait Data: Sync + Send + Debug + Sized + 'static {
    fn metric_name(&self) -> &str;

    fn value(&self) -> ProtoValue;

    fn labels(&self) -> impl Iterator<Item = (&str, &str)>;

    fn from_proto<P>(proto: P) -> Result<Self>
    where
        P: ProtoItem + Send + Sync + 'static;
}
