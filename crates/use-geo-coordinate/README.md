# use-geo-coordinate

Primitive geographic coordinate vocabulary for `RustUse`.

`use-geo-coordinate` provides small Earth-oriented coordinate primitives such as
latitude, longitude, geographic coordinates, coordinate pairs, and coordinate
format labels.

## Scope

- latitude and longitude validation for Earth-oriented coordinates
- simple coordinate pair composition
- descriptive coordinate format vocabulary

## Non-goals

- geodesic calculations
- projection conversion
- routing behavior
- geometry-engine primitives

## Example

```rust
use use_geo_coordinate::{CoordinateFormat, CoordinatePair, GeoCoordinate, Latitude, Longitude};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let latitude = Latitude::new(51.5074)?;
let longitude = Longitude::new(-0.1278)?;
let pair = CoordinatePair::new(latitude, longitude);
let coordinate = GeoCoordinate::from(pair);

assert_eq!(coordinate.latitude(), latitude);
assert_eq!(coordinate.longitude(), longitude);
assert_eq!(CoordinateFormat::DegreesMinutesSeconds.to_string(), "degrees-minutes-seconds");
# Ok(())
# }
```
