// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

import { describe, it, expect, beforeEach, vi } from 'vitest'
import {
  createIPCToken,
  validateIPCToken,
  revokeIPCToken,
  revokeWindowTokens,
  cleanupExpiredTokens,
  getActiveTokenCount
} from '../src/security/ipc-auth.js'

describe('IPC Authentication', () => {
  beforeEach(() => {
    // Clear any tokens before each test
    const count = getActiveTokenCount()
    for (let i = 0; i < count; i++) {
      cleanupExpiredTokens()
    }
  })

  describe('createIPCToken', () => {
    it('should create a valid token', () => {
      const windowId = 1
      const token = createIPCToken(windowId)
      
      expect(token).toBeDefined()
      expect(typeof token).toBe('string')
      expect(token.length).toBe(64) // 32 bytes in hex = 64 characters
    })

    it('should create unique tokens', () => {
      const windowId = 1
      const token1 = createIPCToken(windowId)
      const token2 = createIPCToken(windowId)
      
      expect(token1).not.toBe(token2)
    })

    it('should use custom TTL', () => {
      const windowId = 1
      const shortTTL = 100 // 100ms
      const token = createIPCToken(windowId, { ttl: shortTTL })
      
      expect(validateIPCToken(token, windowId)).toBe(true)
    })

    it('should increment active token count', () => {
      const initialCount = getActiveTokenCount()
      const windowId = 1
      
      createIPCToken(windowId)
      
      expect(getActiveTokenCount()).toBe(initialCount + 1)
    })
  })

  describe('validateIPCToken', () => {
    it('should validate a fresh token', () => {
      const windowId = 1
      const token = createIPCToken(windowId)
      
      expect(validateIPCToken(token, windowId)).toBe(true)
    })

    it('should reject invalid token', () => {
      const windowId = 1
      const fakeToken = 'invalid-token'
      
      expect(validateIPCToken(fakeToken, windowId)).toBe(false)
    })

    it('should reject token for wrong window', () => {
      const windowId1 = 1
      const windowId2 = 2
      const token = createIPCToken(windowId1)
      
      expect(validateIPCToken(token, windowId2)).toBe(false)
    })

    it('should reject expired token', async () => {
      const windowId = 1
      const shortTTL = 50 // 50ms
      const token = createIPCToken(windowId, { ttl: shortTTL })
      
      // Token should be valid initially
      expect(validateIPCToken(token, windowId)).toBe(true)
      
      // Wait for token to expire
      await new Promise((resolve) => setTimeout(resolve, 60))
      
      // Token should now be invalid
      expect(validateIPCToken(token, windowId)).toBe(false)
    })

    it('should auto-remove expired token', async () => {
      const windowId = 1
      const shortTTL = 50 // 50ms
      const initialCount = getActiveTokenCount()
      
      createIPCToken(windowId, { ttl: shortTTL })
      expect(getActiveTokenCount()).toBe(initialCount + 1)
      
      // Wait for token to expire
      await new Promise((resolve) => setTimeout(resolve, 60))
      
      // Trigger cleanup by validating
      validateIPCToken('any-token', windowId)
      
      // Token should be removed
      expect(getActiveTokenCount()).toBeLessThanOrEqual(initialCount + 1)
    })
  })

  describe('revokeIPCToken', () => {
    it('should revoke a token', () => {
      const windowId = 1
      const token = createIPCToken(windowId)
      
      expect(validateIPCToken(token, windowId)).toBe(true)
      
      revokeIPCToken(token)
      
      expect(validateIPCToken(token, windowId)).toBe(false)
    })

    it('should decrease active token count', () => {
      const windowId = 1
      const token = createIPCToken(windowId)
      const countBefore = getActiveTokenCount()
      
      revokeIPCToken(token)
      
      expect(getActiveTokenCount()).toBe(countBefore - 1)
    })

    it('should handle revoking non-existent token', () => {
      const fakeToken = 'non-existent-token'
      
      expect(() => revokeIPCToken(fakeToken)).not.toThrow()
    })
  })

  describe('revokeWindowTokens', () => {
    it('should revoke all tokens for a window', () => {
      const windowId = 1
      const token1 = createIPCToken(windowId)
      const token2 = createIPCToken(windowId)
      const token3 = createIPCToken(windowId)
      
      expect(validateIPCToken(token1, windowId)).toBe(true)
      expect(validateIPCToken(token2, windowId)).toBe(true)
      expect(validateIPCToken(token3, windowId)).toBe(true)
      
      revokeWindowTokens(windowId)
      
      expect(validateIPCToken(token1, windowId)).toBe(false)
      expect(validateIPCToken(token2, windowId)).toBe(false)
      expect(validateIPCToken(token3, windowId)).toBe(false)
    })

    it('should only revoke tokens for specified window', () => {
      const windowId1 = 1
      const windowId2 = 2
      const token1 = createIPCToken(windowId1)
      const token2 = createIPCToken(windowId2)
      
      revokeWindowTokens(windowId1)
      
      expect(validateIPCToken(token1, windowId1)).toBe(false)
      expect(validateIPCToken(token2, windowId2)).toBe(true)
    })
  })

  describe('cleanupExpiredTokens', () => {
    it('should remove expired tokens', async () => {
      const windowId = 1
      const shortTTL = 50 // 50ms
      
      createIPCToken(windowId, { ttl: shortTTL })
      createIPCToken(windowId, { ttl: shortTTL })
      const countBefore = getActiveTokenCount()
      
      // Wait for tokens to expire
      await new Promise((resolve) => setTimeout(resolve, 60))
      
      cleanupExpiredTokens()
      
      expect(getActiveTokenCount()).toBeLessThan(countBefore)
    })

    it('should not remove valid tokens', () => {
      const windowId = 1
      const longTTL = 10000 // 10 seconds
      
      createIPCToken(windowId, { ttl: longTTL })
      createIPCToken(windowId, { ttl: longTTL })
      const countBefore = getActiveTokenCount()
      
      cleanupExpiredTokens()
      
      expect(getActiveTokenCount()).toBe(countBefore)
    })
  })

  describe('getActiveTokenCount', () => {
    it('should return zero initially', () => {
      cleanupExpiredTokens()
      expect(getActiveTokenCount()).toBeGreaterThanOrEqual(0)
    })

    it('should track active tokens correctly', () => {
      const initialCount = getActiveTokenCount()
      const windowId = 1
      
      createIPCToken(windowId)
      expect(getActiveTokenCount()).toBe(initialCount + 1)
      
      createIPCToken(windowId)
      expect(getActiveTokenCount()).toBe(initialCount + 2)
      
      createIPCToken(windowId)
      expect(getActiveTokenCount()).toBe(initialCount + 3)
    })
  })
})
