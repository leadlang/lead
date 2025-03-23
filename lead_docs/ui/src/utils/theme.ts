function systemDark() {
  return window.matchMedia('(prefers-color-scheme: dark)').matches;
}

export function getTheme(): 'light' | 'dark' | 'system' {
  const theme = localStorage.getItem('theme') || 'system';

  return theme as unknown as 'light' | 'dark' | 'system';
}

export function initTheme() {
  const theme = getTheme();

  let force = false;
  switch (theme) {
    case 'light':
      force = false;
      break;
    case 'dark':
      force = true;
      break;
    default:
      force = systemDark();
  }

  console.log('Dark', force);

  document.querySelector('html')?.classList.toggle('dark', force);
}

export function setTheme(theme: 'dark' | 'light' | 'system') {
  localStorage.setItem('theme', theme);

  initTheme();
}
