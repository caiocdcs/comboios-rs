<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { getStationTimetable } from '$lib/api';
  import { parseDelayMinutes, formatTimeWithDelay } from '$lib/types';
  import type { StationBoard, TrainEntry } from '$lib/types';

  let stationId = $page.params.id;
  let boards: StationBoard[] = [];
  let loading = true;
  let error: string | null = null;

  $: {
    stationId = $page.params.id;
    loadTimetable();
  }

  async function loadTimetable() {
    if (!stationId) return;
    
    loading = true;
    error = null;
    
    try {
      const response = await getStationTimetable(stationId);
      boards = response.data;
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to load timetable';
      boards = [];
    } finally {
      loading = false;
    }
  }

  function goBack() {
    goto('/');
  }

  function getAllTrains(): Array<{ board: StationBoard; train: TrainEntry }> {
    const result: Array<{ board: StationBoard; train: TrainEntry }> = [];
    for (const board of boards) {
      for (const train of board.trains) {
        result.push({ board, train });
      }
    }
    return result;
  }

  $: allTrains = getAllTrains();
</script>

<div class="container mx-auto px-4 py-8">
  <div class="mb-6">
    <button class="btn btn-ghost btn-sm" on:click={goBack}>
      ← Back
    </button>
  </div>

  <div class="card bg-base-100 shadow-xl">
    <div class="card-body">
      {#if loading}
        <div class="flex justify-center py-12">
          <span class="loading loading-spinner loading-lg"></span>
        </div>
      {:else if error}
        <div class="alert alert-error">
          <span>{error}</span>
        </div>
      {:else if allTrains.length === 0}
        <div class="text-center text-gray-500 py-12">
          No trains found for this station.
        </div>
      {:else}
        <h2 class="card-title text-2xl mb-6">
          {boards[0]?.station_name || 'Station'} - Upcoming Trains
        </h2>

        <div class="overflow-x-auto">
          <table class="table table-zebra w-full">
            <thead>
              <tr>
                <th>Service</th>
                <th>Origin</th>
                <th>Destination</th>
                <th>Time</th>
                <th>Train</th>
                <th>Delay</th>
                <th>Status</th>
              </tr>
            </thead>
            <tbody>
              {#each allTrains as { train }}
                {@const delayMinutes = parseDelayMinutes(train.observations)}
                <tr class="hover:bg-base-200">
                  <td>
                    <span class="badge badge-primary badge-sm">{train.service_type}</span>
                  </td>
                  <td>{train.origin_station_name}</td>
                  <td>{train.destination_station_name}</td>
                  <td class="font-mono">{formatTimeWithDelay(train.time, delayMinutes)}</td>
                  <td class="font-mono">{train.train_number}</td>
                  <td>
                    {#if delayMinutes && delayMinutes > 0}
                      <span class="badge badge-error badge-sm">+{delayMinutes} min</span>
                    {:else}
                      <span class="badge badge-success badge-sm">On time</span>
                    {/if}
                  </td>
                  <td>
                    {#if train.has_passed}
                      <span class="badge badge-ghost badge-sm">Departed</span>
                    {:else}
                      <span class="badge badge-success badge-sm">Scheduled</span>
                    {/if}
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      {/if}
    </div>
  </div>
</div>
