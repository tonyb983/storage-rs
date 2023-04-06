// Copyright (c) 2023 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Path utilities.

use std::path::{Component, Path, PathBuf};

/// Extension methods for [`Path`].
pub trait PathExt {
    /// Normalizes a path using purely lexical analysis.
    ///
    /// The following normalization rules are applied iteratively:
    ///
    ///   * Multiple contiguous path separators are replaced with a single
    ///     [`MAIN_SEPARATOR`].
    ///   * Current directory components (`.`) are removed.
    ///   * Parent directory components (`..`) that do not occur at the
    ///     beginning of the path are removed along with the preceding
    ///     component.
    ///    * Parent directory components at the start of a rooted path
    ///      (e.g., `/..`) are removed.
    ///    * Empty paths are replaced with ".".
    ///
    /// The returned path ends in a separator only if it represents the root
    /// directory.
    ///
    /// This method is a port of Go's [`path.Clean`] function.
    ///
    /// [`path.Clean`]: https://pkg.go.dev/path#Clean
    /// [`MAIN_SEPARATOR`]: std::path::MAIN_SEPARATOR
    fn clean(&self) -> PathBuf;
}

impl PathExt for Path {
    fn clean(&self) -> PathBuf {
        let mut buf = PathBuf::new();
        for component in self.components() {
            match component {
                // `.` elements are always redundant and can be dropped.
                Component::CurDir => (),

                // `..` elements require special handling.
                Component::ParentDir => match buf.components().last() {
                    // `..` at beginning or after another `..` needs to be
                    // retained.
                    None | Some(Component::ParentDir) => buf.push(Component::ParentDir),
                    // `..` after a root is a no-op.
                    Some(Component::RootDir) => (),
                    // `..` after a normal component can be normalized by
                    // dropping the prior component.
                    _ => {
                        buf.pop();
                    }
                },

                // All other component types can be pushed verbatim.
                Component::RootDir | Component::Prefix(_) | Component::Normal(_) => {
                    buf.push(component);
                }
            }
        }
        if buf.as_os_str().is_empty() {
            buf.push(".");
        }
        buf
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::PathExt;

    #[test]
    fn test_clean() {
        // These test cases are imported from the Go standard library.
        for (input, output) in &[
            // Already clean.
            ("", "."),
            ("abc", "abc"),
            ("abc/def", "abc/def"),
            ("a/b/c", "a/b/c"),
            (".", "."),
            ("..", ".."),
            ("../..", "../.."),
            ("../../abc", "../../abc"),
            ("/abc", "/abc"),
            ("/", "/"),
            // Remove trailing slash.
            ("abc/", "abc"),
            ("abc/def/", "abc/def"),
            ("a/b/c/", "a/b/c"),
            ("./", "."),
            ("../", ".."),
            ("../../", "../.."),
            ("/abc/", "/abc"),
            // Remove doubled slash.
            ("abc//def//ghi", "abc/def/ghi"),
            ("//abc", "/abc"),
            ("///abc", "/abc"),
            ("//abc//", "/abc"),
            ("abc//", "abc"),
            // Remove . elements.
            ("abc/./def", "abc/def"),
            ("/./abc/def", "/abc/def"),
            ("abc/.", "abc"),
            // Remove .. elements.
            ("abc/def/ghi/../jkl", "abc/def/jkl"),
            ("abc/def/../ghi/../jkl", "abc/jkl"),
            ("abc/def/..", "abc"),
            ("abc/def/../..", "."),
            ("/abc/def/../..", "/"),
            ("abc/def/../../..", ".."),
            ("/abc/def/../../..", "/"),
            ("abc/def/../../../ghi/jkl/../../../mno", "../../mno"),
            // Combinations.
            ("abc/./../def", "def"),
            ("abc//./../def", "def"),
            ("abc/../../././../def", "../../def"),
        ] {
            println!("clean({input}) = {output}");
            assert_eq!(Path::new(input).clean(), Path::new(output));
        }
    }
}
