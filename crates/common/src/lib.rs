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
#![cfg_attr(
    test,
    allow(
        unused,
        dead_code,
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
    )
)]

mod config;
mod error;
mod time;

pub use config::{Config, MaybeConfig};
pub use error::{Error, Result};
pub use time::{current_timestamp, Timestamp};
