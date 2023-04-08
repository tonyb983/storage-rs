//! File System Utilities

pub use walkdir;
pub use walkdir::{DirEntry as WalkDirEntry, Result as WalkDirResult, WalkDir};

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

/// Walks the directory at `path` using the `walkdir` crate
pub fn walk_dir(path: &std::path::Path) -> impl Iterator<Item = WalkDirResult<WalkDirEntry>> {
    WalkDir::new(path).into_iter()
}

/// Walks the directory at `path` with the given [options](WalkDirOptions)
pub fn walk_dir_with(
    path: &std::path::Path,
    opts: &WalkDirOptions,
) -> impl Iterator<Item = WalkDirResult<WalkDirEntry>> {
    opts.apply_to(WalkDir::new(path)).into_iter()
}

/// Walks the directory at `path`, filtering out any errors (inaccessible files, etc.)
pub fn walk_dir_valid(path: &std::path::Path) -> impl Iterator<Item = WalkDirEntry> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(std::result::Result::ok)
}

/// Walks the directory at `path` using the given [`opts`](WalkDirOptions) and filtering out any errors (inaccessible files, etc.)
pub fn walk_dir_valid_with(
    path: &std::path::Path,
    opts: &WalkDirOptions,
) -> impl Iterator<Item = WalkDirEntry> {
    opts.apply_to(WalkDir::new(path))
        .into_iter()
        .filter_map(std::result::Result::ok)
}

/// Options that can be applied to the directory walker in [`walk_dir_with`](walk_dir_with) and [`walk_dir_valid_with`](walk_dir_valid_with)
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct WalkDirOptions {
    /// Produces the entries in the directory before any sub-directories
    pub contents_first: Option<bool>,
    /// Follow symbolic links
    pub follow_links: Option<bool>,
    /// The maximum number of open file descriptors
    pub max_open: Option<usize>,
    /// The minimum depth to descend into
    pub min_depth: Option<usize>,
    /// The maximum depth to descend into
    pub max_depth: Option<usize>,
    /// Only follow entries on the same file system
    pub same_file_system: Option<bool>,
    /// Sort the entries by their file name for a stable order
    pub sort_by_filename: Option<bool>,
}

impl WalkDirOptions {
    /// Applies these options to the given [`WalkDir`](walkdir::WalkDir) instance
    #[must_use]
    pub fn apply_to(&self, mut walker: WalkDir) -> WalkDir {
        if let Some(contents_first) = self.contents_first {
            walker = walker.contents_first(contents_first);
        }
        if let Some(follow_links) = self.follow_links {
            walker = walker.follow_links(follow_links);
        }
        if let Some(max_open) = self.max_open {
            walker = walker.max_open(max_open);
        }
        if let Some(min_depth) = self.min_depth {
            walker = walker.min_depth(min_depth);
        }
        if let Some(max_depth) = self.max_depth {
            walker = walker.max_depth(max_depth);
        }
        if let Some(same_file_system) = self.same_file_system {
            walker = walker.same_file_system(same_file_system);
        }
        if let Some(sort_by_filename) = self.sort_by_filename {
            if sort_by_filename {
                walker = walker.sort_by_file_name();
            }
        }

        walker
    }
}

impl WalkDirOptions {
    /// Sets the `sort_by_filename` option
    #[must_use]
    pub fn with_sort_by_filename(self, sort_by_filename: bool) -> Self {
        Self {
            sort_by_filename: Some(sort_by_filename),
            ..self
        }
    }

    /// Sets the `same_file_system` option
    #[must_use]
    pub fn with_same_file_system(self, same_file_system: bool) -> Self {
        Self {
            same_file_system: Some(same_file_system),
            ..self
        }
    }

    /// Sets the `max_depth` option
    #[must_use]
    pub fn with_max_depth(self, max_depth: usize) -> Self {
        Self {
            max_depth: Some(max_depth),
            ..self
        }
    }

    /// Sets the `min_depth` option
    #[must_use]
    pub fn with_min_depth(self, min_depth: usize) -> Self {
        Self {
            min_depth: Some(min_depth),
            ..self
        }
    }

    /// Sets the `max_open` option which controls the maximum number of open file descriptors
    #[must_use]
    pub fn with_max_open(self, max_open: usize) -> Self {
        Self {
            max_open: Some(max_open),
            ..self
        }
    }

    /// Sets the `contents_first` option which produces directory contents before sub-directories
    #[must_use]
    pub fn with_contents_first(self, contents_first: bool) -> Self {
        Self {
            contents_first: Some(contents_first),
            ..self
        }
    }

    /// Sets the `follow_links` option which controls whether the walker follows symbolic links
    #[must_use]
    pub fn with_follow_links(self, follow_links: bool) -> Self {
        Self {
            follow_links: Some(follow_links),
            ..self
        }
    }
}

/// Creates a new `OpenOptions` with:
/// - `read` set to `true`
/// - `append` set to `false`
/// - `create` set to `false`
/// - `write` set to `false`
/// - `truncate` set to `false`
#[must_use]
pub fn read_only() -> std::fs::OpenOptions {
    std::fs::OpenOptions::new()
        .read(true)
        .append(false)
        .create(false)
        .write(false)
        .truncate(false)
        .clone()
}

/// Creates a new `OpenOptions` with:
/// - `read` set to `false`
/// - `write` set to `true`
/// - `append` set to `true`
/// - `create` set to `false`
/// - `truncate` set to `true`
#[must_use]
pub fn open_write_truncate() -> std::fs::OpenOptions {
    std::fs::OpenOptions::new()
        .read(false)
        .write(true)
        .append(true)
        .create(false)
        .truncate(true)
        .clone()
}

/// Creates a new `OpenOptions` with:
/// - `read` set to `false`
/// - `write` set to `true`
/// - `append` set to `true`
/// - `create` set to `false`
/// - `truncate` set to `false`
#[must_use]
pub fn open_write_append() -> std::fs::OpenOptions {
    std::fs::OpenOptions::new()
        .read(false)
        .write(true)
        .append(true)
        .create(false)
        .truncate(false)
        .clone()
}

/// Creates a new `OpenOptions` with:
/// - `read` set to `false`
/// - `write` set to `true`
/// - `append` set to `false`
/// - `create` set to `true`
/// - `truncate` set to `true`
#[must_use]
pub fn create_write_truncate() -> std::fs::OpenOptions {
    std::fs::OpenOptions::new()
        .read(false)
        .write(true)
        .append(false)
        .create(true)
        .truncate(true)
        .clone()
}

/// Creates a new `OpenOptions` with:
/// - `read` set to `false`
/// - `write` set to `true`
/// - `append` set to `true`
/// - `create` set to `true`
/// - `truncate` set to `false`
#[must_use]
pub fn create_write_append() -> std::fs::OpenOptions {
    std::fs::OpenOptions::new()
        .read(false)
        .write(true)
        .append(true)
        .create(true)
        .truncate(false)
        .clone()
}
