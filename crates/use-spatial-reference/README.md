# use-spatial-reference

Primitive spatial reference vocabulary for `RustUse`.

`use-spatial-reference` provides small descriptive types for EPSG codes,
spatial-reference identifiers, spatial-reference systems, and coordinate
reference systems.

## Non-goals

- EPSG registry fetching
- external-registry validation
- coordinate transformation
- deep CRS parsing

## Example

```rust
use use_spatial_reference::{CoordinateReferenceSystem, EpsgCode, SpatialReferenceId};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let epsg = EpsgCode::new(4326)?;
let identifier = SpatialReferenceId::new("WGS84")?;
let crs = CoordinateReferenceSystem::from_epsg(epsg);

assert_eq!(epsg.to_string(), "EPSG:4326");
assert_eq!(identifier.as_str(), "WGS84");
assert_eq!(crs.to_string(), "EPSG:4326");
# Ok(())
# }
```
