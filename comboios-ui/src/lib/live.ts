/**
 * Keep a page's data fresh while it is on screen.
 *
 * Calls `refresh` every `intervalMs` while the page is visible, and right
 * away when the user comes back to the tab/app if the data is older than
 * `staleAfterMs` (a phone pulled out of a pocket must not show old times).
 * Returns a cleanup function for `onMount`.
 */
export function liveRefresh(
  refresh: () => void,
  getLastUpdated: () => Date | null,
  { intervalMs = 60_000, staleAfterMs = 20_000 } = {}
): () => void {
  const timer = setInterval(() => {
    if (document.visibilityState === 'visible') refresh();
  }, intervalMs);

  const onVisible = () => {
    if (document.visibilityState !== 'visible') return;
    const last = getLastUpdated();
    if (!last || Date.now() - last.getTime() > staleAfterMs) refresh();
  };
  document.addEventListener('visibilitychange', onVisible);

  return () => {
    clearInterval(timer);
    document.removeEventListener('visibilitychange', onVisible);
  };
}
