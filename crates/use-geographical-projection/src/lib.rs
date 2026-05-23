#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn non_empty_text(value: impl AsRef<str>) -> Result<String, ProjectionTextError> {
    let trimmed = value.as_ref().trim();

    if trimmed.is_empty() {
        Err(ProjectionTextError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

fn normalized_token(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace(['_', ' '], "-")
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProjectionTextError {
    Empty,
}

impl fmt::Display for ProjectionTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("projection text cannot be empty"),
        }
    }
}

impl Error for ProjectionTextError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProjectionKindParseError {
    Empty,
}

impl fmt::Display for ProjectionKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("projection kind cannot be empty"),
        }
    }
}

impl Error for ProjectionKindParseError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProjectionParameterError {
    EmptyKey,
    EmptyValue,
}

impl fmt::Display for ProjectionParameterError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyKey => formatter.write_str("projection parameter key cannot be empty"),
            Self::EmptyValue => formatter.write_str("projection parameter value cannot be empty"),
        }
    }
}

impl Error for ProjectionParameterError {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ProjectionName(String);

impl ProjectionName {
    /// Creates a projection name from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`ProjectionTextError::Empty`] when the trimmed value is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, ProjectionTextError> {
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

impl AsRef<str> for ProjectionName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for ProjectionName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ProjectionName {
    type Err = ProjectionTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ProjectionKind {
    Mercator,
    WebMercator,
    TransverseMercator,
    LambertConformalConic,
    AlbersEqualArea,
    Equirectangular,
    Orthographic,
    Stereographic,
    Unknown,
    Custom(String),
}

impl fmt::Display for ProjectionKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mercator => formatter.write_str("mercator"),
            Self::WebMercator => formatter.write_str("web-mercator"),
            Self::TransverseMercator => formatter.write_str("transverse-mercator"),
            Self::LambertConformalConic => formatter.write_str("lambert-conformal-conic"),
            Self::AlbersEqualArea => formatter.write_str("albers-equal-area"),
            Self::Equirectangular => formatter.write_str("equirectangular"),
            Self::Orthographic => formatter.write_str("orthographic"),
            Self::Stereographic => formatter.write_str("stereographic"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for ProjectionKind {
    type Err = ProjectionKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(ProjectionKindParseError::Empty);
        }

        Ok(match normalized_token(trimmed).as_str() {
            "mercator" => Self::Mercator,
            "web-mercator" => Self::WebMercator,
            "transverse-mercator" => Self::TransverseMercator,
            "lambert-conformal-conic" => Self::LambertConformalConic,
            "albers-equal-area" => Self::AlbersEqualArea,
            "equirectangular" => Self::Equirectangular,
            "orthographic" => Self::Orthographic,
            "stereographic" => Self::Stereographic,
            "unknown" => Self::Unknown,
            _ => Self::Custom(trimmed.to_string()),
        })
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ProjectionParameter {
    key: String,
    value: String,
}

impl ProjectionParameter {
    /// Creates a projection parameter from non-empty key and value text.
    ///
    /// # Errors
    ///
    /// Returns [`ProjectionParameterError::EmptyKey`] when the trimmed key is empty.
    /// Returns [`ProjectionParameterError::EmptyValue`] when the trimmed value is empty.
    pub fn new(
        key: impl AsRef<str>,
        value: impl AsRef<str>,
    ) -> Result<Self, ProjectionParameterError> {
        let key = key.as_ref().trim();
        let value = value.as_ref().trim();

        if key.is_empty() {
            return Err(ProjectionParameterError::EmptyKey);
        }

        if value.is_empty() {
            return Err(ProjectionParameterError::EmptyValue);
        }

        Ok(Self {
            key: key.to_string(),
            value: value.to_string(),
        })
    }

    #[must_use]
    pub fn key(&self) -> &str {
        &self.key
    }

    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl fmt::Display for ProjectionParameter {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}={}", self.key, self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ProjectionKind, ProjectionKindParseError, ProjectionName, ProjectionParameter,
        ProjectionParameterError, ProjectionTextError,
    };

    #[test]
    fn valid_projection_name() -> Result<(), ProjectionTextError> {
        let projection_name = ProjectionName::new("WGS 84 / World Mercator")?;

        assert_eq!(projection_name.as_str(), "WGS 84 / World Mercator");
        Ok(())
    }

    #[test]
    fn empty_projection_name_rejected() {
        assert_eq!(ProjectionName::new("   "), Err(ProjectionTextError::Empty));
    }

    #[test]
    fn projection_kind_display_parse() -> Result<(), ProjectionKindParseError> {
        assert_eq!(ProjectionKind::WebMercator.to_string(), "web-mercator");
        assert_eq!(
            "lambert conformal conic".parse::<ProjectionKind>()?,
            ProjectionKind::LambertConformalConic
        );
        Ok(())
    }

    #[test]
    fn custom_projection_kind() -> Result<(), ProjectionKindParseError> {
        assert_eq!(
            "azimuthal-equidistant".parse::<ProjectionKind>()?,
            ProjectionKind::Custom(String::from("azimuthal-equidistant"))
        );
        Ok(())
    }

    #[test]
    fn projection_parameter_construction() -> Result<(), ProjectionParameterError> {
        let parameter = ProjectionParameter::new("central-meridian", "0")?;

        assert_eq!(parameter.key(), "central-meridian");
        assert_eq!(parameter.value(), "0");
        assert_eq!(parameter.to_string(), "central-meridian=0");
        Ok(())
    }
}
