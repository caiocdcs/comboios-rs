import { error } from '@sveltejs/kit';
import { getStationTimetable, searchStations } from '$lib/api';
import type { PageLoad } from './$types';
import type { StationBoard } from '$lib/types';

export const load: PageLoad = async ({ params }) => {
  try {
    const response = await getStationTimetable(params.id);
    let stationName = response.data[0]?.station_name;
    
    if (!stationName) {
      try {
        const stations = await searchStations(params.id.replace('94-', ''));
        stationName = stations.data[0]?.name || params.id;
      } catch {
        stationName = params.id;
      }
    }
    
    return {
      boards: response.data as StationBoard[],
      stationId: params.id,
      stationName
    };
  } catch (err) {
    console.error('Failed to load station timetable:', err);
    throw error(500, 'Failed to load station information');
  }
};
