# Electron DRM Shell (Pattern B)

**Status**: Scaffolding/Outline — Implementation deferred to post-v1

This package represents Pattern B: an **optional, separately-distributed Electron module** for features requiring capabilities not available in Tauri's WebView (e.g., Widevine DRM for protected content streaming).

## Architecture Overview

### Pattern A vs Pattern B

| Aspect | Pattern A (Sidecar) | Pattern B (Module) |
|--------|---------------------|--------------------|
| **Distribution** | Bundled with main app | Separate installer |
| **Update Channel** | App update cycle | Independent signed updates |
| **Installation** | Always present | On-demand, user-initiated |
| **Use Case** | Dev/testing, minimal features | Production DRM, advanced features |
| **Binary Size** | ~100MB (minimal sidecar) | Full Electron (~200-300MB) |
| **Code Signing** | Inherits app signature | Separate signing required |

### Why Pattern B?

1. **Optional Dependency**: Users who don't need DRM don't download 200+ MB of Electron
2. **Compliance**: Some DRM providers require specific update/signing flows
3. **Independent Updates**: Security patches to Electron module without full app release
4. **Resource Efficiency**: Only users who need protected content pay the disk/memory cost

## Security Posture

### Enforced Flags (Non-Negotiable)
- `contextIsolation: true`
- `nodeIntegration: false`
- `sandbox: true`
- `devTools: false` in production
- Content Security Policy (CSP) enforced via response headers

### IPC Communication
- **Protocol**: HTTP over `localhost` only (no remote connections)
- **Authentication**: Ephemeral token generated per session, passed via environment variable
- **Token Rotation**: New token per window spawn; tokens expire on window close
- **Rejection Policy**: Any request without valid token → immediate 401 response

### Example IPC Flow
```
1. Tauri generates token: PLAYA_AUTH_TOKEN=<random-32-char>
2. Tauri spawns Electron module with token in env
3. Electron starts localhost server on random port
4. Electron validates token on every request
5. On window close, token invalidated
```

## Installation Strategy

### User-Initiated Install
```
1. User clicks "Enable DRM Features" in settings
2. App checks if module installed:
   - Yes → launch module
   - No → show install dialog
3. Install dialog options:
   - Download & Install (fetches signed .dmg/.exe/.deb)
   - Cancel (feature remains unavailable)
4. Post-install: register module path in app config
5. Future launches: direct spawn
```

### Module Detection Logic
```rust
// In detect_electron_module():
1. Check app config for registered module path
2. Verify binary exists and signature valid
3. Check version compatibility
4. Return detection result
```

### Update Flow
1. Module checks for updates independently (on launch or background)
2. Notifies Tauri host if update available
3. User chooses to update or skip
4. Module downloads delta patch or full installer
5. Applies update and restarts if necessary

## Implementation Plan (Post-v1)

### Phase 1: Packaging
- [ ] Add `electron-builder` configuration for macOS/Windows/Linux
- [ ] Configure code signing (Apple Developer ID, Windows Authenticode)
- [ ] Set up update server and delta patching (e.g., `electron-updater`)
- [ ] Create platform-specific installers (.dmg, .exe, .deb)

### Phase 2: Backend Detection
- [ ] Implement `detect_electron_module()` function
- [ ] Add config file schema for module registration (`~/.playa/modules.json`)
- [ ] Version compatibility checks
- [ ] Signature verification on module binary

### Phase 3: Installation Flow
- [ ] Add Tauri command: `install_electron_module()`
- [ ] Download progress reporting via IPC channel
- [ ] Post-install validation and registration
- [ ] Rollback on failed install

### Phase 4: IPC Handshake
- [ ] Implement token-based authentication server in Electron module
- [ ] Tauri→Electron command protocol (JSON-RPC or custom)
- [ ] Bi-directional event channel for status updates
- [ ] Timeout and retry logic

### Phase 5: Update Mechanism
- [ ] Background update check on module launch
- [ ] User notification for available updates
- [ ] Automatic delta download and apply
- [ ] Rollback on corrupted update

### Phase 6: UI Integration
- [ ] Settings page: "Manage Modules" section
- [ ] Install/update/uninstall actions
- [ ] Module version display and status indicator
- [ ] "About" info with license/attribution

## Current Structure (Outline)

```
packages/electron-drm-shell/
├── README.md               # This file
├── package.json            # Minimal outline
├── electron-builder.yml    # TODO: Platform configs
├── src/
│   ├── main.ts            # TODO: Module entry point
│   ├── ipc.ts             # TODO: IPC server with auth
│   ├── drm.ts             # TODO: DRM-specific logic
│   └── updater.ts         # TODO: Auto-update logic
└── assets/
    └── icon.png           # TODO: Module icon
```

## Configuration Example (Future)

**`electron-builder.yml`**
```yaml
appId: com.playa.drm-shell
productName: Playa DRM Module
copyright: Copyright © 2025

mac:
  category: public.app-category.utilities
  hardenedRuntime: true
  gatekeeperAssess: false
  entitlements: build/entitlements.mac.plist
  entitlementsInherit: build/entitlements.mac.plist
  target:
    - target: dmg
      arch: [x64, arm64]

win:
  target:
    - target: nsis
      arch: [x64]
  certificateSubjectName: "Playa Inc."

linux:
  target:
    - target: deb
      arch: [x64]
  category: Utility

publish:
  provider: generic
  url: https://updates.playa.app/drm-module/
```

## Security Considerations

### Code Signing Requirements
- **macOS**: Apple Developer ID certificate, notarization via `notarytool`
- **Windows**: Authenticode certificate from trusted CA (e.g., DigiCert)
- **Linux**: GPG signing of .deb/.rpm packages

### Threat Model
- **Rogue Module**: Signature verification prevents installation of unsigned binaries
- **Token Theft**: Ephemeral tokens passed via env (not exposed to user scripts)
- **MITM**: Localhost-only communication; no network exposure
- **Privilege Escalation**: Sandbox enforced; no Node.js access from renderer

### Compliance Notes
- DRM providers (e.g., Widevine) may have specific distribution requirements
- Check licensing terms before bundling DRM components
- Some regions restrict DRM technology exports

## Testing Strategy

### Unit Tests
- IPC handshake with valid/invalid tokens
- Module detection logic (present/absent/corrupted)
- Version compatibility checks
- Signature verification

### Integration Tests
- Full install flow (download → verify → register)
- Update flow (check → download → apply)
- Spawn and IPC communication end-to-end
- Uninstall and cleanup

### Manual Testing
- Platform-specific installer behavior
- Code signing validation (Gatekeeper, SmartScreen)
- Network failure scenarios during download
- Rollback after failed update

## References

- [Electron Builder Documentation](https://www.electron.build/)
- [Electron Updater](https://www.electron.build/auto-update)
- [Widevine CDM in Electron](https://www.electronjs.org/docs/latest/tutorial/widevine)
- [Code Signing Guide](https://www.electron.build/code-signing)

## Status

**Current**: Scaffolding complete, implementation deferred to post-v1
**Next Milestone**: Implement Phase 1 (Packaging) after Pattern A proven in production
**Blockers**: DRM provider selection and licensing clarity (see spec clarifications)
