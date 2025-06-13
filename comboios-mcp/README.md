# CP-PT MCP Server

Model Context Protocol (MCP) server for accessing Portuguese train (CP - Comboios de Portugal) information through AI assistants.

## Overview

This is an MCP server that provides AI assistants with tools to access Portuguese train data. It implements the Model Context Protocol, allowing AI assistants like Claude, GPT-4, and others to search for stations, get timetables, and retrieve train details through structured function calls. This component was built as part of learning Rust and exploring the Model Context Protocol.

## Features

- AI assistant integration through MCP
- Station search tool
- Timetable retrieval tool
- Train details tool
- Stdio transport communication
- Structured JSON responses

## Quick Start

### Prerequisites

- Rust 1.75+ (uses edition 2024)
- Cargo
- An MCP-compatible client (Claude Desktop, Continue, etc.)

### Running the MCP Server

```bash
# Run the MCP server
cargo run -p cp-pt-mcp

# For development with detailed logging
RUST_LOG=debug cargo run -p cp-pt-mcp
```

### Testing with MCP Inspector

```bash
# Install MCP Inspector (Node.js required)
npm install -g @modelcontextprotocol/inspector

# Run with inspector
npx @modelcontextprotocol/inspector cargo run -p cp-pt-mcp
```

## MCP Tools

The server provides three tools that AI assistants can use:

### get_stations_by_name

Search for train stations by name.

Parameters:
- `station_name` (string) - Name of the station to search for

Example usage by AI:
```
User: "Find train stations in Porto"
AI calls: get_stations_by_name(station_name="Porto")
```

### get_station_timetable

Get departure and arrival timetable for a specific station.

Parameters:
- `station_id` (string) - Station ID obtained from station search

Example usage by AI:
```
User: "What trains are departing from Porto Campanhã?"
AI calls: get_station_timetable(station_id="94001")
```

### get_train_details

Get detailed information about a specific train.

Parameters:
- `train_id` (integer) - Train number

Example usage by AI:
```
User: "Tell me about train 123"
AI calls: get_train_details(train_id=123)
```

## Integration with AI Clients

### Claude Desktop

Add to your Claude Desktop configuration file:

macOS: `~/Library/Application Support/Claude/claude_desktop_config.json`
Windows: `%APPDATA%\Claude\claude_desktop_config.json`

```json
{
  "mcpServers": {
    "cp-pt-trains": {
      "command": "cargo",
      "args": ["run", "-p", "cp-pt-mcp", "--manifest-path", "/path/to/cp-pt-rust/Cargo.toml"]
    }
  }
}
```

### Continue IDE Extension

Add to your Continue configuration:

```json
{
  "mcpServers": [
    {
      "name": "cp-pt-trains",
      "command": "cargo",
      "args": ["run", "-p", "cp-pt-mcp"],
      "cwd": "/path/to/cp-pt-rust"
    }
  ]
}
```

## Example AI Conversations

### Station Search
```
User: "I need to travel from Lisbon to Porto. Can you find the main stations?"

AI: I'll help you find the main train stations in Lisbon and Porto.
[AI calls get_stations_by_name for both cities]

AI: Here are the main stations:

Lisbon:
- Lisboa - Oriente (ID: 94212)
- Lisboa - Santa Apolónia (ID: 94210)

Porto:
- Porto - Campanhã (ID: 94001)
- Porto - São Bento (ID: 94004)
```

### Journey Planning
```
User: "What trains are leaving Porto Campanhã in the next few hours?"

AI: Let me check the current timetable for Porto - Campanhã station.
[AI calls get_station_timetable with station ID]

AI: Here are the upcoming departures from Porto - Campanhã:
- Train 520 to Lisboa-Oriente at 15:47 (Platform 4)
- Train 522 to Lisboa-Oriente at 17:47 (Platform 2)
- Train 524 to Lisboa-Oriente at 19:47 (Platform 4)
```

## Development

```bash
# Build the MCP server
cargo build -p cp-pt-mcp

# Run tests
cargo test -p cp-pt-mcp

# Build for release
cargo build -p cp-pt-mcp --release

# Enable debug logging
RUST_LOG=debug cargo run -p cp-pt-mcp
```

## Protocol Details

The server uses stdio (standard input/output) transport, which is the most common MCP transport method. Communication happens through JSON-RPC messages over stdin/stdout.

All tool responses follow this format:

```json
{
  "content": [
    {
      "type": "text",
      "text": "JSON response data"
    }
  ]
}
```

## Dependencies

- rmcp - Rust MCP server framework
- cp-pt - Core train data functionality
- tokio - Async runtime
- serde - JSON serialization
- tracing - Structured logging
- reqwest - HTTP client

## Troubleshooting

Common issues:

1. Server not responding
   - Check that the cargo command path is correct
   - Verify Rust and Cargo are installed
   - Enable debug logging to see startup messages

2. Tools not appearing in AI client
   - Restart the AI client after configuration changes
   - Check the MCP server configuration syntax
   - Verify the server starts without errors

3. Network errors
   - Ensure internet connectivity
   - Check if Portuguese train APIs are accessible

Debug commands:
```bash
# Test server startup
cargo run -p cp-pt-mcp 2>&1 | head -20

# Check for compilation errors
cargo check -p cp-pt-mcp
```
