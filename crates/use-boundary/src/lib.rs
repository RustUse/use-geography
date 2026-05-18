#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn non_empty_text(value: impl AsRef<str>) -> Result<String, BoundaryTextError> {
    let trimmed = value.as_ref().trim();

    if trimmed.is_empty() {
        Err(BoundaryTextError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

fn normalized_token(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace(['_', ' '], "-")
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BoundaryTextError {
    Empty,
}

impl fmt::Display for BoundaryTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("boundary text cannot be empty"),
        }
    }
}

impl Error for BoundaryTextError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BoundaryParseError {
    Empty,
}

impl fmt::Display for BoundaryParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("boundary vocabulary cannot be empty"),
        }
    }
}

impl Error for BoundaryParseError {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BoundaryName(String);

impl BoundaryName {
    pub fn new(value: impl AsRef<str>) -> Result<Self, BoundaryTextError> {
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

impl AsRef<str> for BoundaryName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for BoundaryName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for BoundaryName {
    type Err = BoundaryTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BoundaryKind {
    Political,
    Administrative,
    Natural,
    Coastline,
    River,
    Watershed,
    Property,
    ProtectedArea,
    Maritime,
    Unknown,
    Custom(String),
}

impl fmt::Display for BoundaryKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Political => formatter.write_str("political"),
            Self::Administrative => formatter.write_str("administrative"),
            Self::Natural => formatter.write_str("natural"),
            Self::Coastline => formatter.write_str("coastline"),
            Self::River => formatter.write_str("river"),
            Self::Watershed => formatter.write_str("watershed"),
            Self::Property => formatter.write_str("property"),
            Self::ProtectedArea => formatter.write_str("protected-area"),
            Self::Maritime => formatter.write_str("maritime"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for BoundaryKind {
    type Err = BoundaryParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(BoundaryParseError::Empty);
        }

        Ok(match normalized_token(trimmed).as_str() {
            "political" => Self::Political,
            "administrative" => Self::Administrative,
            "natural" => Self::Natural,
            "coastline" => Self::Coastline,
            "river" => Self::River,
            "watershed" => Self::Watershed,
            "property" => Self::Property,
            "protected-area" => Self::ProtectedArea,
            "maritime" => Self::Maritime,
            "unknown" => Self::Unknown,
            _ => Self::Custom(trimmed.to_string()),
        })
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BoundaryStatus {
    Official,
    Disputed,
    Approximate,
    Historical,
    Unknown,
    Custom(String),
}

impl fmt::Display for BoundaryStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Official => formatter.write_str("official"),
            Self::Disputed => formatter.write_str("disputed"),
            Self::Approximate => formatter.write_str("approximate"),
            Self::Historical => formatter.write_str("historical"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for BoundaryStatus {
    type Err = BoundaryParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(BoundaryParseError::Empty);
        }

        Ok(match normalized_token(trimmed).as_str() {
            "official" => Self::Official,
            "disputed" => Self::Disputed,
            "approximate" => Self::Approximate,
            "historical" => Self::Historical,
            "unknown" => Self::Unknown,
            _ => Self::Custom(trimmed.to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BoundaryKind, BoundaryName, BoundaryParseError, BoundaryStatus, BoundaryTextError,
    };

    #[test]
    fn valid_boundary_name() -> Result<(), BoundaryTextError> {
        let boundary_name = BoundaryName::new("Arctic Circle")?;

        assert_eq!(boundary_name.as_str(), "Arctic Circle");
        Ok(())
    }

    #[test]
    fn empty_boundary_name_rejected() {
        assert_eq!(BoundaryName::new("  "), Err(BoundaryTextError::Empty));
    }

    #[test]
    fn boundary_kind_display_parse() -> Result<(), BoundaryParseError> {
        assert_eq!(BoundaryKind::Political.to_string(), "political");
        assert_eq!(
            "protected area".parse::<BoundaryKind>()?,
            BoundaryKind::ProtectedArea
        );
        Ok(())
    }

    #[test]
    fn boundary_status_display_parse() -> Result<(), BoundaryParseError> {
        assert_eq!(BoundaryStatus::Historical.to_string(), "historical");
        assert_eq!(
            "disputed".parse::<BoundaryStatus>()?,
            BoundaryStatus::Disputed
        );
        Ok(())
    }

    #[test]
    fn custom_boundary_kind() -> Result<(), BoundaryParseError> {
        assert_eq!(
            "tribal".parse::<BoundaryKind>()?,
            BoundaryKind::Custom(String::from("tribal"))
        );
        Ok(())
    }
}
