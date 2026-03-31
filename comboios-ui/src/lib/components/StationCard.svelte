<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  
  export let id: string = '';
  export let name: string = '';
  export let region: string = '';
  
  const dispatch = createEventDispatcher();
  
  function handleClick() {
    dispatch('select', { id, name });
  }
</script>

<button
  class="flex items-center justify-between p-4 rounded-xl border border-gray-200 dark:border-gray-700 hover:border-primary-300 dark:hover:border-primary-500 hover:bg-primary-50 dark:hover:bg-primary-900/20 transition-all duration-200 text-left w-full"
  on:click={handleClick}
>
  <div>
    <h3 class="font-semibold text-lg text-gray-900 dark:text-white">{name}</h3>
    {#if region}
      <p class="text-sm text-gray-500 dark:text-gray-400">{region}</p>
    {/if}
    <p class="text-xs text-gray-400 dark:text-gray-500 mt-1">ID: {id}</p>
  </div>
  <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-gray-400 dark:text-gray-500" fill="none" viewBox="0 0 24 24" stroke="currentColor">
    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
  </svg>
</button>