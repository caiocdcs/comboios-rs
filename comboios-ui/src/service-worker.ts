/// <reference types="@sveltejs/kit" />
/// <reference no-default-lib="true"/>
/// <reference lib="esnext" />
/// <reference lib="webworker" />

import { build, files, version } from '$service-worker';

const sw = self as unknown as ServiceWorkerGlobalScope;

const APP_CACHE = `app-${version}`;
const API_CACHE = 'api';

// App shell: hashed JS/CSS from the build, plus everything in static/
const APP_ASSETS = [...build, ...files, '/'];

// Read-only API routes proxied by nginx on the same origin; the last good
// response is kept so a station board still shows on a flaky connection.
const CACHEABLE_API = /^\/(stations|trains)(\/|$)/;

sw.addEventListener('install', (event) => {
  event.waitUntil(
    caches
      .open(APP_CACHE)
      .then((cache) => cache.addAll(APP_ASSETS))
      .then(() => sw.skipWaiting())
  );
});

sw.addEventListener('activate', (event) => {
  event.waitUntil(
    caches
      .keys()
      .then((keys) =>
        Promise.all(
          keys
            .filter((key) => key !== APP_CACHE && key !== API_CACHE)
            .map((key) => caches.delete(key))
        )
      )
      .then(() => sw.clients.claim())
  );
});

async function networkFirst(request: Request, cacheName: string): Promise<Response> {
  const cache = await caches.open(cacheName);
  try {
    const response = await fetch(request);
    if (response.ok) {
      cache.put(request, response.clone());
    }
    return response;
  } catch (err) {
    const cached = await cache.match(request);
    if (cached) return cached;
    throw err;
  }
}

sw.addEventListener('fetch', (event) => {
  const { request } = event;
  if (request.method !== 'GET') return;

  const url = new URL(request.url);
  if (url.origin !== sw.location.origin) return;

  // Hashed build assets and static files never change for a given version
  if (APP_ASSETS.includes(url.pathname)) {
    event.respondWith(
      caches.match(url.pathname).then((cached) => cached ?? fetch(request))
    );
    return;
  }

  if (CACHEABLE_API.test(url.pathname)) {
    event.respondWith(networkFirst(request, API_CACHE));
    return;
  }

  // Client-side routes (/station/…, /train/…) all load the SPA shell
  if (request.mode === 'navigate') {
    event.respondWith(
      fetch(request).catch(async () => (await caches.match('/')) ?? Response.error())
    );
  }
});
