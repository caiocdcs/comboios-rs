<script lang="ts">
  import '../app.css';
  import { onMount } from 'svelte';
  import { navigating } from '$app/stores';
  import ThemeToggle from '$lib/components/ThemeToggle.svelte';

  let offline = false;

  onMount(() => {
    const update = () => (offline = !navigator.onLine);
    update();
    window.addEventListener('online', update);
    window.addEventListener('offline', update);
    return () => {
      window.removeEventListener('online', update);
      window.removeEventListener('offline', update);
    };
  });
</script>

<div class="flex flex-col min-h-screen bg-gray-100 dark:bg-gray-900">
  <!-- pt-[env(...)]: clear the notch/status bar when installed as an app -->
  <header class="sticky top-0 z-40 bg-white/90 dark:bg-gray-800/90 backdrop-blur border-b border-gray-200 dark:border-gray-700 pt-[env(safe-area-inset-top)]">
    <div class="max-w-xl mx-auto px-4 h-14 flex items-center justify-between">
      <a href="/" class="flex items-center gap-2 text-lg font-extrabold tracking-tight text-gray-900 dark:text-white">
        <img src="/icon.svg" alt="" class="h-7 w-7 rounded-md" />
        Comboios
      </a>

      <div class="flex items-center gap-1">
        <a
          href="/about"
          class="p-2 rounded-lg text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
          aria-label="About"
          title="About"
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
        </a>
        <ThemeToggle />
      </div>
    </div>

    {#if $navigating}
      <div class="h-0.5 bg-primary-700 dark:bg-primary-400 animate-pulse"></div>
    {/if}
  </header>

  {#if offline}
    <div class="bg-warning-100 dark:bg-warning-900/50 text-warning-900 dark:text-warning-200 text-sm text-center px-4 py-2" role="status">
      You're offline. Times shown are from your last connection and may be out of date.
    </div>
  {/if}

  <main class="flex-grow py-4 pb-[calc(1rem+env(safe-area-inset-bottom))]">
    <div class="max-w-xl mx-auto px-4">
      <slot />
    </div>
  </main>

  <footer class="border-t border-gray-200 dark:border-gray-700 pb-[env(safe-area-inset-bottom)]">
    <p class="max-w-xl mx-auto px-4 py-4 text-xs text-gray-500 dark:text-gray-400">
      Unofficial. Times come from CP and Infraestruturas de Portugal. <span class="tabular-nums">{COMMIT_HASH}</span>
    </p>
  </footer>
</div>
