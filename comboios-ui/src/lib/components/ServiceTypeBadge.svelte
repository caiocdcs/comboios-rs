<script lang="ts">
  export let serviceType: string = '';
  
  $: badgeClass = serviceType.toLowerCase().includes('intercity') ? 'badge-primary' :
                  serviceType.toLowerCase().includes('alfa') ? 'badge-secondary' :
                  serviceType.toLowerCase().includes('regional') ? 'badge-accent' :
                  'badge-info';
</script>

<span class="badge badge-sm {badgeClass}">
  {serviceType}
</span>