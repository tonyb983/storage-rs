// Copyright (c) 2023 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Option utilities.

use std::fmt;
use std::ops::Deref;

use either::Either;

/// Extension methods for [`std::option::Option`].
pub trait OptionExt<T> {
    /// Converts from `Option<&T>` to `Option<T::Owned>` when `T` implements
    /// [`ToOwned`].
    ///
    /// The canonical use case is converting from an `Option<&str>` to an
    /// `Option<String>`.
    ///
    /// The name is symmetric with [`Option::cloned`].
    fn owned(&self) -> Option<<<T as Deref>::Target as ToOwned>::Owned>
    where
        T: Deref,
        T::Target: ToOwned;

    /// Returns a type that displays the option's value if it is present, or
    /// the provided default otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use xstd::option::OptionExt;
    ///
    /// fn render(number: Option<i32>) -> String {
    ///     format!("Your lucky number is {}.", number.display_or("unknown"))
    /// }
    ///
    /// assert_eq!(render(Some(42)), "Your lucky number is 42.");
    /// assert_eq!(render(None), "Your lucky number is unknown.");
    /// ```
    fn display_or<D>(self, default: D) -> Either<T, D>
    where
        T: fmt::Display,
        D: fmt::Display;

    /// Like [`OptionExt::display_or`], but the default value is computed
    /// only if the option is `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use xstd::option::OptionExt;
    ///
    /// fn render(number: Option<i32>, guess: i32) -> String {
    ///     format!(
    ///         "Your lucky number is {}.",
    ///         number.display_or_else(|| format!("unknown (best guess: {})", guess)),
    ///     )
    /// }
    ///
    /// assert_eq!(render(Some(42), 7), "Your lucky number is 42.");
    /// assert_eq!(render(None, 7), "Your lucky number is unknown (best guess: 7).");
    /// ```
    fn display_or_else<D, R>(self, default: D) -> Either<T, R>
    where
        T: fmt::Display,
        D: FnOnce() -> R,
        R: fmt::Display;
}

impl<T> OptionExt<T> for Option<T> {
    fn owned(&self) -> Option<<<T as Deref>::Target as ToOwned>::Owned>
    where
        T: Deref,
        T::Target: ToOwned,
    {
        self.as_ref().map(|x| x.deref().to_owned())
    }

    fn display_or<D>(self, default: D) -> Either<T, D>
    where
        T: fmt::Display,
        D: fmt::Display,
    {
        match self {
            Some(t) => Either::Left(t),
            None => Either::Right(default),
        }
    }

    fn display_or_else<D, R>(self, default: D) -> Either<T, R>
    where
        T: fmt::Display,
        D: FnOnce() -> R,
        R: fmt::Display,
    {
        match self {
            Some(t) => Either::Left(t),
            None => Either::Right(default()),
        }
    }
}
