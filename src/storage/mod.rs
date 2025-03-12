mod traits;
pub use traits::ExtSlotGetter;
pub use traits::ExtStorageReader;
pub use traits::SlotGetter;
pub use traits::StorageReader;

mod impls;
pub use impls::PlainText;
pub use impls::PlainTextSlot;

mod types;
pub(crate) use types::InternalSlot;
pub(crate) use types::InternalStorage;
