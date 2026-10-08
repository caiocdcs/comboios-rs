# Changelog

## [Unreleased]

### Added
- Installable web app (PWA): home-screen icon, works on a flaky connection with the last boards and journeys you loaded, offline banner.
- Favourite and recent stations on the home screen, plus shortcuts to the main stations.
- Live station board: departures/arrivals tabs, platform, expected vs timetabled time, countdown, auto-refresh.
- Live train tracking: next stop with platform and countdown, expected arrival, stop-by-stop list, share.
- Nix dev shell and `justfile`; `just check` runs the same checks as CI, which now also checks and builds the UI.

### Changed
- New "Night blue" design for light and dark mode with the Overpass typeface; amber marks platforms, orange marks delays.
- Station search works as you type and ignores accents ("sodré", "São Bento" now match).
- Service badges show the short CP code (AP, IC, IR, R, U).
- Simpler header, About and error pages.

### Fixed
- Train journey dates and times use Lisbon time everywhere (server, MCP, UI) instead of the host timezone or UTC.
- The departures/arrivals filter had no control, and trains after midnight sorted first.
- Dark mode could show buttons and badges in the wrong theme.
- "Try again" on the station board hid errors; a journey's last stop showed "Arrived" before the train got there.
- Missing favicon and UI type errors.

## [0.3.0] - 2026-08-17

### Fixed
- Station timetable route now uses `Europe/Lisbon` time instead of host-local time, so boards are correct in UTC containers (was: one hour of already-departed trains shown in summer).
- Passed trains are now dropped from the board response using effective time (scheduled + delay, or eta/etd when present); delayed trains whose scheduled time already passed stay visible via a 60-minute lookback window on the CP `start` parameter.

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