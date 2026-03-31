<script lang="ts">
  import type { ServiceAlert } from '$lib/types';
  
  export let alerts: ServiceAlert[] = [];
  
  $: criticalAlerts = alerts.filter(a => a.severity === 'critical');
  $: warningAlerts = alerts.filter(a => a.severity === 'warning');
  $: infoAlerts = alerts.filter(a => a.severity === 'info');
  
  let showAll = false;
  
  $: visibleAlerts = showAll 
    ? alerts 
    : [...criticalAlerts, ...warningAlerts.slice(0, 2)];
</script>

{#if alerts.length > 0}
  <div class="mb-6">
    {#each visibleAlerts as alert}
      <div class={`alert mb-2 rounded-lg ${
        alert.severity === 'critical' ? 'alert-error bg-rose-50 border-rose-200' :
        alert.severity === 'warning' ? 'alert-warning bg-amber-50 border-amber-200' :
        'alert-info bg-blue-50 border-blue-200'
      }`}>
        <div>
          <div class="flex items-start">
            <div class="flex-shrink-0">
              {#if alert.severity === 'critical'}
                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-rose-500" viewBox="0 0 20 20" fill="currentColor">
                  <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd" />
                </svg>
              {:else if alert.severity === 'warning'}
                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-amber-500" viewBox="0 0 20 20" fill="currentColor">
                  <path fill-rule="evenodd" d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z" clip-rule="evenodd" />
                </svg>
              {:else}
                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-blue-500" viewBox="0 0 20 20" fill="currentColor">
                  <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd" />
                </svg>
              {/if}
            </div>
            <div class="ml-3 flex-1">
              <h3 class={`font-medium ${
                alert.severity === 'critical' ? 'text-rose-800' :
                alert.severity === 'warning' ? 'text-amber-800' :
                'text-blue-800'
              }`}>
                {alert.title}
              </h3>
              <p class={`text-sm mt-1 ${
                alert.severity === 'critical' ? 'text-rose-700' :
                alert.severity === 'warning' ? 'text-amber-700' :
                'text-blue-700'
              }`}>
                {alert.description}
              </p>
              {#if alert.url}
                <a 
                  href={alert.url} 
                  target="_blank" 
                  class={`text-sm font-medium inline-flex items-center mt-2 ${
                    alert.severity === 'critical' ? 'text-rose-600 hover:text-rose-500' :
                    alert.severity === 'warning' ? 'text-amber-600 hover:text-amber-500' :
                    'text-blue-600 hover:text-blue-500'
                  }`}
                >
                  More information
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 ml-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14" />
                  </svg>
                </a>
              {/if}
            </div>
          </div>
        </div>
      </div>
    {/each}
    
    {#if alerts.length > visibleAlerts.length}
      <button 
        class="btn btn-sm btn-ghost"
        on:click={() => showAll = !showAll}
      >
        {showAll ? 'Show less' : `Show all ${alerts.length} alerts`}
      </button>
    {/if}
  </div>
{/if}