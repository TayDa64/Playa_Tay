// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

/**
 * macOS Notarization Script
 * 
 * This script handles the notarization process for macOS builds.
 * Notarization is required for Gatekeeper to allow the app to run on macOS 10.15+
 * 
 * Prerequisites:
 * - Apple Developer Account with valid Developer ID Application certificate
 * - App-specific password generated from appleid.apple.com
 * 
 * Environment variables required:
 * - APPLE_ID: Your Apple ID email
 * - APPLE_ID_PASSWORD: App-specific password
 * - APPLE_TEAM_ID: Your Apple Developer Team ID
 */

const { notarize } = require('@electron/notarize');
const { build } = require('../../package.json');

async function notarizeApp(context) {
  const { electronPlatformName, appOutDir } = context;
  
  // Only notarize for macOS
  if (electronPlatformName !== 'darwin') {
    return;
  }

  // Check if notarization credentials are available
  if (!process.env.APPLE_ID || !process.env.APPLE_ID_PASSWORD || !process.env.APPLE_TEAM_ID) {
    console.warn('Skipping notarization: Required environment variables not set');
    console.warn('Required: APPLE_ID, APPLE_ID_PASSWORD, APPLE_TEAM_ID');
    return;
  }

  const appName = context.packager.appInfo.productFilename;
  const appPath = `${appOutDir}/${appName}.app`;

  console.log(`Notarizing ${appName} at ${appPath}...`);

  try {
    await notarize({
      appPath,
      appleId: process.env.APPLE_ID,
      appleIdPassword: process.env.APPLE_ID_PASSWORD,
      teamId: process.env.APPLE_TEAM_ID,
      tool: 'notarytool' // Use the new notarytool (Xcode 13+)
    });
    
    console.log(`Successfully notarized ${appName}`);
  } catch (error) {
    console.error('Notarization failed:', error);
    throw error;
  }
}

exports.default = notarizeApp;
