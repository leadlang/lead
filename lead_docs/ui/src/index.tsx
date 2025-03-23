import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';

import './globals.css';
import { initTheme } from './utils/theme';
import { Page } from './utils/const';

declare global {
  interface Window {
    leadver: string;
    os: string;
    arch: string;
    target: string;
    workspace: boolean;

    setPage: (_: Page) => void;
  }
}

initTheme();
const rootEl = document.getElementById('root');
if (rootEl) {
  const root = ReactDOM.createRoot(rootEl);
  root.render(
    <React.StrictMode>
      <App />
    </React.StrictMode>,
  );
}
