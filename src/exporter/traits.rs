use crate::sensor::SensorReader;
use crate::storage::SlotGetter;
use crate::storage::StorageReader;

pub trait Collector<L, S>
where
    L: SlotGetter,
    S: StorageReader<Slot = L>,
{
    fn add_sensor(&mut self, sensor: Box<dyn SensorReader<Slot = L, Storage = S>>);
}
