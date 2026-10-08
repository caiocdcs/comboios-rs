<script lang="ts">
  import { onMount } from 'svelte';
  import { getTrainJourney } from '$lib/api';
  import { formatCountdown, lisbonNowMinutes, minutesUntil } from '$lib/date';
  import { expectedStopTime, lastPassedIndex } from '$lib/journey';
  import { parseService } from '$lib/service';
  import { liveRefresh } from '$lib/live';
  import TrainSkeleton from '$lib/components/TrainSkeleton.svelte';
  import JourneyTimeline from '$lib/components/JourneyTimeline.svelte';
  import UpdatedAgo from '$lib/components/UpdatedAgo.svelte';
  import type { TrainDetails } from '$lib/types';

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

  $: stops = train?.stops ?? [];
  $: passedIndex = lastPassedIndex(stops);
  $: nextStop = stops.find((_stop, i) => i > passedIndex) ?? null;
  $: finalStop = stops.length > 0 ? stops[stops.length - 1] : null;
  $: notStarted = passedIndex === -1;
  $: finished = stops.length > 0 && nextStop === null;
</script>

<svelte:head>
  <title>{train ? `${parseService(train.service_type).code} ${train.train_number}` : `Train ${data.trainNumber}`} · Comboios</title>
</svelte:head>

<div class="max-w-xl mx-auto">
  <button type="button" class="inline-flex items-center gap-1 text-sm font-semibold text-primary-700 dark:text-primary-300 mb-2" on:click={goBack}>
    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M15 19l-7-7 7-7" />
    </svg>
    Back
  </button>

  {#if error}
    <div class="rounded-xl border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 px-4 py-8 text-center">
      <p class="font-semibold text-gray-900 dark:text-white mb-1">Couldn't load train {data.trainNumber}</p>
      <p class="text-sm text-gray-600 dark:text-gray-400 mb-4">It may not run today, or the connection dropped. Try again.</p>
      <button class="btn btn-primary" on:click={retry}>Try again</button>
    </div>
  {:else if !train}
    <TrainSkeleton />
  {:else}
    {@const service = parseService(train.service_type)}
    {@const late = train.delay_minutes && train.delay_minutes > 0 ? train.delay_minutes : 0}
    <header class="mb-4">
      <div class="flex items-start justify-between gap-3">
        <div class="min-w-0">
          <h1 class="text-2xl font-extrabold tracking-tight tabular-nums" title={service.name}>
            {service.code} {train.train_number}
          </h1>
          <p class="text-base text-gray-700 dark:text-gray-300">
            {train.origin} → {train.destination}
          </p>
        </div>
        <button
          type="button"
          class="btn btn-ghost btn-circle -mr-2 shrink-0 text-gray-600 dark:text-gray-300"
          aria-label="Share this train"
          title={shareMessage || 'Share'}
          on:click={share}
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8.684 13.342C8.886 12.938 9 12.482 9 12c0-.482-.114-.938-.316-1.342m0 2.684a3 3 0 110-2.684m0 2.684l6.632 3.316m-6.632-6l6.632-3.316m0 0a3 3 0 105.367-2.684 3 3 0 00-5.367 2.684zm0 9.316a3 3 0 105.368 2.684 3 3 0 00-5.368-2.684z" />
          </svg>
        </button>
      </div>
      <div class="flex flex-wrap items-center gap-2 mt-2 text-sm">
        {#if !finished}
          <span class="badge {late ? 'badge-warning' : 'badge-success'} font-bold">
            {late ? `${late} min late` : 'On time'}
          </span>
        {/if}
        {#if shareMessage}
          <span class="text-gray-500 dark:text-gray-400" role="status">{shareMessage}</span>
        {/if}
        {#if train.duration}
          <span class="text-gray-500 dark:text-gray-400">Journey takes {formatDuration(train.duration)}</span>
        {/if}
      </div>
      {#if train.observations}
        <p class="mt-2 text-sm text-gray-600 dark:text-gray-400">{train.observations}</p>
      {/if}
    </header>

    <!-- Where is the train now: the one card on this page -->
    <section class="rounded-xl border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 p-4 mb-4" aria-label="Train position">
      {#if finished && finalStop}
        <p class="text-sm text-gray-500 dark:text-gray-400">Arrived at</p>
        <p class="text-lg font-extrabold text-gray-900 dark:text-white">{finalStop.station_name}</p>
      {:else if nextStop}
        {@const nextExpected = expectedStopTime(nextStop)}
        {@const nextIn = minutesUntil(nextExpected, nowMinutes)}
        <div class="flex items-start justify-between gap-3">
          <div class="min-w-0">
            <p class="text-sm text-gray-500 dark:text-gray-400">{notStarted ? 'Leaves' : 'Next stop'}</p>
            <p class="text-lg font-extrabold text-gray-900 dark:text-white truncate">{nextStop.station_name}</p>
            <p class="tabular-nums">
              <span class="text-lg font-extrabold {(nextStop.delay_minutes ?? 0) > 0 ? 'text-warning-700 dark:text-warning-400' : 'text-gray-900 dark:text-white'}">{nextExpected}</span>
              {#if nextExpected !== nextStop.scheduled_time}
                <span class="text-sm text-gray-500 dark:text-gray-400 line-through ml-1">{nextStop.scheduled_time}</span>
              {/if}
              {#if nextIn !== null && nextIn <= 120}
                <span class="text-sm font-semibold text-gray-700 dark:text-gray-300 ml-2">{formatCountdown(nextIn)}</span>
              {/if}
            </p>
          </div>
          {#if nextStop.platform}
            <span class="platform-badge shrink-0" aria-label="Platform {nextStop.platform}">{nextStop.platform}</span>
          {/if}
        </div>
        {#if finalStop && finalStop !== nextStop}
          {@const finalExpected = expectedStopTime(finalStop)}
          <p class="mt-3 pt-3 border-t border-gray-200 dark:border-gray-700 text-sm text-gray-600 dark:text-gray-400">
            Arrives at {finalStop.station_name}
            <span class="tabular-nums font-bold text-gray-900 dark:text-white ml-1">{finalExpected}</span>
            {#if finalExpected !== finalStop.scheduled_time}
              <span class="tabular-nums line-through ml-1">{finalStop.scheduled_time}</span>
            {/if}
          </p>
        {/if}
      {/if}

      <div class="mt-3">
        <UpdatedAgo {lastUpdated} {refreshing} failed={refreshFailed} onRefresh={refresh} />
      </div>
    </section>

    <JourneyTimeline stops={train.stops} />
  {/if}
</div>
