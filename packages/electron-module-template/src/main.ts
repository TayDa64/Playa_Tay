// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

import { app, BrowserWindow, session } from 'electron'
import { join } from 'path'
import { enforceSecurityFlags, logSecurityStatus } from './security/flags.js'
import { enforceCSP } from './security/csp.js'
import { createIPCToken, revokeWindowTokens } from './security/ipc-auth.js'

/**
 * Creates the main application window with security enforced
 */
async function createWindow(): Promise<void> {
  // Enforce Content Security Policy
  enforceCSP(session.defaultSession)

  // Create window with enforced security flags
  const win = new BrowserWindow(
    enforceSecurityFlags({
      width: 1200,
      height: 800,
      webPreferences: {
        preload: join(__dirname, 'preload.js')
      }
    })
  )

  // Generate IPC authentication token for this window
  const ipcToken = createIPCToken(win.id)
  console.log(`[Electron Module] IPC token generated for window ${win.id}`)

  // Revoke tokens when window is closed
  win.on('closed', () => {
    revokeWindowTokens(win.id)
    console.log(`[Electron Module] Tokens revoked for window ${win.id}`)
  })

  // Load the renderer HTML
  await win.loadFile(join(__dirname, '../src/renderer/index.html'))

  // Log security status
  logSecurityStatus()

  // Development-specific logging
  if (process.env.NODE_ENV !== 'production') {
    console.log(`[Electron Module] Dev mode - IPC Token: ${ipcToken}`)
  }
}

/**
 * Application lifecycle management
 */
app.whenReady().then(async () => {
  await createWindow()

  // macOS: Re-create window when dock icon is clicked and no windows are open
  app.on('activate', async () => {
    if (BrowserWindow.getAllWindows().length === 0) {
      await createWindow()
    }
  })
})

// Quit when all windows are closed (except on macOS)
app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit()
  }
})

// Prevent navigation to external URLs
app.on('web-contents-created', (_event, contents) => {
  contents.on('will-navigate', (event, navigationUrl) => {
    const parsedUrl = new URL(navigationUrl)
    // Only allow navigation to file:// protocol (local files)
    if (parsedUrl.protocol !== 'file:') {
      event.preventDefault()
      console.warn(`[Security] Blocked navigation to: ${navigationUrl}`)
    }
  })

  // Prevent opening new windows
  contents.setWindowOpenHandler(({ url }) => {
    console.warn(`[Security] Blocked new window: ${url}`)
    return { action: 'deny' }
  })
})
