// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

import { randomBytes } from 'crypto'

/**
 * IPC authentication token
 */
export interface IPCToken {
  token: string
  expiresAt: number
  windowId: number
}

/**
 * Token configuration
 */
export interface TokenConfig {
  ttl: number // Time to live in milliseconds
}

/**
 * Default token TTL: 5 minutes
 */
const DEFAULT_TTL = 5 * 60 * 1000

/**
 * Token storage
 */
const tokens = new Map<string, IPCToken>()

/**
 * Generates a cryptographically secure random token
 */
function generateSecureToken(): string {
  return randomBytes(32).toString('hex')
}

/**
 * Creates a new IPC authentication token for a window
 * @param windowId - The BrowserWindow ID
 * @param config - Optional token configuration
 * @returns The generated token
 */
export function createIPCToken(windowId: number, config?: Partial<TokenConfig>): string {
  const ttl = config?.ttl ?? DEFAULT_TTL
  const token = generateSecureToken()
  const expiresAt = Date.now() + ttl

  const ipcToken: IPCToken = {
    token,
    expiresAt,
    windowId
  }

  tokens.set(token, ipcToken)

  // Auto-cleanup expired token
  setTimeout(() => {
    revokeIPCToken(token)
  }, ttl)

  return token
}

/**
 * Validates an IPC token
 * @param token - The token to validate
 * @param windowId - The window ID making the request
 * @returns True if the token is valid
 */
export function validateIPCToken(token: string, windowId: number): boolean {
  const ipcToken = tokens.get(token)

  if (!ipcToken) {
    return false
  }

  // Check if token is expired
  if (Date.now() > ipcToken.expiresAt) {
    revokeIPCToken(token)
    return false
  }

  // Check if token belongs to the correct window
  if (ipcToken.windowId !== windowId) {
    return false
  }

  return true
}

/**
 * Revokes an IPC token
 * @param token - The token to revoke
 */
export function revokeIPCToken(token: string): void {
  tokens.delete(token)
}

/**
 * Revokes all tokens for a specific window
 * @param windowId - The window ID
 */
export function revokeWindowTokens(windowId: number): void {
  for (const [token, ipcToken] of tokens.entries()) {
    if (ipcToken.windowId === windowId) {
      tokens.delete(token)
    }
  }
}

/**
 * Cleans up all expired tokens
 */
export function cleanupExpiredTokens(): void {
  const now = Date.now()
  for (const [token, ipcToken] of tokens.entries()) {
    if (now > ipcToken.expiresAt) {
      tokens.delete(token)
    }
  }
}

/**
 * Gets the number of active tokens
 */
export function getActiveTokenCount(): number {
  return tokens.size
}
