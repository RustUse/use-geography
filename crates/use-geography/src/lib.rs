#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub use use_boundary as boundary;
pub use use_elevation as elevation;
pub use use_geo_coordinate as geo_coordinate;
pub use use_geographic_region as geographic_region;
pub use use_map_scale as map_scale;
pub use use_place as place;
pub use use_projection as projection;
pub use use_spatial_reference as spatial_reference;

#[cfg(test)]
mod tests {
    use super::{
        boundary, elevation, geo_coordinate, geographic_region, map_scale, place, projection,
        spatial_reference,
    };

    #[test]
    fn facade_reexports_geography_primitives() -> Result<(), Box<dyn std::error::Error>> {
        let latitude = geo_coordinate::Latitude::new(48.8566)?;
        let longitude = geo_coordinate::Longitude::new(2.3522)?;
        let pair = geo_coordinate::CoordinatePair::new(latitude, longitude);
        let coordinate = geo_coordinate::GeoCoordinate::from(pair);
        let place_name = place::PlaceName::new("Paris")?;
        let region_name = geographic_region::GeographicRegionName::new("Ile-de-France")?;
        let epsg = spatial_reference::EpsgCode::new(4326)?;
        let height = elevation::Elevation::new(35.0)?;
        let scale = map_scale::MapScale::new(map_scale::ScaleRatio::new(10_000)?);

        assert_eq!(coordinate.latitude(), latitude);
        assert_eq!(coordinate.longitude(), longitude);
        assert_eq!(place_name.as_str(), "Paris");
        assert_eq!(region_name.as_str(), "Ile-de-France");
        assert_eq!(
            boundary::BoundaryKind::Administrative.to_string(),
            "administrative"
        );
        assert_eq!(projection::ProjectionKind::Mercator.to_string(), "mercator");
        assert_eq!(epsg.to_string(), "EPSG:4326");
        assert_eq!(height.meters(), 35.0);
        assert_eq!(scale.to_string(), "1:10000");
        Ok(())
    }
}
