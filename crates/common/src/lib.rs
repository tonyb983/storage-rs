//! Common types and utilities for the `storage` crate/workspace

#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::perf,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::missing_safety_doc,
    rustdoc::all,
    rust_2021_compatibility
)]
#![allow(clippy::module_name_repetitions, clippy::similar_names)]

mod config;
mod error;

pub use config::{Config, MaybeConfig};
pub use error::Error;

pub type Result<T = (), E = Error> = std::result::Result<T, E>;
