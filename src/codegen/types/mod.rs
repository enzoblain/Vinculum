//! Core type system and abstractions used by the code generation layer.
//!
//! This module provides:
//!
//! - [`Function`]: representation of a function signature and its associated metadata
//! - [`Arg`]: representation of a function argument
//! - [`FfiType`] and [`FfiTypeCodegen`]: traits defining type semantics and
//!   code generation behavior across the FFI boundary
//! - [`GenericResolver`]: utility for resolving and stabilizing generic parameters
//! - [`Derive`]: enumeration of supported Rust derive attributes
//! - [`InvalidArgumentName`]: error type for invalid argument identifiers
//!
//! These components form the internal model used to describe functions and
//! types before generating Rust and target-language code.

mod functions;
mod arg;
mod traits;
mod generics;
mod errors;
mod derive;

pub use functions::Function;
pub use arg::Arg;
pub use traits::{FfiType, FfiTypeCodegen};
pub use generics::GenericResolver;
pub use errors::InvalidArgumentName;
pub use derive::Derive;