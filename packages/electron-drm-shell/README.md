# Electron DRM Shell (Pattern B Outline)

This package outlines a separately packaged Electron micro-shell for optional DRM/Widevine functionality.

- Distributed independently from the Tauri host.
- Provides a localhost IPC contract with ephemeral auth.
- Installed on demand and updated via its own signed channel.

Next steps:
- Add electron-builder config for platform installers.
- Implement IPC handshake and token validation.
- Integrate a minimal UI for the protected feature.
