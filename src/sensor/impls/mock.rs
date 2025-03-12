use crate::error::Result;
use crate::sensor::SensorReader;
use crate::storage::InternalSlot;
use crate::storage::InternalStorage;
use crate::storage::PlainText;
use crate::storage::PlainTextSlot;

pub struct MockReader {}

impl MockReader {
    pub fn new() -> Self {
        Self {}
    }
}

impl SensorReader for MockReader {
    type Slot = InternalSlot;
    type Storage = InternalStorage;

    fn read(&self) -> Result<Self::Storage> {
        let storage = PlainText::new();
        Ok(InternalStorage::PlainText(storage))
    }
}
