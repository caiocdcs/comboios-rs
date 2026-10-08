import { addMinutes } from './date';
import type { JourneyStop } from './types';

/** Best known time at a stop: CP prediction, else scheduled + delay */
export function expectedStopTime(stop: JourneyStop): string {
  if (stop.predicted_time) return stop.predicted_time;
  const delay = stop.delay_minutes ?? 0;
  return delay > 0 ? addMinutes(stop.scheduled_time, delay) : stop.scheduled_time;
}

/** Index of the last stop the train has passed, or -1 before departure */
export function lastPassedIndex(stops: JourneyStop[]): number {
  return stops.reduce((last, stop, i) => (stop.has_passed ? i : last), -1);
}
