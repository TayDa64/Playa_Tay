#!/usr/bin/env node

/**
 * Test script to verify Electron integration commands are working
 * This simulates the frontend calling the Tauri backend commands
 */

import { invoke } from '@tauri-apps/api/core';

async function testElectronCommands() {
  console.log('🧪 Testing Electron integration commands...\n');

  try {
    // Test 1: Check if Electron is available
    console.log('1️⃣ Testing is_electron_available...');
    const isAvailable = await invoke('is_electron_available');
    console.log(`   Result: ${JSON.stringify(isAvailable)}\n`);

    // Test 2: Ensure Electron sidecar
    console.log('2️⃣ Testing ensure_electron_sidecar...');
    const ensureResult = await invoke('ensure_electron_sidecar');
    console.log(`   Result: ${JSON.stringify(ensureResult)}\n`);

    // Test 3: Open Electron feature (if available)
    if (isAvailable && isAvailable.ok) {
      console.log('3️⃣ Testing open_electron_feature...');
      const openResult = await invoke('open_electron_feature', {
        url: 'https://example.com',
        pattern: 'A'
      });
      console.log(`   Result: ${JSON.stringify(openResult)}\n`);
    } else {
      console.log('3️⃣ Skipping open_electron_feature (not available)\n');
    }

    // Test 4: Launch Electron (alternative command)
    console.log('4️⃣ Testing launch_electron...');
    const launchResult = await invoke('launch_electron');
    console.log(`   Result: ${JSON.stringify(launchResult)}\n`);

    console.log('✅ All tests completed successfully!');

  } catch (error) {
    console.error('❌ Error testing commands:', error);
  }
}

// Run the tests
testElectronCommands().catch(console.error);
