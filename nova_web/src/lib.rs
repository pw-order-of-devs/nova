#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![cfg_attr(test, deny(rust_2018_idioms))]

//! nova_web - web framework

/// nova server implementation
pub mod server;

/// nova core
pub mod core;

/// nova routing
pub mod routing;

/// nova hello_serde integration
#[cfg(feature = "serde")]
pub mod serde;
