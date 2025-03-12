mod traits;
pub use traits::SlotGetter;
pub use traits::StorageReader;

mod impls;
pub use impls::PlainText;
pub use impls::PlainTextSlot;

mod types;
pub use types::InternalSlot;
pub use types::InternalStorage;
