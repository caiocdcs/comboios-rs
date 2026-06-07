<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { onMount, onDestroy } from 'svelte';
  import { getStationTimetable } from '$lib/api';
  import { ApiException } from '$lib/errors';
  import ServiceTypeBadge from '$lib/components/ServiceTypeBadge.svelte';
  import TrainStatusBadge from '$lib/components/TrainStatusBadge.svelte';
  import StationSkeleton from '$lib/components/StationSkeleton.svelte';
  import Pagination from '$lib/components/Pagination.svelte';
  import type { StationBoard, TrainEntry } from '$lib/types';

  export let data: { boards: StationBoard[]; stationId: string; stationName: string };

  let stationId = data.stationId;
  let boards: StationBoard[] = data.boards;
  let loading = false;
  let error: string | null = null;
  let filter = 'all';
  let refreshInterval: ReturnType<typeof setInterval>;


  let currentPage = 1;
  const itemsPerPage = 10;

  onMount(() => {
    if (boards.length === 0) {
      loadTimetable();
    }
    refreshInterval = setInterval(() => loadTimetable(true), 60000);
  });

  onDestroy(() => {
    clearInterval(refreshInterval);
  });

  async function loadTimetable(silent = false) {
    const currentStationId = $page.params.id;
    if (!currentStationId) return;

    if (!silent) {
      loading = true;
    }
    error = null;

    try {
      const response = await getStationTimetable(currentStationId);
      boards = response.data;
      stationId = currentStationId;
    } catch (e) {
      if (!silent) {
        if (e instanceof ApiException) {
          error = 'Failed to load timetable. Please try again.';
        } else {
          error = 'Something went wrong.';
        }
        boards = [];
      }
    } finally {
      if (!silent) {
        loading = false;
      }
    }
  }

  function goBack() {
    goto('/');
  }

  function getAllTrains(): TrainEntry[] {
    return boards.flatMap(board => board.trains);
  }

  function viewTrainDetails(train: TrainEntry) {
    const today = new Date().toISOString().split('T')[0];
    goto(`/train/${train.train_number}?date=${today}`);
  }

  function getTrainInfo(train: TrainEntry) {
    const arrivalTime = train.estimated_arrival || train.arrival_time;
    const departureTime = train.estimated_departure || train.departure_time;
    const displayTime = arrivalTime || departureTime || '-';
    const scheduledTime = train.estimated_arrival ? train.arrival_time : train.estimated_departure ? train.departure_time : null;
    const delayMinutes = train.delay;
    const isDelayed = delayMinutes && delayMinutes > 0;
    const isCancelled = train.observations && /supress|cancel/i.test(train.observations.toLowerCase());
    const trainStatus = isCancelled ? 'cancelled' : train.has_passed ? 'departed' : isDelayed ? 'delayed' : 'on-time';
    const movementLabel = train.is_departure ? 'Departing' : 'Arriving';
    return { arrivalTime, departureTime, displayTime, scheduledTime, delayMinutes, isDelayed, isCancelled, trainStatus, movementLabel };
  }

  // Pagination computed values
  $: allTrains = getAllTrains();
  $: sortedTrains = [...allTrains].sort((a, b) => {
    const parseTime = (t: string | null | undefined): number => {
      if (!t) return 0;
      const [h, m] = t.split(':').map(Number);
      return (h || 0) * 60 + (m || 0);
    };
    const timeA = parseTime(a.estimated_arrival || a.arrival_time || a.estimated_departure || a.departure_time);
    const timeB = parseTime(b.estimated_arrival || b.arrival_time || b.estimated_departure || b.departure_time);
    return timeA - timeB;
  });
  $: totalPages = Math.ceil(sortedTrains.length / itemsPerPage);
  $: paginatedTrains = sortedTrains.slice(
    (currentPage - 1) * itemsPerPage,
    currentPage * itemsPerPage
  );
  $: totalTrains = sortedTrains.length;
  $: stationName = data.stationName || 'Station';

  // Reset to page 1 when filters change
  $: {
    sortedTrains;
    if (currentPage > totalPages && totalPages > 0) {
      currentPage = totalPages;
    } else if (totalPages === 0) {
      currentPage = 1;
    }
  }

  function handlePageChange(page: number) {
    currentPage = page;
    // Scroll to top of table
    document.querySelector('table')?.scrollIntoView({ behavior: 'smooth' });
  }
</script>

<div class="max-w-7xl mx-auto">
  <!-- Header -->
  <div class="mb-6">
    <button
      class="inline-flex items-center gap-2 px-3 py-2 rounded-lg text-sm font-medium bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-200 hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors mb-4"
      on:click={goBack}
    >
      <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
      </svg>
      Back to stations
    </button>

    <div class="flex flex-col md:flex-row md:items-center md:justify-between">
      <div>
        <h1 class="text-3xl font-bold text-gray-900 dark:text-white">{stationName}</h1>
        <p class="text-gray-600 dark:text-gray-400">Real-time train information</p>
      </div>
      <div class="mt-4 md:mt-0">
        <div class="badge badge-primary badge-lg">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
          Live updates
        </div>
      </div>
    </div>
  </div>

  <!-- Stats Cards -->
  {#if !loading && !error && allTrains.length > 0}
    <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-6">
      <div class="card bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700">
        <div class="card-body p-4">
          <div class="text-sm text-gray-500 dark:text-gray-400">Total Trains</div>
          <div class="text-2xl font-bold text-gray-900 dark:text-white">{allTrains.length}</div>
        </div>
      </div>
      <div class="card bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700">
        <div class="card-body p-4">
          <div class="text-sm text-gray-500 dark:text-gray-400">Delayed</div>
          <div class="text-2xl font-bold text-warning-600 dark:text-warning-500">
            {allTrains.filter(t => t.delay && t.delay > 0).length}
          </div>
        </div>
      </div>
      <div class="card bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700">
        <div class="card-body p-4">
          <div class="text-sm text-gray-500 dark:text-gray-400">Showing</div>
          <div class="text-2xl font-bold text-primary-600 dark:text-primary-400">{sortedTrains.length}</div>
        </div>
      </div>
    </div>
  {/if}

  <!-- Search -->


  <!-- Loading State -->
  {#if loading}
    <StationSkeleton />
  {/if}

  <!-- Error State -->
  {#if error}
    <div class="card bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700">
      <div class="card-body">
        <div class="alert alert-error rounded-lg">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 shrink-0 stroke-current" fill="none" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
          <span>{error}</span>
        </div>
        <div class="mt-4 text-center">
          <button class="btn btn-primary" on:click={loadTimetable}>
            Try Again
          </button>
        </div>
      </div>
    </div>
  {/if}

  <!-- No Results State -->
  {#if !loading && !error && sortedTrains.length === 0 && allTrains.length > 0}
    <div class="card bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700">
      <div class="card-body py-12 text-center">
        <div class="inline-flex items-center justify-center w-16 h-16 bg-gray-100 dark:bg-gray-700 rounded-full mb-4 mx-auto">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8 text-gray-400 dark:text-gray-500" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.172 16.172a4 4 0 015.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
        </div>
        <h3 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">No matching trains</h3>
        <p class="text-gray-600 dark:text-gray-400 mb-4">No trains match your current filters or search term.</p>
        <button
          class="btn btn-primary"
          on:click={() => { filter = 'all'; }}
        >
          Clear filters
        </button>
      </div>
    </div>
  {/if}

  <!-- No Trains State -->
  {#if !loading && !error && allTrains.length === 0}
    <div class="card bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700">
      <div class="card-body py-12 text-center">
        <div class="inline-flex items-center justify-center w-16 h-16 bg-gray-100 dark:bg-gray-700 rounded-full mb-4 mx-auto">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8 text-gray-400 dark:text-gray-500" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
          </svg>
        </div>
        <h3 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">No trains available</h3>
        <p class="text-gray-600 dark:text-gray-400">There are currently no trains scheduled for this station.</p>
      </div>
    </div>
  {/if}

  <!-- Train List -->
  {#if !loading && !error && sortedTrains.length > 0}
    <div class="card bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700">
      <div class="card-body">
        <h2 class="text-xl font-semibold mb-4 text-gray-900 dark:text-white">
          {#if filter === 'departures'}
            Departing Trains
          {:else if filter === 'arrivals'}
            Arriving Trains
          {:else}
            All Trains
          {/if}
          <span class="text-gray-500 dark:text-gray-400 text-base font-normal ml-2">({sortedTrains.length})</span>
        </h2>

        <!-- Mobile Card View -->
        <div class="lg:hidden space-y-3">
          {#each paginatedTrains as train}
            {@const info = getTrainInfo(train)}
            <button
              type="button"
              class="w-full p-4 text-left rounded-lg border border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-900/50 hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
              on:click={() => viewTrainDetails(train)}
            >
              <div class="flex items-start justify-between gap-3">
                <div class="flex-1 min-w-0">
                  <div class="mb-2">
                    <ServiceTypeBadge serviceType={train.service_type} />
                  </div>
                  <div class="text-sm font-medium text-gray-900 dark:text-gray-100 truncate">
                    {train.origin_station_name}
                  </div>
                  <div class="text-xs text-gray-600 dark:text-gray-400 truncate">
                    → {train.destination_station_name}
                  </div>
                  <div class="text-xs text-gray-500 dark:text-gray-400 mt-1">
                    {info.movementLabel}
                  </div>
                </div>
                <div class="text-right flex-shrink-0">
                  <div class="font-mono text-base font-bold text-gray-900 dark:text-gray-100">
                    {info.displayTime}
                  </div>
                  {#if info.arrivalTime && info.departureTime && info.arrivalTime !== info.departureTime}
                    <div class="text-xs text-gray-500 dark:text-gray-400">departs {info.departureTime}</div>
                  {/if}
                  {#if info.isDelayed && info.scheduledTime && info.displayTime !== info.scheduledTime}
                    <div class="text-xs text-gray-500 dark:text-gray-400 line-through">{info.scheduledTime}</div>
                  {/if}
                  {#if info.isDelayed}
                    <div class="text-xs text-warning-600 dark:text-warning-400 font-medium">+{info.delayMinutes} min</div>
                  {/if}
                  {#if train.observations}
                    <div class="text-xs text-red-600 dark:text-red-400 font-medium truncate max-w-[120px]">{train.observations}</div>
                  {/if}
                  <div class="mt-1">
                    <TrainStatusBadge status={info.trainStatus} delayMinutes={info.delayMinutes} />
                  </div>
                </div>
              </div>
              <div class="mt-2 pt-2 border-t border-gray-200 dark:border-gray-700 flex justify-between text-xs text-gray-500 dark:text-gray-400">
                <span>#{train.train_number}</span>
                <span>{train.operator}</span>
              </div>
            </button>
          {/each}
        </div>

        <!-- Desktop Table View -->
        <div class="hidden lg:block overflow-x-auto">
          <div class="min-w-[800px]">
            <!-- Header Row -->
            <div class="grid grid-cols-5 gap-2 border-b-2 border-gray-300 dark:border-gray-600 pb-2 mb-2">
              <div class="text-left font-semibold text-gray-800 dark:text-gray-100 text-sm">Service</div>
              <div class="text-left font-semibold text-gray-800 dark:text-gray-100 text-sm">Route</div>
              <div class="text-left font-semibold text-gray-800 dark:text-gray-100 text-sm">Time</div>
              <div class="text-left font-semibold text-gray-800 dark:text-gray-100 text-sm">Train</div>
              <div class="text-left font-semibold text-gray-800 dark:text-gray-100 text-sm">Status</div>
            </div>
            <!-- Data Rows -->
            {#each paginatedTrains as train}
              {@const info = getTrainInfo(train)}
              <div
                class="grid grid-cols-5 gap-2 py-3 border-b border-gray-200 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-700 cursor-pointer transition-colors items-center"
                on:click={() => viewTrainDetails(train)}
              >
                <div class="text-left">
                  <ServiceTypeBadge serviceType={train.service_type} />
                </div>
                <div class="text-left min-w-0">
                  <div class="font-medium text-gray-900 dark:text-gray-100 truncate">{train.origin_station_name}</div>
                  <div class="text-sm text-gray-600 dark:text-gray-400 truncate">→ {train.destination_station_name}</div>
                  <div class="text-xs text-gray-500 dark:text-gray-400">{info.movementLabel}</div>
                </div>
                <div class="text-left">
                  <div class="font-mono text-gray-900 dark:text-gray-100">
                    {info.displayTime}{#if info.isDelayed && info.scheduledTime && info.displayTime !== info.scheduledTime}<span class="text-gray-500 dark:text-gray-400 text-xs line-through ml-1">{info.scheduledTime}</span><span class="text-warning-600 dark:text-warning-400 text-xs ml-1">+{info.delayMinutes}m</span>{/if}
                  </div>
                  {#if info.arrivalTime && info.departureTime && info.arrivalTime !== info.departureTime}
                    <div class="text-xs text-gray-500 dark:text-gray-400">departs {info.departureTime}</div>
                  {/if}
                  {#if train.observations}
                    <div class="text-xs text-red-600 dark:text-red-400 truncate">{train.observations}</div>
                  {/if}
                </div>
                <div class="text-left">
                  <div class="font-mono text-gray-900 dark:text-gray-100">#{train.train_number}</div>
                  <div class="text-xs text-gray-600 dark:text-gray-400">{train.operator}</div>
                </div>
                <div class="text-left">
                  <TrainStatusBadge status={info.trainStatus} delayMinutes={info.delayMinutes} />
                </div>
              </div>
            {/each}
          </div>
        </div>

        <!-- Pagination -->
        {#if totalPages > 1}
          <Pagination
            {currentPage}
            {totalPages}
            totalItems={sortedTrains.length}
            {itemsPerPage}
            onPageChange={handlePageChange}
          />
        {/if}
      </div>
    </div>
  {/if}
</div>
