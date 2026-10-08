import { error } from '@sveltejs/kit';
import { getStationTimetable } from '$lib/api';
import type { PageLoad } from './$types';
import type { StationBoard } from '$lib/types';

export const load: PageLoad = async ({ params }) => {
  try {
    const response = await getStationTimetable(params.id);
    return {
      boards: response.data as StationBoard[],
      stationId: params.id,
      // The server resolves the name from CP's station list; empty if unknown
      stationName: response.data[0]?.station_name ?? ''
    };
  } catch (err) {
    console.error('Failed to load station timetable:', err);
    throw error(500, 'Failed to load station information');
  }
};
