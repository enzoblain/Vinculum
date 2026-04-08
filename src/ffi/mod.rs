//! Foreign Function Interface (FFI) module.
//!
//! This module provides the core building blocks for exchanging data across
//! language boundaries using Vinculum.
//!
//! It exposes the [`types`] module, which contains:
//! - [`types::Value`]: the unified representation of FFI data
//! - [`types::AcceptedTypes`]: marker trait for backend-supported types
//! - [`types::ToValue`]: conversion trait from Rust values to [`types::Value`]
//! - helper wrapper types (e.g. [`types::Null`], [`types::Handle`], [`types::FnPtr`])
//!
//! Most users should start from [`types`] and use the provided `accepted_*`
//! macros to register supported types.

pub mod types;