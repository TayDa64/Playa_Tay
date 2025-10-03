# Implementation Guide

This guide provides step-by-step instructions for implementing selective Electron integration with Tauri.

## Prerequisites

- Rust 1.77.2 or later
- Node.js 18 or later
- pnpm package manager
- Tauri CLI
- Basic understanding of Rust and JavaScript/TypeScript

## Step 1: Project Setup

### 1.1 Install Dependencies

```bash
# Install Tauri CLI
cargo install tauri-cli

# Install frontend dependencies
pnpm install

# Install Electron (if using as sidecar)
pnpm add -D electron
```

### 1.2 Verify Installation

```bash
# Check Tauri version
cargo tauri --version

# Check Node version
node --version

# Check Electron version
pnpm electron --version
```

## Step 2: Create Tauri Command

### 2.1 Create Command Module

Create a new file `src-tauri/src/commands/electron.rs`:

```rust
use std::process::Command;
use tauri::State;

#[derive(Default)]
pub struct ElectronState {
    pub process: Option<std::process::Child>,
    pub available: bool,
}

#[tauri::command]
pub async fn check_electron_available() -> Result<bool, String> {
    // Check if Electron binary exists
    match Command::new("electron").arg("--version").output() {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

#[tauri::command]
pub async fn launch_electron_feature(
    feature: String,
    state: State<'_, ElectronState>,
) -> Result<String, String> {
    if !state.available {
        return Err("Electron is not available".to_string());
    }

    // Implementation for launching Electron feature
    // This is a placeholder - implement based on your needs
    Ok(format!("Launched feature: {}", feature))
}

#[tauri::command]
pub async fn stop_electron_feature() -> Result<(), String> {
    // Implementation for stopping Electron
    Ok(())
}
```

### 2.2 Register Commands

Update `src-tauri/src/main.rs`:

```rust
mod commands;

use commands::electron::{
    check_electron_available, 
    launch_electron_feature, 
    stop_electron_feature,
    ElectronState
};

fn main() {
    tauri::Builder::default()
        .manage(ElectronState::default())
        .invoke_handler(tauri::generate_handler![
            check_electron_available,
            launch_electron_feature,
            stop_electron_feature,
        ])
        .setup(|app| {
            // Initialize Electron state
            let state: tauri::State<ElectronState> = app.state();
            // Check if Electron is available at startup
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

## Step 3: Create Frontend Integration

### 3.1 Create Electron API Module

Create `src/lib/electron.ts`:

```typescript
import { invoke } from '@tauri-apps/api/core';

export interface ElectronStatus {
    available: boolean;
}

export class ElectronAPI {
    private static instance: ElectronAPI;
    private available: boolean = false;

    private constructor() {}

    static getInstance(): ElectronAPI {
        if (!ElectronAPI.instance) {
            ElectronAPI.instance = new ElectronAPI();
        }
        return ElectronAPI.instance;
    }

    async initialize(): Promise<void> {
        try {
            this.available = await invoke<boolean>('check_electron_available');
        } catch (error) {
            console.error('Failed to check Electron availability:', error);
            this.available = false;
        }
    }

    isAvailable(): boolean {
        return this.available;
    }

    async launchFeature(featureName: string): Promise<string> {
        if (!this.available) {
            throw new Error('Electron is not available');
        }

        return await invoke<string>('launch_electron_feature', {
            feature: featureName
        });
    }

    async stopFeature(): Promise<void> {
        await invoke('stop_electron_feature');
    }
}

// Export singleton instance
export const electronAPI = ElectronAPI.getInstance();
```

### 3.2 Create UI Component

Create `src/components/ElectronControl.svelte`:

```svelte
<script lang="ts">
    import { onMount } from 'svelte';
    import { electronAPI } from '../lib/electron';

    let available = false;
    let loading = false;
    let status = 'Checking Electron status...';

    onMount(async () => {
        await electronAPI.initialize();
        available = electronAPI.isAvailable();
        status = available 
            ? 'Electron is available' 
            : 'Electron is not available - some features disabled';
    });

    async function handleLaunch(featureName: string) {
        loading = true;
        status = `Launching ${featureName}...`;
        
        try {
            const result = await electronAPI.launchFeature(featureName);
            status = `Success: ${result}`;
        } catch (error) {
            status = `Error: ${error}`;
        } finally {
            loading = false;
        }
    }

    async function handleStop() {
        loading = true;
        try {
            await electronAPI.stopFeature();
            status = 'Stopped Electron feature';
        } catch (error) {
            status = `Error stopping: ${error}`;
        } finally {
            loading = false;
        }
    }
</script>

<div class="electron-control">
    <h2>Electron Integration</h2>
    
    <div class="status" class:available class:unavailable={!available}>
        {status}
    </div>

    {#if available}
        <div class="controls">
            <button 
                on:click={() => handleLaunch('example-feature')} 
                disabled={loading}
            >
                Launch Feature
            </button>
            
            <button 
                on:click={handleStop} 
                disabled={loading}
            >
                Stop Feature
            </button>
        </div>
    {:else}
        <div class="fallback">
            <p>Electron features are unavailable.</p>
            <p>Using Tauri-native alternatives.</p>
        </div>
    {/if}
</div>

<style>
    .electron-control {
        padding: 1rem;
        border: 1px solid #ccc;
        border-radius: 8px;
        margin: 1rem 0;
    }

    .status {
        padding: 0.5rem;
        margin: 1rem 0;
        border-radius: 4px;
    }

    .status.available {
        background-color: #d4edda;
        color: #155724;
    }

    .status.unavailable {
        background-color: #fff3cd;
        color: #856404;
    }

    .controls {
        display: flex;
        gap: 0.5rem;
    }

    button {
        padding: 0.5rem 1rem;
        border: none;
        border-radius: 4px;
        background-color: #007bff;
        color: white;
        cursor: pointer;
    }

    button:disabled {
        background-color: #ccc;
        cursor: not-allowed;
    }

    .fallback {
        margin-top: 1rem;
        padding: 1rem;
        background-color: #f8f9fa;
        border-radius: 4px;
    }
</style>
```

## Step 4: Add to Main Application

Update `src/App.svelte` to include the component:

```svelte
<script>
    import ElectronControl from './components/ElectronControl.svelte';
</script>

<main>
    <h1>Playa Tay</h1>
    <ElectronControl />
    <!-- Other components -->
</main>
```

## Step 5: Testing

### 5.1 Create Test File

Create `src-tauri/src/commands/electron_test.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_electron_state_default() {
        let state = ElectronState::default();
        assert_eq!(state.available, false);
        assert!(state.process.is_none());
    }

    #[tokio::test]
    async fn test_check_electron_available() {
        let result = check_electron_available().await;
        assert!(result.is_ok());
    }
}
```

### 5.2 Run Tests

```bash
# Run Rust tests
cd src-tauri
cargo test

# Run frontend tests (if configured)
pnpm test
```

## Step 6: Build and Run

### 6.1 Development Mode

```bash
# Start development server
pnpm run dev

# Or use Tauri dev command
pnpm tauri dev
```

### 6.2 Production Build

```bash
# Build the application
pnpm tauri build
```

## Step 7: Troubleshooting

### Common Issues

1. **Electron not found**
   - Ensure Electron is installed: `pnpm add -D electron`
   - Check PATH includes Electron binary

2. **IPC errors**
   - Verify command names match between Rust and frontend
   - Check that commands are registered in `generate_handler!`

3. **Build failures**
   - Ensure all dependencies are installed
   - Check Rust version compatibility
   - Verify system libraries are available (Linux)

## Next Steps

1. Implement specific Electron features based on your requirements
2. Add error handling and logging
3. Implement IPC communication if using sidecar pattern
4. Add tests for all new functionality
5. Update documentation
6. Configure CI/CD pipeline

## Additional Resources

- [Tauri Commands Guide](https://tauri.app/v2/guides/features/commands)
- [Tauri State Management](https://tauri.app/v2/guides/features/state)
- [Electron Documentation](https://www.electronjs.org/docs/latest)
