<script lang="ts">
  import { expectedStopTime, lastPassedIndex } from '$lib/journey';
  import type { JourneyStop } from '$lib/types';

  export let stops: JourneyStop[] = [];

  /** Passed stops beyond the most recent one are folded away until asked for */
  let showEarlier = false;

  $: passed = lastPassedIndex(stops);
  $: nextIndex = passed + 1 < stops.length ? passed + 1 : -1;
  $: completed = stops.length > 0 && nextIndex === -1;
  $: hiddenCount = showEarlier ? 0 : Math.max(passed, 0);
  $: visible = stops.map((stop, i) => ({ stop, i })).filter(({ i }) => i >= hiddenCount);
</script>

<section class="rounded-xl border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 overflow-hidden">
  <div class="flex items-center justify-between px-4 py-3 border-b border-gray-200 dark:border-gray-700">
    <h2 class="text-base font-semibold">Stops</h2>
    <span class="text-sm text-gray-500 dark:text-gray-400">
      {#if completed}
        Journey completed
      {:else if passed >= 0}
        {passed + 1} of {stops.length} passed
      {:else}
        {stops.length} stops
      {/if}
    </span>
  </div>

  {#if hiddenCount > 0}
    <button
      type="button"
      class="w-full px-4 py-2 text-sm font-medium text-primary-700 dark:text-primary-300 hover:bg-gray-50 dark:hover:bg-gray-700/50 border-b border-gray-200 dark:border-gray-700"
      on:click={() => (showEarlier = true)}
    >
      Show {hiddenCount} earlier {hiddenCount === 1 ? 'stop' : 'stops'}
    </button>
  {/if}

  <ol class="py-2">
    {#each visible as { stop, i } (i)}
      {@const isPassed = i <= passed}
      {@const isNext = i === nextIndex}
      {@const isFirst = i === 0}
      {@const isLast = i === stops.length - 1}
      {@const expected = expectedStopTime(stop)}
      {@const delay = stop.delay_minutes ?? 0}
      <li class="relative flex gap-3 px-4 py-2 {isNext ? 'bg-primary-50 dark:bg-primary-900/30' : ''}">
        <!-- Rail: line through the dot, solid where the train has been -->
        <div class="relative w-4 shrink-0 flex justify-center" aria-hidden="true">
          {#if !isFirst || hiddenCount > 0}
            <span class="absolute top-[-0.5rem] h-[calc(50%+0.5rem)] w-1 {isPassed || isNext
              ? 'bg-primary-700 dark:bg-primary-400'
              : 'bg-gray-300 dark:bg-gray-600'}"></span>
          {/if}
          {#if !isLast}
            <span class="absolute bottom-[-0.5rem] h-[calc(50%+0.5rem)] w-1 {isPassed
              ? 'bg-primary-700 dark:bg-primary-400'
              : 'bg-gray-300 dark:bg-gray-600'}"></span>
          {/if}
          <span class="relative z-10 self-center rounded-full {isNext
            ? 'w-4 h-4 bg-white dark:bg-gray-800 border-4 border-primary-700 dark:border-primary-400'
            : isPassed
              ? 'w-3 h-3 bg-primary-700 dark:bg-primary-400'
              : 'w-3 h-3 bg-white dark:bg-gray-800 border-2 border-gray-400 dark:border-gray-500'}"></span>
          {#if isNext}
            <span class="absolute z-0 self-center w-4 h-4 rounded-full bg-primary-400 animate-ping opacity-50 motion-reduce:hidden"></span>
          {/if}
        </div>

        <!-- Station -->
        <div class="flex-1 min-w-0 py-0.5">
          <div class="truncate {isNext ? 'font-bold text-gray-900 dark:text-white' : isPassed
            ? 'text-gray-500 dark:text-gray-400'
            : 'font-medium text-gray-900 dark:text-gray-100'}">
            {stop.station_name}
          </div>
          {#if isNext || isFirst || isLast}
            <div class="text-xs text-gray-500 dark:text-gray-400">
              {isNext ? 'Next stop' : isFirst ? 'Origin' : 'Destination'}
            </div>
          {/if}
        </div>

        <!-- Platform -->
        <div class="w-7 shrink-0 flex items-center justify-center">
          {#if stop.platform && !isPassed}
            <span class="platform-badge-sm" title="Platform {stop.platform}">{stop.platform}</span>
          {/if}
        </div>

        <!-- Time: expected first, timetabled struck through when it changed -->
        <div class="w-14 shrink-0 text-right font-mono leading-tight py-0.5">
          <div class="{isPassed ? 'text-gray-500 dark:text-gray-400' : delay > 0
            ? 'font-bold text-warning-700 dark:text-warning-400'
            : 'font-semibold text-gray-900 dark:text-white'}">
            {expected}
          </div>
          {#if expected !== stop.scheduled_time}
            <div class="text-xs text-gray-500 dark:text-gray-400 line-through">{stop.scheduled_time}</div>
          {/if}
        </div>
      </li>
    {/each}
  </ol>
</section>
