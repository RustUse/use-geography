# Releasing

This repository uses a first-wave RustUse multi-crate release flow.

1. Publish the focused crates first.
2. Wait for the published versions to become visible on crates.io.
3. Run the facade dry-run once the focused crates are indexed.
4. Publish `use-geography` last.

Suggested focused-crate order:

1. `use-geo-coordinate`
2. `use-place`
3. `use-geographic-region`
4. `use-boundary`
5. `use-geographical-projection`
6. `use-spatial-reference`
7. `use-elevation`
8. `use-map-scale`
9. `use-geography`
