<script lang="ts">
  import { onDestroy } from 'svelte';
  import { searchStations } from '$lib/api';
  import { favorites, recentStations } from '$lib/favorites';
  import type { Station } from '$lib/types';

  /** Shortcuts to the busiest stations, straight to their boards */
  const MAIN_STATIONS = [
    { id: '94-31039', name: 'Lisboa Oriente' },
    { id: '94-30007', name: 'Lisboa Santa Apolónia' },
    { id: '94-69005', name: 'Cais do Sodré' },
    { id: '94-59006', name: 'Lisboa Rossio' },
    { id: '94-2006', name: 'Porto Campanhã' },
    { id: '94-1008', name: 'Porto São Bento' },
    { id: '94-36004', name: 'Coimbra-B' },
    { id: '94-73007', name: 'Faro' }
  ];

  const MIN_QUERY = 2;
  const DEBOUNCE_MS = 250;

  let query = '';
  let results: Station[] = [];
  let searching = false;
  let error: string | null = null;
  let searchedFor = '';

  let timer: ReturnType<typeof setTimeout> | undefined;
  let inFlight: AbortController | undefined;

  // Search as you type: wait for a pause, and cancel any older request so a
  // slow response can never overwrite newer results
  $: scheduleSearch(query);

  function scheduleSearch(q: string) {
    clearTimeout(timer);
    const term = q.trim();
    if (term.length < MIN_QUERY) {
      inFlight?.abort();
      results = [];
      searching = false;
      error = null;
      searchedFor = '';
      return;
    }
    searching = true;
    timer = setTimeout(() => runSearch(term), DEBOUNCE_MS);
  }

  async function runSearch(term: string) {
    inFlight?.abort();
    const controller = new AbortController();
    inFlight = controller;
    try {
      const response = await searchStations(term, controller.signal);
      results = response.data;
      error = null;
      searchedFor = term;
    } catch (e) {
      if (controller.signal.aborted) return;
      error = e instanceof Error ? e.message : 'Failed to search stations';
      results = [];
    } finally {
      if (inFlight === controller) searching = false;
    }
  }

  onDestroy(() => {
    clearTimeout(timer);
    inFlight?.abort();
  });

  $: favoriteIds = new Set($favorites.map((f) => f.id));
  $: recents = $recentStations.filter((s) => !favoriteIds.has(s.id));
  $: showingSearch = query.trim().length >= MIN_QUERY;
</script>

{#snippet stationLink(station: { id: string; name: string }, starred: boolean)}
  <a
    href="/station/{station.id}"
    class="flex items-center gap-3 px-4 py-3 hover:bg-gray-50 dark:hover:bg-gray-700/50 transition-colors"
  >
    {#if starred}
      <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 shrink-0 text-platform" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
        <path d="M11.48 3.5a.56.56 0 011.04 0l2.13 5.11a.56.56 0 00.47.35l5.52.44c.5.04.7.66.32.99l-4.2 3.6a.56.56 0 00-.18.56l1.28 5.39a.56.56 0 01-.84.61l-4.72-2.89a.56.56 0 00-.59 0l-4.72 2.89a.56.56 0 01-.84-.61l1.28-5.39a.56.56 0 00-.18-.56l-4.2-3.6a.56.56 0 01.32-.99l5.52-.44a.56.56 0 00.47-.35l2.13-5.11z" />
      </svg>
    {/if}
    <span class="flex-1 min-w-0 truncate font-medium text-gray-900 dark:text-white">{station.name}</span>
    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 shrink-0 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
    </svg>
  </a>
{/snippet}

{#snippet stationList(title: string, stations: { id: string; name: string }[], starred: boolean)}
  <section class="mb-6">
    <h2 class="text-xs font-semibold uppercase tracking-wide text-gray-500 dark:text-gray-400 mb-2 px-1">{title}</h2>
    <ul class="rounded-xl overflow-hidden border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 divide-y divide-gray-200 dark:divide-gray-700">
      {#each stations as station (station.id)}
        <li>{@render stationLink(station, starred)}</li>
      {/each}
    </ul>
  </section>
{/snippet}

<div class="max-w-3xl mx-auto">
  <!-- Search: results appear as you type -->
  <div class="relative mb-6">
    <svg xmlns="http://www.w3.org/2000/svg" class="absolute left-3 top-1/2 -translate-y-1/2 h-5 w-5 text-gray-400 pointer-events-none" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
    </svg>
    <input
      type="search"
      enterkeyhint="search"
      autocomplete="off"
      autocapitalize="off"
      spellcheck="false"
      aria-label="Search stations"
      placeholder="Search stations, e.g. Oriente"
      class="w-full h-12 pl-10 pr-10 rounded-xl border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 text-gray-900 dark:text-white placeholder-gray-500 dark:placeholder-gray-400 text-base focus:outline-none focus:ring-2 focus:ring-primary-500 dark:focus:ring-primary-400"
      bind:value={query}
    />
    {#if searching}
      <span class="absolute right-3 top-1/2 -translate-y-1/2 loading loading-spinner loading-sm text-gray-400"></span>
    {/if}
  </div>

  {#if showingSearch}
    {#if error}
      <div class="alert alert-error rounded-xl">
        <span>{error}</span>
      </div>
    {:else if results.length > 0}
      {@render stationList('Stations', results, false)}
    {:else if !searching && searchedFor}
      <p class="text-center text-gray-500 dark:text-gray-400 py-8">No stations match "{searchedFor}"</p>
    {/if}
  {:else}
    {#if $favorites.length > 0}
      {@render stationList('Your stations', $favorites, true)}
    {/if}

    {#if recents.length > 0}
      {@render stationList('Recent', recents, false)}
    {/if}

    {@render stationList('Main stations', MAIN_STATIONS, false)}

    {#if $favorites.length === 0}
      <p class="text-sm text-center text-gray-500 dark:text-gray-400 px-4">
        Tip: tap ☆ on a station's board to pin it to the top of this screen.
      </p>
    {/if}
  {/if}
</div>
