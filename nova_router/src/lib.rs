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

//! nova router

/// nova callable
pub mod callable;
/// nova route
pub mod route;
/// nova router
pub mod router;
/// nova routes wrapper
pub mod routes;
/// server routing
pub mod server_routing;
