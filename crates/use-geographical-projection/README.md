# use-geographical-projection

Primitive map projection vocabulary for `RustUse`.

`use-geographical-projection` provides descriptive projection names, projection kinds, and
simple projection parameter records without implementing projection math.

## Non-goals

- projection math
- coordinate conversion
- PROJ integration
- geospatial transformation engines

## Example

```rust
use use_projection::{ProjectionKind, ProjectionName, ProjectionParameter};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let name = ProjectionName::new("World Mercator")?;
let kind = "mercator".parse::<ProjectionKind>()?;
let parameter = ProjectionParameter::new("central-meridian", "0")?;

assert_eq!(name.as_str(), "World Mercator");
assert_eq!(kind, ProjectionKind::Mercator);
assert_eq!(parameter.to_string(), "central-meridian=0");
# Ok(())
# }
```
