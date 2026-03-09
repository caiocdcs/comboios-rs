# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- `StationQuery` builder for ergonomic station searches
- `TimetableFilter` builder for filtering timetables
- `find_stations()` and `get_filtered_timetable()` methods using builders
- Property-based tests using proptest for domain types
- Query builder unit tests
- Documentation examples for public APIs

### Changed
- Made `tracing` dependency optional (enabled by default)
- Improved error types: `NetworkError`, `ParseError`, `ApiError`, `InvalidInput`
- Flattened module exports: `Station`, `Timetable`, `Train` now at crate root
- Removed debug print statements
- Moved integration tests to `tests/` directory

## [0.1.0] - 2025-03-09

### Added
- Initial release with ComboiosApi client
- Station search, timetable, and train details
- REST API server, MCP server, web frontend

[Unreleased]: https://github.com/caiocdcs/comboios-rs/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/caiocdcs/comboios-rs/releases/tag/v0.1.0
