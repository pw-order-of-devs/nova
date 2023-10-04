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

//! `nova_serde` - serde integration
//!
//! ```file_tree
//! `nova_serde`
//! └── serde.rs
//! ```
//!
//! `serde` - integration of serde for Nova

/// serde integration
pub mod serde;
