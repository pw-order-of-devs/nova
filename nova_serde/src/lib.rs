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

//! `nova_serde` - `serde` integration
//!
//! ```file_tree
//! `nova_serde`
//! ├── request
//! └── response
//! ```
//!
//! `request` - integration of `serde` for Nova Request body
//! `response` - integration of `serde` for Nova Response body

/// `serde` integration for request
pub mod request;
/// `serde` integration for response
pub mod response;

pub use serde::{Deserialize, Serialize};
