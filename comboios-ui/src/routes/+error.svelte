<script lang="ts">
  import { page } from '$app/stores';
  import { dev } from '$app/environment';
</script>

<div class="min-h-screen flex items-center justify-center bg-gray-950 text-white px-4">
  <div class="max-w-md text-center">
    <div class="w-16 h-px bg-gradient-to-r from-transparent via-gray-500 to-transparent mx-auto mb-8 animate-pulse"></div>
    
    <h1 class="text-2xl font-light mb-2">
      {$page.status === 404 ? 'Not Found' : 'Derailed'}
    </h1>
    
    <p class="text-gray-400 mb-8">
      {$page.status === 404 
        ? 'The requested resource could not be found.' 
        : 'Something went wrong. Please try again.'}
    </p>
    
    <div class="flex gap-4 justify-center mb-8">
      <button 
        class="px-4 py-2 bg-white/10 hover:bg-white/20 rounded-lg transition-colors"
        onclick={() => history.back()}
      >
        Go Back
      </button>
      <a 
        href="/"
        class="px-4 py-2 bg-white/10 hover:bg-white/20 rounded-lg transition-colors"
      >
        Home
      </a>
    </div>
    
    {#if dev && $page.error?.message}
      <details class="text-left">
        <summary class="cursor-pointer text-sm text-gray-500 hover:text-gray-400">
          Debug
        </summary>
        <pre class="mt-2 p-4 bg-black/50 rounded-lg text-xs text-gray-300 overflow-auto max-h-48">{$page.error?.message}</pre>
      </details>
    {/if}
  </div>
</div>

<style>
  @keyframes pulse {
    0%, 100% { opacity: 0.3; }
    50% { opacity: 0.8; }
  }
  
  .animate-pulse {
    animation: pulse 3s ease-in-out infinite;
  }
</style>
