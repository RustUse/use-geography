# use-elevation

Primitive elevation and depth vocabulary for `RustUse`.

`use-elevation` provides small descriptive types for elevation, depth,
elevation datums, and vertical-reference labels.

## Non-goals

- vertical-datum transformation
- elevation-data fetching
- terrain modeling
- terrain rendering

## Example

```rust
use use_elevation::{Depth, Elevation, ElevationDatum, VerticalReference};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let elevation = Elevation::new(-430.5)?;
let depth = Depth::new(12.0)?;
let datum = "mean sea level".parse::<ElevationDatum>()?;
let reference = VerticalReference::new("MSL")?;

assert_eq!(elevation.meters(), -430.5);
assert_eq!(depth.meters(), 12.0);
assert_eq!(datum, ElevationDatum::MeanSeaLevel);
assert_eq!(reference.as_str(), "MSL");
# Ok(())
# }
```
