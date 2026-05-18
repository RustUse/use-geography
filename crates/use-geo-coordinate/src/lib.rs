#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::fmt;
use std::{error::Error, str::FromStr};

fn normalized_token(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace(['_', ' '], "-")
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GeoCoordinateError {
    LatitudeNotFinite,
    LatitudeOutOfRange,
    LongitudeNotFinite,
    LongitudeOutOfRange,
}

impl fmt::Display for GeoCoordinateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LatitudeNotFinite => formatter.write_str("latitude must be finite"),
            Self::LatitudeOutOfRange => {
                formatter.write_str("latitude must be within -90.0..=90.0 degrees")
            },
            Self::LongitudeNotFinite => formatter.write_str("longitude must be finite"),
            Self::LongitudeOutOfRange => {
                formatter.write_str("longitude must be within -180.0..=180.0 degrees")
            },
        }
    }
}

impl Error for GeoCoordinateError {}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Latitude(f64);

impl Latitude {
    pub fn new(value: f64) -> Result<Self, GeoCoordinateError> {
        if !value.is_finite() {
            return Err(GeoCoordinateError::LatitudeNotFinite);
        }

        if !(-90.0..=90.0).contains(&value) {
            return Err(GeoCoordinateError::LatitudeOutOfRange);
        }

        Ok(Self(value))
    }

    #[must_use]
    pub fn degrees(self) -> f64 {
        self.0
    }
}

impl fmt::Display for Latitude {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} deg", self.degrees())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Longitude(f64);

impl Longitude {
    pub fn new(value: f64) -> Result<Self, GeoCoordinateError> {
        if !value.is_finite() {
            return Err(GeoCoordinateError::LongitudeNotFinite);
        }

        if !(-180.0..=180.0).contains(&value) {
            return Err(GeoCoordinateError::LongitudeOutOfRange);
        }

        Ok(Self(value))
    }

    #[must_use]
    pub fn degrees(self) -> f64 {
        self.0
    }
}

impl fmt::Display for Longitude {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} deg", self.degrees())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GeoCoordinate {
    latitude: Latitude,
    longitude: Longitude,
}

impl GeoCoordinate {
    #[must_use]
    pub const fn new(latitude: Latitude, longitude: Longitude) -> Self {
        Self {
            latitude,
            longitude,
        }
    }

    #[must_use]
    pub const fn latitude(self) -> Latitude {
        self.latitude
    }

    #[must_use]
    pub const fn longitude(self) -> Longitude {
        self.longitude
    }
}

impl fmt::Display for GeoCoordinate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}, {}", self.latitude, self.longitude)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CoordinatePair(Latitude, Longitude);

impl CoordinatePair {
    #[must_use]
    pub const fn new(latitude: Latitude, longitude: Longitude) -> Self {
        Self(latitude, longitude)
    }

    #[must_use]
    pub const fn latitude(self) -> Latitude {
        self.0
    }

    #[must_use]
    pub const fn longitude(self) -> Longitude {
        self.1
    }
}

impl fmt::Display for CoordinatePair {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}, {}", self.0, self.1)
    }
}

impl From<CoordinatePair> for GeoCoordinate {
    fn from(pair: CoordinatePair) -> Self {
        Self::new(pair.latitude(), pair.longitude())
    }
}

impl From<GeoCoordinate> for CoordinatePair {
    fn from(coordinate: GeoCoordinate) -> Self {
        Self::new(coordinate.latitude(), coordinate.longitude())
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum CoordinateFormat {
    DecimalDegrees,
    DegreesMinutesSeconds,
    Unknown,
    Custom(String),
}

impl fmt::Display for CoordinateFormat {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DecimalDegrees => formatter.write_str("decimal-degrees"),
            Self::DegreesMinutesSeconds => formatter.write_str("degrees-minutes-seconds"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoordinateFormatParseError {
    Empty,
}

impl fmt::Display for CoordinateFormatParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("coordinate format cannot be empty"),
        }
    }
}

impl Error for CoordinateFormatParseError {}

impl FromStr for CoordinateFormat {
    type Err = CoordinateFormatParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(CoordinateFormatParseError::Empty);
        }

        Ok(match normalized_token(trimmed).as_str() {
            "decimal-degrees" => Self::DecimalDegrees,
            "degrees-minutes-seconds" | "dms" => Self::DegreesMinutesSeconds,
            "unknown" => Self::Unknown,
            _ => Self::Custom(trimmed.to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CoordinateFormat, CoordinateFormatParseError, CoordinatePair, GeoCoordinate,
        GeoCoordinateError, Latitude, Longitude,
    };

    #[test]
    fn valid_latitude() -> Result<(), GeoCoordinateError> {
        let latitude = Latitude::new(37.7749)?;

        assert_eq!(latitude.degrees(), 37.7749);
        Ok(())
    }

    #[test]
    fn invalid_latitude_rejected() {
        assert_eq!(
            Latitude::new(90.1),
            Err(GeoCoordinateError::LatitudeOutOfRange)
        );
    }

    #[test]
    fn valid_longitude() -> Result<(), GeoCoordinateError> {
        let longitude = Longitude::new(-122.4194)?;

        assert_eq!(longitude.degrees(), -122.4194);
        Ok(())
    }

    #[test]
    fn invalid_longitude_rejected() {
        assert_eq!(
            Longitude::new(-180.1),
            Err(GeoCoordinateError::LongitudeOutOfRange)
        );
    }

    #[test]
    fn coordinate_pair_construction() -> Result<(), GeoCoordinateError> {
        let latitude = Latitude::new(51.5074)?;
        let longitude = Longitude::new(-0.1278)?;
        let pair = CoordinatePair::new(latitude, longitude);
        let coordinate = GeoCoordinate::from(pair);

        assert_eq!(pair.latitude(), latitude);
        assert_eq!(pair.longitude(), longitude);
        assert_eq!(coordinate.latitude(), latitude);
        assert_eq!(coordinate.longitude(), longitude);
        assert_eq!(CoordinatePair::from(coordinate), pair);
        Ok(())
    }

    #[test]
    fn coordinate_format_display_parse() -> Result<(), CoordinateFormatParseError> {
        assert_eq!(
            CoordinateFormat::DecimalDegrees.to_string(),
            "decimal-degrees"
        );
        assert_eq!(
            "degrees minutes seconds".parse::<CoordinateFormat>()?,
            CoordinateFormat::DegreesMinutesSeconds
        );
        assert_eq!(
            "vendor-specific".parse::<CoordinateFormat>()?,
            CoordinateFormat::Custom(String::from("vendor-specific"))
        );
        Ok(())
    }
}
