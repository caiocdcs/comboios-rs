<script lang="ts">
  import { goto } from '$app/navigation';
  import { searchStations } from '$lib/api';
  import SearchInput from '$lib/components/SearchInput.svelte';
  import StationCard from '$lib/components/StationCard.svelte';
  import type { Station } from '$lib/types';

  let query = '';
  let stations: Station[] = [];
  let loading = false;
  let error: string | null = null;
  let searchHistory: string[] = [];

  // Load search history from localStorage on component mount
  $: if (typeof window !== 'undefined' && localStorage) {
    try {
      const saved = localStorage.getItem('searchHistory');
      if (saved) {
        searchHistory = JSON.parse(saved);
      }
    } catch (e) {
      console.warn('Failed to parse search history:', e);
      searchHistory = [];
    }
  }

  async function handleSearch() {
    if (!query.trim()) return;
    
    loading = true;
    error = null;
    
    try {
      const response = await searchStations(query);
      stations = response.data;
      
      // Add to search history
      if (typeof window !== 'undefined') {
        if (!searchHistory.includes(query)) {
          searchHistory = [query, ...searchHistory.slice(0, 4)]; // Keep only last 5
          localStorage.setItem('searchHistory', JSON.stringify(searchHistory));
        }
      }
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to search stations';
      stations = [];
    } finally {
      loading = false;
    }
  }

  function handleStationSelect(e: CustomEvent<{ id: string; name: string }>) {
    goto(`/station/${e.detail.id}`);
  }

  function searchFromHistory(term: string) {
    query = term;
    handleSearch();
  }

  function clearHistory() {
    searchHistory = [];
    if (typeof window !== 'undefined') {
      localStorage.removeItem('searchHistory');
    }
  }

  function handleSearchInput(e: CustomEvent<{ value: string }>) {
    query = e.detail.value;
  }
</script>

<div class="max-w-4xl mx-auto">
  <!-- Hero Section -->
  <div class="text-center mb-12 fade-in">
    <div class="inline-flex items-center justify-center w-16 h-16 bg-primary-100 dark:bg-primary-900/20 rounded-full mb-6">
      <svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8 text-primary-600 dark:text-primary-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8" />
      </svg>
    </div>
    <h1 class="text-4xl md:text-5xl font-bold text-gray-900 dark:text-white mb-4">Comboios de Portugal</h1>
    <p class="text-xl text-gray-600 dark:text-gray-300 max-w-2xl mx-auto">
      Find train schedules, station information, and real-time updates for all CP services across Portugal.
    </p>
  </div>

  <!-- Search Section -->
  <div class="card bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 mb-8 slide-up">
    <div class="card-body p-6 md:p-8">
      <div class="form-control w-full">
        <label class="label">
          <span class="label-text text-lg font-medium text-gray-900 dark:text-white">Search for a station</span>
        </label>
        <SearchInput 
          placeholder="Enter station name (e.g., Lisboa, Porto, Coimbra...)"
          bind:value={query}
          loading={loading}
          on:input={handleSearchInput}
          on:search={handleSearch}
        />
      </div>

      {#if error}
        <div class="alert alert-error mt-4 rounded-lg">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 shrink-0 stroke-current" fill="none" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
          <span>{error}</span>
        </div>
      {/if}
    </div>
  </div>

  <!-- Search History -->
  {#if searchHistory.length > 0}
    <div class="card bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 mb-8 fade-in">
      <div class="card-body">
        <div class="flex justify-between items-center mb-4">
          <h2 class="text-xl font-semibold text-gray-900 dark:text-white">Recent Searches</h2>
          <button 
            class="text-sm text-gray-500 dark:text-gray-400 hover:text-primary-600 dark:hover:text-primary-400 transition-colors"
            on:click={clearHistory}
          >
            Clear history
          </button>
        </div>
        <div class="flex flex-wrap gap-2">
          {#each searchHistory as term}
            <button
              class="badge badge-outline px-3 py-2 rounded-full hover:bg-primary-100 dark:hover:bg-primary-900/30 hover:text-primary-700 dark:hover:text-primary-300 transition-colors"
              on:click={() => searchFromHistory(term)}
            >
              {term}
            </button>
          {/each}
        </div>
      </div>
    </div>
  {/if}

  <!-- Results Section -->
  {#if stations.length > 0}
    <div class="card bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 fade-in">
      <div class="card-body">
        <h2 class="text-2xl font-semibold mb-6 flex items-center text-gray-900 dark:text-white">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 mr-2 text-primary-600 dark:text-primary-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z" />
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z" />
          </svg>
          Stations Found ({stations.length})
        </h2>
        <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
          {#each stations as station}
            <StationCard 
              id={station.id} 
              name={station.name} 
              on:select={handleStationSelect}
            />
          {/each}
        </div>
      </div>
    </div>
  {:else if !loading && query && !error}
    <div class="card bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 fade-in">
      <div class="card-body text-center py-12">
        <div class="inline-flex items-center justify-center w-16 h-16 bg-gray-100 dark:bg-gray-700 rounded-full mb-4">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8 text-gray-400 dark:text-gray-500" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.172 16.172a4 4 0 015.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
        </div>
        <h3 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">No stations found</h3>
        <p class="text-gray-600 dark:text-gray-400">We couldn't find any stations matching "{query}". Try a different search term.</p>
      </div>
    </div>
  {/if}

  <!-- Popular Stations -->
  {#if !query && !loading && stations.length === 0 && !error}
    <div class="card bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 fade-in">
      <div class="card-body">
        <h2 class="text-2xl font-semibold mb-6 flex items-center text-gray-900 dark:text-white">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 mr-2 text-primary-600 dark:text-primary-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3.055 11H5a2 2 0 012 2v1a2 2 0 002 2 2 2 0 012 2v2.945M8 3.935V5.5A2.5 2.5 0 0010.5 8h.5a2 2 0 012 2 2 2 0 104 0 2 2 0 012-2h1.064M15 20.488V18a2 2 0 012-2h3.064M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
          Popular Stations
        </h2>
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
          <button 
            class="p-4 rounded-xl border border-gray-200 dark:border-gray-700 hover:border-primary-300 dark:hover:border-primary-500 hover:bg-primary-50 dark:hover:bg-primary-900/20 transition-all duration-200 text-left"
            on:click={() => { query = 'Lisboa'; handleSearch(); }}
          >
            <h3 class="font-semibold text-lg text-gray-900 dark:text-white">Lisboa</h3>
            <p class="text-sm text-gray-500 dark:text-gray-400">Capital city stations</p>
          </button>
          <button 
            class="p-4 rounded-xl border border-gray-200 dark:border-gray-700 hover:border-primary-300 dark:hover:border-primary-500 hover:bg-primary-50 dark:hover:bg-primary-900/20 transition-all duration-200 text-left"
            on:click={() => { query = 'Porto'; handleSearch(); }}
          >
            <h3 class="font-semibold text-lg text-gray-900 dark:text-white">Porto</h3>
            <p class="text-sm text-gray-500 dark:text-gray-400">Northern Portugal</p>
          </button>
          <button 
            class="p-4 rounded-xl border border-gray-200 dark:border-gray-700 hover:border-primary-300 dark:hover:border-primary-500 hover:bg-primary-50 dark:hover:bg-primary-900/20 transition-all duration-200 text-left"
            on:click={() => { query = 'Coimbra'; handleSearch(); }}
          >
            <h3 class="font-semibold text-lg text-gray-900 dark:text-white">Coimbra</h3>
            <p class="text-sm text-gray-500 dark:text-gray-400">Central Portugal</p>
          </button>
        </div>
      </div>
    </div>
  {/if}
</div>