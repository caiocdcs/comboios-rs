<script lang="ts">
  import { page } from '$app/stores';
  import { dev } from '$app/environment';

  $: notFound = $page.status === 404;
</script>

<svelte:head>
  <title>{notFound ? 'Page not found' : 'Something went wrong'} · Comboios</title>
</svelte:head>

<!-- Rendered inside the layout, so it follows the light/dark theme -->
<div class="max-w-xl mx-auto py-12">
  <h1 class="text-2xl font-extrabold tracking-tight mb-2">
    {notFound ? "This page doesn't exist" : "This page couldn't load"}
  </h1>
  <p class="text-base text-gray-600 dark:text-gray-400 mb-6">
    {notFound
      ? 'The link may be old or mistyped. Find your station from the home screen.'
      : 'The train data service may be busy or your connection dropped. Try again, or go back to your stations.'}
  </p>

  <div class="flex flex-wrap gap-3">
    {#if !notFound}
      <button type="button" class="btn btn-primary" on:click={() => location.reload()}>Try again</button>
    {/if}
    <a href="/" class="btn {notFound ? 'btn-primary' : 'btn-ghost'}">Go to stations</a>
  </div>

  {#if dev && $page.error?.message}
    <details class="mt-8 text-sm">
      <summary class="cursor-pointer text-gray-500 dark:text-gray-400">Error details</summary>
      <pre class="mt-2 p-3 rounded-lg bg-gray-200 dark:bg-gray-800 text-xs overflow-auto max-h-48">{$page.error?.message}</pre>
    </details>
  {/if}
</div>
