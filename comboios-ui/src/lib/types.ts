export interface Station {
  id: string;
  name: string;
}

// Backend response structure
export interface BackendStation {
  code: string;
  designation: string;
}

export interface StationResponse {
  data: Station[];
}

export interface TrainEntry {
  train_number: number;
  origin_station_name: string;
  destination_station_name: string;
  departure_time: string | null;
  arrival_time: string | null;
  platform?: string;
  delay?: number;
  service_type: string;
  has_passed: boolean;
  is_departure: boolean;
  operator: string;
}

export interface StationBoard {
  station_id: string;
  station_name: string;
  trains: TrainEntry[];
}

export interface StationBoardResponse {
  data: StationBoard[];
}

export interface ServiceAlert {
  id: string;
  title: string;
  description: string;
  severity: 'info' | 'warning' | 'critical';
  category: 'infrastructure' | 'schedule' | 'weather' | 'technical' | 'event' | 'other';
  affected_lines: string[];
  affected_stations: string[];
  start_time: string | null;
  end_time: string | null;
  last_updated: string;
  url: string | null;
  source: 'infraestruturas-portugal' | 'comboios-portugal' | 'user-reported';
}

export interface JourneyStop {
  station_name: string;
  station_id: number;
  scheduled_time: string;
  actual_time?: string;
  delay_minutes?: number;
  platform?: string;
  status: 'passed' | 'current' | 'upcoming';
  has_passed?: boolean;
  predicted_time?: string;
}

export interface TrainDetails {
  train_number: number;
  service_type: string;
  origin: string;
  destination: string;
  operator: string;
  stops: JourneyStop[];
  date: string;
  status: 'scheduled' | 'in-progress' | 'completed' | 'delayed';
  delay_minutes?: number;
  observations?: string;
  duration?: string;
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
