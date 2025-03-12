use async_trait::async_trait;

use crate::error::Result;
use crate::sensor::SensorReader;
use crate::storage::PlainText;
use crate::storage::PlainTextSlot;

pub struct MockReader {}

impl MockReader {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl SensorReader for MockReader {
    type Slot = PlainTextSlot;
    type Storage = PlainText;

    async fn read(&self) -> Result<Self::Storage> {
        let storage = PlainText::new();
        Ok(storage)
    }
}
