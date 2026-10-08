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
