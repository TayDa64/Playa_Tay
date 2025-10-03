# Selective Electron Integration Specification

This directory contains the specification and implementation plan for integrating Electron selectively with the Playa Tay Tauri application.

## Files

- **spec.md** - Main specification document containing:
  - Overview and integration patterns
  - Implementation requirements
  - Technical architecture
  - Code examples
  - Testing and security strategies

## Purpose

The goal of this specification is to document how to integrate Electron into a Tauri application selectively, allowing the use of Electron features only where necessary while maintaining the performance benefits of Tauri's native webview approach.

## Key Concepts

### Sidecar Pattern
Run Electron as a separate process that can be invoked by the main Tauri application when needed.

### Micro-Shell Pattern
Use Electron for specific features that require Node.js or Electron-specific APIs while keeping the core application in Tauri.

## Getting Started

1. Review the [spec.md](./spec.md) file for the complete specification
2. Identify which features in your application require Electron
3. Determine which integration pattern best fits your needs
4. Follow the implementation roadmap in the specification

## References

- [Tauri Documentation](https://tauri.app/v2/)
- [Electron Documentation](https://www.electronjs.org/docs/latest)
- [Tauri Sidecar Guide](https://tauri.app/v2/guides/building/sidecar)
