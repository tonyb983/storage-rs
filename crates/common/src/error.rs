// Copyright (c) 2023 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/// Common error type for the `storage` crate/workspace
#[derive(Debug)]
pub enum Error {
    /// Wrapper around [`io::Error`](std::io::Error)
    Io(std::io::Error),
    /// Wrapper around [`string::Error`](std::string::FromUtf8Error)
    Utf8(std::string::FromUtf8Error),
    /// Wrapper around [`notify::Error`](notify::Error)
    Notify(notify::Error),
    /// Other errors
    Other(String),
}

impl From<notify::Error> for Error {
    fn from(err: notify::Error) -> Self {
        Self::Notify(err)
    }
}
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}
impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Self::Utf8(err)
    }
}
impl From<&str> for Error {
    fn from(err: &str) -> Self {
        Self::Other(err.to_string())
    }
}
impl From<String> for Error {
    fn from(err: String) -> Self {
        Self::Other(err)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(err) => write!(f, "io error - {err}"),
            Self::Utf8(err) => write!(f, "utf-8 error - {err}"),
            Self::Notify(err) => write!(f, "notify error - {err}"),
            Self::Other(err) => write!(f, "other error - {err}"),
        }
    }
}

impl std::error::Error for Error {}
