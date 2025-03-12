use async_trait::async_trait;

use crate::error::Result;
use crate::sensor::SensorReader;
use crate::storage::InternalSlot;
use crate::storage::InternalStorage;
use crate::storage::PlainText;
use crate::storage::PlainTextSlot;

pub struct PlainTextAdapter {
    inner: Box<dyn SensorReader<Slot = PlainTextSlot, Storage = PlainText>>,
}

impl PlainTextAdapter {
    pub fn new(inner: Box<dyn SensorReader<Slot = PlainTextSlot, Storage = PlainText>>) -> Self {
        Self { inner }
    }
}

#[async_trait]
impl SensorReader for PlainTextAdapter {
    type Slot = InternalSlot;
    type Storage = InternalStorage;

    async fn read(&self) -> Result<Self::Storage> {
        let plain = self.inner.read().await?;
        Ok(InternalStorage::PlainText(plain))
    }
}
