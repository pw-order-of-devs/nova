#![forbid(unsafe_code)]
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

//! `nova_web` - implementation of web framework
//!
//! ```file_tree
//! `nova_web`
//! ├── core.rs
//! ├── routing.rs
//! ├── request
//! └── server.rs
//! ```
//!
//! `core` - re-exports of `nova_core` items
//! `routing` - integration of `nova_router` into server
//! `serde` - re-exports of `nova_serde` items
//! `server` - implementation of Nova server

/// Nova Server implementation
pub mod server;

/// Nova core features
pub mod core;

/// Nova routing integration
pub mod routing;

/// Nova `serde` integration
#[cfg(feature = "serde")]
pub mod serde;
