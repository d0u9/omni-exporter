use crate::storage::traits::SlotGetter;
use crate::storage::traits::StorageReader;

pub struct PlainTextSlot {
    metric_name: String,
}

impl SlotGetter for PlainTextSlot {
    fn get_metric_name(&self) -> &str {
        &self.metric_name
    }
}

pub struct PlainText {
    slots: Vec<PlainTextSlot>,
}

impl PlainText {
    pub fn new() -> Self {
        Self { slots: vec![] }
    }
}

impl StorageReader for PlainText {
    type Slot = PlainTextSlot;

    fn into_slots(self) -> impl Iterator<Item = Self::Slot> {
        self.slots.into_iter()
    }
}
