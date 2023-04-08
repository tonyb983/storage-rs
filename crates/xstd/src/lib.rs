//! `xstd` - extensions to the standard library
//!
//! Most of this is straight up stolen from the [awesome materialize repo](https://github.com/MaterializeInc/materialize/tree/main/src/ore).
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
#![cfg_attr(nightly_doc_features, feature(doc_cfg))]

#[cfg_attr(nightly_doc_features, doc(cfg(feature = "test")))]
#[cfg(feature = "test")]
pub mod assert;
pub mod bits;
pub mod cast;
pub mod collections;
pub mod display;
pub mod env;
pub mod fs;
pub mod graph;
pub mod hash;
pub mod hint;
pub mod id_gen;
pub mod iter;
pub mod lex;
pub mod now;
pub mod option;
pub mod panic;
pub mod path;
pub mod permutations;
pub mod result;
pub mod stats;
pub mod str;
#[cfg_attr(nightly_doc_features, doc(cfg(feature = "test")))]
#[cfg(feature = "test")]
pub mod test;
pub mod thread;
pub mod vec;
