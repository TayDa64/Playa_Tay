# Electron Integration Configuration

## Sidecar Configuration

When using Electron as a sidecar, you'll need to configure it in your `tauri.conf.json`:

```json
{
  "bundle": {
    "externalBin": [
      "electron-bin/electron"
    ]
  }
}
```

## Environment Variables

Set these environment variables for development:

```bash
# Path to Electron binary
ELECTRON_BIN_PATH=/path/to/electron

# Enable debug mode
ELECTRON_DEBUG=true

# IPC port (optional, defaults to dynamic port)
ELECTRON_IPC_PORT=3001
```

## Build Configuration

### Development Build
```bash
# Build Tauri app in dev mode
pnpm run dev

# Launch Electron sidecar separately
electron ./electron-app
```

### Production Build
```bash
# Build both Tauri and Electron
pnpm run build

# Package with Electron binary
pnpm run tauri build
```

## IPC Configuration

### Message Protocol

Messages between Tauri and Electron should follow this structure:

```typescript
interface IPCMessage {
    id: string;
    command: string;
    payload: any;
    timestamp: number;
}

interface IPCResponse {
    id: string;
    success: boolean;
    data?: any;
    error?: string;
    timestamp: number;
}
```

### Security Configuration

1. **Validate all messages**: Always validate message format and content
2. **Use secure channels**: Use encrypted communication for sensitive data
3. **Implement rate limiting**: Prevent abuse of IPC endpoints
4. **Sanitize inputs**: Never trust input from either side

## Platform-Specific Notes

### macOS
- Electron binary needs to be signed and notarized
- Add entitlements for IPC communication
- Configure app sandbox appropriately

### Windows
- Ensure Electron binary is in the correct architecture (x64/x86)
- Configure Windows Defender exclusions if needed
- Handle UAC prompts appropriately

### Linux
- Ensure required system libraries are available
- Test on multiple distributions
- Handle different desktop environments

## Fallback Strategy

When Electron is not available:

1. **Detection**: Check for Electron binary at startup
2. **Notification**: Inform user about missing features
3. **Graceful degradation**: Disable Electron-specific features
4. **Alternative paths**: Provide Tauri-native alternatives where possible

## Performance Optimization

- **Lazy loading**: Only start Electron when needed
- **Resource pooling**: Reuse Electron instances
- **Memory management**: Monitor and limit memory usage
- **Startup optimization**: Minimize initialization time
