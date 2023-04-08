// Copyright (c) 2023 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{
    fs::Metadata,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::{FileVersion, Result, Timestamp};

/// A serializable version of [`std::fs::Metadata`]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct FsMetadata {
    created: Option<Timestamp>,
    modified: Option<Timestamp>,
    accessed: Option<Timestamp>,
    size: u64,
    file_type: FileKind,
}

impl FsMetadata {
    /// Creates a new [`FsMetadata`] by retrieving the [`std::fs::Metadata`]..
    ///
    /// ## Errors
    /// This function will return an error if [`std::fs::metadata`] fails.
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let meta = std::fs::metadata(path)?;
        Ok(Self::from_metadata(&meta))
    }

    /// Converts a [`Metadata`](std::fs::Metadata) object into this [serializable version](FsMetadata)
    pub fn from_metadata(meta: &Metadata) -> Self {
        let created = meta.created().map(std::convert::Into::into).ok();
        let modified = meta.modified().map(std::convert::Into::into).ok();
        let accessed = meta.accessed().map(std::convert::Into::into).ok();
        let size = meta.len();
        let file_type = meta.into();

        Self {
            created,
            modified,
            accessed,
            size,
            file_type,
        }
    }

    /// Gets the creation time of the file if available
    #[must_use]
    pub fn created(&self) -> Option<Timestamp> {
        self.created
    }

    /// Gets the last modified time of the file if available
    #[must_use]
    pub fn modified(&self) -> Option<Timestamp> {
        self.modified
    }

    /// Gets the last access time of the file if available
    #[must_use]
    pub fn accessed(&self) -> Option<Timestamp> {
        self.accessed
    }

    /// Gets the size of the file this metadata represents
    #[must_use]
    pub fn size(&self) -> u64 {
        self.size
    }

    /// Gets the [`FileKind`] of the file this metadata represents
    #[must_use]
    pub fn file_type(&self) -> FileKind {
        self.file_type
    }
}

impl From<Metadata> for FsMetadata {
    fn from(meta: Metadata) -> Self {
        Self::from_metadata(&meta)
    }
}
impl From<&Metadata> for FsMetadata {
    fn from(meta: &Metadata) -> Self {
        Self::from_metadata(meta)
    }
}

/// Types of files. Used due to the fact that [`std::fs::FileType`] is not serializable
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum FileKind {
    /// A File
    File,
    /// A Directory
    Dir,
    /// A Symbolic Link
    Symlink,
    /// Any other type of file
    Unknown,
}

impl From<Metadata> for FileKind {
    fn from(meta: Metadata) -> Self {
        if meta.is_file() {
            Self::File
        } else if meta.is_dir() {
            Self::Dir
        } else if meta.file_type().is_symlink() {
            Self::Symlink
        } else {
            Self::Unknown
        }
    }
}
impl From<&Metadata> for FileKind {
    fn from(meta: &Metadata) -> Self {
        if meta.is_file() {
            Self::File
        } else if meta.is_dir() {
            Self::Dir
        } else if meta.file_type().is_symlink() {
            Self::Symlink
        } else {
            Self::Unknown
        }
    }
}

/// The metadata for a backup file
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FileMeta {
    /// The backup version
    version: FileVersion,
    /// The timestamp of when the **backup** was created (*not* the original file)
    backup_created: Timestamp,
    /// The path to the original file
    path: PathBuf,
    /// The filesystem metadata for the original file at time of backup
    fs_meta: FsMetadata,
}

impl FileMeta {
    #[must_use]
    pub(crate) fn new(
        version: FileVersion,
        created: Timestamp,
        path: PathBuf,
        fs_meta: FsMetadata,
    ) -> Self {
        Self {
            version,
            backup_created: created,
            path,
            fs_meta,
        }
    }

    /// Creates a new [`FileMeta`] for the file at the given path.
    ///
    /// # Errors
    /// This function will return an error if the given `path` does not point to a valid file.
    ///
    /// This function will return an error if the metadata for the file cannot be retrieved.
    pub fn new_for(path: &std::path::Path, version: FileVersion) -> Result<Self> {
        if !path.exists() {
            return Err(format!("file at path '{}' does not exist", path.display()).into());
        }
        let fs_meta = std::fs::metadata(path)?;
        let created = Timestamp::now();

        Ok(Self::new(
            version,
            created,
            path.to_path_buf(),
            fs_meta.into(),
        ))
    }

    /// Creates a new [`FileMeta`] for the file at the given path with the given `created` timestamp.
    /// Uses the provided `metadata` instead of retrieving it from the filesystem.
    ///
    /// ## Errors
    /// This function will return an error if the given `path` does not point to a valid file.
    pub fn new_from_metadata(
        path: impl AsRef<Path>,
        created: Timestamp,
        metadata: &Metadata,
        version: FileVersion,
    ) -> Result<Self> {
        let fs_meta = metadata.into();

        let this = Self::new(version, created, path.as_ref().to_path_buf(), fs_meta);
        Ok(this)
    }

    /// Overwrites the current metadata with the given `metadata`
    pub fn update_from_metadata(&mut self, metadata: &Metadata) {
        self.fs_meta = metadata.into();
    }

    /// Increments the current file version
    pub fn bump_version(&mut self) {
        self.version.increment();
    }

    /// Sets the `backup_created` field to the current time
    pub fn set_created_now(&mut self) {
        self.backup_created = Timestamp::now();
    }

    /// Gets the [`FileVersion`]
    #[must_use]
    pub fn version(&self) -> &FileVersion {
        &self.version
    }

    /// Gets the time the backup was created
    #[must_use]
    pub fn created(&self) -> &Timestamp {
        &self.backup_created
    }

    /// Gets the original path of this file
    #[must_use]
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    /// Gets the filesystem metadata for this file
    #[must_use]
    pub fn fs_meta(&self) -> &FsMetadata {
        &self.fs_meta
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_testing() {
        assert_eq!(1, 1);
    }
}
