// Copyright (c) 2023 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Display utilities.

use std::fmt::Display;

/// Extension methods for [`std::fmt::Display`].
pub trait DisplayExt {
    /// Formats an object with the "alternative" format (`{:#}`) and returns it.
    fn to_string_alt(&self) -> String;
}

impl<T: Display> DisplayExt for T {
    fn to_string_alt(&self) -> String {
        format!("{self:#}")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn prints_alternate_repr() {
        struct Foo;
        impl Display for Foo {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                if f.alternate() {
                    write!(f, "success")
                } else {
                    write!(f, "fail")
                }
            }
        }

        assert_eq!(Foo.to_string_alt(), "success");
    }
}
