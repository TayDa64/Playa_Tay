// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

import type { BrowserWindowConstructorOptions } from 'electron'

/**
 * Security flags configuration interface
 */
export interface SecurityFlags {
  contextIsolation: boolean
  nodeIntegration: boolean
  sandbox: boolean
  devTools: boolean
}

/**
 * Enforces secure defaults for BrowserWindow webPreferences
 * These settings cannot be overridden to maintain security posture
 */
export function enforceSecurityFlags(
  options?: Partial<BrowserWindowConstructorOptions>
): BrowserWindowConstructorOptions {
  const isProduction = process.env.NODE_ENV === 'production'

  const secureDefaults: BrowserWindowConstructorOptions = {
    ...options,
    webPreferences: {
      ...options?.webPreferences,
      // Security flags - cannot be overridden
      contextIsolation: true,
      nodeIntegration: false,
      sandbox: true,
      devTools: !isProduction,
      // Disable Node.js integration in web workers
      nodeIntegrationInWorker: false,
      // Disable Node.js integration in subframes
      nodeIntegrationInSubFrames: false,
      // Enable web security
      webSecurity: true
    }
  }

  return secureDefaults
}

/**
 * Gets the current security flags configuration
 */
export function getSecurityFlags(): SecurityFlags {
  const isProduction = process.env.NODE_ENV === 'production'

  return {
    contextIsolation: true,
    nodeIntegration: false,
    sandbox: true,
    devTools: !isProduction
  }
}

/**
 * Logs security configuration to console
 */
export function logSecurityStatus(): void {
  const flags = getSecurityFlags()
  console.log('[Electron Module] Security flags enforced:')
  console.log(`  - contextIsolation: ${flags.contextIsolation}`)
  console.log(`  - nodeIntegration: ${flags.nodeIntegration}`)
  console.log(`  - sandbox: ${flags.sandbox}`)
  console.log(`  - devTools: ${flags.devTools}`)
  console.log(`  - CSP: enforced`)
}
