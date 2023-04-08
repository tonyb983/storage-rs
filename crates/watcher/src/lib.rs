//! Storage-Mon
//!
//!  A module responsible for monitoring changes to specified files
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
#![feature(associated_type_defaults)]

mod watcher;

pub use watcher::{NotifyEvent, NotifyWatcher};

pub(crate) use storage_common::{Config, Result};

/// A trait describing the behavior and available functions for a file watcher
pub trait FileWatcher: Send {
    /// The type of the inner configuration used by the file watcher, if any
    type InnerConfig = ();

    /// Returns a list of all files currently being watched
    ///
    /// ## Errors
    /// Errors only if the currently watched files cannot be read (e.g. mutex is poisoned). Very unlikely.
    fn currently_watched(&self) -> Result<Vec<String>>;
    /// Applies the [application config](Config) to the file watcher
    ///
    /// ## Errors
    /// Errors if the file watcher errors while applying the config
    fn apply_app_config(&mut self, config: &Config) -> Result;
    /// Starts the file watcher.
    ///
    /// ## Errors
    /// Any errors that occur while starting the file watcher, or adding the list of watched files to it
    fn start(&mut self) -> Result;
    /// Stops the file watcher.
    ///
    /// ## Errors
    /// Any errors returned while attempting to stop the file watcher
    fn stop(&mut self) -> Result;

    /// Applies both the [application config](storage_common::Config) as well as the [inner config](FileWatcher::InnerConfig)
    /// and starts the file watcher.
    /// Default implementation simply calls [`FileWatcher::apply_inner_config`], [`FileWatcher::apply_app_config`] and then [`FileWatcher::start`].
    ///
    /// ## Errors
    /// Any errors that occur during configuration or start-up will be propagated
    fn start_with_config(
        &mut self,
        app_config: &Config,
        impl_config: &Self::InnerConfig,
    ) -> Result {
        self.apply_app_config(app_config)?;
        self.apply_inner_config(impl_config)?;
        self.start()
    }
    /// Applies the [application config](storage_common::Config) and starts the file watcher.
    /// Default implementation simple calls [`FileWatcher::apply_app_config`] and then [`FileWatcher::start`].
    ///
    /// ## Errors
    /// Any errors that occur during configuration or start-up will be propagated
    fn start_with_app_config(&mut self, config: &Config) -> Result {
        self.apply_app_config(config)?;
        self.start()
    }
    /// Applies the given [inner config](FileWatcher::InnerConfig).
    /// **Default implementation simply returns Ok(()) so if you are writing a custom implementation be sure to override.**
    ///
    /// ## Errors
    /// Any errors that occur during configuration or start-up will be propagated
    fn apply_inner_config(&mut self, _config: &Self::InnerConfig) -> Result {
        Ok(())
    }
}

/// Attempts to create a new file watcher
///
/// ## Errors
/// Errors if the file watcher cannot be created
pub fn create_file_watcher() -> Result<impl FileWatcher> {
    watcher::NotifyWatcher::new()
}
