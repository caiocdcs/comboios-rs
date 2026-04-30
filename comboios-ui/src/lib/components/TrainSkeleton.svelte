<div class="space-y-6 animate-pulse">
  <!-- header card -->
  <div class="bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-xl p-6">
    <div class="flex items-center gap-4 mb-6">
      <div class="h-9 w-20 bg-gray-200 dark:bg-gray-700 rounded"></div>
      <div class="h-7 w-24 bg-gray-200 dark:bg-gray-700 rounded-full"></div>
      <div class="ml-auto h-8 w-28 bg-gray-200 dark:bg-gray-700 rounded-full"></div>
    </div>
    <div class="grid grid-cols-2 md:grid-cols-5 gap-4">
      {#each [1, 2, 3, 4, 5] as _}
        <div>
          <div class="h-3 w-16 bg-gray-200 dark:bg-gray-700 rounded mb-2"></div>
          <div class="h-5 w-24 bg-gray-200 dark:bg-gray-700 rounded"></div>
        </div>
      {/each}
    </div>
  </div>

  <!-- timeline strip -->
  <div class="bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-xl p-6">
    <div class="h-4 w-36 bg-gray-200 dark:bg-gray-700 rounded mb-6"></div>
    <div class="flex items-center gap-0">
      {#each [1, 2, 3, 4, 5] as _, i}
        <div class="flex flex-col items-center flex-shrink-0">
          <div class="w-10 h-10 rounded-full bg-gray-200 dark:bg-gray-700"></div>
          <div class="mt-2 w-16 h-3 bg-gray-200 dark:bg-gray-700 rounded"></div>
          <div class="mt-1 w-10 h-3 bg-gray-200 dark:bg-gray-700 rounded"></div>
        </div>
        {#if i < 4}
          <div class="flex-shrink-0 h-1.5 w-12 bg-gray-200 dark:bg-gray-700 rounded mx-0 self-start mt-4"></div>
        {/if}
      {/each}
    </div>
  </div>

  <!-- stops table -->
  <div class="bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-xl p-6">
    <div class="h-5 w-24 bg-gray-200 dark:bg-gray-700 rounded mb-6"></div>
    <div class="space-y-3">
      {#each [1, 2, 3, 4, 5, 6] as _}
        <div class="flex items-center gap-4 py-2 border-b border-gray-100 dark:border-gray-700/50">
          <div class="h-4 w-6 bg-gray-200 dark:bg-gray-700 rounded flex-shrink-0"></div>
          <div class="flex-1 h-4 bg-gray-200 dark:bg-gray-700 rounded"></div>
          <div class="h-4 w-12 bg-gray-200 dark:bg-gray-700 rounded flex-shrink-0"></div>
          <div class="h-4 w-16 bg-gray-200 dark:bg-gray-700 rounded flex-shrink-0"></div>
          <div class="h-6 w-20 bg-gray-200 dark:bg-gray-700 rounded-full flex-shrink-0"></div>
        </div>
      {/each}
    </div>
  </div>
</div>
