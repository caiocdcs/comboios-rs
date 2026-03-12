<script lang="ts">
  import { goto } from '$app/navigation';
  import { searchStations } from '$lib/api';
  import type { Station } from '$lib/types';

  let query = '';
  let stations: Station[] = [];
  let loading = false;
  let error: string | null = null;

  async function handleSearch() {
    if (!query.trim()) return;
    
    loading = true;
    error = null;
    
    try {
      const response = await searchStations(query);
      stations = response.data;
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to search stations';
      stations = [];
    } finally {
      loading = false;
    }
  }

  function selectStation(station: Station) {
    goto(`/station/${station.id}`);
  }
</script>

<div class="container mx-auto px-4 py-8 max-w-2xl">
  <div class="card bg-base-100 shadow-xl">
    <div class="card-body">
      <h1 class="card-title text-3xl mb-6">Comboios de Portugal</h1>
      
      <div class="form-control w-full">
        <div class="join">
          <input
            type="text"
            placeholder="Search stations..."
            class="input input-bordered join-item flex-1"
            bind:value={query}
            on:keydown={(e) => e.key === 'Enter' && handleSearch()}
          />
          <button class="btn btn-primary join-item" on:click={handleSearch} disabled={loading}>
            {#if loading}
              <span class="loading loading-spinner"></span>
            {:else}
              Search
            {/if}
          </button>
        </div>
      </div>

      {#if error}
        <div class="alert alert-error mt-4">
          <span>{error}</span>
        </div>
      {/if}

      {#if stations.length > 0}
        <div class="mt-6">
          <h2 class="text-xl font-semibold mb-4">Results</h2>
          <div class="space-y-2">
            {#each stations as station}
              <button
                class="btn btn-ghost justify-start w-full text-left"
                on:click={() => selectStation(station)}
              >
                <span class="font-medium">{station.name}</span>
                <span class="text-sm text-gray-500 ml-auto">{station.id}</span>
              </button>
            {/each}
          </div>
        </div>
      {:else if !loading && query && !error}
        <div class="text-center text-gray-500 mt-6">
          No stations found matching "{query}"
        </div>
      {/if}
    </div>
  </div>
</div>
