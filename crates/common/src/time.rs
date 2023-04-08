// Copyright (c) 2023 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Simple timestamp type for abstracting away various time concerns
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct Timestamp(u64);

impl Timestamp {
    /// Creates a new timestamp representing the current time. Convenience function
    /// that calls [`current_timestamp`]
    #[must_use]
    pub fn now() -> Self {
        current_timestamp()
    }

    /// Creates a timestamp from the given number of seconds
    #[must_use]
    pub fn new(secs: u64) -> Self {
        Self(secs)
    }

    /// The number of seconds that this timestamp represents
    #[must_use]
    pub fn as_secs(self) -> u64 {
        self.0
    }

    /// Creates a duration from this timestamp
    #[must_use]
    pub fn as_duration(self) -> Duration {
        Duration::from_secs(self.0)
    }

    /// Creates a system time from this timestamp
    #[must_use]
    pub fn as_system_time(self) -> SystemTime {
        UNIX_EPOCH + Duration::from_secs(self.0)
    }
}

impl From<Duration> for Timestamp {
    fn from(duration: Duration) -> Self {
        Self(duration.as_secs())
    }
}
impl From<SystemTime> for Timestamp {
    fn from(time: SystemTime) -> Self {
        Self(
            time.duration_since(UNIX_EPOCH)
                .expect("system time before unix epoch")
                .as_secs(),
        )
    }
}

/// Creates a new timestamp representing the current time
#[must_use]
pub fn current_timestamp() -> Timestamp {
    let secs_since_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("unable to get time since epoch")
        .as_secs();
    Timestamp(secs_since_epoch)
}
