# Contributing

## Prerequisites

Either Nix (`nix develop`, or `direnv allow` with the provided `.envrc`), which
provides everything below, or:

- Rust 1.85+ (edition 2024)
- Bun (and Node.js 22 for parity with the Docker build)
- [just](https://just.systems)

## Development

```bash
just dev     # API server + UI with hot reload
just check   # run before submitting: same checks as CI
```

## Guidelines

- Run `just check` before submitting
- Keep changes focused
- Open an issue first for significant changes

Licensed under MIT OR Apache-2.0.