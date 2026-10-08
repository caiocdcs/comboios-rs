<!-- Mirrors the journey page (title, position card, stop list) so nothing jumps when data arrives -->
<div class="motion-safe:animate-pulse" aria-busy="true" aria-label="Loading train">
  <div class="mb-4 space-y-2">
    <div class="h-7 w-32 rounded bg-gray-200 dark:bg-gray-700"></div>
    <div class="h-4 w-56 rounded bg-gray-200 dark:bg-gray-700"></div>
    <div class="h-5 w-20 rounded-full bg-gray-200 dark:bg-gray-700"></div>
  </div>

  <div class="rounded-xl border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 p-4 mb-4 flex justify-between">
    <div class="space-y-2">
      <div class="h-3 w-16 rounded bg-gray-200 dark:bg-gray-700"></div>
      <div class="h-6 w-40 rounded bg-gray-200 dark:bg-gray-700"></div>
      <div class="h-5 w-24 rounded bg-gray-200 dark:bg-gray-700"></div>
    </div>
    <div class="h-9 w-9 rounded-md bg-gray-200 dark:bg-gray-700"></div>
  </div>

  <div class="rounded-xl border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 py-2">
    {#each [1, 2, 3, 4, 5] as _}
      <div class="flex items-center gap-3 px-4 py-3">
        <div class="h-3 w-3 rounded-full bg-gray-200 dark:bg-gray-700"></div>
        <div class="flex-1 h-4 rounded bg-gray-200 dark:bg-gray-700"></div>
        <div class="h-4 w-12 rounded bg-gray-200 dark:bg-gray-700"></div>
      </div>
    {/each}
  </div>
</div>
