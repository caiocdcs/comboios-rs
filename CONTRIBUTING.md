# Contributing to Comboios-RS

Thank you for your interest in contributing!

## Prerequisites

- Rust 1.75+
- Node.js 18+ (for frontend)
- Bun

## Build & Run

```bash
# Backend
cargo run -p comboios-server

# Frontend
cd comboios-ui
bun install
bun run dev
```

## Code Style

- Follow existing patterns in the codebase
- Run `cargo clippy` before committing
- Keep changes focused and minimal

## Submitting Changes

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Ensure tests pass: `cargo test`
5. Submit a pull request with clear description

## Questions?

Open an issue for questions or discussion.

This project is dual-licensed under MIT OR Apache-2.0.