#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![cfg_attr(test, deny(rust_2018_idioms))]

//! nova router

/// nova callable
pub mod callable;
/// nova route
pub mod route;
/// nova router
pub mod router;
