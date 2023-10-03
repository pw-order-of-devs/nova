#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![cfg_attr(test, deny(rust_2018_idioms))]
//! nova core structures, traits, types

/// nova errors
pub mod errors;
/// nova request
pub mod request;
/// nova response
pub mod response;
/// nova types
pub mod types;

/// nova hello_serde integration
pub mod serde;

/// extensions
pub mod ext;
