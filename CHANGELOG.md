# Changelog

## [Unreleased]

### Added
- Installable PWA: web manifest, app icons, iOS home-screen meta tags and a service worker that caches the app shell and the last station boards/journeys (network-first) for flaky connections, with an offline banner.
- Favourite stations: star a station to pin it to the top of the home screen.
- Vite dev/preview proxy for the API, so `bun run dev` works against a local server.

### Fixed
- Station search is accent-insensitive and matches every word ("sodré", "São Bento", "Campanhã" found nothing because CP station names have no diacritics); blank queries are rejected.
- Train journey dates (server, MCP) and journey stop status use `Europe/Lisbon` instead of the host timezone; the UI computes "today" in Lisbon instead of UTC.
- Timetable "Try Again" button passed the click event as the `silent` flag, hiding the loading state and errors on retry.
- Missing `favicon.png`; svelte-check errors and a11y warnings.
- UI is now a client-only SPA (`ssr = false`), matching how nginx serves it; page loads in dev/preview no longer fail with relative API URLs.

### Changed
- nginx: `service-worker.js`, the manifest and HTML are revalidated on each load; hashed `/_app/immutable` assets are cached for a year.
- Search input tuned for mobile keyboards (search key, no autocorrect/capitalisation).

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