# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-03-23

### Added
- SvelteKit-based web frontend (comboios-ui) with dark mode
- Station search by name via CP API
- Real-time station board from IP API with train delays
- Background credential refresh every 55 minutes
- REST API server with Axum
- MCP (Model Context Protocol) server for AI assistants

### Changed
- Migrated from deprecated CP API to Infraestruturas de Portugal (IP) API
- Refactored to provider architecture for station board data
- Improved error handling with automatic fallback between CP and IP adapters

### Removed
- Deprecated `get_train_details()` endpoint (API no longer available)
- Dioxus-based frontend (replaced by SvelteKit)

## [0.1.0] - 2024-12-XX

### Added
- Initial release with core library
- Station search functionality
- Basic train information retrieval
- REST API server