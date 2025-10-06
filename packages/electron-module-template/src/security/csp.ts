// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

import type { Session, OnHeadersReceivedListenerDetails } from 'electron'

/**
 * Content Security Policy configuration
 */
export interface CSPConfig {
  defaultSrc?: string[]
  scriptSrc?: string[]
  styleSrc?: string[]
  imgSrc?: string[]
  connectSrc?: string[]
  fontSrc?: string[]
  objectSrc?: string[]
  mediaSrc?: string[]
  frameSrc?: string[]
}

/**
 * Default strict CSP configuration
 */
const DEFAULT_CSP: CSPConfig = {
  defaultSrc: ["'self'"],
  scriptSrc: ["'self'"],
  styleSrc: ["'self'", "'unsafe-inline'"], // Allow inline styles for basic styling
  imgSrc: ["'self'", 'data:', 'https:'],
  connectSrc: ["'self'", 'https:'],
  fontSrc: ["'self'"],
  objectSrc: ["'none'"],
  mediaSrc: ["'self'"],
  frameSrc: ["'none'"]
}

/**
 * Builds a CSP string from configuration
 */
function buildCSPString(config: CSPConfig): string {
  const directives: string[] = []

  const directiveMap: Record<keyof CSPConfig, string> = {
    defaultSrc: 'default-src',
    scriptSrc: 'script-src',
    styleSrc: 'style-src',
    imgSrc: 'img-src',
    connectSrc: 'connect-src',
    fontSrc: 'font-src',
    objectSrc: 'object-src',
    mediaSrc: 'media-src',
    frameSrc: 'frame-src'
  }

  for (const [key, directiveName] of Object.entries(directiveMap)) {
    const values = config[key as keyof CSPConfig]
    if (values && values.length > 0) {
      directives.push(`${directiveName} ${values.join(' ')}`)
    }
  }

  return directives.join('; ') + ';'
}

/**
 * Applies Content Security Policy to a session
 * @param session - Electron session object
 * @param customConfig - Optional custom CSP configuration (merged with defaults)
 */
export function enforceCSP(session: Session, customConfig?: Partial<CSPConfig>): void {
  const config: CSPConfig = {
    ...DEFAULT_CSP,
    ...customConfig
  }

  const cspString = buildCSPString(config)

  session.webRequest.onHeadersReceived(
    (
      details: OnHeadersReceivedListenerDetails,
      callback: (response: { responseHeaders?: Record<string, string[]> }) => void
    ) => {
      callback({
        responseHeaders: {
          ...details.responseHeaders,
          'Content-Security-Policy': [cspString]
        }
      })
    }
  )
}

/**
 * Gets the default CSP configuration
 */
export function getDefaultCSP(): CSPConfig {
  // Return a deep copy to prevent mutation
  return {
    defaultSrc: [...DEFAULT_CSP.defaultSrc!],
    scriptSrc: [...DEFAULT_CSP.scriptSrc!],
    styleSrc: [...DEFAULT_CSP.styleSrc!],
    imgSrc: [...DEFAULT_CSP.imgSrc!],
    connectSrc: [...DEFAULT_CSP.connectSrc!],
    fontSrc: [...DEFAULT_CSP.fontSrc!],
    objectSrc: [...DEFAULT_CSP.objectSrc!],
    mediaSrc: [...DEFAULT_CSP.mediaSrc!],
    frameSrc: [...DEFAULT_CSP.frameSrc!]
  }
}

/**
 * Gets the CSP string representation
 */
export function getCSPString(config?: Partial<CSPConfig>): string {
  const finalConfig: CSPConfig = {
    ...DEFAULT_CSP,
    ...config
  }
  return buildCSPString(finalConfig)
}
