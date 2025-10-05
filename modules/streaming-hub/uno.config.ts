import { defineConfig, presetUno, presetAttributify, presetIcons } from 'unocss';

// M1 Streaming Hub - UnoCSS Configuration
// Theme: Follows VISION.md color palette
// Performance: Atomic CSS for minimal bundle size

export default defineConfig({
  presets: [
    presetUno(),
    presetAttributify(),
    presetIcons({
      scale: 1.2,
      cdn: 'https://esm.sh/',
    }),
  ],

  theme: {
    colors: {
      // VISION.md color palette
      primary: {
        DEFAULT: '#3b82f6',
        dark: '#2563eb',
        light: '#60a5fa',
      },
      secondary: {
        DEFAULT: '#8b5cf6',
        dark: '#7c3aed',
        light: '#a78bfa',
      },
      success: '#10b981',
      warning: '#f59e0b',
      error: '#ef4444',

      // Neutral grays
      gray: {
        50: '#f9fafb',
        100: '#f3f4f6',
        200: '#e5e7eb',
        300: '#d1d5db',
        400: '#9ca3af',
        500: '#6b7280',
        600: '#4b5563',
        700: '#374151',
        800: '#1f2937',
        900: '#111827',
      },
    },

    fontFamily: {
      sans: ['Inter', 'system-ui', 'sans-serif'],
      mono: ['JetBrains Mono', 'monospace'],
    },

    spacing: {
      // 4px base unit for consistency
      xs: '0.25rem',  // 4px
      sm: '0.5rem',   // 8px
      md: '1rem',     // 16px
      lg: '1.5rem',   // 24px
      xl: '2rem',     // 32px
      '2xl': '3rem',  // 48px
    },
  },

  shortcuts: {
    // Common patterns
    'btn': 'px-4 py-2 rounded-lg font-medium transition-colors',
    'btn-primary': 'btn bg-primary text-white hover:bg-primary-dark',
    'btn-secondary': 'btn bg-gray-200 text-gray-900 hover:bg-gray-300',
    'card': 'bg-white rounded-xl shadow-sm p-md',
    'card-dark': 'bg-gray-800 rounded-xl shadow-sm p-md',
  },

  // Performance optimization
  safelist: [],
});
