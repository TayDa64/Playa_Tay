import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import UnoCSS from 'unocss/vite';
import { resolve } from 'path';

// M1 Streaming Hub - Vite Configuration
// Port: 1421 (per plan.md)
// Security: CSP-compliant build, no inline scripts

export default defineConfig({
  plugins: [
    UnoCSS(),
    svelte(),
  ],

  resolve: {
    alias: {
      $lib: resolve(__dirname, './src/lib'),
      $components: resolve(__dirname, './src/components'),
      $views: resolve(__dirname, './src/views'),
    },
  },

  server: {
    port: 1421,
    strictPort: true,
    host: 'localhost',
  },

  // Optimize for module isolation
  build: {
    target: 'esnext',
    minify: 'esbuild',
    sourcemap: true,
    rollupOptions: {
      output: {
        manualChunks: {
          'video-player': ['video.js'],
          'state': ['zustand'],
        },
      },
    },
  },

  // Clear console for clean development experience
  clearScreen: false,

  // Tauri-specific optimizations
  envPrefix: ['VITE_', 'TAURI_'],
});
