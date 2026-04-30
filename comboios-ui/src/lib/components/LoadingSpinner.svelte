<script lang="ts">
  export let size: 'sm' | 'md' | 'lg' = 'md';
  export let label = '';

  const dim = { sm: 32, md: 48, lg: 64 }[size];
  const stroke = { sm: 3, md: 2.5, lg: 2 }[size];
</script>

<div class="flex flex-col items-center justify-center gap-3">
  <svg
    width={dim}
    height={dim}
    viewBox="0 0 48 48"
    fill="none"
    xmlns="http://www.w3.org/2000/svg"
    aria-hidden="true"
  >
    <!-- Track -->
    <circle
      cx="24" cy="24" r="20"
      stroke="currentColor"
      stroke-width={stroke}
      class="text-gray-200 dark:text-gray-700"
    />
    <!-- Spinning arc -->
    <circle
      cx="24" cy="24" r="20"
      stroke="currentColor"
      stroke-width={stroke}
      stroke-linecap="round"
      stroke-dasharray="30 96"
      class="text-primary-500 origin-center animate-spin"
      style="animation-duration:0.9s;"
    />
  </svg>
  {#if label}
    <p class="text-sm text-gray-500 dark:text-gray-400">{label}</p>
  {/if}
</div>
