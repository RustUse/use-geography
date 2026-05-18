# use-geographic-region

Primitive geographic region vocabulary for `RustUse`.

`use-geographic-region` provides descriptive names, identifiers, and kinds for
geographic regions without storing or validating GIS geometry.

## Non-goals

- GIS polygons
- official-boundary validation
- boundary-data fetching
- spatial indexing

## Example

```rust
use use_geographic_region::{GeographicRegionId, GeographicRegionKind, GeographicRegionName};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let name = GeographicRegionName::new("Pacific Northwest")?;
let kind = "cultural".parse::<GeographicRegionKind>()?;
let id = GeographicRegionId::new("pacific-northwest")?;

assert_eq!(name.as_str(), "Pacific Northwest");
assert_eq!(kind, GeographicRegionKind::Cultural);
assert_eq!(id.as_str(), "pacific-northwest");
# Ok(())
# }
```
