# comboios-rs

Unofficial Rust client for Portuguese train data (CP / Infraestruturas de Portugal), with a REST API server, MCP server, and SvelteKit web UI.

## Workspace

| Crate | Description |
|---|---|
| `comboios-core` | Core library. Wraps the CP API Gateway and the Infraestruturas de Portugal public API. Handles credential fetching, caching, request routing, and mapping raw responses to typed domain models (`TrainJourney`, `StationBoard`, `JourneyStop`, etc.). |
| `comboios-server` | Axum HTTP server that exposes `comboios-core` as a REST API. Rotates CP credentials automatically in the background. |
| `comboios-mcp` | MCP (Model Context Protocol) server. Exposes station search, timetables, and train journeys as tools consumable by AI assistants. |
| `comboios-ui` | SvelteKit frontend. Station search, live departure/arrival boards, and a stop-by-stop train journey view with real-time delay indicators. |

## Running

**API server**
```bash
cargo run -p comboios-server
```

**Web UI**
```bash
cd comboios-ui
bun install && bun run dev
```

**MCP server**
```bash
cargo run -p comboios-mcp
```

## API Endpoints

| Method | Path | Description |
|---|---|---|
| GET | `/ping` | Health check |
| GET | `/stations?query=Lisboa` | Search stations by name |
| GET | `/stations/timetable/{id}` | Live departure/arrival board |
| GET | `/trains/{id}/journey` | Train journey with stop-by-stop status |
| GET | `/diagnostics` | CP and IP API reachability |
| GET | `/refresh` | Force CP credential rotation |

## Configuration

All settings have defaults and can be overridden via environment variables.

| Variable | Default | Description |
|---|---|---|
| `HOST` | `0.0.0.0` | Bind address |
| `PORT` | `3000` | Port |
| `RUST_LOG` | `comboios_server=debug,tower_http=debug` | Log filter |
| `REQUEST_TIMEOUT_SECS` | `30` | Per-request timeout |
| `CREDENTIAL_REFRESH_SECS` | `3300` | CP credential rotation interval |
| `CP_API_URL` | `https://api-gateway.cp.pt/cp/services/travel-api` | CP base URL |
| `IP_API_URL` | `https://www.infraestruturasdeportugal.pt` | IP base URL |

## Library Usage

```toml
[dependencies]
comboios-core = { path = "comboios-core" }
```

```rust
use comboios_core::Comboios;

#[tokio::main]
async fn main() -> Result<(), comboios_core::Error> {
    let client = Comboios::new().await?;

    let stations = client.search_stations("Porto").await?;
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let board = client.get_station_timetable("94-31039", &today, None).await?;
    let journey = client.get_train_journey("530", &today).await?;

    Ok(())
}
```

`Comboios::new()` scrapes live credentials from `cp.pt` on startup and caches them. The server rotates them automatically every 55 minutes.

## Data Sources

- **CP API Gateway** (`api-gateway.cp.pt`) — timetables, train journeys, real-time delays. Credentials are fetched automatically from `cp.pt`.
- **Infraestruturas de Portugal** (`infraestruturasdeportugal.pt`) — public API, used as fallback for train journey data.

Both are unofficial endpoints. They may change without notice.

## Docker

```bash
docker build -t comboios-rs .
docker run -p 3000:3000 comboios-rs
```

## Development

```bash
cargo build
cargo test
cargo clippy -- -W clippy::pedantic
```
