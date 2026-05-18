# use-geography

Facade crate for `RustUse` geography primitives.

`use-geography` reexports focused child crates for descriptive Earth-oriented
vocabulary. It is not a GIS engine, mapping framework, geocoder, routing engine,
map renderer, spatial database, or projection engine.

## Scope

- thin facade over focused geography child crates
- descriptive geographic coordinates, places, regions, boundaries, projections,
  spatial references, elevation, and map scale
- composition-friendly exports for downstream crates

## Non-goals

- geodesic calculations
- projection math
- coordinate transformation
- geocoding or reverse geocoding
- routing or route planning
- map rendering or tile behavior
- network access or data fetching

## Example

```rust
use use_geography::{
	boundary, elevation, geo_coordinate, geographic_region, map_scale, place, projection,
	spatial_reference,
};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let latitude = geo_coordinate::Latitude::new(37.7749)?;
let longitude = geo_coordinate::Longitude::new(-122.4194)?;
let pair = geo_coordinate::CoordinatePair::new(latitude, longitude);
let coordinate = geo_coordinate::GeoCoordinate::from(pair);
let place_name = place::PlaceName::new("San Francisco")?;
let region_name = geographic_region::GeographicRegionName::new("Northern California")?;
let boundary_kind = boundary::BoundaryKind::Political;
let projection_kind = projection::ProjectionKind::WebMercator;
let epsg = spatial_reference::EpsgCode::new(4326)?;
let elevation = elevation::Elevation::new(16.0)?;
let scale = map_scale::MapScale::new(map_scale::ScaleRatio::new(25_000)?);

assert_eq!(coordinate.latitude(), latitude);
assert_eq!(coordinate.longitude(), longitude);
assert_eq!(place_name.as_str(), "San Francisco");
assert_eq!(region_name.as_str(), "Northern California");
assert_eq!(boundary_kind.to_string(), "political");
assert_eq!(projection_kind.to_string(), "web-mercator");
assert_eq!(epsg.to_string(), "EPSG:4326");
assert_eq!(elevation.meters(), 16.0);
assert_eq!(scale.to_string(), "1:25000");
# Ok(())
# }
```
