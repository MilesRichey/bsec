//! Provides a fake [`BmeSensor`] implementation to be used in unit tests.
//!
//! This module is only available if the **test-support** feature is enabled.

use super::*;
use core::fmt;

#[cfg(feature = "alloc")]
use alloc::vec;

/// "Unit" error type with only a single variant.
#[derive(Copy, Clone, Debug)]
pub struct UnitError;

#[cfg(feature = "std")]
impl std::error::Error for UnitError {}

impl fmt::Display for UnitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_fmt(format_args!("{:?}", self))
    }
}

/// Fake [`BmeSensor`] implementation.
///
/// Stores a single constant measurement that will always be returned.
pub struct FakeBmeSensor {
    measurement: nb::Result<Vec<Input>, UnitError>,
}

impl FakeBmeSensor {
    /// Create a new [`FakeBmeSensor`] always returning `measurement`.
    pub fn new(measurement: nb::Result<Vec<Input>, UnitError>) -> Self {
        Self { measurement }
    }
}

impl Default for FakeBmeSensor {
    fn default() -> Self {
        Self::new(Ok(vec![]))
    }
}

impl BmeSensor for FakeBmeSensor {
    type Error = UnitError;

    fn start_measurement(&mut self, _: &BmeSettingsHandle<'_>) -> Result<Duration, UnitError> {
        Ok(Duration::new(0, 0))
    }

    fn get_measurement(&mut self) -> nb::Result<Vec<Input>, UnitError> {
        self.measurement.clone()
    }
}
