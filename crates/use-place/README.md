# use-place

Primitive place vocabulary for `RustUse`.

`use-place` provides small descriptive primitives for place names, place kinds,
and stable place identifiers.

## Non-goals

- geocoding
- reverse geocoding
- gazetteer data
- political-boundary validation

## Example

```rust
use use_place::{PlaceId, PlaceKind, PlaceName};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let name = PlaceName::new("Golden Gate Bridge")?;
let kind = "landmark".parse::<PlaceKind>()?;
let id = PlaceId::new("golden-gate-bridge")?;

assert_eq!(name.as_str(), "Golden Gate Bridge");
assert_eq!(kind, PlaceKind::Landmark);
assert_eq!(id.as_str(), "golden-gate-bridge");
# Ok(())
# }
```
