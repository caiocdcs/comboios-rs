<script lang="ts">
  import { onMount } from 'svelte';
  import { formatAge } from '$lib/date';

  export let lastUpdated: Date | null;
  export let refreshing = false;
  /** The last background refresh failed: data shown is older than it looks */
  export let failed = false;
  export let onRefresh: () => void;

  let now = new Date();

  onMount(() => {
    const tick = setInterval(() => (now = new Date()), 5_000);
    return () => clearInterval(tick);
  });

  $: age = lastUpdated ? formatAge(lastUpdated, now) : '';
</script>

<div class="flex items-center justify-between gap-3 text-sm" aria-live="polite">
  <span class={failed ? 'text-warning-700 dark:text-warning-400' : 'text-gray-500 dark:text-gray-400'}>
    {#if refreshing}
      Updating…
    {:else if failed}
      Couldn't update · data from {age}
    {:else if lastUpdated}
      Updated {age}
    {/if}
  </span>
  <button
    type="button"
    class="btn btn-ghost btn-sm gap-1"
    on:click={onRefresh}
    disabled={refreshing}
    aria-label="Refresh"
  >
    <svg
      xmlns="http://www.w3.org/2000/svg"
      class="h-4 w-4 {refreshing ? 'animate-spin' : ''}"
      fill="none"
      viewBox="0 0 24 24"
      stroke="currentColor"
    >
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
    </svg>
    Refresh
  </button>
</div>
