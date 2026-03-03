# Changelog

All notable changes to this project are documented in this file.

The format is based on Keep a Changelog, and the project follows Semantic Versioning.

## [Unreleased]

- No changes yet.

## [0.5.0] - 2026-03-03

### Changed

- Package metadata refreshed (`name = "orion-infra"`, description/license/repository/keywords/categories).
- Config lifecycle now integrates with `orion_conf` and uses `OrionConfResult`.
- Dependencies upgraded/aligned:
  - `orion-error` -> `0.6`
  - added `orion_conf = 0.5` (`yaml`, `toml` features)
  - `thiserror = 2.0`, `log = 0.4`, `toml = 0.9`, `serde = 1.0`, `anyhow = 1.0`, `scopeguard = 1.2`, `flexi_logger = 0.31`

### Fixed

- `src/path.rs`: adapted to `orion-error 0.6` API (`UvsFrom` / `from_res` flow).
- `src/logging/configure.rs`: resolved `clippy::collapsible_if` under `-D warnings`.

### Docs

- Added and updated `README.md` and `CHANGELOG.md`.

## [0.2.0] - 2025-07-17

### Added

- Initial import of infrastructure utility modules (config/logging/path/types).
