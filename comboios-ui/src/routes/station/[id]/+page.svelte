<script lang="ts">
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';
  import { getStationTimetable } from '$lib/api';
  import { addMinutes, formatCountdown, lisbonNowMinutes, lisbonToday, minutesUntil } from '$lib/date';
  import { favorites, toggleFavorite } from '$lib/favorites';
  import { liveRefresh } from '$lib/live';
  import { ApiException } from '$lib/errors';
  import ServiceTypeBadge from '$lib/components/ServiceTypeBadge.svelte';
  import StationSkeleton from '$lib/components/StationSkeleton.svelte';
  import UpdatedAgo from '$lib/components/UpdatedAgo.svelte';
  import type { StationBoard, TrainEntry } from '$lib/types';

  export let data: { boards: StationBoard[]; stationId: string; stationName: string };

  type Mode = 'departures' | 'arrivals';

  /** How far ahead the list initially reaches; "Show later" extends it */
  const WINDOW_STEP_MINUTES = 120;
  /** Always show at least this many trains, even on a quiet line */
  const MIN_VISIBLE = 8;

  let boards: StationBoard[] = [];
  let lastUpdated: Date | null = null;
  let loading = false;
  let refreshing = false;
  let refreshFailed = false;
  let error: string | null = null;
  let mode: Mode = 'departures';
  let windowMinutes = WINDOW_STEP_MINUTES;
  let nowMinutes = lisbonNowMinutes();

  // New station (first load or navigating between stations): take the loader's data
  $: {
    boards = data.boards;
    lastUpdated = new Date();
    refreshFailed = false;
    windowMinutes = WINDOW_STEP_MINUTES;
  }

  onMount(() => {
    const stopRefresh = liveRefresh(() => loadTimetable(true), () => lastUpdated);
    // Countdowns ("in 4 min") follow the clock between refreshes
    const clock = setInterval(() => (nowMinutes = lisbonNowMinutes()), 15_000);
    return () => {
      stopRefresh();
      clearInterval(clock);
    };
  });

  async function loadTimetable(background = false) {
    if (refreshing || loading) return;
    if (background) refreshing = true;
    else loading = true;
    error = null;

    try {
      const response = await getStationTimetable(data.stationId);
      boards = response.data;
      lastUpdated = new Date();
      refreshFailed = false;
    } catch (e) {
      if (background && boards.length > 0) {
        // Keep showing the last good board, flagged as stale
        refreshFailed = true;
      } else {
        error = e instanceof ApiException ? 'Failed to load timetable. Please try again.' : 'Something went wrong.';
        boards = [];
      }
    } finally {
      loading = false;
      refreshing = false;
      nowMinutes = lisbonNowMinutes();
    }
  }

  function viewTrainDetails(train: TrainEntry) {
    goto(`/train/${train.train_number}?date=${lisbonToday()}`);
  }

  interface Row {
    train: TrainEntry;
    /** Timetabled time at this station for the selected mode */
    scheduled: string;
    /** Best known actual time: CP estimate, else scheduled + delay */
    expected: string;
    delay: number;
    cancelled: boolean;
    passed: boolean;
    /** Minutes from now until `expected` (negative once gone) */
    until: number;
  }

  function toRow(train: TrainEntry, m: Mode, now: number): Row | null {
    const scheduled = m === 'departures' ? train.departure_time : train.arrival_time;
    if (!scheduled) return null; // terminates here (no departure) / starts here (no arrival)

    const delay = train.delay && train.delay > 0 ? train.delay : 0;
    const estimate = m === 'departures' ? train.estimated_departure : train.estimated_arrival;
    const expected = estimate || (delay ? addMinutes(scheduled, delay) : scheduled);
    const cancelled = !!train.observations && /supress|cancel/i.test(train.observations);

    return {
      train,
      scheduled,
      expected,
      delay,
      cancelled,
      passed: train.has_passed,
      until: minutesUntil(expected, now) ?? 0
    };
  }

  $: allTrains = boards.flatMap((board) => board.trains);
  $: rows = allTrains
    .map((t) => toRow(t, mode, nowMinutes))
    .filter((r): r is Row => r !== null)
    .sort((a, b) => a.until - b.until);
  $: visibleRows = rows.filter((r, i) => i < MIN_VISIBLE || r.until <= windowMinutes);
  $: hiddenCount = rows.length - visibleRows.length;
  $: delayedCount = rows.filter((r) => r.delay > 0 && !r.passed).length;
  $: stationName = data.stationName || 'Station';
  $: isFavorite = $favorites.some((f) => f.id === data.stationId);
</script>

<div class="max-w-3xl mx-auto">
  <!-- Header -->
  <div class="mb-4">
    <a
      href="/"
      class="inline-flex items-center gap-2 px-3 py-2 rounded-lg text-sm font-medium bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-200 hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors mb-3"
    >
      <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
      </svg>
      Stations
    </a>

    <div class="flex items-start justify-between gap-3">
      <h1 class="text-2xl md:text-3xl font-bold text-gray-900 dark:text-white">{stationName}</h1>
      <button
        type="button"
        class="btn btn-ghost btn-circle shrink-0 {isFavorite ? 'text-yellow-500' : 'text-gray-400'}"
        aria-label={isFavorite ? 'Remove from favourites' : 'Add to favourites'}
        aria-pressed={isFavorite}
        on:click={() => toggleFavorite({ id: data.stationId, name: stationName })}
      >
        <svg xmlns="http://www.w3.org/2000/svg" class="h-7 w-7" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2" fill={isFavorite ? 'currentColor' : 'none'}>
          <path stroke-linecap="round" stroke-linejoin="round" d="M11.48 3.5a.56.56 0 011.04 0l2.13 5.11a.56.56 0 00.47.35l5.52.44c.5.04.7.66.32.99l-4.2 3.6a.56.56 0 00-.18.56l1.28 5.39a.56.56 0 01-.84.61l-4.72-2.89a.56.56 0 00-.59 0l-4.72 2.89a.56.56 0 01-.84-.61l1.28-5.39a.56.56 0 00-.18-.56l-4.2-3.6a.56.56 0 01.32-.99l5.52-.44a.56.56 0 00.47-.35l2.13-5.11z" />
        </svg>
      </button>
    </div>
  </div>

  <!-- Departures / Arrivals -->
  <div role="tablist" class="grid grid-cols-2 gap-1 p-1 mb-3 rounded-xl bg-gray-200 dark:bg-gray-800">
    {#each [['departures', 'Departures'], ['arrivals', 'Arrivals']] as [value, label]}
      <button
        type="button"
        role="tab"
        aria-selected={mode === value}
        class="py-2 rounded-lg text-sm font-semibold transition-colors {mode === value
          ? 'bg-white dark:bg-gray-700 text-gray-900 dark:text-white shadow-sm'
          : 'text-gray-600 dark:text-gray-400'}"
        on:click={() => {
          mode = value === 'arrivals' ? 'arrivals' : 'departures';
          windowMinutes = WINDOW_STEP_MINUTES;
        }}
      >
        {label}
      </button>
    {/each}
  </div>

  {#if !loading && !error}
    <div class="mb-3">
      <UpdatedAgo {lastUpdated} {refreshing} failed={refreshFailed} onRefresh={() => loadTimetable(true)} />
    </div>
  {/if}

  {#if loading}
    <StationSkeleton />
  {:else if error}
    <div class="card bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700">
      <div class="card-body text-center">
        <div class="alert alert-error rounded-lg">
          <span>{error}</span>
        </div>
        <div class="mt-4">
          <button class="btn btn-primary" on:click={() => loadTimetable()}>Try Again</button>
        </div>
      </div>
    </div>
  {:else if rows.length === 0}
    <div class="card bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700">
      <div class="card-body py-12 text-center">
        <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-1">No upcoming {mode}</h3>
        <p class="text-gray-600 dark:text-gray-400">There are no {mode} scheduled at this station right now.</p>
      </div>
    </div>
  {:else}
    <p class="text-sm text-gray-500 dark:text-gray-400 mb-2">
      {rows.length} {mode}{#if delayedCount > 0} · <span class="text-warning-600 dark:text-warning-400 font-medium">{delayedCount} delayed</span>{/if}
    </p>

    <ul class="rounded-xl overflow-hidden border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 divide-y divide-gray-200 dark:divide-gray-700">
      {#each visibleRows as row (`${row.train.train_number}-${row.scheduled}`)}
        {@const t = row.train}
        <li>
          <button
            type="button"
            class="w-full flex items-center gap-3 px-4 py-3 text-left hover:bg-gray-50 dark:hover:bg-gray-700/50 transition-colors {row.passed ? 'opacity-50' : ''}"
            on:click={() => viewTrainDetails(t)}
          >
            <!-- Time: live time first, timetabled time struck through when it changed -->
            <div class="w-16 shrink-0 font-mono leading-tight">
              <div class="text-lg font-bold {row.cancelled
                ? 'line-through text-error'
                : row.delay > 0
                  ? 'text-warning-600 dark:text-warning-400'
                  : 'text-gray-900 dark:text-white'}">
                {row.expected}
              </div>
              {#if row.expected !== row.scheduled && !row.cancelled}
                <div class="text-xs text-gray-500 dark:text-gray-400 line-through">{row.scheduled}</div>
              {/if}
            </div>

            <!-- Where it goes (departures) / comes from (arrivals) -->
            <div class="flex-1 min-w-0">
              <div class="font-semibold text-gray-900 dark:text-white truncate">
                {mode === 'departures' ? t.destination_station_name : t.origin_station_name}
              </div>
              <div class="flex items-center gap-2 mt-0.5 text-xs text-gray-500 dark:text-gray-400">
                <ServiceTypeBadge serviceType={t.service_type} />
                <span>{t.train_number}</span>
                {#if row.cancelled}
                  <span class="text-error font-semibold">Cancelled</span>
                {:else if row.passed}
                  <span>{mode === 'departures' ? 'Departed' : 'Arrived'}</span>
                {:else}
                  {#if row.delay > 0}
                    <span class="text-warning-600 dark:text-warning-400 font-semibold">+{row.delay} min</span>
                  {/if}
                  {#if row.until <= 60}
                    <span class="font-medium text-gray-700 dark:text-gray-300">{formatCountdown(row.until)}</span>
                  {/if}
                {/if}
              </div>
              {#if t.observations && !row.cancelled}
                <div class="text-xs text-error mt-0.5 truncate">{t.observations}</div>
              {/if}
            </div>

            <!-- Platform: the thing you need on the concourse -->
            <div class="shrink-0 text-center w-12">
              <div class="text-[10px] uppercase tracking-wide text-gray-500 dark:text-gray-400">Plat.</div>
              <div class="text-xl font-bold font-mono {t.platform ? 'text-primary-700 dark:text-primary-300' : 'text-gray-400'}">
                {t.platform || '–'}
              </div>
            </div>
          </button>
        </li>
      {/each}
    </ul>

    {#if hiddenCount > 0}
      <button
        type="button"
        class="btn btn-ghost w-full mt-2"
        on:click={() => (windowMinutes += WINDOW_STEP_MINUTES)}
      >
        Show later {mode} ({hiddenCount})
      </button>
    {/if}
  {/if}
</div>
