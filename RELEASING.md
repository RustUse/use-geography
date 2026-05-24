# Releasing

This repository uses a first-wave RustUse multi-crate release flow.

1. Publish the focused crates first.
2. Wait for the published versions to become visible on crates.io.
3. Run the facade dry-run once the focused crates are indexed.
4. Publish `use-geography` last.

Suggested focused-crate order:

1. `use-geo-coordinate`
2. `use-place`
3. `use-address`
4. `use-geographic-region`
5. `use-boundary`
6. `use-geographical-projection`
7. `use-spatial-reference`
8. `use-elevation`
9. `use-map-scale`
10. `use-geography`
