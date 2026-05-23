#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn non_empty_text(value: impl AsRef<str>) -> Result<String, SpatialReferenceTextError> {
    let trimmed = value.as_ref().trim();

    if trimmed.is_empty() {
        Err(SpatialReferenceTextError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialReferenceTextError {
    Empty,
}

impl fmt::Display for SpatialReferenceTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("spatial reference text cannot be empty"),
        }
    }
}

impl Error for SpatialReferenceTextError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialReferenceValueError {
    ZeroEpsgCode,
}

impl fmt::Display for SpatialReferenceValueError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroEpsgCode => formatter.write_str("EPSG code must be greater than zero"),
        }
    }
}

impl Error for SpatialReferenceValueError {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SpatialReferenceId(String);

impl SpatialReferenceId {
    /// Creates a spatial reference identifier from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`SpatialReferenceTextError::Empty`] when the trimmed value is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, SpatialReferenceTextError> {
        non_empty_text(value).map(Self)
    }

    #[must_use]
    pub fn from_epsg(code: EpsgCode) -> Self {
        Self(code.to_string())
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for SpatialReferenceId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for SpatialReferenceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for SpatialReferenceId {
    type Err = SpatialReferenceTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EpsgCode(u32);

impl EpsgCode {
    /// Creates an EPSG code from a non-zero integer.
    ///
    /// # Errors
    ///
    /// Returns [`SpatialReferenceValueError::ZeroEpsgCode`] when `value` is zero.
    pub const fn new(value: u32) -> Result<Self, SpatialReferenceValueError> {
        if value == 0 {
            return Err(SpatialReferenceValueError::ZeroEpsgCode);
        }

        Ok(Self(value))
    }

    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

impl fmt::Display for EpsgCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "EPSG:{}", self.get())
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SpatialReferenceSystem {
    identifier: SpatialReferenceId,
}

impl SpatialReferenceSystem {
    #[must_use]
    pub const fn new(identifier: SpatialReferenceId) -> Self {
        Self { identifier }
    }

    #[must_use]
    pub const fn identifier(&self) -> &SpatialReferenceId {
        &self.identifier
    }
}

impl fmt::Display for SpatialReferenceSystem {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.identifier.fmt(formatter)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CoordinateReferenceSystem {
    identifier: SpatialReferenceId,
}

impl CoordinateReferenceSystem {
    #[must_use]
    pub const fn new(identifier: SpatialReferenceId) -> Self {
        Self { identifier }
    }

    #[must_use]
    pub fn from_epsg(code: EpsgCode) -> Self {
        Self::new(SpatialReferenceId::from_epsg(code))
    }

    #[must_use]
    pub fn from_system(system: SpatialReferenceSystem) -> Self {
        Self::new(system.identifier)
    }

    #[must_use]
    pub const fn identifier(&self) -> &SpatialReferenceId {
        &self.identifier
    }
}

impl fmt::Display for CoordinateReferenceSystem {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.identifier.fmt(formatter)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CoordinateReferenceSystem, EpsgCode, SpatialReferenceId, SpatialReferenceTextError,
        SpatialReferenceValueError,
    };

    #[test]
    fn valid_epsg_code() -> Result<(), SpatialReferenceValueError> {
        let epsg = EpsgCode::new(3857)?;

        assert_eq!(epsg.get(), 3857);
        Ok(())
    }

    #[test]
    fn zero_epsg_code_rejected() {
        assert_eq!(
            EpsgCode::new(0),
            Err(SpatialReferenceValueError::ZeroEpsgCode)
        );
    }

    #[test]
    fn spatial_reference_id_construction() -> Result<(), SpatialReferenceTextError> {
        let identifier = SpatialReferenceId::new("WGS84")?;

        assert_eq!(identifier.as_str(), "WGS84");
        Ok(())
    }

    #[test]
    fn empty_spatial_reference_id_rejected() {
        assert_eq!(
            SpatialReferenceId::new("   "),
            Err(SpatialReferenceTextError::Empty)
        );
    }

    #[test]
    fn crs_identifier_display() -> Result<(), SpatialReferenceValueError> {
        let crs = CoordinateReferenceSystem::from_epsg(EpsgCode::new(4326)?);

        assert_eq!(crs.to_string(), "EPSG:4326");
        Ok(())
    }
}
