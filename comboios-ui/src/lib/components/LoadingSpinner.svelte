<script lang="ts">
  export let size: 'sm' | 'md' | 'lg' = 'md';
  export let color: 'primary' | 'secondary' | 'accent' = 'primary';
  
  $: sizeClass = {
    'sm': 'loading-sm',
    'md': 'loading-md',
    'lg': 'loading-lg'
  }[size];
  
  $: colorClass = {
    'primary': 'text-primary-600',
    'secondary': 'text-secondary-600',
    'accent': 'text-accent-600'
  }[color];
</script>

<div class="flex items-center justify-center">
  <span class={`loading loading-spinner ${sizeClass} ${colorClass}`}></span>
</div>