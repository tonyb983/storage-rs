//! Storage-Store
//!
//!  A module that manages the creation of compressed and versioned backups whenever a
//!  change is detected. It should handle file I/O, compression (e.g., gzip), and back-
//!  up history.
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

mod backup;
mod header;
mod meta;
mod version;

pub use backup::{extract_header_and_meta, BackupFile, BackupManager, CompressedBackupFile};
pub use header::FileHeader;
pub use meta::{FileKind, FileMeta, FsMetadata};
pub use version::SaturatingFileVersion as FileVersion;
pub use version::{SaturatingFileVersion, WrappingFileVersion};

pub(crate) use storage_common::{Config, Result, Timestamp};
pub(crate) const BUFFER_SIZE: usize = 4096;
