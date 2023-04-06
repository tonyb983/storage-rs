//! Test utilities.

use std::sync::mpsc::{self, RecvTimeoutError};
use std::sync::Once;
use std::thread;
use std::time::Duration;

use anyhow::bail;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

static LOG_INIT: Once = Once::new();

/// Initialize global logger, using the [`tracing_subscriber`] crate, with
/// sensible defaults.
///
/// It is safe to call `init_logging` multiple times. Since `cargo test` does
/// not run tests in any particular order, each must call `init_logging`.
pub fn init_logging() {
    init_logging_default("info");
}

/// Initialize global logger, using the [`tracing_subscriber`] crate.
///
/// The default log level will be set to the value passed in.
///
/// It is safe to call `init_logging_level` multiple times. Since `cargo test` does
/// not run tests in any particular order, each must call `init_logging`.
///
/// ## Panics
/// Panics if the environmental filter cannot be created.
pub fn init_logging_default(level: &str) {
    LOG_INIT.call_once(|| {
        let filter = EnvFilter::try_from_env("MZ_LOG_FILTER")
            .or_else(|_| EnvFilter::try_new(level))
            .unwrap();
        FmtSubscriber::builder()
            .with_env_filter(filter)
            .with_test_writer()
            .init();
    });
}

/// Runs a function with a timeout.
///
/// The provided closure is invoked on a thread. If the thread completes
/// normally within the provided `duration`, its result is returned. If the
/// thread panics within the provided `duration`, the panic is propagated to the
/// thread calling `timeout`. Otherwise, a timeout error is returned.
///
/// Note that if the invoked function does not complete in the timeout, it is
/// not killed; it is left to wind down normally. Therefore this function is
/// only appropriate in tests, where the resource leak doesn't matter.
///
/// ## Panics
/// Panics if the thread is unable to be joined
///
/// ## Errors
/// Errors if the thread times out or is disconnected
pub fn timeout<F, T>(duration: Duration, f: F) -> Result<T, anyhow::Error>
where
    F: FnOnce() -> Result<T, anyhow::Error> + Send + 'static,
    T: Send + 'static,
{
    // Use the drop of `tx` to indicate that the thread is finished. This
    // ensures that `tx` is dropped even if `f` panics. No actual value is ever
    // sent on `tx`.
    let (tx, rx) = mpsc::channel();
    let thread = thread::spawn(|| {
        let _tx = tx;
        f()
    });
    match rx.recv_timeout(duration) {
        Ok(()) => unreachable!(),
        Err(RecvTimeoutError::Disconnected) => thread.join().unwrap(),
        Err(RecvTimeoutError::Timeout) => bail!("thread timed out"),
    }
}
