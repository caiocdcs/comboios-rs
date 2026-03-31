import type { StationResponse, StationBoardResponse, BackendStation, TrainDetails } from './types';
import { handleApiResponse } from './errors';

const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:3000';

export async function searchStations(query: string): Promise<StationResponse> {
  const response = await fetch(`${API_BASE_URL}/stations?query=${encodeURIComponent(query)}`);
  const rawData = await handleApiResponse<{ data: BackendStation[] }>(response);
  
  const mappedData = rawData.data.map((station: BackendStation) => ({
    id: station.code,
    name: station.designation
  }));
  
  return {
    data: mappedData
  };
}

export async function getStationTimetable(stationId: string): Promise<StationBoardResponse> {
  const response = await fetch(`${API_BASE_URL}/stations/timetable/${encodeURIComponent(stationId)}`);
  const rawData = await handleApiResponse<{ data: any[] }>(response);
  
  const mappedData = rawData.data.map((board: any) => ({
    ...board,
    station_id: String(board.station_id)
  }));
  
  return {
    data: mappedData
  };
}

export async function getTrainJourney(trainId: string, date?: string): Promise<TrainDetails> {
  let url = `${API_BASE_URL}/trains/${encodeURIComponent(trainId)}/journey`;
  if (date) {
    url += `?date=${encodeURIComponent(date)}`;
  }
  
  const response = await fetch(url);
  const rawData = await handleApiResponse<{ data: any }>(response);
  return mapToTrainDetails(rawData.data);
}

function mapToTrainDetails(journey: any): TrainDetails {
  const delayMinutes = journey.delay_minutes;
  
  return {
    train_number: parseInt(journey.train_number, 10),
    service_type: journey.service_type,
    origin: typeof journey.origin === 'string' ? journey.origin : journey.origin.designation,
    destination: typeof journey.destination === 'string' ? journey.destination : journey.destination.designation,
    operator: journey.operator,
    date: new Date().toISOString().split('T')[0],
    status: delayMinutes ? 'delayed' : 'scheduled',
    delay_minutes: delayMinutes,
    observations: journey.observations,
    duration: journey.duration,
    stops: journey.stops.map((stop: any) => {
      const statusStr = (stop.status || '').toLowerCase();
      const hasPassed = stop.has_passed === true || statusStr === 'passed';
      
      return {
        station_name: typeof stop.station === 'string' ? stop.station : stop.station.designation,
        station_id: typeof stop.station === 'string' ? stop.station : stop.station.code,
        scheduled_time: stop.scheduled_departure || stop.scheduled_arrival,
        actual_time: stop.actual_departure || stop.actual_arrival,
        delay_minutes: stop.delay_minutes,
        platform: stop.platform,
        status: hasPassed ? 'passed' : 'upcoming',
        has_passed: hasPassed,
        predicted_time: stop.predicted_time,
      };
    }),
  };
}
