# Electron Module Template

Secure Electron module template for the Playa Tay framework with enforced security best practices.

## 🔒 Security Features

This template enforces security by default and cannot be easily overridden:

- **Context Isolation**: Enabled - Separates preload scripts from web content
- **Node Integration**: Disabled - Prevents direct Node.js access from renderer
- **Sandbox**: Enabled - Runs renderer in a sandboxed process
- **Content Security Policy**: Strict CSP enforced via middleware
- **IPC Authentication**: Token-based authentication with TTL
- **DevTools**: Automatically disabled in production
- **Remote Module**: Disabled
- **Navigation Protection**: Blocks external navigation and new windows

## 📁 Project Structure

```
packages/electron-module-template/
├── src/
│   ├── main.ts                    # Main process entry point
│   ├── preload.ts                 # Preload script with context bridge
│   ├── security/
│   │   ├── flags.ts              # Security flag enforcement
│   │   ├── csp.ts                # Content Security Policy helper
│   │   └── ipc-auth.ts           # IPC token authentication
│   └── renderer/
│       ├── index.html            # Renderer UI
│       └── style.css             # Styling
├── build/
│   ├── entitlements.mac.plist    # macOS sandboxing entitlements
│   └── notarize.js               # macOS notarization script
├── assets/
│   ├── icon.svg                  # App icon (convert to PNG for production)
│   └── background.svg            # Installer background (convert to PNG for production)
├── electron-builder.yml          # Multi-platform build config
├── package.json
├── tsconfig.json
└── eslint.config.js
```

## 🚀 Usage

### Installation

```bash
cd packages/electron-module-template
pnpm install
```

### Development

```bash
# Run in development mode
pnpm dev

# Type checking
pnpm ts:check

# Linting
pnpm eslint:check
pnpm eslint:fix
```

### Building

```bash
# Build TypeScript
pnpm build

# Build for specific platform
pnpm build:mac      # macOS (DMG for x64 and arm64)
pnpm build:win      # Windows (NSIS installer)
pnpm build:linux    # Linux (DEB and RPM)
```

Built packages will be in `dist-packages/`.

## 🛡️ Security Helpers

### Security Flags (`src/security/flags.ts`)

```typescript
import { enforceSecurityFlags, logSecurityStatus } from './security/flags.js'

// Automatically enforce security settings
const win = new BrowserWindow(
  enforceSecurityFlags({
    width: 1200,
    height: 800
  })
)

// Log current security configuration
logSecurityStatus()
```

**What it does:**
- Forces `contextIsolation: true`
- Forces `nodeIntegration: false`
- Forces `sandbox: true`
- Sets `devTools` based on NODE_ENV
- Disables remote module, plugins, and node integration in workers

### Content Security Policy (`src/security/csp.ts`)

```typescript
import { enforceCSP, getDefaultCSP } from './security/csp.js'
import { session } from 'electron'

// Apply strict CSP to session
enforceCSP(session.defaultSession)

// Customize CSP if needed (merge with defaults)
enforceCSP(session.defaultSession, {
  scriptSrc: ["'self'", "'unsafe-eval'"], // Only if necessary
  connectSrc: ["'self'", 'https://api.example.com']
})
```

**Default Policy:**
```
default-src 'self';
script-src 'self';
style-src 'self' 'unsafe-inline';
img-src 'self' data: https:;
connect-src 'self' https:;
font-src 'self';
object-src 'none';
media-src 'self';
frame-src 'none';
```

### IPC Authentication (`src/security/ipc-auth.ts`)

```typescript
import { 
  createIPCToken, 
  validateIPCToken, 
  revokeWindowTokens 
} from './security/ipc-auth.js'

// Create token when window opens
const token = createIPCToken(win.id, { ttl: 5 * 60 * 1000 }) // 5 min TTL

// Validate token in IPC handler
ipcMain.handle('secure-action', (event, token) => {
  const windowId = BrowserWindow.fromWebContents(event.sender)?.id
  if (!windowId || !validateIPCToken(token, windowId)) {
    throw new Error('Invalid or expired token')
  }
  // Proceed with secure action
})

// Revoke tokens when window closes
win.on('closed', () => {
  revokeWindowTokens(win.id)
})
```

**Features:**
- Cryptographically secure token generation (32 bytes)
- Configurable TTL (default 5 minutes)
- Automatic expiry and cleanup
- Per-window token validation
- Token revocation on window close

## 🧪 Testing

```bash
pnpm test
```

Tests include:
- Security flag enforcement validation
- IPC token generation and expiry
- CSP configuration
- Preload API exposure contract

## 📦 Building for Distribution

### Asset Requirements

Before building for production, replace the SVG assets with high-quality PNG files:

- **Icon**: `assets/icon.png` - 1024x1024 PNG (macOS/Linux) or 256x256 ICO (Windows)
- **Background**: `assets/background.png` - 540x380 PNG (macOS DMG installer background)

The SVG files provided are placeholders for development. For production builds:

```bash
# Convert SVG to PNG (requires ImageMagick or similar)
convert assets/icon.svg -resize 1024x1024 assets/icon.png
convert assets/background.svg -resize 540x380 assets/background.png

# Or use online tools like https://svgtopng.com
```

### macOS

**Prerequisites:**
- Apple Developer account
- Valid Developer ID Application certificate
- For notarization: APPLE_ID, APPLE_ID_PASSWORD, APPLE_TEAM_ID env vars

```bash
export APPLE_ID="your@apple.id"
export APPLE_ID_PASSWORD="app-specific-password"
export APPLE_TEAM_ID="TEAMID123"
pnpm build:mac
```

### Windows

**Prerequisites:**
- Code signing certificate (optional but recommended)
- Set certificate details in electron-builder.yml

```bash
pnpm build:win
```

### Linux

```bash
pnpm build:linux
```

## 🔧 Configuration

### Electron Builder (`electron-builder.yml`)

The configuration includes:
- **macOS**: DMG with hardened runtime, universal builds (x64/arm64)
- **Windows**: NSIS installer with one-click and custom install options
- **Linux**: DEB and RPM packages

Customize as needed for your module requirements.

### TypeScript (`tsconfig.json`)

Strict mode enabled with:
- `noImplicitAny: true` - No implicit any types allowed
- `strict: true` - All strict type checking options
- `noUnusedLocals: true` - Unused local variables not allowed

### ESLint (`eslint.config.js`)

Security-focused linting with:
- TypeScript ESLint recommended rules
- Security plugin for common vulnerabilities
- No `any` types allowed (`@typescript-eslint/no-explicit-any: error`)

## ⚠️ Security Notes

1. **Never disable security features** - The helpers enforce secure defaults that should not be overridden
2. **Keep Electron updated** - Regularly update to get security patches
3. **Review CSP changes** - Be very cautious when relaxing CSP directives
4. **Validate all IPC** - Always use token authentication for sensitive IPC calls
5. **Test in production mode** - Security features behave differently in development
6. **Code signing** - Always sign production builds for distribution
7. **Asset security** - Only load trusted local assets, never from untrusted URLs

## 📖 Additional Resources

- [Electron Security Best Practices](https://www.electronjs.org/docs/latest/tutorial/security)
- [Content Security Policy](https://developer.mozilla.org/en-US/docs/Web/HTTP/CSP)
- [Electron Builder Documentation](https://www.electron.build/)
- [Tauri Security](https://tauri.app/v1/references/architecture/security/)

## 📝 License

Apache-2.0 OR MIT
