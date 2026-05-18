# use-boundary

Primitive boundary vocabulary for `RustUse`.

`use-boundary` provides descriptive boundary names, kinds, and statuses without
storing polygon geometry or performing map rendering.

## Non-goals

- polygon topology
- boundary intersection validation
- dispute resolution
- map rendering

## Example

```rust
use use_boundary::{BoundaryKind, BoundaryName, BoundaryStatus};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let name = BoundaryName::new("Continental Divide")?;
let kind = "watershed".parse::<BoundaryKind>()?;
let status = "approximate".parse::<BoundaryStatus>()?;

assert_eq!(name.as_str(), "Continental Divide");
assert_eq!(kind, BoundaryKind::Watershed);
assert_eq!(status, BoundaryStatus::Approximate);
# Ok(())
# }
```
