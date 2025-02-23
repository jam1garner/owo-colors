# Changelog

## [4.2.0] - 2025-02-22

### Added

- `Style::prefix_formatter` and `Style::suffix_formatter` return `Display` formatters for the prefix and the suffix of a style, respectively.
- All the `*Display` types now have an `into_styled` function that converts those types into a `Styled`, erasing type parameters.
- Even more methods are now `const`.

### Changed

- The `Color` and `DynColor` traits are now explicitly marked sealed (i.e. downstream crates cannot implement them).

  These traits were already effectively sealed due to a number of hidden methods that were not part of the API, but they are now explicitly so. In that sense this is not a breaking change, so it's being released under a new minor version rather than a major version.

## [4.1.1] - 2025-02-22

### Added

- The vast majority of owo-colors is now usable in const contexts.

### Fixed

- Documentation for `Stream` is now rendered properly. Thanks [purplesyringa](https://github.com/purplesyringa) for the contribution!
- Replace brittle const-promotion-based unsafe code with safe code. Thanks [Manish](https://github.com/Manishearth) for the contribution!

### Other

- owo-colors now lives under its own organization, https://github.com/owo-colors.

[4.2.0]: https://github.com/owo-colors/owo-colors/releases/tag/v4.2.0
[4.1.1]: https://github.com/owo-colors/owo-colors/releases/tag/v4.1.1

For information about earlier versions, see [the commit history](https://github.com/jam1garner/owo-colors/commits/master).
