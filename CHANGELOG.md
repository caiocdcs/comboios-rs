# Changelog

## [Unreleased]

### Added
- Installable PWA: web manifest, app icons, iOS home-screen meta tags and a service worker that caches the app shell and the last station boards/journeys (network-first) for flaky connections, with an offline banner.
- Favourite stations: star a station to pin it to the top of the home screen.
- Vite dev/preview proxy for the API, so `bun run dev` works against a local server.
- Nix flake dev shell (Rust, bun, Node 22, just) with `.envrc`, and a `justfile` (`dev`, `check`, `test`, `build`, `up`, ...); `just check` mirrors CI.
- Station board: departures/arrivals tabs (departures by default), large platform badge, live time with the timetabled time struck through, countdown ("in 4 min"), "updated 40 s ago" with manual refresh, and refresh on returning to the app; shows the next ~2 hours with "Show later" instead of 10-per-page pagination.
- Train journey: live "next stop" summary (expected time, countdown, platform) and expected arrival at the destination; auto-refreshes every 30 s while visible; Share button.

### Fixed
- Station search is accent-insensitive and matches every word ("sodré", "São Bento", "Campanhã" found nothing because CP station names have no diacritics); blank queries are rejected.
- Train journey dates (server, MCP) and journey stop status use `Europe/Lisbon` instead of the host timezone; the UI computes "today" in Lisbon instead of UTC.
- Timetable "Try Again" button passed the click event as the `silent` flag, hiding the loading state and errors on retry.
- Missing `favicon.png`; svelte-check errors and a11y warnings.
- Station board sorted by time of day only, so trains after midnight appeared before late-evening ones; the departures/arrivals filter had no control and was never applied.
- Journey page always labelled the last stop "Arrived", even before the train left.
- Dark mode was split in two: the toggle switched Tailwind's `dark` class while daisyUI components (buttons, badges, tabs) followed the OS setting, so they could disagree. Both now follow the same choice (`data-theme` kept in sync).
- "On time" and "journey completed" indicators had no colour (`success-*` palette was never defined); light-only badge and heading styles in `app.css` were unreadable in dark mode.
- Service badge colour matching never matched CP's values ("Intercidades"), and regional trains used the same amber as platforms.
- UI is now a client-only SPA (`ssr = false`), matching how nginx serves it; page loads in dev/preview no longer fail with relative API URLs.

### Changed
- Home screen: search as you type (debounced; stale requests cancelled), with "Your stations", "Recent" (last 5 boards opened) and one-tap "Main stations" shortcuts instead of the hero banner and search-term history.
- Train journey: one vertical stop list (rail line, next stop highlighted, expected vs timetabled times, platforms, earlier stops folded away) replaces the sideways-scrolling timeline and the duplicate "All stops" cards/table.
- Header: one compact row (logo, About, theme toggle) instead of a hamburger menu for two links; respects the iPhone notch/home-bar safe areas when installed.
- Removed unused components (AlertBanner, LoadingSpinner, Pagination, SearchInput, StationCard, TrainStatusBadge) and unused type exports.
- New "Night blue" colour scheme for light and dark mode: navy-tinted neutrals (dark mode is navy, not grey), indigo brand, amber platform badges, and green/amber/red for on time/delayed/cancelled. Text colours checked against WCAG AA (4.5:1). App icon and status-bar colours updated to match.
- Service badges show the short CP code (AP, IC, IR, R, U) with the full name as a tooltip, instead of the raw "IC|Intercidades" value.
- System fonts instead of Google Fonts (faster first load, works offline, no third-party request).
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