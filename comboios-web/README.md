# CP-PT Viewer

Interactive web application for browsing Portuguese train (CP - Comboios de Portugal) information and schedules.

## Overview

This is a web application built with Dioxus that provides an interface for searching train stations, viewing timetables, and exploring train details. It was created as part of learning Rust and exploring modern web development with WebAssembly.

## Features

- Station search functionality
- Live timetables with departure and arrival information
- Detailed train information with route details
- Responsive design for different screen sizes
- Built with Rust and WebAssembly for performance

## Technology Stack

- Dioxus - Rust framework for building web UIs
- WebAssembly - Compiled Rust running in the browser
- Tailwind CSS - Utility-first CSS framework
- Reqwest - HTTP client for API communication
- Serde - JSON serialization and deserialization

## Quick Start

### Prerequisites

- Rust 1.75+ (uses edition 2021)
- Cargo
- `wasm32-unknown-unknown` target for web builds

### Installing WebAssembly Target

```bash
rustup target add wasm32-unknown-unknown
```

### Running the Application

```bash
# Run the web application in development mode
cargo run -p cp-pt-web --features web

# Run as a desktop application
cargo run -p cp-pt-web --features desktop

# Run for mobile (requires additional setup)
cargo run -p cp-pt-web --features mobile
```

## Application Structure

### Routes

- `/` - Search screen for finding stations
- `/station/{station_id}` - Timetable for a specific station
- `/train/{train_id}` - Detailed train information

### Components

```
src/
├── main.rs              # Application entry point and routing
├── components/          # Reusable UI components
├── views/               # Route-specific views
├── api/                 # API client code
└── domain/              # Data models and types
```

## Development

```bash
# Build css using tailwind
npm run build:css

# Build for web (development)
cargo build -p cp-pt-web

# Run locally
dx serve

# Build for web (release)
cargo build -p cp-pt-viewer --release

# Build for desktop
cargo build -p cp-pt-viewer --features desktop

# Build for mobile
cargo build -p cp-pt-viewer --features mobile
```

### Build Features

```toml
[features]
default = ["web"]
web = ["dioxus/web"]
desktop = ["dioxus/desktop"]
mobile = ["dioxus/mobile"]
```

## Assets

Static assets are located in the `assets/` directory:

```
assets/
├── favicon.ico          # Application icon
├── styling/
│   └── main.css        # Custom styles
└── tailwind.css        # Tailwind utilities
```

## Dependencies

- dioxus - Rust web framework
- console_log - Browser console logging
- reqwest - HTTP client
- serde - JSON serialization
- log - Logging interface

## Browser Compatibility

The application supports modern browsers with WebAssembly support:

- Chrome 57+
- Firefox 52+
- Safari 11+
- Edge 16+
