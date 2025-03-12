use async_trait::async_trait;

use crate::error::Result;
use crate::storage::ExtStorageReader;
use crate::storage::SlotGetter;
use crate::storage::StorageReader;

#[async_trait]
pub trait SensorReader: Send + Sync {
    type Slot: SlotGetter;
    type Storage: StorageReader<Slot = Self::Slot>;

    async fn read(&self) -> Result<Self::Storage>;
}

#[async_trait]
pub trait ExtSensorReader {
    async fn read(&self) -> Result<Box<dyn ExtStorageReader>>;
}
