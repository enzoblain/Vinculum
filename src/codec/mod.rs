pub(crate) mod accepted;
mod deserialize;
mod serialize;
mod value;

pub use accepted::{AcceptedTypes, ToValue};
pub use value::{Array, FnPtr, Handle, Null, Tuple, Value};

pub(super) const BUFFER_SIZE: usize = 1028 * 1028;
