// Copyright (c) 2023 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Utilities for bit and byte manipulation

/// Increases `p` as little as possible (including possibly 0)
/// such that it becomes a multiple of `N`.
#[must_use]
pub const fn align_up<const N: usize>(p: usize) -> usize {
    if p % N == 0 {
        p
    } else {
        p + (N - (p % N))
    }
}
