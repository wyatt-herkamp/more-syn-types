pub mod doc;
pub mod include;
#[doc(inline)]
pub use doc::{DocAttribute, HasDocAttributes};
#[doc(inline)]
pub use include::{include_bytes, include_str, IOOrParseError, IncludeMacro};
