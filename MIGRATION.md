# Migration Plan: Dioxus → SvelteKit

## Overview

Migrating the comboios-web frontend from Dioxus (Rust/WASM) to SvelteKit (TypeScript) for simpler deployment and maintenance.

## Why SvelteKit?

- **Simpler**: Less boilerplate than React/Vue
- **Compiled**: No virtual DOM, better performance
- **TypeScript-first**: Excellent type safety
- **Static adapter**: Build to static files, host anywhere
- **File-based routing**: Intuitive page structure

## Architecture

```
comboios-ui/ (new SvelteKit frontend)
├── src/
│   ├── lib/
│   │   ├── api.ts          # API client for Rust backend
│   │   └── types.ts        # TypeScript interfaces
│   ├── routes/
│   │   ├── +page.svelte    # Home (station search)
│   │   ├── station/
│   │   │   └── [id]/+page.svelte
│   │   └── train/
│   │       └── [id]/+page.svelte
│   └── app.html
├── static/
├── svelte.config.js        # Static adapter config
├── tailwind.config.js
└── package.json

comboios-server/ (existing Rust backend)
└── Runs on localhost:3000
```

## API Endpoints

Backend provides REST API on `http://localhost:3000`:

- `GET /stations?query={name}` - Search stations
- `GET /stations/timetable/{id}` - Get station board
- `GET /trains/{id}` - Deprecated (returns message)

## Migration Steps

1. **Setup**
   - Initialize SvelteKit with static adapter
   - Configure Tailwind CSS
   - Set up TypeScript

2. **Core Components**
   - API client with fetch
   - Station search component
   - Station timetable component
   - Train details (deprecated warning)

3. **Build & Deploy**
   - `npm run build` → generates `build/` folder
   - Serve static files with any web server

## Development

```bash
cd comboios-ui
npm install
npm run dev        # Development server
npm run build      # Production build
```

## Deployment

```bash
# Build
npm run build

# Deploy
rsync -avz build/ server:/var/www/comboios/
```

## Timeline

1. Setup project structure - 30 min
2. Create API client - 30 min
3. Build search page - 1 hour
4. Build timetable page - 1 hour
5. Styling & polish - 1 hour
6. Testing - 30 min

Total: ~4 hours
