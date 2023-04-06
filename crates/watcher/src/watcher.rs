// Copyright (c) 2023 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
use super::{Config, Result};

use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use crossbeam_channel::{unbounded, Receiver};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};

pub type NotifyEvent = Result<notify::Event, notify::Error>;

/// A [`FileWatcher`](super::FileWatcher) implementation using the [`notify`] crate
#[derive(Debug)]
pub struct NotifyWatcher {
    events: Receiver<NotifyEvent>,
    notify_config: notify::Config,
    is_watching: bool,
    watcher: RecommendedWatcher,
    watched_files: Arc<Mutex<Vec<String>>>,
}

impl NotifyWatcher {
    pub fn new() -> Result<Self> {
        let (tx, rx) = unbounded();
        let config = notify::Config::default().with_poll_interval(Duration::from_secs(5));
        let watcher = notify::RecommendedWatcher::new(tx, config)?;
        let watched_files = Arc::new(Mutex::new(Vec::new()));

        let file_watcher = Self {
            events: rx,
            is_watching: false,
            notify_config: config,
            watcher,
            watched_files,
        };

        Ok(file_watcher)
    }

    pub fn watched_files(&self) -> Vec<String> {
        let files = match self.watched_files.lock() {
            Ok(files) => files,
            Err(e) => {
                println!("poisoned mutex: {e:?}");
                return Vec::new();
            }
        };
        files.clone()
    }

    pub fn update_watched_files(&mut self, files: Vec<String>) -> Result<()> {
        let currently_watching = self.is_watching;
        if currently_watching {
            self.stop_watch()?;
        }

        self.watched_files.lock().expect("mutex poisoned").clear();
        self.watched_files = Arc::new(Mutex::new(files));
        if currently_watching {
            self.start_watch()?;
        }

        Ok(())
    }

    pub fn is_watching(&self) -> bool {
        self.is_watching
    }

    pub fn set_poll_interval(&mut self, millis: u64) -> Result<(), notify::Error> {
        self.notify_config = self
            .notify_config
            .with_poll_interval(Duration::from_millis(millis));
        self.watcher.configure(self.notify_config)?;
        Ok(())
    }

    pub fn set_compare_contents(&mut self, compare: bool) -> Result<(), notify::Error> {
        self.notify_config = self.notify_config.with_compare_contents(compare);
        self.watcher.configure(self.notify_config)?;
        Ok(())
    }

    pub fn event_stream(&self) -> &Receiver<NotifyEvent> {
        &self.events
    }

    pub(crate) fn inner_watcher(&self) -> &RecommendedWatcher {
        &self.watcher
    }

    fn start_watch(&mut self) -> Result<()> {
        if self.is_watching {
            return Ok(());
        }
        for file in self.watched_files.lock().expect("mutex poisoned").iter() {
            self.watcher
                .watch(std::path::Path::new(file), RecursiveMode::NonRecursive)?;
        }

        self.is_watching = true;
        Ok(())
    }

    fn stop_watch(&mut self) -> Result<()> {
        if !self.is_watching {
            return Ok(());
        }
        for file in self.watched_files.lock().expect("mutex poisoned").iter() {
            self.watcher.unwatch(std::path::Path::new(file))?;
        }
        self.is_watching = false;
        Ok(())
    }
}

impl super::FileWatcher for NotifyWatcher {
    type InnerConfig = notify::Config;

    fn currently_watched(&self) -> Result<Vec<String>> {
        Ok(self.watched_files())
    }

    fn apply_app_config(&mut self, config: &Config) -> Result {
        let file_list = config.read_tracked_files()?;
        self.update_watched_files(file_list)?;
        self.watcher.configure(
            notify::Config::default()
                .with_poll_interval(std::time::Duration::from_millis(config.delay())),
        )?;
        Ok(())
    }

    fn start(&mut self) -> Result {
        self.start_watch()
    }

    fn stop(&mut self) -> Result {
        self.stop_watch()
    }

    fn start_with_config(
        &mut self,
        app_config: &Config,
        impl_config: &Self::InnerConfig,
    ) -> Result {
        self.apply_app_config(app_config)?;
        self.apply_inner_config(impl_config)?;
        self.start()
    }

    fn start_with_app_config(&mut self, config: &Config) -> Result {
        self.apply_app_config(config)?;
        self.start()
    }

    fn apply_inner_config(&mut self, config: &Self::InnerConfig) -> Result {
        self.watcher.configure(*config)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::FileWatcher;
    use super::*;

    fn setup_test_directory() -> tempfile::TempDir {
        let temp_dir = tempfile::tempdir().expect("failed to create temp dir");
        let temp_dir_path = temp_dir.path();
        std::fs::write(temp_dir_path.join("file1.txt"), "test").expect("failed to write test file");
        std::fs::write(temp_dir_path.join("file2.txt"), "test").expect("failed to write test file");
        temp_dir
    }

    #[test]
    fn watcher_test() {
        use std::io::Write;
        let temp = setup_test_directory();

        assert!(temp.path().exists());
        assert!(temp.path().join("file1.txt").exists());
        assert!(temp.path().join("file2.txt").exists());

        let mut watcher = NotifyWatcher::new().expect("failed to create watcher");
        watcher
            .set_poll_interval(100)
            .expect("unable to set polling interval");
        // watcher
        //     .set_compare_contents(true)
        //     .expect("unable to set polling interval");
        watcher
            .update_watched_files(vec![temp.path().to_str().unwrap().to_string()])
            .expect("unable to update watched files");
        watcher.start().expect("unable to start watcher");

        let file1 = temp.path().join("file1.txt");
        let file2 = temp.path().join("file2.txt");
        let new_file = temp.path().join("file3.txt");
        let rename_file = temp.path().join("file100.txt");

        let bg = std::thread::spawn(move || {
            println!("starting bg");
            std::thread::sleep(std::time::Duration::from_millis(50));
            let mut file = std::fs::File::create(&file1).expect("failed to open file1");
            write!(file, "hello world").expect("unable to write to file1");
            let mut file = std::fs::File::create(new_file).expect("failed to open file1");
            write!(file, "hello world").expect("unable to write to file1");
            std::fs::rename(file2, rename_file).expect("unable to rename file");
            xstd::fs::touch(&file1).expect("touch file1 failed");
        });

        let counter = std::thread::spawn(move || {
            println!("starting counter");
            std::thread::sleep(std::time::Duration::from_millis(100));
            let mut counter = 0usize;

            for _ in 0..25 {
                let event = match watcher.event_stream().try_recv() {
                    Ok(event) => event,
                    Err(err) => {
                        println!("channel error - {err:?}");
                        break;
                    }
                };

                match event {
                    Ok(event) => {
                        println!("event - {event:?}");
                        counter += 1;
                    }
                    Err(err) => {
                        println!("event error - {err:?}");
                        continue;
                    }
                }
            }

            counter
        });

        std::thread::sleep(std::time::Duration::from_millis(100));

        bg.join().expect("unable to join file-io thread");
        let event_count = counter.join().expect("unable to join counter thread");
        println!("event count: {}", event_count);
        assert_ne!(event_count, 0, "at least one event should be received");
    }
}
