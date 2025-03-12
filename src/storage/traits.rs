pub trait SlotGetter {
    fn get_metric_name(&self) -> &str;
}

pub trait StorageReader {
    type Slot: SlotGetter;

    fn into_slots(self) -> impl Iterator<Item = Self::Slot>;
}

pub trait ExtStorageReader {
    fn slots<'a>(&'a self) -> Box<dyn Iterator<Item = Box<dyn ExtSlotGetter + 'a>>>;
}

pub trait ExtSlotGetter {
    fn get_metric_name(&self) -> &str;
}
