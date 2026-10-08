/**
 * Today's date in Portugal as YYYY-MM-DD.
 *
 * `new Date().toISOString()` is UTC, which is the previous day between
 * midnight and 01:00 Lisbon time in summer.
 */
export function lisbonToday(): string {
  // en-CA formats dates as YYYY-MM-DD
  return new Intl.DateTimeFormat('en-CA', { timeZone: 'Europe/Lisbon' }).format(new Date());
}

const lisbonClock = new Intl.DateTimeFormat('en-GB', {
  timeZone: 'Europe/Lisbon',
  hour: '2-digit',
  minute: '2-digit',
  hourCycle: 'h23'
});

/** Current Lisbon wall-clock time as minutes since midnight */
export function lisbonNowMinutes(now: Date = new Date()): number {
  return toMinutes(lisbonClock.format(now)) ?? 0;
}

/** Parse "HH:MM" into minutes since midnight */
export function toMinutes(time: string | null | undefined): number | null {
  if (!time) return null;
  const [h, m] = time.split(':').map(Number);
  if (Number.isNaN(h) || Number.isNaN(m)) return null;
  return h * 60 + m;
}

/** Add minutes to an "HH:MM" time, wrapping past midnight */
export function addMinutes(time: string, minutes: number): string {
  const total = (((toMinutes(time) ?? 0) + minutes) % 1440 + 1440) % 1440;
  const pad = (n: number) => String(n).padStart(2, '0');
  return `${pad(Math.floor(total / 60))}:${pad(total % 60)}`;
}

/**
 * Minutes from now until an "HH:MM" Lisbon time (negative if in the past).
 *
 * Boards and journeys span at most a few hours either side of now, so a
 * difference of more than 12 hours means the time is across midnight:
 * 00:10 seen at 23:50 is in 20 minutes, not 23h40 ago.
 */
export function minutesUntil(time: string | null | undefined, nowMinutes: number): number | null {
  const t = toMinutes(time);
  if (t === null) return null;
  let diff = t - nowMinutes;
  if (diff < -720) diff += 1440;
  if (diff > 720) diff -= 1440;
  return diff;
}

/** "now", "in 4 min", "in 1 h 05" for upcoming times */
export function formatCountdown(minutes: number | null): string {
  if (minutes === null) return '';
  if (minutes <= 0) return 'now';
  if (minutes < 60) return `in ${minutes} min`;
  const h = Math.floor(minutes / 60);
  return `in ${h} h ${String(minutes % 60).padStart(2, '0')}`;
}

/** "just now", "40 s ago", "3 min ago" */
export function formatAge(since: Date, now: Date = new Date()): string {
  const seconds = Math.max(0, Math.round((now.getTime() - since.getTime()) / 1000));
  if (seconds < 10) return 'just now';
  if (seconds < 60) return `${seconds} s ago`;
  return `${Math.floor(seconds / 60)} min ago`;
}
