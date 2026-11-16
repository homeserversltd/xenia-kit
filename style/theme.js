// Shared theme control, lifted from Coronatio's shell theme functions.
(() => {
  'use strict';

  const headerStateKey = 'coronatio.flask-react-header.v1';
  const preferredThemeKey = 'preferred-theme';
  const themeDataKey = 'themeData';
  const themeButton = document.querySelector('[data-theme-button]');
  const savedHeaderState = (() => { try { return JSON.parse(localStorage.getItem(headerStateKey) || '{}'); } catch (_) { return {}; } })();
  const savedPreferredTheme = localStorage.getItem(preferredThemeKey);
  let themeCatalog;
  let themes = [];
  const headerState = { theme: savedPreferredTheme || savedHeaderState?.theme };

  function saveHeaderState() {
    localStorage.setItem(headerStateKey, JSON.stringify({ theme: headerState.theme }));
    localStorage.setItem(preferredThemeKey, headerState.theme);
    localStorage.setItem(themeDataKey, JSON.stringify({ name: headerState.theme, values: themeCatalog.themes[headerState.theme] || {} }));
  }

  function themeToCss(theme) {
    if (!theme) return '';
    const lines = [];
    Object.entries(theme).forEach(([key, value]) => {
      lines.push('  --theme-' + key + ': ' + value + ';');
    });
    const hexToRgb = value => {
      const match = /^#([0-9a-f]{6})$/i.exec(String(value || '').trim());
      if (!match) return null;
      const number = Number.parseInt(match[1], 16);
      return [(number >> 16) & 255, (number >> 8) & 255, number & 255].join(', ');
    };
    const primaryRgb = hexToRgb(theme.primary);
    const backgroundRgb = hexToRgb(theme.background);
    lines.push('  --theme-accent-soft: color-mix(in srgb, var(--theme-accent) 16%, transparent);');
    if (primaryRgb) lines.push('  --theme-primary-rgb: ' + primaryRgb + ';');
    if (backgroundRgb) lines.push('  --theme-background-rgb: ' + backgroundRgb + ';');
    return ':root {\n' + lines.join('\n') + '\n}';
  }

  function ensureThemeStyleElement() {
    let style = document.querySelector('style[data-theme-styles]');
    if (!style) {
      style = document.createElement('style');
      style.setAttribute('data-theme-styles', '');
      document.head.appendChild(style);
    }
    return style;
  }

  function applyTheme() {
    if (!themes.includes(headerState.theme)) headerState.theme = themeCatalog.default || themes[0];
    const theme = themeCatalog.themes[headerState.theme];
    document.documentElement.dataset.theme = headerState.theme;
    ensureThemeStyleElement().textContent = themeToCss(theme);
    saveHeaderState();
    if (themeButton) {
      const label = headerState.theme;
      const span = themeButton.querySelector('span');
      if (span) span.textContent = label;
      themeButton.title = 'Current theme: ' + label + '. Click to switch theme.';
    }
  }

  function cycleTheme() {
    if (!themes.length) return;
    const currentIndex = Math.max(0, themes.indexOf(headerState.theme));
    headerState.theme = themes[(currentIndex + 1) % themes.length];
    saveHeaderState();
    applyTheme();
  }
  themeButton?.addEventListener('click', cycleTheme);

  async function loadThemeCatalog() {
    try {
      const response = await fetch('/api/themes');
      if (!response.ok) throw new Error('HTTP ' + response.status);
      const catalog = await response.json();
      if (!catalog.themes || Object.keys(catalog.themes).length === 0) throw new Error('empty theme catalog');
      themeCatalog = catalog;
      themes = Object.keys(catalog.themes);
      if (!themes.includes(themeCatalog.default)) throw new Error('invalid default theme');
      headerState.theme = headerState.theme || themeCatalog.default;
      applyTheme();
    } catch (error) {
      themes = [];
      if (themeButton) themeButton.title = 'Theme catalog unavailable: ' + error;
    }
  }
  loadThemeCatalog();
})();
