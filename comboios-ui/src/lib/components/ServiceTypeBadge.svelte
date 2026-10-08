<script lang="ts">
  import { parseService } from '$lib/service';

  /** CP service as sent by the API, e.g. "IC|Intercidades", "AP|Alfa Pendular", "U|Urbano" */
  export let serviceType: string = '';

  $: ({ code, name } = parseService(serviceType));

  // Long-distance services stand out; regional and urban stay quiet.
  // Amber is reserved for platforms.
  $: tone = ['AP', 'ALFA'].includes(code.toUpperCase())
    ? 'bg-primary-700 text-white dark:bg-primary-400 dark:text-gray-900'
    : ['IC', 'IN', 'INT'].includes(code.toUpperCase())
      ? 'bg-primary-100 text-primary-800 dark:bg-primary-900/70 dark:text-primary-200'
      : 'bg-gray-200 text-gray-700 dark:bg-gray-700 dark:text-gray-200';
</script>

<span class="inline-flex items-center h-5 px-1.5 rounded text-[11px] font-bold tracking-wide {tone}" title={name}>
  {code}
</span>
