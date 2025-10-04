# Headless CI and Testing Guidance

This document provides guidance for running Electron-based features in headless/containerized environments.

## Problem Statement

Electron requires a display server to create windows. In CI environments (Docker, GitHub Actions) running without a graphical environment, Electron will fail with errors like:
```
Error: Cannot open display
```

## Solutions

### Option 1: Skip UI Tests (Recommended for v1)

The integration test suite in `examples/api/src-tauri/tests/electron_integration.rs` is designed to work in headless environments:

- **Availability checks**: Always run (no display required)
- **Spawn tests**: Conditionally skip when `DISPLAY` is not set

```rust
if std::env::var("DISPLAY").is_ok() || cfg!(target_os = "windows") || cfg!(target_os = "macos") {
  // Test actual window spawn
} else {
  println!("Skipping spawn test: headless environment");
}
```

**Benefits**:
- No additional dependencies
- Fast CI execution
- Tests core error handling and availability detection

**Limitations**:
- Does not test actual window rendering
- Cannot verify visual aspects

### Option 2: Xvfb (X Virtual Framebuffer)

Xvfb creates a virtual display server, allowing Electron to run without physical display hardware.

#### Installation (Ubuntu/Debian)

```bash
sudo apt-get update
sudo apt-get install -y xvfb
```

#### Usage

**Inline with commands**:
```bash
xvfb-run -a cargo test -p api --test electron_integration
```

**Set DISPLAY for session**:
```bash
export DISPLAY=:99
Xvfb :99 -screen 0 1024x768x24 &
cargo test -p api --test electron_integration
```

**GitHub Actions example**:
```yaml
- name: Install Xvfb
  run: sudo apt-get install -y xvfb

- name: Run integration tests with Xvfb
  run: xvfb-run -a cargo test -p api --test electron_integration
```

#### Electron-Specific Requirements

Electron also needs additional system libraries on Linux:

```bash
sudo apt-get install -y \
  libnss3 \
  libnspr4 \
  libdbus-1-3 \
  libatk1.0-0 \
  libatk-bridge2.0-0 \
  libcups2 \
  libdrm2 \
  libxkbcommon0 \
  libxcomposite1 \
  libxdamage1 \
  libxfixes3 \
  libxrandr2 \
  libgbm1 \
  libasound2
```

These are already installed in our CI workflow (`.github/workflows/ci.yml`).

### Option 3: Wayland (Future)

Electron supports Wayland as a display protocol. In theory, headless Wayland compositors could be used, but this is less common in CI and not recommended for v1.

## Current CI Strategy

Our `.github/workflows/ci.yml` uses **Option 1 (Skip UI)** for v1:

1. Install Linux dependencies (including Electron runtime deps)
2. Build sidecar and copy resources
3. Run `cargo check` to verify compilation
4. Run integration tests (spawn tests skip in headless)
5. Build with Rust Tauri CLI using CI config

**Rationale**:
- Fast builds
- No flaky display issues
- Tests core error handling (not_installed, spawn_error codes)
- Validates that sidecar can be detected and invoked

## Testing Strategy by Environment

| Environment | Availability Check | Spawn Test | Window Rendering |
|-------------|-------------------|------------|------------------|
| **Dev (local)** | ✅ | ✅ | ✅ |
| **CI (headless)** | ✅ | ⏭️ Skip | ⏭️ Skip |
| **CI + Xvfb** | ✅ | ✅ | ⚠️ Partial* |

\* Xvfb allows spawn but does not test actual visual rendering

## Recommended Approach for Contributors

### Local Development
Run full test suite:
```bash
cargo test -p api --test electron_integration
```

### CI/PR Validation
Option 1 (current):
```bash
# Tests skip spawn automatically if no display
cargo test -p api --test electron_integration
```

Option 2 (with Xvfb):
```bash
xvfb-run -a cargo test -p api --test electron_integration
```

### Manual Spawn Testing in CI
If you need to validate actual spawning in CI, add an optional job:

```yaml
test-electron-spawn:
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - name: Install dependencies
      run: |
        sudo apt-get update
        sudo apt-get install -y xvfb libnss3 libnspr4 ...
    - name: Setup Rust/Node
      # ... (same as main job)
    - name: Install Electron sidecar
      run: pnpm -F @playa/electron-shell install
    - name: Run spawn tests
      run: xvfb-run -a cargo test -p api --test electron_integration::tests::test_open_electron_feature_success
```

## Debugging Tips

### Check if DISPLAY is set
```bash
echo $DISPLAY
# If empty, Electron will fail
```

### Verify Xvfb is running
```bash
ps aux | grep Xvfb
```

### Test Electron manually
```bash
export DISPLAY=:99
Xvfb :99 -screen 0 1024x768x24 &
cd packages/electron-shell
pnpm run dev
```

### Check for missing libraries
```bash
ldd node_modules/.bin/electron
# Look for "not found" entries
```

## Future Improvements

- [ ] Add optional Xvfb CI job for comprehensive spawn validation
- [ ] Investigate headless Wayland compositors (e.g., weston --backend=headless-backend.so)
- [ ] Add visual regression tests with screenshot capture (requires display)
- [ ] Document macOS/Windows CI runner considerations

## References

- [Electron Testing Documentation](https://www.electronjs.org/docs/latest/tutorial/automated-testing)
- [Xvfb Man Page](https://www.x.org/releases/X11R7.6/doc/man/man1/Xvfb.1.xhtml)
- [GitHub Actions Display Server Setup](https://github.com/marketplace/actions/setup-display-action)
