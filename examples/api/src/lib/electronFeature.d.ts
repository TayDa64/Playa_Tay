/**
 * Type definitions for electronFeature module
 */

declare module '../lib/electronFeature.js' {
  /**
   * Check if Electron runtime is available
   */
  export function isElectronAvailable(): Promise<boolean>;

  /**
   * Attempt to ensure Electron sidecar is installed (dev-only helper)
   */
  export function ensureElectronSidecar(): Promise<{ ok: boolean; message?: string }>;

  /**
   * Open a feature that requires Electron (Pattern A/B)
   */
  export function openElectronFeature(
    url: string,
    options?: { retry?: boolean }
  ): Promise<{ ok: boolean; code?: string; message?: string }>;
}

/**
 * Check if Electron runtime is available
 */
export function isElectronAvailable(): Promise<boolean>;

/**
 * Attempt to ensure Electron sidecar is installed (dev-only helper)
 */
export function ensureElectronSidecar(): Promise<{ ok: boolean; message?: string }>;

/**
 * Open a feature that requires Electron (Pattern A/B)
 */
export function openElectronFeature(
  url: string,
  options?: { retry?: boolean }
): Promise<{ ok: boolean; code?: string; message?: string }>;
