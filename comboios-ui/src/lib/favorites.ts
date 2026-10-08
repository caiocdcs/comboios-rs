import { writable } from 'svelte/store';

export interface FavoriteStation {
  id: string;
  name: string;
}

const STORAGE_KEY = 'favoriteStations';

function load(): FavoriteStation[] {
  if (typeof localStorage === 'undefined') return [];
  try {
    const saved = JSON.parse(localStorage.getItem(STORAGE_KEY) ?? '[]');
    return Array.isArray(saved) ? saved : [];
  } catch {
    return [];
  }
}

/** Stations pinned to the home screen, persisted in localStorage */
export const favorites = writable<FavoriteStation[]>(load());

favorites.subscribe((list) => {
  if (typeof localStorage === 'undefined') return;
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(list));
  } catch {
    // Storage full or disabled (private mode): favourites just won't persist
  }
});

export function toggleFavorite(station: FavoriteStation) {
  favorites.update((list) =>
    list.some((f) => f.id === station.id)
      ? list.filter((f) => f.id !== station.id)
      : [...list, station]
  );
}

const RECENT_KEY = 'recentStations';
const MAX_RECENT = 5;

function loadRecent(): FavoriteStation[] {
  if (typeof localStorage === 'undefined') return [];
  try {
    const saved = JSON.parse(localStorage.getItem(RECENT_KEY) ?? '[]');
    return Array.isArray(saved) ? saved : [];
  } catch {
    return [];
  }
}

/** Stations opened most recently, newest first */
export const recentStations = writable<FavoriteStation[]>(loadRecent());

recentStations.subscribe((list) => {
  if (typeof localStorage === 'undefined') return;
  try {
    localStorage.setItem(RECENT_KEY, JSON.stringify(list));
  } catch {
    // Storage unavailable: recents just won't persist
  }
});

export function addRecentStation(station: FavoriteStation) {
  recentStations.update((list) =>
    [station, ...list.filter((s) => s.id !== station.id)].slice(0, MAX_RECENT)
  );
}
