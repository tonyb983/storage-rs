//! File System Utilities

/// A simple implementation of `% touch path` (ignores existing files)
///
/// ## Errors
/// Returns an error if the file at the given `path` cannot be created or modified
pub fn touch(path: &std::path::Path) -> std::io::Result<()> {
    match std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(path)
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
