import type { StationResponse, StationBoardResponse } from './types';

const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:3000';

export async function searchStations(query: string): Promise<StationResponse> {
  const response = await fetch(`${API_BASE_URL}/stations?query=${encodeURIComponent(query)}`);
  if (!response.ok) {
    throw new Error(`Failed to search stations: ${response.statusText}`);
  }
  return response.json();
}

export async function getStationTimetable(stationId: string): Promise<StationBoardResponse> {
  const response = await fetch(`${API_BASE_URL}/stations/timetable/${encodeURIComponent(stationId)}`);
  if (!response.ok) {
    throw new Error(`Failed to get timetable: ${response.statusText}`);
  }
  return response.json();
}
