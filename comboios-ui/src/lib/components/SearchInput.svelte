<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  
  export let placeholder: string = 'Search...';
  export let value: string = '';
  export let loading: boolean = false;
  
  const dispatch = createEventDispatcher();
  
  function handleInput(e: Event) {
    const target = e.target as HTMLInputElement;
    dispatch('input', { value: target.value });
  }
  
  function handleKeyDown(e: KeyboardEvent) {
    if (e.key === 'Enter') {
      dispatch('search');
    }
  }
  
  function handleClick() {
    dispatch('search', { value });
  }
</script>

<div class="join w-full">
  <input
    type="text"
    placeholder={placeholder}
    class="input input-bordered join-item flex-1 bg-white dark:bg-gray-800 border-gray-300 dark:border-gray-600 text-gray-900 dark:text-white placeholder-gray-500 dark:placeholder-gray-400"
    bind:value
    on:input={handleInput}
    on:keydown={handleKeyDown}
    disabled={loading}
  />
  <button 
    class="btn btn-primary join-item px-6" 
    on:click={handleClick}
    disabled={loading}
  >
    {#if loading}
      <span class="loading loading-spinner loading-sm"></span>
      Searching...
    {:else}
      <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
      </svg>
      Search
    {/if}
  </button>
</div>