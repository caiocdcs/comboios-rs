# comboios-rs tasks. Run `just` to list them.
# Tools come from the Nix dev shell: `nix develop` (or direnv with .envrc).

ui := "comboios-ui"

# List recipes
default:
    @just --list

# Install UI dependencies
ui-install:
    cd {{ui}} && bun install --frozen-lockfile

# Run the API server on http://localhost:3000
server:
    cargo run -p comboios-server

# Run the UI dev server on http://localhost:5173 (proxies the API to :3000)
ui: ui-install
    cd {{ui}} && bun run dev

# Run API server and UI together; Ctrl-C stops both
dev: ui-install
    #!/usr/bin/env bash
    set -euo pipefail
    trap 'kill 0' EXIT
    cargo run -p comboios-server &
    (cd {{ui}} && bun run dev) &
    wait

# Run the MCP server on stdio
mcp:
    cargo run -p comboios-mcp

# Format Rust code
fmt:
    cargo fmt --all

# Everything CI checks: fmt, clippy, tests, UI type-check and build
check: ui-install
    cargo fmt --all -- --check
    cargo clippy --workspace --all-targets -- -D warnings
    cargo test --workspace
    cd {{ui}} && bun run check
    cd {{ui}} && bun run build

# Run Rust tests
test:
    cargo test --workspace

# Release build of the server and the static UI
build: ui-install
    cargo build --release -p comboios-server
    cd {{ui}} && bun run build

# Build and start server + UI containers (UI on http://localhost:8080)
up:
    docker compose up --build -d

# Stop the containers
down:
    docker compose down
