# Changelog

[Keep a Changelog]:    https://keepachangelog.com/en/1.0.0/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog][], and this project adheres to
[Semantic Versioning][].


## 0.2.2 (12 November 2024)

### Changed

  - Updated lint configuration for Rust 1.82
  - Updated crate dependencies


## 0.2.1 (09 September 2024)

### Added

  - Added feature flag for `utoipa`


## 0.2.0 (09 September 2024)

### Added

  - Added MSRV (Minimum Supported Rust Version) in `Cargo.toml`, set to 1.81.0

### Changed

  - Changed use of `once_cell::Lazy` to `LazyLock` and removed `once_cell`
    dependency
  - Updated lint configuration for Rust 1.80
  - Updated lint configuration for Rust 1.81
  - Updated crate dependencies
  - Linted tests
  - Moved linting configuration to `Cargo.toml`


## 0.1.1 (02 April 2024)

### Changed

  - Updated lint configuration for Rust 1.76
  - Updated lint configuration for Rust 1.77
  - Updated crate dependencies


## 0.1.0 (10 December 2023)

### Added

  - Added `country` module
      - Added `Country` enum
      - Added `CountryCode` enum
  - Added `currency` module
      - Added `Currency` enum
      - Added `CurrencyCode` enum
  - Added `language` module
      - Added `Language` enum
      - Added `LanguageCode` enum
  - Added basic trait implementations for all enums
      - `AsStr`
      - `Clone`
      - `Copy`
      - `Debug`
      - `Deserialize`
      - `Display`
      - `Eq`
      - `From`
      - `FromStr`
      - `Hash`
      - `PartialEq`
      - `Serialize`
      - `ToSchema`
      - `TryFrom`
  - Added property getters for `Country`, `Currency`, and `Language` enums
  - Added entity lookup methods for code enums
      - Added `CountryCode.country()`
      - Added `CurrencyCode.currency()`
      - Added `LanguageCode.language()`
  - Added `::all()` function for all enums
  - Added `CountryCode` alpha code checking and conversion functions
      - Added `CountryCode.is_alpha2()` and `.is_alpha3()`
      - Added `CountryCode.to_alpha2()` and `.to_alpha3()`
  - Added basic country, currency, and language data
      - Added ISO 3166-1 countries with alpha2/alpha3/num codes and basic names
      - Added ISO 4217 currencies with alpha3/num codes, names, and decimal
        places
      - Added ISO 639-1 languages with alpha2/num codes and names
      - Added relationships between countries, currencies, and languages
  - Added unit tests
  - Added README documentation
  - Added project roadmap


