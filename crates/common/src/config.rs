//! # Storage-Config
//!
//!  A module for handling configuration files (e.g., TOML, YAML, or JSON).
//!  This will store the list of monitored files/directories, backup settings,
//!  and other app configurations.

/// The main configuration used by the application but with optional fields
#[derive(Debug, Clone, Default)]
pub struct MaybeConfig {
    delay: Option<u64>,
    app_dir: Option<String>,
    store_dir: Option<String>,
    tracking_list: Option<String>,
}

/// The main configuration used by the application
#[derive(Debug, Clone)]
pub struct Config {
    delay: u64,
    app_dir: String,
    store_dir: String,
    tracking_list: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            delay: 1000,
            app_dir: String::from("~/.storage-app-data"),
            store_dir: String::from("~/.storage-app-data/.store"),
            tracking_list: String::from("~/.storage-app-store/tracking_list.json"),
        }
    }
}

impl Config {
    /// Creates a new default config
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Gets file watcher delay (in milliseconds)
    #[must_use]
    pub fn delay(&self) -> u64 {
        self.delay
    }

    /// Gets the path to the main application directory as a string
    #[must_use]
    pub fn app_dir(&self) -> &str {
        &self.app_dir
    }

    /// Gets the path to the main application directory
    #[must_use]
    pub fn app_dir_path(&self) -> &std::path::Path {
        std::path::Path::new(self.app_dir())
    }

    /// Gets the path to the storage directory as a string
    #[must_use]
    pub fn store_dir(&self) -> &str {
        &self.store_dir
    }

    /// Gets the path to the storage directory
    #[must_use]
    pub fn store_dir_path(&self) -> &std::path::Path {
        std::path::Path::new(self.store_dir())
    }

    /// Gets the path to the tracking list file as a string
    #[must_use]
    pub fn tracking_list(&self) -> &str {
        &self.tracking_list
    }

    /// Gets the path to the tracking list file
    #[must_use]
    pub fn tracking_list_path(&self) -> &std::path::Path {
        std::path::Path::new(self.tracking_list())
    }

    /// Converts this config into a [`MaybeConfig`]
    #[must_use]
    pub fn into_maybe(self) -> MaybeConfig {
        MaybeConfig {
            delay: Some(self.delay),
            app_dir: Some(self.app_dir),
            store_dir: Some(self.store_dir),
            tracking_list: Some(self.tracking_list),
        }
    }

    /// Combines this config overwriting any values that are set in `other`
    #[must_use]
    pub fn extend_with(&self, other: &MaybeConfig) -> Self {
        let mut new = self.clone();
        if let Some(delay) = other.delay {
            new.delay = delay;
        }
        if let Some(app_dir) = &other.app_dir {
            new.app_dir = app_dir.clone();
        }
        if let Some(store_dir) = &other.store_dir {
            new.store_dir = store_dir.clone();
        }
        if let Some(tracking_list) = &other.tracking_list {
            new.tracking_list = tracking_list.clone();
        }
        new
    }

    // TODO: This should be a serialiized list of files and loaded through serde instead of plaintext
    /// Reads the tracking list file and returns a list of files/directories to track
    ///
    /// ## Errors
    /// Errors if the tracking list file cannot be opened or read
    pub fn read_tracked_files(&self) -> super::Result<Vec<String>> {
        use std::io::BufRead;
        let mut files = Vec::new();
        let file = std::fs::File::open(self.tracking_list_path())?;
        let reader = std::io::BufReader::new(file);
        for line in reader.lines() {
            files.push(line?);
        }
        Ok(files)
    }

    /// Initializing the application folder, creating the main directory if it does not exist,
    /// the storage directory if it does not exist, and the tracking list file if it does not exist
    ///
    /// ## Errors
    /// Errors if any call to `std::fs::create_dir_all` or `std::fs::File::create` fails
    pub fn init_app_structure(&self) -> super::Result {
        use std::io::Write;
        if !self.app_dir_path().exists() {
            std::fs::create_dir_all(self.app_dir_path())?;
        }
        if !self.store_dir_path().exists() {
            std::fs::create_dir_all(self.store_dir_path())?;
        }
        if !self.tracking_list_path().exists() {
            let mut tracking_file = std::fs::File::create(self.tracking_list_path())?;
            tracking_file.write_all(b"{}")?;
        }

        Ok(())
    }
}
