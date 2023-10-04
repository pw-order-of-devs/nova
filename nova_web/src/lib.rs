#![deny(clippy::all)]
#![deny(clippy::cargo)]
#![deny(clippy::complexity)]
#![deny(clippy::correctness)]
#![deny(clippy::nursery)]
#![deny(clippy::pedantic)]
#![deny(clippy::perf)]
#![deny(clippy::style)]
#![deny(clippy::suspicious)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, deny(rust_2018_idioms))]

//! `nova_web` - web framework

/// nova server implementation
pub mod server;

/// nova core
pub mod core;

/// nova routing
pub mod routing;

/// nova serde integration
#[cfg(feature = "serde")]
pub mod serde;
