// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

import { contextBridge } from 'electron'

/**
 * Minimal preload script with context isolation
 * Exposes only safe, specific APIs to the renderer
 */

/**
 * API exposed to renderer process
 */
const api = {
  // Module information
  module: {
    name: 'electron-module-template',
    version: '0.1.0'
  },

  // Safe methods that can be exposed
  platform: process.platform,
  versions: {
    node: process.versions.node,
    chrome: process.versions.chrome,
    electron: process.versions.electron
  }
}

// Expose the API to the renderer process
contextBridge.exposeInMainWorld('electronAPI', api)

// Log initialization in development
if (process.env.NODE_ENV !== 'production') {
  console.log('[Preload] Context bridge initialized')
  console.log('[Preload] API exposed:', Object.keys(api))
}
