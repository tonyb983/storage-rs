//! # Storage-Config
//!
//!  A module for handling configuration files (e.g., TOML, YAML, or JSON).
//!  This will store the list of monitored files/directories, backup settings,
//!  and other app configurations.

#[derive(Debug, Clone, Default)]
pub struct MaybeConfig {
    delay: Option<u64>,
    app_dir: Option<String>,
    store_dir: Option<String>,
    tracking_list: Option<String>,
}

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
    pub fn new() -> Self {
        Self::default()
    }

    pub fn delay(&self) -> u64 {
        self.delay
    }

    pub fn app_dir(&self) -> &str {
        &self.app_dir
    }

    pub fn app_dir_path(&self) -> &std::path::Path {
        std::path::Path::new(self.app_dir())
    }

    pub fn store_dir(&self) -> &str {
        &self.store_dir
    }

    pub fn store_dir_path(&self) -> &std::path::Path {
        std::path::Path::new(self.store_dir())
    }

    pub fn tracking_list(&self) -> &str {
        &self.tracking_list
    }

    pub fn tracking_list_path(&self) -> &std::path::Path {
        std::path::Path::new(self.tracking_list())
    }

    pub fn into_maybe(self) -> MaybeConfig {
        MaybeConfig {
            delay: Some(self.delay),
            app_dir: Some(self.app_dir),
            store_dir: Some(self.store_dir),
            tracking_list: Some(self.tracking_list),
        }
    }

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
            tracking_file.write_all(b"")?;
        }

        Ok(())
    }
}
