use crate::storage::PlainText;
use crate::storage::PlainTextSlot;
use crate::storage::SlotGetter;
use crate::storage::StorageReader;

pub enum InternalStorage {
    PlainText(PlainText),
}

impl StorageReader for InternalStorage {
    type Slot = InternalSlot;

    fn into_slots(self) -> impl Iterator<Item = Self::Slot> {
        match self {
            InternalStorage::PlainText(storage) => {
                storage.into_slots().map(InternalSlot::PlainText)
            }
        }
    }
}

pub enum InternalSlot {
    PlainText(PlainTextSlot),
}

impl SlotGetter for InternalSlot {
    fn get_metric_name(&self) -> &str {
        match self {
            InternalSlot::PlainText(slot) => slot.get_metric_name(),
        }
    }
}
