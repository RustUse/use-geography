# use-map-scale

Primitive map scale vocabulary for `RustUse`.

`use-map-scale` provides small descriptive types for scale ratios, map scale,
map resolution, and zoom levels without implementing map tile behavior.

## Non-goals

- tile math
- slippy-map logic
- map rendering
- map-tile fetching

## Example

```rust
use use_map_scale::{MapResolution, MapScale, ScaleRatio, ZoomLevel};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let ratio = ScaleRatio::new(25_000)?;
let scale = MapScale::new(ratio);
let resolution = MapResolution::new(2.5)?;
let zoom = ZoomLevel::new(10);

assert_eq!(scale.to_string(), "1:25000");
assert_eq!(resolution.units_per_pixel(), 2.5);
assert_eq!(zoom.level(), 10);
# Ok(())
# }
```
