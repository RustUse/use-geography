#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn non_empty_text(value: impl AsRef<str>) -> Result<String, PlaceTextError> {
    let trimmed = value.as_ref().trim();

    if trimmed.is_empty() {
        Err(PlaceTextError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

fn normalized_token(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace(['_', ' '], "-")
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlaceTextError {
    Empty,
}

impl fmt::Display for PlaceTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("place text cannot be empty"),
        }
    }
}

impl Error for PlaceTextError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlaceKindParseError {
    Empty,
}

impl fmt::Display for PlaceKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("place kind cannot be empty"),
        }
    }
}

impl Error for PlaceKindParseError {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PlaceName(String);

impl PlaceName {
    pub fn new(value: impl AsRef<str>) -> Result<Self, PlaceTextError> {
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

impl AsRef<str> for PlaceName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for PlaceName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PlaceName {
    type Err = PlaceTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PlaceKind {
    Continent,
    Country,
    Region,
    State,
    Province,
    County,
    City,
    Town,
    Village,
    Neighborhood,
    Landmark,
    NaturalFeature,
    Unknown,
    Custom(String),
}

impl fmt::Display for PlaceKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Continent => formatter.write_str("continent"),
            Self::Country => formatter.write_str("country"),
            Self::Region => formatter.write_str("region"),
            Self::State => formatter.write_str("state"),
            Self::Province => formatter.write_str("province"),
            Self::County => formatter.write_str("county"),
            Self::City => formatter.write_str("city"),
            Self::Town => formatter.write_str("town"),
            Self::Village => formatter.write_str("village"),
            Self::Neighborhood => formatter.write_str("neighborhood"),
            Self::Landmark => formatter.write_str("landmark"),
            Self::NaturalFeature => formatter.write_str("natural-feature"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for PlaceKind {
    type Err = PlaceKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(PlaceKindParseError::Empty);
        }

        Ok(match normalized_token(trimmed).as_str() {
            "continent" => Self::Continent,
            "country" => Self::Country,
            "region" => Self::Region,
            "state" => Self::State,
            "province" => Self::Province,
            "county" => Self::County,
            "city" => Self::City,
            "town" => Self::Town,
            "village" => Self::Village,
            "neighborhood" | "neighbourhood" => Self::Neighborhood,
            "landmark" => Self::Landmark,
            "natural-feature" => Self::NaturalFeature,
            "unknown" => Self::Unknown,
            _ => Self::Custom(trimmed.to_string()),
        })
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PlaceId(String);

impl PlaceId {
    pub fn new(value: impl AsRef<str>) -> Result<Self, PlaceTextError> {
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

impl AsRef<str> for PlaceId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for PlaceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PlaceId {
    type Err = PlaceTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::{PlaceId, PlaceKind, PlaceKindParseError, PlaceName, PlaceTextError};

    #[test]
    fn valid_place_name() -> Result<(), PlaceTextError> {
        let place_name = PlaceName::new("Quito")?;

        assert_eq!(place_name.as_str(), "Quito");
        Ok(())
    }

    #[test]
    fn empty_place_name_rejected() {
        assert_eq!(PlaceName::new("   "), Err(PlaceTextError::Empty));
    }

    #[test]
    fn place_kind_display_parse() -> Result<(), PlaceKindParseError> {
        assert_eq!(PlaceKind::City.to_string(), "city");
        assert_eq!(
            "natural feature".parse::<PlaceKind>()?,
            PlaceKind::NaturalFeature
        );
        Ok(())
    }

    #[test]
    fn custom_place_kind() -> Result<(), PlaceKindParseError> {
        assert_eq!(
            "district".parse::<PlaceKind>()?,
            PlaceKind::Custom(String::from("district"))
        );
        Ok(())
    }

    #[test]
    fn place_id_construction() -> Result<(), PlaceTextError> {
        let place_id = PlaceId::new("sf-ca-us")?;

        assert_eq!(place_id.as_str(), "sf-ca-us");
        Ok(())
    }
}
