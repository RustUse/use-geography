#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn non_empty_text(value: impl AsRef<str>) -> Result<String, VerticalReferenceTextError> {
    let trimmed = value.as_ref().trim();

    if trimmed.is_empty() {
        Err(VerticalReferenceTextError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

fn normalized_token(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace(['_', ' '], "-")
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ElevationValueError {
    NonFinite,
    NegativeDepth,
}

impl fmt::Display for ElevationValueError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFinite => formatter.write_str("elevation value must be finite"),
            Self::NegativeDepth => formatter.write_str("depth must be zero or greater"),
        }
    }
}

impl Error for ElevationValueError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ElevationDatumParseError {
    Empty,
}

impl fmt::Display for ElevationDatumParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("elevation datum cannot be empty"),
        }
    }
}

impl Error for ElevationDatumParseError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VerticalReferenceTextError {
    Empty,
}

impl fmt::Display for VerticalReferenceTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("vertical reference cannot be empty"),
        }
    }
}

impl Error for VerticalReferenceTextError {}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Elevation(f64);

impl Elevation {
    /// Creates an elevation value in meters.
    ///
    /// # Errors
    ///
    /// Returns [`ElevationValueError::NonFinite`] when the value is not finite.
    pub const fn new(value: f64) -> Result<Self, ElevationValueError> {
        if !value.is_finite() {
            return Err(ElevationValueError::NonFinite);
        }

        Ok(Self(value))
    }

    #[must_use]
    pub const fn meters(self) -> f64 {
        self.0
    }
}

impl fmt::Display for Elevation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} m", self.meters())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Depth(f64);

impl Depth {
    /// Creates a non-negative depth value in meters.
    ///
    /// # Errors
    ///
    /// Returns [`ElevationValueError::NonFinite`] when the value is not finite.
    /// Returns [`ElevationValueError::NegativeDepth`] when the value is negative.
    pub fn new(value: f64) -> Result<Self, ElevationValueError> {
        if !value.is_finite() {
            return Err(ElevationValueError::NonFinite);
        }

        if value < 0.0 {
            return Err(ElevationValueError::NegativeDepth);
        }

        Ok(Self(value))
    }

    #[must_use]
    pub const fn meters(self) -> f64 {
        self.0
    }
}

impl fmt::Display for Depth {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} m", self.meters())
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ElevationDatum {
    MeanSeaLevel,
    Ellipsoid,
    Geoid,
    Local,
    Unknown,
    Custom(String),
}

impl fmt::Display for ElevationDatum {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MeanSeaLevel => formatter.write_str("mean-sea-level"),
            Self::Ellipsoid => formatter.write_str("ellipsoid"),
            Self::Geoid => formatter.write_str("geoid"),
            Self::Local => formatter.write_str("local"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for ElevationDatum {
    type Err = ElevationDatumParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(ElevationDatumParseError::Empty);
        }

        Ok(match normalized_token(trimmed).as_str() {
            "mean-sea-level" | "msl" => Self::MeanSeaLevel,
            "ellipsoid" => Self::Ellipsoid,
            "geoid" => Self::Geoid,
            "local" => Self::Local,
            "unknown" => Self::Unknown,
            _ => Self::Custom(trimmed.to_string()),
        })
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct VerticalReference(String);

impl VerticalReference {
    /// Creates a vertical reference label from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`VerticalReferenceTextError::Empty`] when the trimmed value is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, VerticalReferenceTextError> {
        non_empty_text(value).map(Self)
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

impl AsRef<str> for VerticalReference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for VerticalReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for VerticalReference {
    type Err = VerticalReferenceTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::{Depth, Elevation, ElevationDatum, ElevationDatumParseError, ElevationValueError};

    #[test]
    fn positive_elevation() -> Result<(), ElevationValueError> {
        let elevation = Elevation::new(8848.86)?;

        assert!((elevation.meters() - 8848.86).abs() < f64::EPSILON);
        Ok(())
    }

    #[test]
    fn negative_elevation() -> Result<(), ElevationValueError> {
        let elevation = Elevation::new(-86.0)?;

        assert!((elevation.meters() - -86.0).abs() < f64::EPSILON);
        Ok(())
    }

    #[test]
    fn positive_depth() -> Result<(), ElevationValueError> {
        let depth = Depth::new(11_000.0)?;

        assert!((depth.meters() - 11_000.0).abs() < f64::EPSILON);
        Ok(())
    }

    #[test]
    fn elevation_datum_display_parse() -> Result<(), ElevationDatumParseError> {
        assert_eq!(ElevationDatum::Geoid.to_string(), "geoid");
        assert_eq!(
            "mean sea level".parse::<ElevationDatum>()?,
            ElevationDatum::MeanSeaLevel
        );
        Ok(())
    }

    #[test]
    fn custom_datum() -> Result<(), ElevationDatumParseError> {
        assert_eq!(
            "chart-datum".parse::<ElevationDatum>()?,
            ElevationDatum::Custom(String::from("chart-datum"))
        );
        Ok(())
    }
}
