<!-- Mirrors the station board (header row + train rows) so nothing jumps when data arrives -->
<div class="rounded-xl overflow-hidden border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 motion-safe:animate-pulse" aria-busy="true" aria-label="Loading board">
  <div class="flex items-center gap-3 px-4 py-2 border-b border-gray-200 dark:border-gray-700">
    <div class="h-3 w-8 rounded bg-gray-200 dark:bg-gray-700"></div>
  </div>
  {#each [1, 2, 3, 4, 5, 6] as _}
    <div class="flex items-center gap-3 px-4 py-3 border-b last:border-b-0 border-gray-200 dark:border-gray-700">
      <div class="w-16 shrink-0"><div class="h-6 w-12 rounded bg-gray-200 dark:bg-gray-700"></div></div>
      <div class="flex-1 space-y-2">
        <div class="h-4 w-2/3 rounded bg-gray-200 dark:bg-gray-700"></div>
        <div class="h-3 w-1/3 rounded bg-gray-200 dark:bg-gray-700"></div>
      </div>
      <div class="w-16 shrink-0 flex justify-center"><div class="h-9 w-9 rounded-md bg-gray-200 dark:bg-gray-700"></div></div>
    </div>
  {/each}
</div>
