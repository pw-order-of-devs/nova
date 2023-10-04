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
//! `nova_core` - structures, traits, types

/// nova errors
pub mod errors;
/// nova request
pub mod request;
/// nova response
pub mod response;
/// nova types
pub mod types;

/// nova serde integration
pub mod serde;

/// extensions
pub mod ext;

/// validators
pub(crate) mod validators;
