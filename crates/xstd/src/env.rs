// Copyright (c) 2023 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Process environment utilities.

use std::env;
use std::ffi::OsStr;

/// Reports whether the environment variable `key` is set to a truthy value in
/// the current process's environment.
///
/// The empty string and the string "0" are considered false. All other values
/// are considered true.
pub fn is_var_truthy<K>(key: K) -> bool
where
    K: AsRef<OsStr>,
{
    match env::var_os(key) {
        None => false,
        Some(val) => val != "0" && val != "",
    }
}
