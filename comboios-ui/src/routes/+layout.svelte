<script lang="ts">
  import '../app.css';
  import { afterNavigate, beforeNavigate } from '$app/navigation';
  import { page } from '$app/stores';
  import ThemeToggle from '$lib/components/ThemeToggle.svelte';
  import LoadingSpinner from '$lib/components/LoadingSpinner.svelte';
  
  let menuOpen = false;
  let isNavigating = false;
  
  beforeNavigate(() => {
    isNavigating = true;
  });
  
  afterNavigate(() => {
    isNavigating = false;
    menuOpen = false;
  });
  
  function toggleMenu() {
    menuOpen = !menuOpen;
  }
  
  function closeMenu() {
    menuOpen = false;
  }
</script>

<div class="flex flex-col min-h-screen bg-gray-100 dark:bg-gray-900">
  <header class="bg-white dark:bg-gray-800 shadow-sm sticky top-0 z-40">
    <div class="max-w-7xl mx-auto px-4">
      <div class="flex items-center justify-between py-3">
        <a href="/" class="flex items-center gap-2 text-xl md:text-2xl font-bold text-primary-700 dark:text-primary-400 hover:text-primary-800 dark:hover:text-primary-300 transition-colors">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-7 w-7 md:h-8 md:w-8" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8" />
          </svg>
          <span class="hidden sm:inline">Comboios de Portugal</span>
          <span class="sm:hidden">Comboios</span>
        </a>
        
        <div class="flex items-center gap-2">
          {#if isNavigating}
            <div class="flex items-center gap-2 mr-2">
              <LoadingSpinner size="sm" />
              <span class="text-xs text-gray-500 dark:text-gray-400 hidden sm:inline">Loading...</span>
            </div>
          {/if}
          
          <ThemeToggle />
          
          <button 
            class="md:hidden p-2 rounded-lg text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
            on:click={toggleMenu}
            aria-label="Toggle menu"
            aria-expanded={menuOpen}
          >
            {#if menuOpen}
              <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
              </svg>
            {:else}
              <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
              </svg>
            {/if}
          </button>
        </div>
      </div>
      
      <nav class="hidden md:flex items-center gap-4 pb-3">
        <a 
          href="/" 
          class="text-sm font-medium px-3 py-1.5 rounded-lg transition-colors {$page.url.pathname === '/' ? 'bg-primary-100 dark:bg-primary-900/30 text-primary-700 dark:text-primary-300' : 'text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700'}"
        >
          Stations
        </a>
        <a 
          href="/about" 
          class="text-sm font-medium px-3 py-1.5 rounded-lg transition-colors {$page.url.pathname === '/about' ? 'bg-primary-100 dark:bg-primary-900/30 text-primary-700 dark:text-primary-300' : 'text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700'}"
        >
          About
        </a>
      </nav>
    </div>
    
    {#if menuOpen}
      <div class="md:hidden border-t border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800">
        <nav class="flex flex-col p-2">
          <a 
            href="/" 
            class="flex items-center gap-3 px-4 py-3 text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg transition-colors font-medium"
            on:click={closeMenu}
          >
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z" />
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z" />
            </svg>
            Stations
          </a>
          <a 
            href="/about" 
            class="flex items-center gap-3 px-4 py-3 text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg transition-colors font-medium"
            on:click={closeMenu}
          >
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
            About
          </a>
        </nav>
      </div>
    {/if}
  </header>
  
  {#if isNavigating}
    <div class="h-1 bg-primary-600 dark:bg-primary-400 animate-pulse"></div>
  {/if}
  
  <main class="flex-grow py-6">
    <div class="max-w-7xl mx-auto px-4">
      <slot />
    </div>
  </main>
  
  <footer class="border-t border-gray-300 dark:border-gray-700 bg-white dark:bg-gray-800">
    <div class="max-w-7xl mx-auto px-4 py-6">
      <div class="text-center text-sm text-gray-500 dark:text-gray-400">
        <p>Comboios de Portugal - Unofficial application</p>
      </div>
    </div>
  </footer>
</div>
