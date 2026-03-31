<script lang="ts">
  export let status: 'on-time' | 'delayed' | 'departed' | 'scheduled' = 'scheduled';
  export let delayMinutes: number | null = null;
  
  $: badgeClass = {
    'on-time': 'badge-success',
    'delayed': 'badge-warning',
    'departed': 'badge-ghost',
    'scheduled': 'badge-info'
  }[status];
  
  $: text = {
    'on-time': 'On time',
    'delayed': delayMinutes ? `+${delayMinutes} min` : 'Delayed',
    'departed': 'Departed',
    'scheduled': 'Scheduled'
  }[status];
</script>

<span class="badge badge-sm {badgeClass}">
  {text}
</span>