# Local Setup Troubleshooting Guide

## Prerequisites Checklist

Before running the application locally, ensure you have these installed:

### Required Tools
- [ ] **Node.js 18+** - Check: `node --version`
- [ ] **pnpm 8+** - Check: `pnpm --version`
- [ ] **Rust 1.77+** - Check: `rustc --version`
- [ ] **Cargo** - Check: `cargo --version`

### Platform-Specific Requirements

#### macOS
```bash
# Install Xcode Command Line Tools
xcode-select --install

# Verify installation
xcode-select -p
```

#### Windows
```bash
# Install Visual Studio C++ Build Tools
# Download from: https://visualstudio.microsoft.com/downloads/
# Select "Desktop development with C++"

# Or install via winget
winget install Microsoft.VisualStudio.2022.BuildTools
```

#### Linux (Ubuntu/Debian)
```bash
# Install required system libraries
sudo apt update
sudo apt install -y \
  libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libxdo-dev \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev

# For Electron support
sudo apt install -y \
  libgtk-3-dev \
  libx11-dev \
  libxext-dev \
  libxfixes-dev \
  libxcb1-dev \
  libxrender-dev \
  libxrandr-dev
```

## Step-by-Step Setup (Local Machine)

### 1. Clone and Install Dependencies

```bash
# Clone the repository
git clone https://github.com/TayDa64/Playa_Tay.git
cd Playa_Tay

# Install all dependencies
pnpm install

# This will install:
# - Root workspace dependencies
# - All package dependencies (packages/*)
# - Example app dependencies (examples/api)
```

**Common Issue**: If `pnpm install` fails:
```bash
# Clear pnpm cache
pnpm store prune

# Remove node_modules and lockfile
rm -rf node_modules pnpm-lock.yaml

# Reinstall
pnpm install
```

### 2. Build the @tauri-apps/api Package

```bash
# This is required for Tauri to work
pnpm -F @tauri-apps/api build

# Verify the build
ls packages/api/dist/
# Should see: index.d.ts, index.js, etc.
```

**Common Issue**: TypeScript compilation errors
```bash
# Ensure TypeScript is installed
pnpm add -g typescript

# Check TypeScript version
tsc --version
```

### 3. Build the Electron Sidecar

```bash
# Build the Electron shell
pnpm -F @playa/electron-shell build

# Verify the build
ls packages/electron-shell/dist/
# Should see: main.js

# Copy to resources directory
mkdir -p examples/api/src-tauri/resources/electron-shell
cp packages/electron-shell/dist/main.js examples/api/src-tauri/resources/electron-shell/main.js
```

**Common Issue**: Missing Electron
```bash
# Install Electron explicitly
cd packages/electron-shell
pnpm install electron
cd ../..
```

### 4. Build the Frontend

```bash
# Build the Svelte frontend
pnpm --filter api build

# Verify the build
ls examples/api/dist/
# Should see: index.html, assets/, etc.
```

**Common Issue**: Vite or Svelte errors
```bash
# Check for conflicting dependencies
cd examples/api
pnpm list svelte
pnpm list vite

# Reinstall if needed
pnpm install
```

### 5. Run the Application

#### Method 1: Using Cargo (Most Reliable)

```bash
# Terminal 1: Start frontend dev server
cd examples/api
pnpm run dev
# Wait for: "ready at http://localhost:1420/"

# Terminal 2: Start Tauri backend
cd examples/api/src-tauri
cargo run

# If you see "Compiling..." - this is normal on first run (takes 5-10 min)
```

#### Method 2: Using Tauri CLI (Alternative)

```bash
# Install Tauri CLI globally
cargo install tauri-cli --version ^2.0.0

# Run from examples/api
cd examples/api
cargo tauri dev
```

#### Method 3: Using pnpm script (If Node CLI works)

```bash
cd examples/api
pnpm run tauri dev
```

## Common Errors and Solutions

### Error: "Failed to initialize GTK backend"
**Cause**: No display server (X11/Wayland) available
**Solution**:
- On Linux: Ensure you're running in a graphical environment (not SSH/container)
- Try: `echo $DISPLAY` - should return something like `:0` or `:1`
- If empty, you're in a headless environment

### Error: "Cannot find native binding"
**Cause**: @tauri-apps/cli native module not built correctly
**Solution**:
```bash
# Remove node_modules and rebuild
cd examples/api
rm -rf node_modules
pnpm install

# Or use Rust CLI instead
cargo install tauri-cli
cd examples/api
cargo tauri dev
```

### Error: "WebKit not found" (Linux)
**Cause**: Missing system libraries
**Solution**:
```bash
sudo apt install -y libwebkit2gtk-4.1-dev
```

### Error: "Could not find Cargo.toml"
**Cause**: Running from wrong directory
**Solution**:
```bash
# Ensure you're in the right directory
cd /path/to/Playa_Tay/examples/api/src-tauri
cargo run

# Or use full path
cd /path/to/Playa_Tay/examples/api
cargo tauri dev
```

### Error: "port 1420 already in use"
**Cause**: Previous dev server still running
**Solution**:
```bash
# Find and kill the process
lsof -ti:1420 | xargs kill -9

# Or use a different port
cd examples/api
pnpm run dev -- --port 3000

# Update tauri.conf.json devUrl to match
```

### Error: Rust compilation takes too long
**Cause**: First compile includes all dependencies
**Solution**:
- Wait it out (5-10 minutes on first run)
- Subsequent runs will be much faster (incremental compilation)
- Use `cargo build --release` for optimized build (takes longer but faster runtime)

### Error: "Electron not found" when clicking button
**Cause**: Electron sidecar not built or copied
**Solution**:
```bash
# Rebuild and copy
pnpm -F @playa/electron-shell build
mkdir -p examples/api/src-tauri/resources/electron-shell
cp packages/electron-shell/dist/main.js examples/api/src-tauri/resources/electron-shell/main.js

# Restart the app
```

## Verification Steps

After successful setup, verify everything works:

### 1. Check Tauri App Launches
- Main window opens
- UI shows "Welcome" page
- Console has no errors

### 2. Check Electron Button State
- Button shows "(checking...)" briefly on startup
- Button shows one of:
  - Enabled (blue) - Electron available
  - Disabled "(unavailable)" - Electron not found

### 3. Test Electron Feature
If button is disabled:
1. Open terminal in app directory
2. Run: `pnpm -F @playa/electron-shell install`
3. Run: `pnpm -F @playa/electron-shell build`
4. Copy to resources (see step 3 above)
5. Click "Retry" in modal or restart app

If button is enabled:
1. Click "Open Electron Feature"
2. New Electron window should open
3. Should load https://example.com
4. Both windows run independently

### 4. Check Security Logs
In the terminal running `cargo run`, you should see:
```
[Electron Sidecar] Security flags enforced:
  - contextIsolation: true
  - nodeIntegration: false
  - sandbox: true
  - devTools: true
  - CSP: enforced
```

## Debug Mode

For detailed debugging:

```bash
# Set Rust debug level
RUST_LOG=debug cargo run

# Or for Tauri-specific logs
RUST_LOG=tauri=debug cargo run

# For full trace
RUST_LOG=trace cargo run
```

## Getting Help

If you're still having issues, collect this information:

1. **System Info**:
```bash
uname -a
node --version
pnpm --version
rustc --version
cargo --version
```

2. **VS Code Version**:
- Help → About
- Paste version info

3. **Error Logs**:
- Full terminal output
- Browser console errors (F12)
- Tauri log files

4. **Diagnostic Commands**:
```bash
# Check Tauri setup
cd examples/api/src-tauri
cargo tauri info

# Check file structure
ls -la examples/api/src-tauri/resources/electron-shell/
ls -la packages/electron-shell/dist/
```

## Quick Test Script

Save this as `test-local-setup.sh` and run it:

```bash
#!/bin/bash
set -e

echo "🔍 Testing local setup..."

echo "✓ Checking Node.js..."
node --version

echo "✓ Checking pnpm..."
pnpm --version

echo "✓ Checking Rust..."
rustc --version

echo "✓ Checking Cargo..."
cargo --version

echo "✓ Installing dependencies..."
pnpm install

echo "✓ Building @tauri-apps/api..."
pnpm -F @tauri-apps/api build

echo "✓ Building Electron sidecar..."
pnpm -F @playa/electron-shell build

echo "✓ Copying sidecar to resources..."
mkdir -p examples/api/src-tauri/resources/electron-shell
cp packages/electron-shell/dist/main.js examples/api/src-tauri/resources/electron-shell/main.js

echo "✓ Building frontend..."
pnpm --filter api build

echo "✓ Checking Tauri setup..."
cd examples/api/src-tauri
cargo tauri info

echo "✅ Setup complete! Ready to run."
echo ""
echo "To start the app:"
echo "  Terminal 1: cd examples/api && pnpm run dev"
echo "  Terminal 2: cd examples/api/src-tauri && cargo run"
```

Make it executable and run:
```bash
chmod +x test-local-setup.sh
./test-local-setup.sh
```

## Next Steps

Once you have the app running:
1. Follow the testing checklist in `TESTING_GUIDE.md`
2. Test all scenarios (not_installed, available, spawn errors)
3. Verify security flags are enforced
4. Check both windows run independently

If you encounter specific errors not covered here, please share:
- Exact error message
- Which step failed
- Your OS and version
- Output of `cargo tauri info`
