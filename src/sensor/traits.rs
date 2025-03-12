use crate::error::Result;
use crate::storage::ExtStorageReader;
use crate::storage::SlotGetter;
use crate::storage::StorageReader;

pub trait SensorReader {
    type Slot: SlotGetter;
    type Storage: StorageReader<Slot = Self::Slot>;

    fn read(&self) -> Result<Self::Storage>;
}

pub trait ExtSensorReader {
    fn read(&self) -> Result<Box<dyn ExtStorageReader>>;
}
