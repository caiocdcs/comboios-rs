<script lang="ts">
  export let currentPage: number;
  export let totalPages: number;
  export let totalItems: number;
  export let itemsPerPage: number = 10;
  export let onPageChange: (page: number) => void;
  
  $: startItem = (currentPage - 1) * itemsPerPage + 1;
  $: endItem = Math.min(currentPage * itemsPerPage, totalItems);
  
  $: pageNumbers = (() => {
    const pages: (number | string)[] = [];
    if (totalPages <= 7) {
      for (let i = 1; i <= totalPages; i++) pages.push(i);
    } else {
      pages.push(1);
      if (currentPage > 3) pages.push('...');
      for (let i = Math.max(2, currentPage - 1); i <= Math.min(totalPages - 1, currentPage + 1); i++) {
        pages.push(i);
      }
      if (currentPage < totalPages - 2) pages.push('...');
      pages.push(totalPages);
    }
    return pages;
  })();
</script>

<div class="flex flex-col sm:flex-row items-center justify-between gap-4 py-4">
  <div class="text-sm text-gray-700 dark:text-gray-300">
    Showing {startItem}-{endItem} of {totalItems} trains
  </div>
  
  <div class="flex items-center gap-1">
    <button
      class="px-3 py-1.5 rounded-lg text-sm font-medium bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-200 hover:bg-gray-200 dark:hover:bg-gray-600 disabled:opacity-40 disabled:cursor-not-allowed transition-colors"
      disabled={currentPage === 1}
      on:click={() => onPageChange(currentPage - 1)}
    >
      ← Prev
    </button>
    
    {#each pageNumbers as page}
      {#if page === '...'}
        <span class="px-2 py-1.5 text-gray-500 dark:text-gray-400">...</span>
      {:else}
        <button
          class="px-3 py-1.5 rounded-lg text-sm font-medium transition-colors {currentPage === page 
            ? 'bg-primary-600 text-white' 
            : 'bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-200 hover:bg-gray-200 dark:hover:bg-gray-600'}"
          on:click={() => onPageChange(page as number)}
        >
          {page}
        </button>
      {/if}
    {/each}
    
    <button
      class="px-3 py-1.5 rounded-lg text-sm font-medium bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-200 hover:bg-gray-200 dark:hover:bg-gray-600 disabled:opacity-40 disabled:cursor-not-allowed transition-colors"
      disabled={currentPage === totalPages}
      on:click={() => onPageChange(currentPage + 1)}
    >
      Next →
    </button>
  </div>
</div>
