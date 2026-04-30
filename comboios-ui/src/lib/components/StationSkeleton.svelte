<div class="space-y-6">
  <!-- stat cards -->
  <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
    {#each [1, 2, 3] as _}
      <div class="bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-xl p-4 animate-pulse">
        <div class="h-3 w-24 bg-gray-200 dark:bg-gray-700 rounded mb-3"></div>
        <div class="h-7 w-12 bg-gray-200 dark:bg-gray-700 rounded"></div>
      </div>
    {/each}
  </div>

  <!-- train list card -->
  <div class="bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-xl p-6 animate-pulse">
    <div class="h-5 w-32 bg-gray-200 dark:bg-gray-700 rounded mb-6"></div>
    <div class="space-y-4">
      {#each [1, 2, 3, 4, 5] as _}
        <div class="flex items-center gap-4 py-3 border-b border-gray-100 dark:border-gray-700/50">
          <div class="h-6 w-16 bg-gray-200 dark:bg-gray-700 rounded-full flex-shrink-0"></div>
          <div class="flex-1 space-y-2">
            <div class="h-4 w-48 bg-gray-200 dark:bg-gray-700 rounded"></div>
            <div class="h-3 w-32 bg-gray-200 dark:bg-gray-700 rounded"></div>
          </div>
          <div class="h-5 w-14 bg-gray-200 dark:bg-gray-700 rounded flex-shrink-0"></div>
          <div class="h-6 w-20 bg-gray-200 dark:bg-gray-700 rounded-full flex-shrink-0"></div>
        </div>
      {/each}
    </div>
  </div>
</div>
