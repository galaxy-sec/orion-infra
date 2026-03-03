# Changelog

All notable changes to this project are documented in this file.

The format is loosely based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Upgraded key dependencies for compatibility updates:
  - `orion-error`: `0.3` -> `0.6`
  - `thiserror`: pinned to `2.0`
  - `log`: pinned to `0.4`
  - `toml`: pinned to `0.9`
  - `serde` / `serde_derive`: pinned to `1.0`
  - `anyhow`: pinned to `1.0`
  - `scopeguard`: `1.2.0` -> `1.2`
  - `flexi_logger`: pinned to `0.31`

### Fixed

- Fixed `orion-error 0.6` API migration issues in `src/path.rs`:
  - replaced `UvsResFrom` with `UvsFrom`
  - updated resource error construction to `from_res()` + context chaining
  - removed unsupported `Serialize` derive from `PathReason`
- Fixed Clippy warning `collapsible_if` in `src/logging/configure.rs` to pass `-D warnings`.

## [0.2.0] - 2025-07-17

### Added

- Initial import of the infrastructure utility crate with:
  - config lifecycle and TOML helpers
  - logging configuration helpers
  - path utility helpers
  - shared traits and types

### Changed

- Project version updated to `0.2.0`.

### Maintenance

- General style/lint cleanup.
- Updated ignore-related project housekeeping.
