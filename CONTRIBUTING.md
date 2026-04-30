# Contributing

## Prerequisites

- Rust (edition 2024)
- Node.js 20+ / Bun (for the frontend)

## Development

```bash
cargo build
cargo test
cargo clippy -- -W clippy::pedantic
```

```bash
cd comboios-ui
bun install && bun run dev
```

## Guidelines

- Run clippy and tests before submitting
- Keep changes focused
- Open an issue first for significant changes

Licensed under MIT OR Apache-2.0.