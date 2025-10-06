// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

import { describe, it, expect } from 'vitest'
import { enforceSecurityFlags, getSecurityFlags } from '../src/security/flags.js'

describe('Security Flags', () => {
  describe('enforceSecurityFlags', () => {
    it('should enforce context isolation', () => {
      const options = enforceSecurityFlags({})
      expect(options.webPreferences?.contextIsolation).toBe(true)
    })

    it('should disable node integration', () => {
      const options = enforceSecurityFlags({})
      expect(options.webPreferences?.nodeIntegration).toBe(false)
    })

    it('should enable sandbox', () => {
      const options = enforceSecurityFlags({})
      expect(options.webPreferences?.sandbox).toBe(true)
    })

    it('should disable node integration in workers', () => {
      const options = enforceSecurityFlags({})
      expect(options.webPreferences?.nodeIntegrationInWorker).toBe(false)
    })

    it('should disable node integration in subframes', () => {
      const options = enforceSecurityFlags({})
      expect(options.webPreferences?.nodeIntegrationInSubFrames).toBe(false)
    })

    it('should enable web security', () => {
      const options = enforceSecurityFlags({})
      expect(options.webPreferences?.webSecurity).toBe(true)
    })

    it('should preserve other options', () => {
      const options = enforceSecurityFlags({
        width: 800,
        height: 600,
        title: 'Test Window'
      })
      expect(options.width).toBe(800)
      expect(options.height).toBe(600)
      expect(options.title).toBe('Test Window')
    })

    it('should disable devTools in production', () => {
      const originalEnv = process.env.NODE_ENV
      process.env.NODE_ENV = 'production'
      
      const options = enforceSecurityFlags({})
      expect(options.webPreferences?.devTools).toBe(false)
      
      process.env.NODE_ENV = originalEnv
    })

    it('should enable devTools in development', () => {
      const originalEnv = process.env.NODE_ENV
      process.env.NODE_ENV = 'development'
      
      const options = enforceSecurityFlags({})
      expect(options.webPreferences?.devTools).toBe(true)
      
      process.env.NODE_ENV = originalEnv
    })

    it('should not allow overriding security-critical settings', () => {
      const options = enforceSecurityFlags({
        webPreferences: {
          contextIsolation: false, // Attempt to override
          nodeIntegration: true,   // Attempt to override
          sandbox: false           // Attempt to override
        }
      })
      
      // Security settings should be enforced
      expect(options.webPreferences?.contextIsolation).toBe(true)
      expect(options.webPreferences?.nodeIntegration).toBe(false)
      expect(options.webPreferences?.sandbox).toBe(true)
    })
  })

  describe('getSecurityFlags', () => {
    it('should return current security configuration', () => {
      const flags = getSecurityFlags()
      expect(flags).toHaveProperty('contextIsolation')
      expect(flags).toHaveProperty('nodeIntegration')
      expect(flags).toHaveProperty('sandbox')
      expect(flags).toHaveProperty('devTools')
    })

    it('should return correct production flags', () => {
      const originalEnv = process.env.NODE_ENV
      process.env.NODE_ENV = 'production'
      
      const flags = getSecurityFlags()
      expect(flags.contextIsolation).toBe(true)
      expect(flags.nodeIntegration).toBe(false)
      expect(flags.sandbox).toBe(true)
      expect(flags.devTools).toBe(false)
      
      process.env.NODE_ENV = originalEnv
    })

    it('should return correct development flags', () => {
      const originalEnv = process.env.NODE_ENV
      process.env.NODE_ENV = 'development'
      
      const flags = getSecurityFlags()
      expect(flags.contextIsolation).toBe(true)
      expect(flags.nodeIntegration).toBe(false)
      expect(flags.sandbox).toBe(true)
      expect(flags.devTools).toBe(true)
      
      process.env.NODE_ENV = originalEnv
    })
  })
})
