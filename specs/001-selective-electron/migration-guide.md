# Migration Guide: Adding Selective Electron to Existing Tauri App

This guide helps you integrate Electron selectively into an existing Tauri application without breaking existing functionality.

## Overview

This migration approach ensures:
- ✅ All existing Tauri features continue to work
- ✅ Electron is added as an optional enhancement
- ✅ Graceful degradation when Electron is unavailable
- ✅ Minimal changes to existing codebase
- ✅ No breaking changes for users

## Pre-Migration Checklist

Before starting the migration:

- [ ] Backup your current codebase
- [ ] Document existing features and dependencies
- [ ] Identify features that require Electron
- [ ] Test current application thoroughly
- [ ] Review system requirements

## Migration Steps

### Step 1: Assess Current Application

**1.1 Inventory Existing Features**

Create a list of all current features:
```
Existing Features:
- Feature A (Tauri native) ✅
- Feature B (Tauri native) ✅
- Feature C (needs Electron) ⚠️
- Feature D (Tauri native) ✅
```

**1.2 Identify Electron Dependencies**

List features that specifically need Electron:
- Node.js module compatibility
- Electron-specific APIs
- Legacy code requiring Node runtime
- Features requiring multiple windows with different contexts

### Step 2: Create Feature Flags

**2.1 Add Configuration File**

Create `src-tauri/src/config/features.rs`:

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureFlags {
    pub electron_enabled: bool,
    pub electron_features: Vec<String>,
}

impl Default for FeatureFlags {
    fn default() -> Self {
        Self {
            electron_enabled: false,
            electron_features: vec![],
        }
    }
}

impl FeatureFlags {
    pub fn is_feature_available(&self, feature: &str) -> bool {
        self.electron_enabled && self.electron_features.contains(&feature.to_string())
    }
}
```

**2.2 Update Main Application**

Modify `src-tauri/src/main.rs`:

```rust
mod config;
use config::features::FeatureFlags;

fn main() {
    let features = FeatureFlags::default();
    
    tauri::Builder::default()
        .manage(features)
        .invoke_handler(tauri::generate_handler![
            // ... existing commands
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### Step 3: Add Electron Commands Alongside Existing

**3.1 Create New Module (Don't Modify Existing)**

Create `src-tauri/src/commands/electron.rs` (as shown in implementation guide)

**3.2 Register New Commands**

```rust
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // EXISTING COMMANDS - DO NOT MODIFY
            existing_command_1,
            existing_command_2,
            // NEW ELECTRON COMMANDS
            commands::electron::check_electron_available,
            commands::electron::launch_electron_feature,
        ])
        // ... rest of setup
}
```

### Step 4: Update Frontend Gradually

**4.1 Add Electron Module Without Breaking Existing Code**

Create new file `src/lib/electron.ts` (don't modify existing files):

```typescript
// New module for Electron features
import { invoke } from '@tauri-apps/api/core';

export class ElectronFeatures {
    private available = false;

    async initialize() {
        try {
            this.available = await invoke('check_electron_available');
        } catch {
            this.available = false;
        }
    }

    isAvailable() {
        return this.available;
    }

    // Add new Electron-specific methods here
}
```

**4.2 Update Existing Components Conditionally**

Instead of replacing existing components, add conditional logic:

```svelte
<script>
    import { onMount } from 'svelte';
    import { electronFeatures } from '../lib/electron';
    
    let useElectron = false;

    onMount(async () => {
        await electronFeatures.initialize();
        useElectron = electronFeatures.isAvailable();
    });

    function handleFeature() {
        if (useElectron) {
            // Use Electron implementation
            handleWithElectron();
        } else {
            // Use existing Tauri implementation
            handleWithTauri();
        }
    }
</script>

<!-- Existing UI remains unchanged -->
<button on:click={handleFeature}>
    Execute Feature
</button>
```

### Step 5: Test Existing Functionality

**5.1 Create Test Suite for Existing Features**

```bash
# Test without Electron
pnpm tauri dev

# Verify all existing features work
# Document any issues
```

**5.2 Test with Electron**

```bash
# Install Electron
pnpm add -D electron

# Test with Electron available
pnpm tauri dev

# Verify new features work
# Verify existing features still work
```

### Step 6: Gradual Rollout Strategy

**Phase 1: Internal Testing**
- Deploy to test environment
- Test all existing features
- Test new Electron features
- Gather feedback

**Phase 2: Beta Release**
- Deploy to subset of users
- Monitor for issues
- Collect performance metrics
- Fix issues before full rollout

**Phase 3: Full Release**
- Deploy to all users
- Monitor system performance
- Provide fallback options
- Document any issues

## Rollback Plan

If issues occur during migration:

### Quick Rollback
```bash
# Revert to previous commit
git revert HEAD

# Or reset to previous version
git reset --hard <previous-commit>

# Rebuild and redeploy
pnpm tauri build
```

### Disable Electron Without Rollback
```rust
// In src-tauri/src/main.rs
let features = FeatureFlags {
    electron_enabled: false,  // Disable Electron
    electron_features: vec![],
};
```

## Compatibility Matrix

Ensure compatibility across all components:

| Component | Before Migration | After Migration | Status |
|-----------|-----------------|-----------------|---------|
| Tauri Core | v2.x | v2.x | ✅ Same |
| Frontend Framework | Svelte/React/Vue | Svelte/React/Vue | ✅ Same |
| Build System | Existing | Existing + Electron | ⚠️ Extended |
| Commands | Existing set | Existing + New | ✅ Additive |

## Testing Checklist

After migration, verify:

- [ ] All existing Tauri commands work
- [ ] All existing UI components render correctly
- [ ] Application starts and closes properly
- [ ] State management still functions
- [ ] File system operations work
- [ ] Network requests succeed
- [ ] Window management works
- [ ] System tray functions (if used)
- [ ] Updates work (if using updater)
- [ ] Notifications work
- [ ] New Electron features work (when available)
- [ ] Graceful degradation (when Electron unavailable)

## Performance Comparison

Monitor these metrics before and after migration:

| Metric | Before | After | Target |
|--------|--------|-------|--------|
| Startup Time | X ms | Y ms | ≤ X + 100ms |
| Memory Usage | X MB | Y MB | ≤ X + 50MB |
| Binary Size | X MB | Y MB | ≤ X + 20MB |
| CPU Usage (idle) | X% | Y% | ≤ X + 5% |

## Common Issues and Solutions

### Issue 1: Electron Not Detected
**Symptom**: Application works but Electron features unavailable

**Solution**:
```bash
# Verify Electron is installed
pnpm list electron

# If not installed
pnpm add -D electron

# Verify PATH
which electron
```

### Issue 2: Existing Features Break
**Symptom**: Previously working features now fail

**Solution**:
1. Check command registration order
2. Verify no naming conflicts
3. Review state management changes
4. Check for import errors

### Issue 3: Performance Degradation
**Symptom**: Application slower after migration

**Solution**:
1. Profile application performance
2. Check for unnecessary Electron initialization
3. Implement lazy loading
4. Optimize IPC communication

## Best Practices

1. **Never Remove Working Code**: Add new code alongside existing
2. **Use Feature Flags**: Make Electron features optional
3. **Test Thoroughly**: Test with and without Electron
4. **Document Everything**: Document all changes
5. **Monitor Performance**: Track performance metrics
6. **Plan Rollback**: Always have a rollback strategy
7. **Gradual Migration**: Migrate one feature at a time
8. **User Communication**: Inform users of changes

## Support and Resources

- [Tauri Discord](https://discord.com/invite/tauri)
- [GitHub Issues](https://github.com/tauri-apps/tauri/issues)
- [Documentation](https://tauri.app)

## Post-Migration Tasks

- [ ] Update user documentation
- [ ] Update developer documentation
- [ ] Create release notes
- [ ] Update CI/CD pipelines
- [ ] Monitor error logs
- [ ] Collect user feedback
- [ ] Plan future iterations
