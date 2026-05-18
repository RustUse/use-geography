#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn non_empty_text(value: impl AsRef<str>) -> Result<String, GeographicRegionTextError> {
    let trimmed = value.as_ref().trim();

    if trimmed.is_empty() {
        Err(GeographicRegionTextError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

fn normalized_token(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace(['_', ' '], "-")
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GeographicRegionTextError {
    Empty,
}

impl fmt::Display for GeographicRegionTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("geographic region text cannot be empty"),
        }
    }
}

impl Error for GeographicRegionTextError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GeographicRegionKindParseError {
    Empty,
}

impl fmt::Display for GeographicRegionKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("geographic region kind cannot be empty"),
        }
    }
}

impl Error for GeographicRegionKindParseError {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GeographicRegionName(String);

impl GeographicRegionName {
    pub fn new(value: impl AsRef<str>) -> Result<Self, GeographicRegionTextError> {
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

impl AsRef<str> for GeographicRegionName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for GeographicRegionName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for GeographicRegionName {
    type Err = GeographicRegionTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GeographicRegionKind {
    Continent,
    Country,
    Administrative,
    Political,
    Cultural,
    Natural,
    Climate,
    Economic,
    Watershed,
    Biome,
    Unknown,
    Custom(String),
}

impl fmt::Display for GeographicRegionKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Continent => formatter.write_str("continent"),
            Self::Country => formatter.write_str("country"),
            Self::Administrative => formatter.write_str("administrative"),
            Self::Political => formatter.write_str("political"),
            Self::Cultural => formatter.write_str("cultural"),
            Self::Natural => formatter.write_str("natural"),
            Self::Climate => formatter.write_str("climate"),
            Self::Economic => formatter.write_str("economic"),
            Self::Watershed => formatter.write_str("watershed"),
            Self::Biome => formatter.write_str("biome"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for GeographicRegionKind {
    type Err = GeographicRegionKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(GeographicRegionKindParseError::Empty);
        }

        Ok(match normalized_token(trimmed).as_str() {
            "continent" => Self::Continent,
            "country" => Self::Country,
            "administrative" => Self::Administrative,
            "political" => Self::Political,
            "cultural" => Self::Cultural,
            "natural" => Self::Natural,
            "climate" => Self::Climate,
            "economic" => Self::Economic,
            "watershed" => Self::Watershed,
            "biome" => Self::Biome,
            "unknown" => Self::Unknown,
            _ => Self::Custom(trimmed.to_string()),
        })
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GeographicRegionId(String);

impl GeographicRegionId {
    pub fn new(value: impl AsRef<str>) -> Result<Self, GeographicRegionTextError> {
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

impl AsRef<str> for GeographicRegionId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for GeographicRegionId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for GeographicRegionId {
    type Err = GeographicRegionTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        GeographicRegionId, GeographicRegionKind, GeographicRegionKindParseError,
        GeographicRegionName, GeographicRegionTextError,
    };

    #[test]
    fn valid_region_name() -> Result<(), GeographicRegionTextError> {
        let region_name = GeographicRegionName::new("Andean Highlands")?;

        assert_eq!(region_name.as_str(), "Andean Highlands");
        Ok(())
    }

    #[test]
    fn empty_region_name_rejected() {
        assert_eq!(
            GeographicRegionName::new("   "),
            Err(GeographicRegionTextError::Empty)
        );
    }

    #[test]
    fn region_kind_display_parse() -> Result<(), GeographicRegionKindParseError> {
        assert_eq!(GeographicRegionKind::Watershed.to_string(), "watershed");
        assert_eq!(
            "administrative".parse::<GeographicRegionKind>()?,
            GeographicRegionKind::Administrative
        );
        Ok(())
    }

    #[test]
    fn custom_region_kind() -> Result<(), GeographicRegionKindParseError> {
        assert_eq!(
            "ecoregion".parse::<GeographicRegionKind>()?,
            GeographicRegionKind::Custom(String::from("ecoregion"))
        );
        Ok(())
    }

    #[test]
    fn region_id_construction() -> Result<(), GeographicRegionTextError> {
        let region_id = GeographicRegionId::new("andean-highlands")?;

        assert_eq!(region_id.as_str(), "andean-highlands");
        Ok(())
    }
}
