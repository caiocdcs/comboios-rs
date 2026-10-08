<script lang="ts">
  import { onMount } from 'svelte';
  import { getTrainJourney } from '$lib/api';
  import { addMinutes, formatCountdown, lisbonNowMinutes, minutesUntil } from '$lib/date';
  import { liveRefresh } from '$lib/live';
  import ServiceTypeBadge from '$lib/components/ServiceTypeBadge.svelte';
  import TrainSkeleton from '$lib/components/TrainSkeleton.svelte';
  import JourneyTimeline from '$lib/components/JourneyTimeline.svelte';
  import UpdatedAgo from '$lib/components/UpdatedAgo.svelte';
  import type { JourneyStop, TrainDetails } from '$lib/types';

  export let data: { train?: TrainDetails; error?: string; trainNumber: string; date: string };

  let train: TrainDetails | undefined;
  let lastUpdated: Date | null = null;
  let refreshing = false;
  let refreshFailed = false;
  let nowMinutes = lisbonNowMinutes();
  let shareMessage = '';

  // New train (first load or navigation): take the loader's data
  $: {
    train = data.train;
    lastUpdated = new Date();
    refreshFailed = false;
  }
  $: error = data.error;

  onMount(() => {
    // A moving train changes faster than a station board
    const stopRefresh = liveRefresh(refresh, () => lastUpdated, { intervalMs: 30_000 });
    const clock = setInterval(() => (nowMinutes = lisbonNowMinutes()), 15_000);
    return () => {
      stopRefresh();
      clearInterval(clock);
    };
  });

  async function refresh() {
    if (refreshing) return;
    refreshing = true;
    try {
      train = await getTrainJourney(data.trainNumber, data.date);
      lastUpdated = new Date();
      refreshFailed = false;
    } catch {
      // Keep the last good journey on screen, flagged as stale
      refreshFailed = true;
    } finally {
      refreshing = false;
      nowMinutes = lisbonNowMinutes();
    }
  }

  function goBack() {
    window.history.back();
  }

  function retry() {
    window.location.reload();
  }

  async function share() {
    if (!train) return;
    const delay = train.delay_minutes && train.delay_minutes > 0 ? ` (+${train.delay_minutes} min)` : '';
    const text = `Train ${train.train_number} ${train.origin} → ${train.destination}${delay}`;
    const url = window.location.href;
    try {
      if (navigator.share) {
        await navigator.share({ title: `Train ${train.train_number}`, text, url });
      } else {
        await navigator.clipboard.writeText(`${text}\n${url}`);
        shareMessage = 'Link copied';
        setTimeout(() => (shareMessage = ''), 2000);
      }
    } catch {
      // Share sheet dismissed
    }
  }

  function formatDuration(duration: string | undefined): string {
    if (!duration) return '';
    if (duration.includes('h')) {
      return duration;
    }
    const parts = duration.split(':');
    if (parts.length >= 2) {
      const hours = parseInt(parts[0], 10);
      const minutes = parseInt(parts[1], 10);
      if (hours > 0) {
        return `${hours}h ${minutes}m`;
      }
      return `${minutes}m`;
    }
    return duration;
  }

  /** Best known time at a stop: predicted, else scheduled + delay */
  function expectedTime(stop: JourneyStop): string {
    if (stop.predicted_time) return stop.predicted_time;
    const delay = stop.delay_minutes ?? 0;
    return delay > 0 ? addMinutes(stop.scheduled_time, delay) : stop.scheduled_time;
  }

  $: stops = train?.stops ?? [];
  $: lastPassedIndex = stops.reduce((last, stop, i) => (stop.has_passed ? i : last), -1);
  $: trainCurrentIndex = Math.max(lastPassedIndex, 0);
  $: nextStop = stops.find((stop, i) => i > lastPassedIndex) ?? null;
  $: finalStop = stops.length > 0 ? stops[stops.length - 1] : null;
  $: notStarted = lastPassedIndex === -1;
  $: finished = stops.length > 0 && nextStop === null;

  function getStopStatus(_stop: JourneyStop, index: number): 'passed' | 'current' | 'upcoming' {
    if (index < trainCurrentIndex) return 'passed';
    if (index === trainCurrentIndex) return 'current';
    return 'upcoming';
  }
</script>

<div class="max-w-6xl mx-auto">
  <button
    class="inline-flex items-center gap-2 px-3 py-2 rounded-lg text-sm font-medium bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-200 hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors mb-4"
    on:click={goBack}
  >
    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
    </svg>
    Back
  </button>

  {#if error}
    <div class="card bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700">
      <div class="card-body items-center justify-center py-12 text-center">
        <div class="text-error-600 dark:text-error-400 mb-4">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-12 w-12 mx-auto" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
        </div>
        <h2 class="text-2xl font-bold text-gray-900 dark:text-white mb-2">Failed to Load Train Details</h2>
        <p class="text-gray-600 dark:text-gray-400 mb-6">{error}</p>
        <button class="btn btn-primary" on:click={retry}>
          Try Again
        </button>
      </div>
    </div>
  {:else if !train}
    <TrainSkeleton />
  {:else}
    <div class="card bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 mb-6">
      <div class="card-body">
        <div class="flex flex-col md:flex-row md:items-center md:justify-between gap-4 mb-4">
          <div class="flex items-center gap-3">
            <h1 class="text-2xl md:text-3xl font-bold text-gray-900 dark:text-white">
              #{train.train_number}
            </h1>
            <ServiceTypeBadge serviceType={train.service_type} />
          </div>
          <div class="flex items-center gap-2">
          <button type="button" class="btn btn-ghost btn-sm gap-1" on:click={share}>
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8.684 13.342C8.886 12.938 9 12.482 9 12c0-.482-.114-.938-.316-1.342m0 2.684a3 3 0 110-2.684m0 2.684l6.632 3.316m-6.632-6l6.632-3.316m0 0a3 3 0 105.367-2.684 3 3 0 00-5.367 2.684zm0 9.316a3 3 0 105.368 2.684 3 3 0 00-5.368-2.684z" />
            </svg>
            {shareMessage || 'Share'}
          </button>
          {#if train.delay_minutes && train.delay_minutes > 0}
            <div class="badge badge-warning gap-2 text-base px-4 py-3">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
              <span>Delayed {train.delay_minutes} min</span>
            </div>
          {:else}
            <div class="badge badge-success gap-2 text-base px-4 py-3">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
              </svg>
              <span>On Time</span>
            </div>
          {/if}
          </div>
        </div>

        <div class="grid grid-cols-2 md:grid-cols-5 gap-4 text-sm">
          <div>
            <div class="text-gray-500 dark:text-gray-400 text-xs uppercase tracking-wide">Origin</div>
            <div class="font-semibold text-gray-900 dark:text-white">{train.origin}</div>
          </div>
          <div>
            <div class="text-gray-500 dark:text-gray-400 text-xs uppercase tracking-wide">Destination</div>
            <div class="font-semibold text-gray-900 dark:text-white">{train.destination}</div>
          </div>
          <div>
            <div class="text-gray-500 dark:text-gray-400 text-xs uppercase tracking-wide">Duration</div>
            <div class="font-medium text-gray-700 dark:text-gray-300">
              {#if train.duration}
                {formatDuration(train.duration)}
                {#if train.delay_minutes && train.delay_minutes > 0}
                  <span class="text-warning-700 dark:text-warning-400">(+{train.delay_minutes} min)</span>
                {/if}
              {:else}
                -
              {/if}
            </div>
          </div>
          <div>
            <div class="text-gray-500 dark:text-gray-400 text-xs uppercase tracking-wide">Operator</div>
            <div class="font-medium text-gray-700 dark:text-gray-300">{train.operator}</div>
          </div>
          <div>
            <div class="text-gray-500 dark:text-gray-400 text-xs uppercase tracking-wide">Date</div>
            <div class="font-medium text-gray-700 dark:text-gray-300">{train.date}</div>
          </div>
        </div>
      </div>
    </div>

    <!-- Where is the train now -->
    <div class="card bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 mb-4">
      <div class="card-body p-4 gap-3">
        {#if finished && finalStop}
          <div>
            <div class="text-xs uppercase tracking-wide text-gray-500 dark:text-gray-400">Arrived</div>
            <div class="text-lg font-bold text-gray-900 dark:text-white">{finalStop.station_name}</div>
          </div>
        {:else if nextStop}
          {@const nextExpected = expectedTime(nextStop)}
          {@const nextIn = minutesUntil(nextExpected, nowMinutes)}
          <div class="flex items-start justify-between gap-3">
            <div class="min-w-0">
              <div class="text-xs uppercase tracking-wide text-gray-500 dark:text-gray-400">
                {notStarted ? `Departs ${nextStop.station_name}` : 'Next stop'}
              </div>
              {#if !notStarted}
                <div class="text-lg font-bold text-gray-900 dark:text-white truncate">{nextStop.station_name}</div>
              {/if}
              <div class="font-mono text-gray-900 dark:text-white">
                <span class="text-lg font-bold {(nextStop.delay_minutes ?? 0) > 0 ? 'text-warning-700 dark:text-warning-400' : ''}">{nextExpected}</span>
                {#if nextExpected !== nextStop.scheduled_time}
                  <span class="text-sm text-gray-500 dark:text-gray-400 line-through ml-1">{nextStop.scheduled_time}</span>
                {/if}
                {#if nextIn !== null && nextIn <= 120}
                  <span class="text-sm font-sans font-medium text-gray-700 dark:text-gray-300 ml-2">{formatCountdown(nextIn)}</span>
                {/if}
              </div>
            </div>
            {#if nextStop.platform}
              <div class="shrink-0 flex flex-col items-center">
                <div class="text-[10px] uppercase tracking-wide text-gray-500 dark:text-gray-400 mb-0.5">Plat.</div>
                <span class="platform-badge">{nextStop.platform}</span>
              </div>
            {/if}
          </div>
          {#if finalStop && finalStop !== nextStop}
            {@const finalExpected = expectedTime(finalStop)}
            <div class="text-sm text-gray-600 dark:text-gray-400 border-t border-gray-200 dark:border-gray-700 pt-3">
              Arrives {finalStop.station_name} at
              <span class="font-mono font-semibold text-gray-900 dark:text-white">{finalExpected}</span>
              {#if finalExpected !== finalStop.scheduled_time}
                <span class="font-mono line-through ml-1">{finalStop.scheduled_time}</span>
              {/if}
            </div>
          {/if}
        {/if}

        <UpdatedAgo {lastUpdated} {refreshing} failed={refreshFailed} onRefresh={refresh} />
      </div>
    </div>

    <JourneyTimeline
      stops={train.stops}
    />

    <div class="card bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 mt-6">
      <div class="card-body">
        <h2 class="text-xl font-semibold mb-4 text-gray-900 dark:text-white">
          All Stops
        </h2>

        <!-- Mobile Card View -->
        <div class="lg:hidden space-y-3">
          {#each train.stops as stop, i}
            {@const delay = stop.delay_minutes}
            {@const stopStatus = getStopStatus(stop, i)}
            {@const isFirst = i === 0}
            {@const isLast = i === train.stops.length - 1}

            <div class="p-4 rounded-lg border border-gray-200 dark:border-gray-700 {stopStatus === 'current' ? 'bg-primary-50 dark:bg-primary-900/20 border-l-4 border-l-primary-500' : 'bg-gray-50 dark:bg-gray-900/50'}">
              <div class="flex items-start justify-between mb-2">
                <div class="flex items-center gap-2">
                  <span class="w-6 h-6 rounded-full bg-gray-200 dark:bg-gray-700 flex items-center justify-center text-xs font-bold text-gray-600 dark:text-gray-300">{i + 1}</span>
                  <span class="font-semibold text-gray-900 dark:text-white">{stop.station_name}</span>
                  {#if stopStatus === 'current'}
                    <span class="w-2 h-2 bg-primary-500 rounded-full animate-ping"></span>
                  {/if}
                </div>
                <div class="text-right">
                  {#if isFirst}
                    <span class="badge badge-info badge-sm">Origin</span>
                  {:else if isLast}
                    <span class="badge {stop.has_passed ? 'badge-success' : 'badge-outline'} badge-sm">{stop.has_passed ? 'Arrived' : 'Destination'}</span>
                  {:else if stopStatus === 'current'}
                    <span class="badge badge-primary badge-sm">Current</span>
                  {:else if stopStatus === 'passed'}
                    <span class="badge badge-ghost badge-sm">Passed</span>
                  {:else}
                    <span class="badge badge-outline badge-sm">Upcoming</span>
                  {/if}
                </div>
              </div>

              <div class="flex flex-wrap gap-3 text-sm">
                <div class="flex items-center gap-1">
                  <span class="text-gray-500 dark:text-gray-400">Time:</span>
                  <span class="font-mono font-medium text-gray-900 dark:text-gray-100">{stop.scheduled_time}</span>
                  {#if stop.predicted_time}
                    <span class="text-warning-700 dark:text-warning-400">→ {stop.predicted_time}</span>
                  {/if}
                </div>
                {#if stop.platform}
                  <div class="flex items-center gap-1">
                    <span class="text-gray-500 dark:text-gray-400">Plat:</span>
                    <span class="platform-badge-sm">{stop.platform}</span>
                  </div>
                {/if}
                {#if delay && delay > 0}
                  <div class="flex items-center gap-1">
                    <span class="text-error-600 dark:text-error-400 font-medium">+{delay} min</span>
                  </div>
                {:else if stopStatus !== 'upcoming'}
                  <span class="text-success-700 dark:text-success-400 text-xs">On time</span>
                {/if}
              </div>
            </div>
          {/each}
        </div>

        <!-- Desktop Table View -->
        <div class="hidden lg:block overflow-x-auto">
          <table class="table w-full text-sm">
            <thead>
              <tr class="border-b border-gray-300 dark:border-gray-600">
                <th class="bg-gray-100 dark:bg-gray-700 text-gray-800 dark:text-gray-100 font-semibold w-8">#</th>
                <th class="bg-gray-100 dark:bg-gray-700 text-gray-800 dark:text-gray-100 font-semibold text-left">Station</th>
                <th class="bg-gray-100 dark:bg-gray-700 text-gray-800 dark:text-gray-100 font-semibold text-center w-20">Platform</th>
                <th class="bg-gray-100 dark:bg-gray-700 text-gray-800 dark:text-gray-100 font-semibold text-center w-24">Scheduled</th>
                <th class="bg-gray-100 dark:bg-gray-700 text-gray-800 dark:text-gray-100 font-semibold text-center w-24">Predicted</th>
                <th class="bg-gray-100 dark:bg-gray-700 text-gray-800 dark:text-gray-100 font-semibold text-center w-20">Delay</th>
                <th class="bg-gray-100 dark:bg-gray-700 text-gray-800 dark:text-gray-100 font-semibold text-center w-24">Status</th>
              </tr>
            </thead>
            <tbody>
              {#each train.stops as stop, i}
                {@const delay = stop.delay_minutes}
                {@const stopStatus = getStopStatus(stop, i)}
                {@const isFirst = i === 0}
                {@const isLast = i === train.stops.length - 1}

                <tr class="border-b border-gray-200 dark:border-gray-700 {stopStatus === 'current' ? 'bg-primary-50 dark:bg-primary-900/20' : ''}">
                  <td class="py-3 px-2 text-gray-600 dark:text-gray-400 text-center">
                    {#if stopStatus === 'current'}
                      <span class="inline-block w-2 h-2 bg-primary-500 rounded-full animate-pulse"></span>
                    {:else}
                      {i + 1}
                    {/if}
                  </td>
                  <td class="py-3 px-2 font-medium text-gray-900 dark:text-gray-100">{stop.station_name}</td>
                  <td class="py-3 px-2 text-center">
                    {#if stop.platform}
                      <span class="platform-badge">{stop.platform}</span>
                    {:else}
                      <span class="text-gray-500 dark:text-gray-400">-</span>
                    {/if}
                  </td>
                  <td class="py-3 px-2 text-center font-mono text-gray-900 dark:text-gray-100">{stop.scheduled_time}</td>
                  <td class="py-3 px-2 text-center">
                    {#if stop.predicted_time}
                      <span class="font-mono text-warning-700 dark:text-warning-400 font-semibold">{stop.predicted_time}</span>
                    {:else}
                      <span class="text-gray-500 dark:text-gray-400">-</span>
                    {/if}
                  </td>
                  <td class="py-3 px-2 text-center">
                    {#if delay && delay > 0}
                      <span class="badge badge-warning badge-sm">+{delay} min</span>
                    {:else if stopStatus !== 'upcoming'}
                      <span class="badge badge-success badge-sm">On time</span>
                    {:else}
                      <span class="text-gray-500 dark:text-gray-400">-</span>
                    {/if}
                  </td>
                  <td class="py-3 px-2 text-center">
                    {#if isFirst}
                      <span class="badge badge-info badge-sm">Origin</span>
                    {:else if isLast}
                      <span class="badge {stop.has_passed ? 'badge-success' : 'badge-outline'} badge-sm">{stop.has_passed ? 'Arrived' : 'Destination'}</span>
                    {:else if stopStatus === 'current'}
                      <span class="badge badge-primary badge-sm">Current</span>
                    {:else if stopStatus === 'passed'}
                      <span class="badge badge-ghost badge-sm">Passed</span>
                    {:else}
                      <span class="badge badge-outline badge-sm">Upcoming</span>
                    {/if}
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      </div>
    </div>

    {#if train.observations}
      <div class="mt-4 text-center text-sm text-gray-500 dark:text-gray-400">
        {train.observations}
      </div>
    {/if}
  {/if}
</div>

<style>
  .card {
    transition: background-color 0.2s ease;
  }
</style>
