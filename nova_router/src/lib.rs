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

//! `nova_router` - routing implementation and integration
//!
//! ```file_tree
//! `nova_router`
//! ├── callable.rs
//! ├── router.rs
//! ├── route.rs
//! ├── routes.rs
//! └── server_routing.rs
//! ```
//!
//! `callable` - definition of callable handler
//! `router` - definition of Nova Router
//! `route` - definition of Nova Route
//! `routes` - wrapper of Nova Routes
//! `server_routing` - trait for integration of routing

/// Nova callable definition
pub mod callable;
/// Nova route definition
pub mod route;
/// Nova router definition
pub mod router;
/// Nova routes wrapper
pub mod routes;
/// Server routing integration
pub mod server_routing;
