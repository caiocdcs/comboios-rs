# Comboios UI

A SvelteKit-based web frontend for browsing Portuguese train (CP - Comboios de Portugal) information and schedules.

## Overview

This is a modern, lightweight web application built with SvelteKit and TypeScript. It connects to the comboios-server REST API to provide real-time train information.

## Features

- Station search by name
- Real-time timetables with departure/arrival information
- Delay information parsed from service messages
- Responsive design with Tailwind CSS + DaisyUI
- Fast static build for easy deployment

## Technology Stack

- **SvelteKit** - Modern web framework
- **TypeScript** - Type-safe JavaScript
- **Tailwind CSS** - Utility-first CSS framework
- **DaisyUI** - Tailwind components
- **Fetch API** - Native HTTP client

## Quick Start

### Prerequisites

- Bun 1.0+ (or Node.js 18+)
- Backend server running on localhost:3000

### Development

```bash
bun install
bun run dev    # http://localhost:5173
```

### Production Build

```bash
bun run build  # Outputs to build/
```

### Deployment

```bash
bun run build
rsync -avz build/ your-server:/var/www/comboios/
```

## Environment Variables

- `VITE_API_URL` - Backend API URL (default: http://localhost:3000)

## Project Structure

```
src/
├── lib/
│   ├── api.ts      # API client
│   └── types.ts    # TypeScript interfaces
├── routes/
│   ├── +page.svelte            # Home (station search)
│   ├── station/[id]/+page.svelte # Timetable view
│   └── +layout.svelte            # Root layout
└── app.html
```

## API Integration

Connects to comboios-server REST API:

- `GET /stations?query={name}` - Search stations
- `GET /stations/timetable/{id}` - Get station board

## Development Notes

This SvelteKit frontend provides a modern, static-site approach that's easier to deploy and maintain compared to the previous Dioxus-based implementation.
