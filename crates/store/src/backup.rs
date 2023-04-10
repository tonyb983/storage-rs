// Copyright (c) 2023 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{
    fs::Metadata,
    io::{BufReader, BufWriter, Read, Write},
    path::{Path, PathBuf},
};

use brotli::CompressorWriter;
use serde::{Deserialize, Serialize};
use xstd::{
    cast::CastFrom,
    fs::{create_write_truncate, read_only},
};

use crate::{Config, FileHeader, FileMeta, FileVersion, Result, Timestamp};

/// A file that has been backed up
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BackupFile {
    header: FileHeader,
    meta: FileMeta,
    file_bytes: Vec<u8>,
}

impl BackupFile {
    /// Create a new (**Version 1**) backup file from the file at the given path
    ///
    /// ## Errors
    /// - Function returns an error if any io operations fail.
    /// - Function returns an error if the serialization of [`FileMeta`] fails (this is used to get the size of the metadata for [`FileHeader`]).
    pub fn create_new(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let (raw_meta, file_bytes) = Self::extract_file_info(path)?;
        let meta =
            FileMeta::new_from_metadata(path, Timestamp::now(), &raw_meta, FileVersion::new())?;
        let meta_size = rmp_serde::to_vec(&meta)?.len();

        let header = FileHeader::new(meta_size, file_bytes.len());

        let backup_file = Self {
            header,
            meta,
            file_bytes,
        };

        Ok(backup_file)
    }

    /// Updates this backup file. This should be called when a change is detected in the original file.
    /// It updates the [`FileMeta`] from the current metadata, bumps the version, and updates the file bytes.
    ///
    /// ## Errors
    /// - Function returns an error if any IO operations fail.
    /// - Function returns an error if the serialization of [`FileMeta`] fails (this is used to get the size of the metadata for [`FileHeader`]).
    pub fn update_backup(&mut self) -> Result<()> {
        let (raw_meta, file_bytes) = Self::extract_file_info(self.meta.path())?;
        self.meta.update_from_metadata(&raw_meta);
        self.meta.bump_version();
        let meta_size = rmp_serde::to_vec(&self.meta)?.len();

        self.header = FileHeader::new(meta_size, file_bytes.len());
        self.file_bytes = file_bytes;

        Ok(())
    }

    /// Compresses this backup file into a [`CompressedBackupFile`] using `brotli`
    ///
    /// ## Errors
    /// - Function returns an error if any IO operations fail.
    /// - Function returns an error if the `rmp_serde` serialization fails.
    /// - Function returns an error if `brotli` compression fails.
    ///
    /// ## Panics
    /// Function panics if any of the various size assertions fail. These might be changed to `debug_`
    /// assertions or removed completely once I have verified that the function works as expected.
    ///
    /// See also: [`CompressedBackupFile::try_decompress`]
    pub fn try_compress(self) -> Result<CompressedBackupFile> {
        // Convert header to bytes using bytemuck
        let header_bytes = bytemuck::bytes_of(&self.header);
        assert_eq!(
            header_bytes.len(),
            std::mem::size_of::<FileHeader>(),
            "header_bytes should be the same size as FileHeader"
        );

        // Convert metadata to bytes using rmp_serde
        let meta_bytes = rmp_serde::to_vec(&self.meta)?;
        assert_eq!(
            meta_bytes.len(),
            self.header.meta_size,
            "meta bytes should be the size indicated by the header"
        );

        assert_eq!(
            self.file_bytes.len(),
            self.header.file_size,
            "meta bytes should be the size indicated by the header"
        );

        let total_size =
            std::mem::size_of::<FileHeader>() + self.file_bytes.len() + meta_bytes.len();
        let mut bytes = Vec::with_capacity(total_size);
        bytes.extend_from_slice(header_bytes);
        bytes.extend_from_slice(&meta_bytes);
        bytes.extend_from_slice(&self.file_bytes);
        assert_eq!(
            bytes.len(),
            total_size,
            "bytes.len() should be the expected/calculated total size"
        );

        let mut compressed_bytes = Vec::with_capacity(bytes.capacity());
        {
            let mut compressor =
                CompressorWriter::new(&mut compressed_bytes, crate::BUFFER_SIZE, 11, 22);
            compressor.write_all(&bytes)?;
            compressor.flush()?;
        }

        Ok(CompressedBackupFile::new(compressed_bytes))
    }

    /// Extracts the metadata and reads the bytes from the file at the given path
    fn extract_file_info(path: impl AsRef<Path>) -> Result<(Metadata, Vec<u8>)> {
        let path = path.as_ref();
        let raw_metadata = std::fs::metadata(path)?;
        let file_size = CastFrom::cast_from(raw_metadata.len());
        let mut file_bytes = Vec::with_capacity(file_size);
        {
            let mut reader = BufReader::new(read_only().open(path)?);
            let bytes_read = reader.read_to_end(&mut file_bytes)?;
            assert_eq!(
                bytes_read, file_size,
                "bytes_read should be the same as file_size"
            );
        }
        Ok((raw_metadata, file_bytes))
    }
}

/// A compressed backup file, ready to be written to disk
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CompressedBackupFile(Vec<u8>);

impl CompressedBackupFile {
    /// Creates a new [`CompressedBackupFile`] from the given bytes
    #[must_use]
    pub fn new(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    /// Attempts to decompress this [`CompressedBackupFile`] into a [`BackupFile`]
    ///
    /// ## Errors
    /// - Function returns an error if any IO operations fail.
    /// - Function returns an error if the `brotli` decompression fails.
    /// - Function returns an error if the `rmp_serde` deserialization fails.
    ///
    /// ## Panics
    /// Function panics if any of the various size assertions fail. These will eventually be changed to `debug_`
    /// or possibly removed completely once I have verified that the function works as expected.
    pub fn try_decompress(self) -> Result<BackupFile> {
        let mut decompressed_bytes = Vec::with_capacity(self.0.len());
        let mut reader = BufReader::new(&self.0[..]);

        let mut decompressor = brotli::Decompressor::new(&mut reader, crate::BUFFER_SIZE);
        decompressor.read_to_end(&mut decompressed_bytes)?;
        let (header, rest) = FileHeader::try_from_bytes(&decompressed_bytes)?;
        let (meta_bytes, file_bytes) = rest.split_at(header.meta_size);

        assert_eq!(
            meta_bytes.len(),
            header.meta_size,
            "meta bytes should be the size indicated by the header"
        );
        assert_eq!(
            file_bytes.len(),
            header.file_size,
            "file bytes should be the size indicated by the header"
        );

        let bytes: Vec<u8> = file_bytes.into();

        let meta = rmp_serde::from_slice(meta_bytes)?;
        Ok(BackupFile {
            header,
            meta,
            file_bytes: bytes,
        })
    }

    /// Writes this [`CompressedBackupFile`] to the given path, overwriting any existing file.
    ///
    /// ## Errors
    /// - Function returns an error if [`std::fs::File::open`] fails.  
    /// - Function returns an error if the IO ops [`std::io::Write::write_all`] or [`std::io::Write::flush`] fail.
    pub fn write_to_file(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();
        let mut writer = BufWriter::new(create_write_truncate().open(path)?);
        writer.write_all(&self.0)?;
        writer.flush()?;
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct BackupInfo {
    header: FileHeader,
    meta: FileMeta,
    backup_path: PathBuf,
}

/// The main interface for backing up and retreiving files
#[derive(Debug)]
pub struct BackupManager {
    config: Config,
    file_info: Vec<BackupInfo>,
}

impl BackupManager {
    /// Creates a new [`BackupManager`] with the given [`Config`]. This will scan the backup
    /// store folder to collect all metadata.
    ///
    /// ## Errors
    /// - `std::io::Error` if there is an error reading the backup store folder or any of the individual backup files
    pub fn new(config: Config) -> Result<Self> {
        let mut this = Self {
            config,
            file_info: vec![],
        };
        this.collect_backup_info()?;
        Ok(this)
    }

    /// Update the [`Config`] used by the [`BackupManager`]
    pub fn update_config(&mut self, config: Config) {
        self.config = config;
    }

    fn store_path(&self) -> &Path {
        self.config.store_dir_path()
    }

    fn collect_backup_info(&mut self) -> Result {
        let mut infos = vec![];

        for entry in std::fs::read_dir(self.store_path())? {
            let entry = entry?;
            let backup_path = entry.path();

            let (header, meta) = extract_header_and_meta(&backup_path)?;
            infos.push(BackupInfo {
                header,
                meta,
                backup_path,
            });
        }

        self.file_info = infos;
        Ok(())
    }
}

/// Given a path (to a **backup** file), extract only the [`FileHeader`] and the [`FileMeta`] without
/// reading the actual file bytes.
///
/// ## Errors
/// - Returns an IO error if the backup file cannot be opened, or the buffered reader fails to read
/// the specified number of bytes.
/// - Returns a Serde error if `rmp_serde` fails to deserialize the [`FileMeta`]
pub fn extract_header_and_meta(backup_path: impl AsRef<Path>) -> Result<(FileHeader, FileMeta)> {
    let mut reader = BufReader::new(read_only().open(&backup_path)?);
    let mut header_buf = vec![0; std::mem::size_of::<FileHeader>()];
    reader.read_exact(&mut header_buf)?;
    let header = FileHeader::try_from_bytes_exact(&header_buf)?;

    let mut meta_buf = vec![0; header.meta_size];
    reader.read_exact(&mut meta_buf)?;
    let meta: FileMeta = rmp_serde::from_slice(&meta_buf)?;
    Ok((header, meta))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_temp_file() -> std::fs::File {
        tempfile::tempfile().expect("failed to create temp file")
    }

    fn create_named_temp_file() -> tempfile::NamedTempFile {
        tempfile::NamedTempFile::new().expect("failed to create named temp file")
    }

    #[test]
    fn roundtrip_test() {
        const FILE_TEXT: &str = "The quick brown fox jumps over the lazy dog.";
        let mut file = create_named_temp_file();
        write!(file, "{FILE_TEXT}").expect("failed to write to temp file");
        let path = file.path();

        let result = BackupFile::create_new(path);
        assert!(
            result.is_ok(),
            "BackupFile::create_new failed: {}",
            result.unwrap_err()
        );
        let backup = result.unwrap();
        {
            let file_text = String::from_utf8(backup.file_bytes.clone())
                .expect("failed to create string from file bytes");
            assert_eq!(
                file_text, FILE_TEXT,
                "file text should be the same after compression and decompression"
            );
        }
        let backup_copy = backup.clone();
        println!("backup: {backup:#?}");
        let result = backup.try_compress();
        assert!(
            result.is_ok(),
            "BackupFile::try_compress failed: {}",
            result.unwrap_err()
        );
        let compressed = result.unwrap();
        let result = compressed.try_decompress();
        assert!(
            result.is_ok(),
            "CompressedBackupFile::try_decompress failed: {}",
            result.unwrap_err()
        );
        let decompressed = result.unwrap();
        let file_text = String::from_utf8(decompressed.file_bytes)
            .expect("failed to create string from file bytes");
        assert_eq!(
            file_text, FILE_TEXT,
            "file text should be the same after compression and decompression"
        );
    }
}
