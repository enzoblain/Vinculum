//! Vinculum core library.
//!
//! This crate provides the core abstractions for building FFI bridges using
//! a unified and strongly-typed value system.
//!
//! The [`ffi`] module contains:
//! - the [`ffi::types::Value`] type used to represent data across boundaries
//! - traits such as [`ffi::types::AcceptedTypes`] and [`ffi::types::ToValue`]
//! - macros to register supported types
//!
//! Most users will primarily interact with the [`ffi`] module.

pub mod ffi;