# use-geography

> Composable sets of primitive Rust utility crates for fellow crustaceans.

`use-geography` is a primitive geography vocabulary set for `RustUse`.
It describes geographic coordinates, places, geographic regions, boundaries,
projections, spatial references, elevation, and map scale as small, composable
Rust value types.

The root `use-geography` crate is a facade crate only. Most implementation
belongs in focused child crates under `crates/`, where the vocabulary stays
small, explicit, and independently useful.

## Workspace crates

- `use-geography`: thin facade crate that reexports the focused geography crates
- `use-geo-coordinate`: geographic coordinate primitives such as latitude and longitude
- `use-place`: descriptive place names, kinds, and identifiers
- `use-geographic-region`: descriptive geographic region names, kinds, and identifiers
- `use-boundary`: descriptive boundary names, kinds, and statuses
- `use-projection`: descriptive map projection names, kinds, and parameters
- `use-spatial-reference`: descriptive spatial reference identifiers and EPSG codes
- `use-elevation`: elevation, depth, datum, and vertical-reference vocabulary
- `use-map-scale`: descriptive map scale, resolution, and zoom vocabulary

## Non-goals

`use-geography` is not trying to become:

- a GIS engine
- a mapping framework
- a geocoder or reverse geocoder
- a route planner or routing engine
- a map renderer or tile client
- a spatial database
- a projection math engine
- a coordinate transformation engine
- a spatial indexing engine
- a boundary resolver
- an address parser
- a command-line app

The workspace stays on the descriptive side of the boundary: stable vocabulary,
conservative validation, and small Earth-oriented primitives that downstream
crates can compose into larger systems.

## Complementary crates

`use-geography` complements other RustUse sets rather than replacing them.
Common adjacent sets include:

- `use-geometry` for abstract geometry such as points, lines, polygons, and shapes
- `use-locale` for locale-oriented region and country-code concepts
- `use-data` for generic data modeling helpers
- `use-validate` for broader validation layers in application code
- `use-units` for standalone unit vocabulary
- `use-measure` for explicit measurement primitives

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
let region = geographic_region::GeographicRegionName::new("Northern California")?;
let boundary_kind = boundary::BoundaryKind::Political;
let projection_kind = projection::ProjectionKind::WebMercator;
let epsg = spatial_reference::EpsgCode::new(4326)?;
let elevation = elevation::Elevation::new(16.0)?;
let scale = map_scale::MapScale::new(map_scale::ScaleRatio::new(25_000)?);

assert_eq!(latitude.degrees(), 37.7749);
assert_eq!(longitude.degrees(), -122.4194);
assert_eq!(coordinate.latitude(), latitude);
assert_eq!(place_name.as_str(), "San Francisco");
assert_eq!(region.as_str(), "Northern California");
assert_eq!(boundary_kind.to_string(), "political");
assert_eq!(projection_kind.to_string(), "web-mercator");
assert_eq!(epsg.get(), 4326);
assert_eq!(elevation.meters(), 16.0);
assert_eq!(scale.to_string(), "1:25000");
# Ok(())
# }
```

The example only describes geographic concepts. It does not geocode, route,
render, fetch, transform, or analyze anything.

## Status

This first version is intentionally conservative. The goal is long-lived,
descriptive geographic vocabulary, not geospatial framework behavior.
