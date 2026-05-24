# use-address

Primitive address component vocabulary for `RustUse`.

`use-address` provides small descriptive primitives for address lines, street
parts, localities, administrative areas, postal codes, country codes, and
lightweight address metadata.

It models address components. It does not parse free-form addresses, geocode,
reverse geocode, validate against postal services, format via `CLDR`,
calculate shipping, look up maps, or resolve coordinates.

## Relationship to `use-locale`

`use-address` models address components and address-oriented metadata.
`use-locale` models locale identifiers and formatting context.
`use-address` may carry lightweight formatting hints, but it does not implement
locale-aware address formatting in v0.1.

## Non-goals

- address parsing
- geocoding
- reverse geocoding
- postal-service validation
- shipping-rate lookup
- `CLDR` formatting
- map lookup
- coordinate resolution

## Example

```rust
use use_address::{Address, AddressCountryCode, AddressLine, Locality};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let address = Address::new()
    .with_line(AddressLine::new("123 Main St")?);

let address = Address {
    locality: Some(Locality::new("Fort Wayne")?),
    country_code: Some(AddressCountryCode::new("us")?),
    ..address
};

assert!(!address.is_empty());
assert!(address.has_country());
# Ok(())
# }
```
