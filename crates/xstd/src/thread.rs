//! Thread utilities.

use std::thread::JoinHandle;

/// Wraps a [`JoinHandle`] so that the child thread is joined when the handle is
/// dropped, rather than detached. If the child thread panics,
/// `JoinOnDropHandle` will panic when dropped.
#[derive(Debug)]
pub struct JoinOnDropHandle<T>(Option<JoinHandle<T>>);

impl<T> Drop for JoinOnDropHandle<T> {
    fn drop(&mut self) {
        self.0.take().unwrap().join().unwrap();
    }
}

/// Wraps a [`JoinHandle`] so that the child thread is unparked (and then
/// detached as usual) when the handle is dropped.
#[derive(Debug)]
pub struct UnparkOnDropHandle<T>(JoinHandle<T>);

impl<T> Drop for UnparkOnDropHandle<T> {
    fn drop(&mut self) {
        self.0.thread().unpark();
    }
}

/// Extension methods for [`JoinHandle`].
///
/// Adds the following methods:
/// `join_on_drop` - joins a thread when the handle is dropped
/// `unpark_on_drop` - unparks a thread when the handle is dropped
pub trait JoinHandleExt<T> {
    /// Converts a [`JoinHandle`] into a [`JoinOnDropHandle`].
    fn join_on_drop(self) -> JoinOnDropHandle<T>;

    /// Converts a [`JoinHandle`] into an [`UnparkOnDropHandle`].
    fn unpark_on_drop(self) -> UnparkOnDropHandle<T>;
}

impl<T> JoinHandleExt<T> for JoinHandle<T> {
    fn join_on_drop(self) -> JoinOnDropHandle<T> {
        JoinOnDropHandle(Some(self))
    }

    fn unpark_on_drop(self) -> UnparkOnDropHandle<T> {
        UnparkOnDropHandle(self)
    }
}
