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
  estimated_departure?: string | null;
  estimated_arrival?: string | null;
  platform?: string;
  delay?: number;
  service_type: string;
  has_passed: boolean;
  is_departure: boolean;
  operator: string;
  observations?: string | null;
}

export interface StationBoard {
  station_id: string;
  station_name: string;
  trains: TrainEntry[];
}

export interface StationBoardResponse {
  data: StationBoard[];
}

export interface JourneyStop {
  station_name: string;
  station_id: number;
  scheduled_time: string;
  actual_time?: string;
  delay_minutes?: number;
  platform?: string;
  status: "passed" | "current" | "upcoming";
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
  status: "scheduled" | "in-progress" | "completed" | "delayed";
  delay_minutes?: number;
  observations?: string;
  duration?: string;
}
