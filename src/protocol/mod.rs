mod traits;
pub use traits::ProtocolGetter;
pub use traits::ProtocolSetter;

pub mod plain;

mod types;
pub use types::Value as ProtoValue;
