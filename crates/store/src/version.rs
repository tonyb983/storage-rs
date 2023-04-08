//! `version` module defines types for versioning files and directories

mod wrapping {
    use serde::{Deserialize, Serialize};
    /// Simple incrementing version counter for files.
    ///
    /// **[`FileVersion`]s wrap when added / incremented, but saturate when subtracted/decremented.**
    ///
    /// [`FileVersion`]s should always have a non-zero value and the default value is 1.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
    pub struct FileVersion(u32);

    impl Default for FileVersion {
        fn default() -> Self {
            Self(1)
        }
    }

    impl FileVersion {
        /// An invalid [`FileVersion`] that has an inner value of zero.
        ///
        /// I'm not sure if this non-zero non-sense will be useful at all but I'm keeping it for now.
        pub const INVALID: Self = Self(0);

        /// Creates a new [`FileVersion`] with a version number of 1
        #[must_use]
        pub fn new() -> Self {
            Self::default()
        }

        /// Creates a new **invalid** [`FileVersion`]
        #[must_use]
        pub fn new_invalid() -> Self {
            Self::INVALID
        }

        /// Creates a new file version
        ///
        /// ## Panics
        /// Panics if `version` is zero
        #[must_use]
        pub(crate) fn new_with_version(version: u32) -> Self {
            // TODO: `assert` or `debug_assert`?
            assert!(
                version != 0,
                "attempting to create FileVersion with value of zero"
            );
            Self(version)
        }

        /// Checks if this [`FileVersion`] is valid.
        ///
        /// ***A [`FileVersion`] is valid if it is non-zero.***
        #[must_use]
        pub fn is_valid(&self) -> bool {
            self.0 != 0
        }

        /// Gets the version number.
        #[must_use]
        pub fn get(&self) -> u32 {
            self.0
        }

        /// Increment the version number by one. Rolls over to 1 if this value hits `u32::MAX`
        ///
        /// ## Panics
        /// Panics if called on an invalid [`FileVersion`] (i.e. one with a value of zero)
        pub fn increment(&mut self) {
            // TODO: `assert` or `debug_assert`?
            assert!(self.is_valid(), "cannot increment an invalid version!");
            *self += 1;
        }

        /// Increment the version number by `n`. Rolls over to 1 if this value hits `u32::MAX`
        ///
        /// ## Panics
        /// Panics if called on an invalid [`FileVersion`] (i.e. one with a value of zero)
        pub fn increment_n(&mut self, n: u32) {
            // TODO: `assert` or `debug_assert`?
            assert!(self.is_valid(), "cannot increment an invalid version!");
            *self += n;
        }
    }

    impl std::fmt::Display for FileVersion {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl PartialOrd<u32> for FileVersion {
        fn partial_cmp(&self, other: &u32) -> Option<std::cmp::Ordering> {
            self.0.partial_cmp(other)
        }
    }
    impl PartialEq<u32> for FileVersion {
        fn eq(&self, other: &u32) -> bool {
            self.0 == *other
        }
    }

    impl std::ops::Add for FileVersion {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            let wrapped = self.0.wrapping_add(rhs.0);
            if wrapped == 0 {
                Self::Output::new_with_version(1)
            } else {
                Self::Output::new_with_version(wrapped)
            }
        }
    }
    impl std::ops::Add<u32> for FileVersion {
        type Output = Self;

        fn add(self, rhs: u32) -> Self::Output {
            let wrapped = self.0.wrapping_add(rhs);
            if wrapped == 0 {
                Self::Output::new_with_version(1)
            } else {
                Self::Output::new_with_version(wrapped)
            }
        }
    }
    impl std::ops::AddAssign for FileVersion {
        fn add_assign(&mut self, rhs: Self) {
            let wrapped = self.0.wrapping_add(rhs.0);
            if wrapped == 0 {
                self.0 = 1;
            } else {
                self.0 = wrapped;
            }
        }
    }
    impl std::ops::AddAssign<u32> for FileVersion {
        fn add_assign(&mut self, rhs: u32) {
            let wrapped = self.0.wrapping_add(rhs);
            if wrapped == 0 {
                self.0 = 1;
            } else {
                self.0 = wrapped;
            }
        }
    }
    impl std::ops::Add<i32> for FileVersion {
        type Output = Self;

        fn add(self, rhs: i32) -> Self::Output {
            if !self.is_valid() {
                return self;
            }
            let value = self.0.wrapping_add_signed(rhs);
            if value == 0 {
                Self::Output::new_with_version(1)
            } else {
                Self::Output::new_with_version(value)
            }
        }
    }
    impl std::ops::AddAssign<i32> for FileVersion {
        fn add_assign(&mut self, rhs: i32) {
            if !self.is_valid() {
                return;
            }
            let value = self.0.wrapping_add_signed(rhs);
            if value == 0 {
                self.0 = 1;
            } else {
                self.0 = value;
            }
        }
    }

    impl std::ops::Sub for FileVersion {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            let sat = self.0.saturating_sub(rhs.0);
            if sat == 0 {
                Self::Output::new_with_version(1)
            } else {
                Self::Output::new_with_version(sat)
            }
        }
    }
    impl std::ops::Sub<u32> for FileVersion {
        type Output = Self;

        fn sub(self, rhs: u32) -> Self::Output {
            let sat = self.0.saturating_sub(rhs);
            if sat == 0 {
                Self::Output::new_with_version(1)
            } else {
                Self::Output::new_with_version(sat)
            }
        }
    }
    impl std::ops::SubAssign for FileVersion {
        fn sub_assign(&mut self, rhs: Self) {
            let result = self.0.saturating_sub(rhs.0);
            if result == 0 {
                self.0 = 1;
            } else {
                self.0 = result;
            }
        }
    }
    impl std::ops::SubAssign<u32> for FileVersion {
        fn sub_assign(&mut self, rhs: u32) {
            let result = self.0.saturating_sub(rhs);
            if result == 0 {
                self.0 = 1;
            } else {
                self.0 = result;
            }
        }
    }
    impl std::ops::Sub<i32> for FileVersion {
        type Output = Self;

        fn sub(self, rhs: i32) -> Self::Output {
            if !self.is_valid() {
                return self;
            }
            let value = self.0.wrapping_add_signed(-rhs);
            if value == 0 {
                Self::Output::new_with_version(1)
            } else {
                Self::Output::new_with_version(value)
            }
        }
    }
    impl std::ops::SubAssign<i32> for FileVersion {
        fn sub_assign(&mut self, rhs: i32) {
            if !self.is_valid() {
                return;
            }
            let value = self.0.wrapping_add_signed(-rhs);
            if value == 0 {
                self.0 = 1;
            } else {
                self.0 = value;
            }
        }
    }
}

mod saturating {
    use serde::{Deserialize, Serialize};
    /// Simple incrementing version counter for files.
    ///
    /// **[`FileVersion`]s saturates on `1` and `u32::MAX`**
    ///
    /// [`FileVersion`]s should always have a non-zero value and the default value is 1.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
    pub struct FileVersion(u32);

    impl Default for FileVersion {
        fn default() -> Self {
            Self(1)
        }
    }

    impl FileVersion {
        /// An invalid [`FileVersion`] that has an inner value of zero.
        ///
        /// I'm not sure if this non-zero non-sense will be useful at all but I'm keeping it for now.
        pub const INVALID: Self = Self(0);

        /// Creates a new [`FileVersion`] with a version number of 1
        #[must_use]
        pub fn new() -> Self {
            Self::default()
        }

        /// Creates a new **invalid** [`FileVersion`]
        #[must_use]
        pub fn new_invalid() -> Self {
            Self::INVALID
        }

        /// Creates a new file version
        ///
        /// ## Panics
        /// Panics if `version` is zero
        #[must_use]
        pub(crate) fn new_with_version(version: u32) -> Self {
            // TODO: `assert` or `debug_assert`?
            assert!(
                version != 0,
                "attempting to create FileVersion with value of zero"
            );
            Self(version)
        }

        /// Checks if this [`FileVersion`] is valid.
        ///
        /// ***A [`FileVersion`] is valid if it is non-zero.***
        #[must_use]
        pub fn is_valid(&self) -> bool {
            self.0 != 0
        }

        /// Gets the version number.
        ///
        /// Same as [`FileVersion::value`]
        #[must_use]
        pub fn get(&self) -> u32 {
            self.0
        }

        /// Gets the version number.
        ///
        /// Same as [`FileVersion::get`]
        #[must_use]
        pub fn value(&self) -> u32 {
            self.0
        }

        /// Increment the version number by one. Saturates if the inner value hits `u32::MAX`
        ///
        /// ## Panics
        /// Panics if called on an invalid [`FileVersion`] (i.e. one with a value of zero)
        pub fn increment(&mut self) {
            // TODO: `assert` or `debug_assert`?
            assert!(self.is_valid(), "cannot increment an invalid version!");
            *self += 1u32;
        }

        /// Increment the version number by `n`. Saturates if the inner value hits `u32::MAX`
        ///
        /// ## Panics
        /// Panics if called on an invalid [`FileVersion`] (i.e. one with a value of zero)
        pub fn increment_n(&mut self, n: u32) {
            // TODO: `assert` or `debug_assert`?
            assert!(self.is_valid(), "cannot increment an invalid version!");
            *self += n;
        }
    }

    impl std::fmt::Display for FileVersion {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl PartialOrd<u32> for FileVersion {
        fn partial_cmp(&self, other: &u32) -> Option<std::cmp::Ordering> {
            self.0.partial_cmp(other)
        }
    }
    impl PartialEq<u32> for FileVersion {
        fn eq(&self, other: &u32) -> bool {
            self.0 == *other
        }
    }

    impl std::ops::Add for FileVersion {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            if !self.is_valid() || !rhs.is_valid() {
                return self;
            }
            let value = self.0.saturating_add(rhs.0);
            if value == 0 {
                Self::Output::new_with_version(1)
            } else {
                Self::Output::new_with_version(value)
            }
        }
    }
    impl std::ops::Add<u32> for FileVersion {
        type Output = Self;

        fn add(self, rhs: u32) -> Self::Output {
            if !self.is_valid() {
                return self;
            }
            let value = self.0.saturating_add(rhs);
            if value == 0 {
                Self::Output::new_with_version(1)
            } else {
                Self::Output::new_with_version(value)
            }
        }
    }
    impl std::ops::AddAssign for FileVersion {
        fn add_assign(&mut self, rhs: Self) {
            if !self.is_valid() || !rhs.is_valid() {
                return;
            }
            let value = self.0.saturating_add(rhs.0);
            if value == 0 {
                self.0 = 1;
            } else {
                self.0 = value;
            }
        }
    }
    impl std::ops::AddAssign<u32> for FileVersion {
        fn add_assign(&mut self, rhs: u32) {
            if !self.is_valid() {
                return;
            }
            let value = self.0.saturating_add(rhs);
            if value == 0 {
                self.0 = 1;
            } else {
                self.0 = value;
            }
        }
    }
    impl std::ops::Add<i32> for FileVersion {
        type Output = Self;

        fn add(self, rhs: i32) -> Self::Output {
            if !self.is_valid() {
                return self;
            }
            let value = self.0.saturating_add_signed(rhs);
            if value == 0 {
                Self::Output::new_with_version(1)
            } else {
                Self::Output::new_with_version(value)
            }
        }
    }
    impl std::ops::AddAssign<i32> for FileVersion {
        fn add_assign(&mut self, rhs: i32) {
            if !self.is_valid() {
                return;
            }
            let value = self.0.saturating_add_signed(rhs);
            if value == 0 {
                self.0 = 1;
            } else {
                self.0 = value;
            }
        }
    }

    impl std::ops::Sub for FileVersion {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            if !self.is_valid() || !rhs.is_valid() {
                return self;
            }
            let value = self.0.saturating_sub(rhs.0);
            if value == 0 {
                Self::Output::new_with_version(1)
            } else {
                Self::Output::new_with_version(value)
            }
        }
    }
    impl std::ops::Sub<u32> for FileVersion {
        type Output = Self;

        fn sub(self, rhs: u32) -> Self::Output {
            if !self.is_valid() {
                return self;
            }
            let sat = self.0.saturating_sub(rhs);
            if sat == 0 {
                Self::Output::new_with_version(1)
            } else {
                Self::Output::new_with_version(sat)
            }
        }
    }
    impl std::ops::SubAssign for FileVersion {
        fn sub_assign(&mut self, rhs: Self) {
            if !self.is_valid() || !rhs.is_valid() {
                return;
            }
            let value = self.0.saturating_sub(rhs.0);
            if value == 0 {
                self.0 = 1;
            } else {
                self.0 = value;
            }
        }
    }
    impl std::ops::SubAssign<u32> for FileVersion {
        fn sub_assign(&mut self, rhs: u32) {
            if !self.is_valid() {
                return;
            }
            let value = self.0.saturating_sub(rhs);
            if value == 0 {
                self.0 = 1;
            } else {
                self.0 = value;
            }
        }
    }
    impl std::ops::Sub<i32> for FileVersion {
        type Output = Self;

        fn sub(self, rhs: i32) -> Self::Output {
            if !self.is_valid() {
                return self;
            }
            let value = self.0.saturating_add_signed(-rhs);
            if value == 0 {
                Self::Output::new_with_version(1)
            } else {
                Self::Output::new_with_version(value)
            }
        }
    }
    impl std::ops::SubAssign<i32> for FileVersion {
        fn sub_assign(&mut self, rhs: i32) {
            if !self.is_valid() {
                return;
            }
            let value = self.0.saturating_add_signed(-rhs);
            if value == 0 {
                self.0 = 1;
            } else {
                self.0 = value;
            }
        }
    }
}

pub use saturating::FileVersion as SaturatingFileVersion;
pub use wrapping::FileVersion as WrappingFileVersion;
