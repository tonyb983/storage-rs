// Copyright (c) 2023 Tony Barbitta
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Extensions to `std::hint`.

/// A function that is opaque to the optimizer, used to prevent the compiler
/// from optimizing away computations in a benchmark.
///
/// This variant is stable-compatible, but it may cause some performance
/// overhead or fail to prevent code from being eliminated.
///
/// When `std::hint::black_box` is stabilized, this function can be removed.
pub fn black_box<T>(dummy: T) -> T {
    unsafe {
        let ret = std::ptr::read_volatile(&dummy);
        std::mem::forget(dummy);
        ret
    }
}