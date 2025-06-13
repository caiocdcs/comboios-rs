# CP-PT API

REST API server for accessing Portuguese train (CP - Comboios de Portugal) information and schedules.

## Overview

This is a REST API server built with Axum that provides HTTP endpoints for accessing Portuguese train data. It serves as a web service wrapper around the cp-pt-core library. This component was built as part of learning Rust and exploring web frameworks.

## Features

- RESTful API with clean HTTP endpoints
- Built with Axum for performance
- Comprehensive logging and tracing
- CORS support for web applications
- Proper HTTP status codes and error responses
- Async/await for non-blocking I/O

## Quick Start

### Prerequisites

- Rust 1.75+ (uses edition 2024)
- Cargo

### Running the Server

```bash
# Run on default address (127.0.0.1:3000)
cargo run -p cp-pt-server

# Server will start and listen on http://127.0.0.1:3000
```

### Testing the API

```bash
# Search for stations
curl "http://127.0.0.1:3000/stations/Lisboa"

# Get station timetable
curl "http://127.0.0.1:3000/stations/94001/timetable"

# Get train details
curl "http://127.0.0.1:3000/trains/123"
```

## API Endpoints

### Station Search
```
GET /stations/{name}
```

Search for train stations by name.

Parameters:
- `name` (path) - Station name to search for

Example:
```bash
curl "http://127.0.0.1:3000/stations/Porto"
```

Response:
```json
{
  "data": [
    {
      "node_id": "94001",
      "name": "Porto - Campanhã",
      "latitude": 41.149898,
      "longitude": -8.583933
    }
  ]
}
```

### Station Timetable
```
GET /stations/{id}/timetable
```

Get departure and arrival timetable for a specific station.

Parameters:
- `id` (path) - Station ID from station search response

Example:
```bash
curl "http://127.0.0.1:3000/stations/94001/timetable"
```

Response:
```json
[
  {
    "train_number": 123,
    "departure_time": "14:30",
    "destination": "Lisboa - Oriente",
    "platform": "2",
    "delay": null
  }
]
```

### Train Details
```
GET /trains/{id}
```

Get detailed information about a specific train including all stops.

Parameters:
- `id` (path) - Train number

Example:
```bash
curl "http://127.0.0.1:3000/trains/123"
```

Response:
```json
{
  "number": 123,
  "destination": "Lisboa - Oriente",
  "stations": [
    {
      "name": "Porto - Campanhã",
      "arrival_time": "14:30",
      "departure_time": "14:30",
      "platform": "2"
    }
  ],
  "current_delay": null
}
```

## Configuration

### Server Address

The server runs on `127.0.0.1:3000` by default. To change this, modify the main.rs file:

```rust
let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
```

### Environment Variables

- `RUST_LOG` - Controls logging levels (e.g., `RUST_LOG=debug`)

## Error Responses

The API returns appropriate HTTP status codes and error messages:

### 404 Not Found
```json
{
  "error": "Station not found",
  "message": "No station found with the given ID"
}
```

### 500 Internal Server Error
```json
{
  "error": "Internal server error",
  "message": "An unexpected error occurred"
}
```

## Development

```bash
# Build the API server
cargo build -p cp-pt-server

# Run tests
cargo test -p cp-pt-server

# Build for release
cargo build -p cp-pt-server --release

# Run with debug logging
RUST_LOG=debug cargo run -p cp-pt-server
```

## Dependencies

- axum - Web framework
- tokio - Async runtime
- tower - Service middleware
- tower-http - HTTP-specific middleware
- tracing - Structured logging
- serde - Serialization
- cp-pt - Core domain logic
- reqwest - HTTP client
