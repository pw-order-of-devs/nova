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

//! `nova_core` - structures, traits, types
//!
//! ```file_tree
//! `nova_core`
//! ├── errors - mod.rs
//! ├── request.rs
//! ├── response.rs
//! └── types - mod.rs
//!     ├── headers.rs
//!     ├── path.rs
//!     ├── protocol.rs
//!     ├── query.rs
//!     ├── request_type.rs
//!     └── status.rs
//! ├── ext - ...
//! └── validators - ...
//! ```
//!
//! `errors` - definition of errors returned by Nova crate
//! `request` - definition of Nova Http Request
//! `response` - definition of Nova Http Response
//! `serde` - crate integration with serde
//! `types` - definitions of structs used in crate
//! `ext` - internal extensions
//! `validators` - internal validators

/// Nova errors definitions
pub mod errors;
/// Nova Http Request definition
pub mod request;
/// Nova Http Response definition
pub mod response;
/// Nova Types definitions
pub mod types;

/// Nova `serde` integration
pub mod serde;

/// extensions
pub mod ext;

/// validators
pub(crate) mod validators;
