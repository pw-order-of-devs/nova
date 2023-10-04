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
#![allow(clippy::self_named_constructors)]
#![cfg_attr(test, deny(rust_2018_idioms))]

//! `nova_middleware` - middleware implementation and integration
//!
//! ```file_tree
//! `nova_middleware`
//! ├── middleware.rs
//! ├── middlewares.rs
//! └── server_middleware.rs
//! ```
//!
//! `middleware` - definition of middleware handler
//! `middlewares` - wrapper of Nova Middleware
//! `server_middleware` - trait for integration of middleware

/// Nova middleware trait
pub mod middleware;
/// Nova middleware wrapper
pub mod middlewares;
/// Server middleware integration
pub mod server_middleware;
