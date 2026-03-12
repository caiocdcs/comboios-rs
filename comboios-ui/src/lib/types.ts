export interface Station {
  id: string;
  name: string;
}

export interface StationResponse {
  data: Station[];
}

export interface TrainEntry {
  train_number: number;
  train_number_alt: number;
  origin_station_name: string;
  destination_station_name: string;
  origin_station_id: number;
  destination_station_id: number;
  time: string;
  date: string;
  observations: string;
  service_type: string;
  has_passed: boolean;
  operator: string;
}

export interface StationBoard {
  station_id: number;
  station_name: string;
  request_type: number;
  trains: TrainEntry[];
}

export interface StationBoardResponse {
  data: StationBoard[];
}

export function parseDelayMinutes(observations: string): number | null {
  if (!observations) return null;
  const match = observations.toLowerCase().match(/atraso\s+de\s+(\d+)\s+min/);
  return match ? parseInt(match[1], 10) : null;
}

export function formatTimeWithDelay(time: string, delayMinutes: number | null): string {
  if (!delayMinutes || delayMinutes <= 0) return time;
  return `${time} (+${delayMinutes} min)`;
}
