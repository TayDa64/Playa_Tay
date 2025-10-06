// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

import { describe, it, expect } from 'vitest'
import { getDefaultCSP, getCSPString } from '../src/security/csp.js'

describe('Content Security Policy', () => {
  describe('getDefaultCSP', () => {
    it('should return default CSP configuration', () => {
      const csp = getDefaultCSP()
      
      expect(csp).toBeDefined()
      expect(csp.defaultSrc).toEqual(["'self'"])
      expect(csp.scriptSrc).toEqual(["'self'"])
      expect(csp.styleSrc).toEqual(["'self'", "'unsafe-inline'"])
      expect(csp.imgSrc).toEqual(["'self'", 'data:', 'https:'])
      expect(csp.connectSrc).toEqual(["'self'", 'https:'])
      expect(csp.fontSrc).toEqual(["'self'"])
      expect(csp.objectSrc).toEqual(["'none'"])
      expect(csp.mediaSrc).toEqual(["'self'"])
      expect(csp.frameSrc).toEqual(["'none'"])
    })

    it('should not share references', () => {
      const csp1 = getDefaultCSP()
      const csp2 = getDefaultCSP()
      
      csp1.scriptSrc?.push('test')
      expect(csp2.scriptSrc).not.toContain('test')
    })
  })

  describe('getCSPString', () => {
    it('should build valid CSP string from default config', () => {
      const cspString = getCSPString()
      
      expect(cspString).toContain("default-src 'self'")
      expect(cspString).toContain("script-src 'self'")
      expect(cspString).toContain("style-src 'self' 'unsafe-inline'")
      expect(cspString).toContain("img-src 'self' data: https:")
      expect(cspString).toContain("connect-src 'self' https:")
      expect(cspString).toContain("font-src 'self'")
      expect(cspString).toContain("object-src 'none'")
      expect(cspString).toContain("media-src 'self'")
      expect(cspString).toContain("frame-src 'none'")
    })

    it('should end with semicolon', () => {
      const cspString = getCSPString()
      expect(cspString.endsWith(';')).toBe(true)
    })

    it('should use semicolon as separator', () => {
      const cspString = getCSPString()
      const directives = cspString.split(';').filter((d) => d.trim())
      
      expect(directives.length).toBeGreaterThan(5)
    })

    it('should merge custom config with defaults', () => {
      const cspString = getCSPString({
        scriptSrc: ["'self'", "'unsafe-eval'"],
        connectSrc: ["'self'", 'https://api.example.com']
      })
      
      expect(cspString).toContain("script-src 'self' 'unsafe-eval'")
      expect(cspString).toContain('connect-src \'self\' https://api.example.com')
      // Other defaults should still be present
      expect(cspString).toContain("default-src 'self'")
      expect(cspString).toContain("object-src 'none'")
    })

    it('should handle empty arrays', () => {
      const cspString = getCSPString({
        scriptSrc: []
      })
      
      // Should not include script-src if empty
      expect(cspString).not.toContain('script-src')
    })

    it('should build strict CSP by default', () => {
      const cspString = getCSPString()
      
      // Should not contain unsafe directives except necessary ones
      expect(cspString).not.toContain("'unsafe-eval'")
      expect(cspString).not.toContain("script-src 'unsafe-inline'")
      
      // Should block objects and frames
      expect(cspString).toContain("object-src 'none'")
      expect(cspString).toContain("frame-src 'none'")
    })

    it('should allow necessary unsafe-inline for styles', () => {
      const cspString = getCSPString()
      
      // Style inline is allowed for basic styling (common need)
      expect(cspString).toContain("style-src 'self' 'unsafe-inline'")
    })

    it('should support data URIs for images', () => {
      const cspString = getCSPString()
      
      expect(cspString).toContain('img-src \'self\' data: https:')
    })

    it('should allow HTTPS connections', () => {
      const cspString = getCSPString()
      
      expect(cspString).toContain('connect-src \'self\' https:')
      expect(cspString).toContain('img-src \'self\' data: https:')
    })
  })

  describe('CSP directive formatting', () => {
    it('should properly space multiple values', () => {
      const cspString = getCSPString({
        scriptSrc: ["'self'", "'nonce-abc123'", 'https://cdn.example.com']
      })
      
      expect(cspString).toContain("script-src 'self' 'nonce-abc123' https://cdn.example.com")
    })

    it('should handle single value directives', () => {
      const cspString = getCSPString({
        objectSrc: ["'none'"]
      })
      
      expect(cspString).toContain("object-src 'none'")
    })
  })

  describe('Security validation', () => {
    it('should block all objects by default', () => {
      const csp = getDefaultCSP()
      expect(csp.objectSrc).toEqual(["'none'"])
    })

    it('should block all frames by default', () => {
      const csp = getDefaultCSP()
      expect(csp.frameSrc).toEqual(["'none'"])
    })

    it('should not allow eval by default', () => {
      const cspString = getCSPString()
      expect(cspString).not.toContain("'unsafe-eval'")
    })

    it('should not allow inline scripts by default', () => {
      const cspString = getCSPString()
      const scriptSrcMatch = cspString.match(/script-src[^;]+/)
      expect(scriptSrcMatch).toBeDefined()
      expect(scriptSrcMatch![0]).not.toContain("'unsafe-inline'")
    })

    it('should default to self for most resources', () => {
      const csp = getDefaultCSP()
      expect(csp.defaultSrc).toContain("'self'")
      expect(csp.scriptSrc).toContain("'self'")
      expect(csp.styleSrc).toContain("'self'")
      expect(csp.fontSrc).toContain("'self'")
      expect(csp.mediaSrc).toContain("'self'")
    })
  })
})
