<script lang="ts">
  import { onMount, tick } from 'svelte';
  import type { TrainDetails } from '$lib/types';

  export let stops: TrainDetails['stops'] = [];

  let scrollContainer: HTMLDivElement;

  $: currentIndex = (() => {
    let lastPassedIdx = -1;
    for (let i = 0; i < stops.length; i++) {
      if (stops[i].has_passed) lastPassedIdx = i;
    }
    return lastPassedIdx !== -1 ? lastPassedIdx : 0;
  })();

  $: currentStop = stops[currentIndex] || null;
  $: overallDelay = currentStop?.delay_minutes || 0;

  function getStopStatus(index: number): 'passed' | 'current' | 'upcoming' {
    if (index < currentIndex) return 'passed';
    if (index === currentIndex) return 'current';
    return 'upcoming';
  }

  function getLineClass(idx: number): string {
    if (idx < currentIndex) return 'bg-green-700';
    if (idx === currentIndex) return 'bg-primary-500';
    return 'bg-gray-300 dark:bg-gray-600';
  }

  async function scrollToCurrent() {
    await tick();
    if (!scrollContainer || stops.length === 0) return;
    const targetIdx = currentIndex !== -1 ? currentIndex : stops.length - 1;
    const el = scrollContainer.querySelectorAll('[data-index]')[targetIdx] as HTMLElement;
    if (!el) return;
    const cr = scrollContainer.getBoundingClientRect();
    const er = el.getBoundingClientRect();
    const targetScroll =
      er.left - cr.left + scrollContainer.scrollLeft - cr.width / 2 + er.width / 2;
    scrollContainer.scrollTo({ left: Math.max(0, targetScroll), behavior: 'smooth' });
  }

  onMount(scrollToCurrent);
  $: if (stops.length > 0 && scrollContainer) scrollToCurrent();
</script>

<div class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 overflow-hidden">

  <!-- Header -->
  <div class="px-4 py-3 border-b border-gray-100 dark:border-gray-700">
    <div class="flex flex-wrap items-center justify-between gap-4">
      <div class="flex items-center gap-3">
        <h3 class="text-sm font-semibold text-gray-800 dark:text-gray-100">Journey Timeline</h3>
        {#if currentIndex === stops.length - 1}
          <span class="inline-flex items-center gap-1 px-2 py-0.5 rounded-full text-xs font-semibold bg-success-100 dark:bg-success-900/30 text-success-700 dark:text-success-300">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
            </svg>
            Completed
          </span>
        {:else if overallDelay > 0}
          <span class="inline-flex items-center gap-1 px-2 py-0.5 rounded-full text-xs font-semibold bg-warning-100 dark:bg-warning-900/30 text-warning-700 dark:text-warning-300">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
            +{ overallDelay } min delay
          </span>
        {:else if currentIndex > 0}
          <span class="inline-flex items-center gap-1 px-2 py-0.5 rounded-full text-xs font-semibold bg-success-100 dark:bg-success-900/30 text-success-700 dark:text-success-300">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
            </svg>
            On time
          </span>
        {/if}
      </div>
      <div class="flex items-center gap-3 text-xs">
        <span class="flex items-center gap-1">
          <span class="w-2.5 h-2.5 rounded-full bg-success-500"></span>
          <span class="text-gray-600 dark:text-gray-400">Passed</span>
        </span>
        {#if currentIndex < stops.length - 1}
          <span class="flex items-center gap-1">
            <span class="w-2.5 h-2.5 rounded-full bg-primary-500 animate-pulse"></span>
            <span class="text-gray-600 dark:text-gray-400">Current</span>
          </span>
          <span class="flex items-center gap-1">
            <span class="w-2.5 h-2.5 rounded-full bg-gray-400 dark:bg-gray-600"></span>
            <span class="text-gray-600 dark:text-gray-400">Upcoming</span>
          </span>
        {/if}
      </div>
    </div>
  </div>
  <!-- Scrollable timeline row -->
  <div
    bind:this={scrollContainer}
    class="overflow-x-auto min-h-[160px] max-h-[200px] lg:min-h-[200px] lg:max-h-[260px]"
  >
    <div class="inline-flex items-start px-4 py-6 gap-0">
      {#each stops as stop, i}
        {@const status = getStopStatus(i)}
        {@const isLast = i === stops.length - 1}
        {@const delay = stop.delay_minutes}
        {@const hasDelay = delay && delay > 0}

        <!-- Stop column -->
        <div
          class="flex flex-col items-center flex-shrink-0"
          data-index={i}
          data-status={status}
        >
          <!-- Icon wrapper: fixed 40x40, overflow visible so ping ring can expand outside -->
          <div class="relative flex items-center justify-center" style="width:40px;height:40px;">

            <!-- Ping ring sits on inset-0 of the 40px box, expands outward via overflow:visible -->
            {#if status === 'current'}
              <span class="absolute inset-0 rounded-full bg-primary-400 animate-ping opacity-40"></span>
            {/if}

            <!-- Icon circle -->
            <div class="relative z-10 flex items-center justify-center w-10 h-10 rounded-full border-4 transition-all duration-300 {
              status === 'passed'  ? 'bg-green-700 border-green-800' :
              status === 'current' ? 'bg-primary-500 border-primary-200 shadow-lg shadow-primary-500/40' :
                                     'bg-white dark:bg-gray-700 border-gray-300 dark:border-gray-500'
            }">
              {#if status === 'passed'}
                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M5 13l4 4L19 7" />
                </svg>
              {:else if status === 'current'}
                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8" />
                </svg>
              {:else}
                <span class="text-gray-600 dark:text-gray-300 text-sm font-bold">{i + 1}</span>
              {/if}
            </div>

            <!-- Delay badge: z-20 so it sits above ping ring, no effect on layout height -->
            {#if hasDelay && status !== 'passed'}
              <div class="absolute -top-1 -right-1 z-20 w-5 h-5 rounded-full bg-error-500 text-white text-[10px] font-bold flex items-center justify-center shadow-md">
                !
              </div>
            {/if}
          </div>

          <!-- Label below icon -->
          <div class="mt-2 text-center w-[72px] lg:w-[100px]">
            <div
              class="text-xs lg:text-sm font-semibold text-gray-900 dark:text-gray-100 leading-tight"
              title={stop.station_name}
              style="display:-webkit-box;-webkit-line-clamp:2;-webkit-box-orient:vertical;overflow:hidden;"
            >
              {stop.station_name}
            </div>

            {#if stop.platform}
              <div class="mt-0.5 hidden lg:flex justify-center">
                <span class="px-1.5 py-0.5 rounded bg-primary-100 dark:bg-primary-900/40 text-primary-700 dark:text-primary-300 text-[10px] font-mono font-bold">
                  {stop.platform}
                </span>
              </div>
            {/if}

            <div class="mt-0.5">
              <span class="text-xs font-mono text-gray-600 dark:text-gray-300">{stop.scheduled_time}</span>
            </div>

            {#if hasDelay && status !== 'passed'}
              <div class="mt-1 inline-flex items-center justify-center px-1.5 py-0.5 rounded bg-error-100 dark:bg-error-900/40 text-error-600 dark:text-error-400 text-xs font-bold">
                +{delay}m
              </div>
            {/if}
          </div>
        </div>

        <!-- Connector: self-start + padding-top:16px centres the bar on the 40px icon row -->
        {#if !isLast}
          <div class="flex-shrink-0 self-start" style="padding-top:16px;">
            <div class="h-1.5 w-12 lg:w-16 rounded-full transition-colors duration-500 {getLineClass(i)}"></div>
          </div>
        {/if}
      {/each}
    </div>
  </div>

  <!-- Footer -->
  <div class="px-4 py-2 border-t border-gray-100 dark:border-gray-700 bg-gray-50 dark:bg-gray-900/50">
    <div class="flex items-center justify-between text-xs">
      <span class="text-gray-500 dark:text-gray-400">Scroll to view all stops</span>
      {#if currentIndex === stops.length - 1}
        <span class="font-medium text-success-600 dark:text-success-400">Journey Completed</span>
      {:else}
        <span class="font-medium text-gray-700 dark:text-gray-300">{currentIndex + 1} of {stops.length}</span>
      {/if}
    </div>
  </div>
</div>
