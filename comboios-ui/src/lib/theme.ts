export type Theme = 'light' | 'dark';

const STATUS_BAR = { light: '#f5f7fb', dark: '#0a1020' } as const;

/** Apply a theme to Tailwind (`dark` class), daisyUI (`data-theme`) and the phone status bar */
export function applyTheme(theme: Theme) {
  const root = document.documentElement;
  root.classList.toggle('dark', theme === 'dark');
  root.setAttribute('data-theme', theme);

  // An explicit choice overrides the media-specific theme-color tags
  document.querySelectorAll('meta[name="theme-color"]').forEach((meta) => {
    meta.setAttribute('content', STATUS_BAR[theme]);
  });
}

export function currentTheme(): Theme {
  return document.documentElement.classList.contains('dark') ? 'dark' : 'light';
}
