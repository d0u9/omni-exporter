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

impl SensorReader for PlainTextAdapter {
    type Slot = InternalSlot;
    type Storage = InternalStorage;

    fn read(&self) -> Result<Self::Storage> {
        let plain = self.inner.read()?;
        Ok(InternalStorage::PlainText(plain))
    }
}
