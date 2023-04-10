// Copyright (c) 2023 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use bytemuck::{checked::try_pod_read_unaligned, Pod, Zeroable};
use serde::{Deserialize, Serialize};
use xstd::result::ResultExt;

use crate::Result;

/// Small, plain data type representing the header of a backup file, indicated the
/// size of the metadata bytes and the size of the file bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct FileHeader {
    /// The size of the metadata bytes that follow the header
    pub meta_size: usize,
    /// The size of the file bytes that follow the metadata bytes
    pub file_size: usize,
}

unsafe impl Zeroable for FileHeader {}
unsafe impl Pod for FileHeader {}

impl FileHeader {
    /// Create a new [`FileHeader`] with the given metadata size and file size
    #[must_use]
    pub fn new(meta_size: usize, file_size: usize) -> Self {
        Self {
            meta_size,
            file_size,
        }
    }

    /// Attempts to extract a [`FileHeader`] from the given byte slice. This version
    /// accepts **only** a slice of the exact size of [`FileHeader`].
    ///
    /// ## Errors
    /// - Errors if bytemuck is unable to convert slice to [`FileHeader`] or if the slice
    /// is not the correct size
    pub fn try_from_bytes_exact(bytes: &[u8]) -> Result<Self> {
        let this: Self = bytemuck::try_pod_read_unaligned(bytes).map_err_to_string()?;
        Ok(this)
    }

    /// Attempts to extract a [`FileHeader`] from the given byte slice. This version
    /// will return the newly created [`FileHeader`] and any remaining bytes.
    ///
    /// ## Errors
    /// - Errors if bytemuck is unable to convert slice to [`FileHeader`]
    pub fn try_from_bytes(bytes: &[u8]) -> Result<(Self, &[u8])> {
        let (header, bytes) = try_pull_pod(bytes)?;
        Ok((header, bytes))
    }
}

impl Default for FileHeader {
    fn default() -> Self {
        Self::new(std::mem::size_of::<crate::FileMeta>(), 1)
    }
}

#[inline]
pub(crate) fn try_pull_pod<T: Pod>(bytes: &[u8]) -> Result<(T, &[u8])> {
    let position = std::mem::size_of::<T>();
    if bytes.len() >= position {
        let (head, tail) = bytes.split_at(position);
        let a: T = try_pod_read_unaligned(head).map_err_to_string()?;
        Ok((a, tail))
    } else {
        Err("oopsy poopsy".into())
    }
}
