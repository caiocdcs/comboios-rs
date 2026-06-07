<script lang="ts">
  export let status: 'on-time' | 'delayed' | 'departed' | 'scheduled' | 'cancelled' = 'scheduled';
  export let delayMinutes: number | null = null;

  $: badgeClass = {
    'on-time': 'badge-success',
    'delayed': 'badge-warning',
    'departed': 'badge-ghost',
    'scheduled': 'badge-info',
    'cancelled': 'badge-error'
  }[status];

  $: text = {
    'on-time': 'On time',
    'delayed': delayMinutes ? `+${delayMinutes} min` : 'Delayed',
    'departed': 'Departed',
    'scheduled': 'Scheduled',
    'cancelled': 'Cancelled'
  }[status];
</script>

<span class="badge badge-sm {badgeClass}">
  {text}
</span>
