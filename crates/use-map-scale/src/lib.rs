#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::fmt;
use std::error::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MapScaleError {
    ZeroScaleRatio,
    ResolutionNotFinite,
    ResolutionNotPositive,
}

impl fmt::Display for MapScaleError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroScaleRatio => formatter.write_str("scale ratio must be greater than zero"),
            Self::ResolutionNotFinite => formatter.write_str("map resolution must be finite"),
            Self::ResolutionNotPositive => {
                formatter.write_str("map resolution must be greater than zero")
            },
        }
    }
}

impl Error for MapScaleError {}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ScaleRatio(u32);

impl ScaleRatio {
    pub fn new(denominator: u32) -> Result<Self, MapScaleError> {
        if denominator == 0 {
            return Err(MapScaleError::ZeroScaleRatio);
        }

        Ok(Self(denominator))
    }

    #[must_use]
    pub const fn denominator(self) -> u32 {
        self.0
    }
}

impl fmt::Display for ScaleRatio {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "1:{}", self.denominator())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MapScale {
    ratio: ScaleRatio,
}

impl MapScale {
    #[must_use]
    pub const fn new(ratio: ScaleRatio) -> Self {
        Self { ratio }
    }

    #[must_use]
    pub const fn ratio(self) -> ScaleRatio {
        self.ratio
    }
}

impl fmt::Display for MapScale {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.ratio.fmt(formatter)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct MapResolution(f64);

impl MapResolution {
    pub fn new(units_per_pixel: f64) -> Result<Self, MapScaleError> {
        if !units_per_pixel.is_finite() {
            return Err(MapScaleError::ResolutionNotFinite);
        }

        if units_per_pixel <= 0.0 {
            return Err(MapScaleError::ResolutionNotPositive);
        }

        Ok(Self(units_per_pixel))
    }

    #[must_use]
    pub fn units_per_pixel(self) -> f64 {
        self.0
    }
}

impl fmt::Display for MapResolution {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} units-per-pixel", self.units_per_pixel())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ZoomLevel(u8);

impl ZoomLevel {
    #[must_use]
    pub const fn new(level: u8) -> Self {
        Self(level)
    }

    #[must_use]
    pub const fn level(self) -> u8 {
        self.0
    }
}

impl fmt::Display for ZoomLevel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.level().fmt(formatter)
    }
}

#[cfg(test)]
mod tests {
    use super::{MapResolution, MapScale, MapScaleError, ScaleRatio, ZoomLevel};

    #[test]
    fn valid_scale_ratio() -> Result<(), MapScaleError> {
        let ratio = ScaleRatio::new(50_000)?;

        assert_eq!(ratio.denominator(), 50_000);
        Ok(())
    }

    #[test]
    fn zero_scale_ratio_rejected() {
        assert_eq!(ScaleRatio::new(0), Err(MapScaleError::ZeroScaleRatio));
    }

    #[test]
    fn map_resolution_construction() -> Result<(), MapScaleError> {
        let resolution = MapResolution::new(4.0)?;

        assert_eq!(resolution.units_per_pixel(), 4.0);
        Ok(())
    }

    #[test]
    fn zoom_level_construction() {
        let zoom = ZoomLevel::new(12);

        assert_eq!(zoom.level(), 12);
    }

    #[test]
    fn display_behavior() -> Result<(), MapScaleError> {
        let scale = MapScale::new(ScaleRatio::new(25_000)?);

        assert_eq!(scale.to_string(), "1:25000");
        assert_eq!(ZoomLevel::new(8).to_string(), "8");
        Ok(())
    }
}
