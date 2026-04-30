# Changelog

## [Unreleased]

## [0.2.0] - 2026-04-30

### Added
- `comboios-core` library with typed domain models: `TrainJourney`, `StationBoard`, `JourneyStop`, `ServiceAlert`
- `Comboios` client — fetches and caches CP credentials automatically from `cp.pt`
- Background credential rotation (every 55 minutes)
- Stop-by-stop train journey tracking with real-time delay and status
- `comboios-server` REST API (Axum): station search, timetables, train journeys, diagnostics
- `comboios-mcp` MCP server for AI assistant integration
- `comboios-ui` SvelteKit frontend with station search, live boards, journey timeline
- Station ID mapping between CP and IP formats (`to_cp_id`, `to_ip_id`)
- `StationQuery` and `TrainEntryFilter` builder types
- All server config driven by environment variables with compiled-in defaults

### Changed
- Renamed core crate from `comboios` to `comboios-core`
- Removed duplicate `providers/` architecture; single adapter layer (`adapters/`) is now the only implementation
- `CpAdapter` base URL is now configurable for testing
- Shared `USER_AGENT` and base URLs extracted to `constants.rs`
- Regex patterns compiled once via `OnceLock` instead of per-call
- `get_config()` uses read lock on hot path, write lock only on credential refresh

### Fixed
- `impl Default for Comboios` removed (was panicking)
- `Instant::now() - Duration` subtraction that could panic on low-uptime systems
- Timeline icon overlap in the UI journey view

## [0.1.0] - 2024-12-01

### Added
- Initial release
- Station search and basic train information retrieval
- REST API server