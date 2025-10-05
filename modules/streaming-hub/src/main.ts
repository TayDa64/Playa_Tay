// M1 Streaming Hub - Entry Point
// Phase 1 (Foundation): Initialize Svelte application

import '@unocss/reset/tailwind.css';
import 'uno.css';
import App from './App.svelte';

const app = new App({
  target: document.getElementById('app')!,
});

export default app;
