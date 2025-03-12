pub trait SlotGetter {
    fn get_metric_name(&self) -> &str;
}

pub trait StorageReader {
    type Slot: SlotGetter;

    fn into_slots(self) -> impl Iterator<Item = Self::Slot>;
}
