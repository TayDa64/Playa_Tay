# Priority Statements for AI Agents

1. When executing terminal commands, do not use the `sleep` command. Instead, use the terminalID to check the terminal's background state.
2. Before running tests, use the `get_errors` tool to check for and resolve errors, as this may help tests pass successfully.
3. When writing TypeScript code, avoid using `any`. Prefer `unknown` with explicit type guards or clauses.
4. Do not make assumptions. When unsure, always reference your research with a web search and use official documentation.
5. When creating 3D worlds or objects with three.js, review the documentation for gravity and camera controls before implementation.

# Electron Usage in This Repository

This monorepo includes packages for Electron integration:
- `packages/electron-drm-shell`
- `packages/electron-shell`

These are provided for compatibility, testing, or migration purposes. The primary framework is Tauri (Rust backend, system webview frontend). Electron is not the main runtime for production apps in this repository, but these packages may be used for:
- Comparing Tauri and Electron behaviors
- Supporting legacy Electron-based workflows
- Testing DRM or shell features in an Electron context

Refer to the respective package `README.md` files for details on usage and integration patterns. When contributing, prioritize Tauri-native solutions unless Electron is explicitly required for a given task.

# Copilot Instructions for Playa_Tay (Tauri Framework)

## Overview
This repository is the monorepo for the Tauri framework, a toolkit for building secure, small, and fast desktop applications using Rust for the backend and any web technology for the frontend. The architecture is modular, with each major component in its own crate or package. Tauri leverages system webviews (via WRY) and window management (via TAO) for cross-platform support.

## Architecture & Key Components
- **Core Crates**: `crates/tauri` (main orchestrator), `tauri-build`, `tauri-codegen`, `tauri-macros`, `tauri-runtime`, `tauri-runtime-wry`, `tauri-utils`.
- **Tooling**: `crates/tauri-cli` (Rust CLI), `packages/cli` (JS wrapper), `packages/api` (JS/TS API for frontend integration), `crates/tauri-bundler` (packaging), `create-tauri-app` (scaffolding new projects).
- **Plugins**: Extend Tauri via Rust and JS glue code. See `crates/tauri-plugin` and [plugin docs](https://v2.tauri.app/develop/plugins/).
- **External Dependencies**: Uses [TAO](https://github.com/tauri-apps/tao) for windowing and [WRY](https://github.com/tauri-apps/wry) for webview abstraction.

## Developer Workflows
- **Build (Rust)**: Use `cargo build` in the relevant crate directory.
- **Build (Full App)**: Use `pnpm tauri build` or `npm run tauri build` from the project root for a full build (frontend + backend).
- **Development Mode**: `pnpm tauri dev` or `npm run tauri dev` starts the frontend dev server and backend watcher with hot reload.
- **Info**: `pnpm tauri info` for project diagnostics.
- **Testing**: Rust tests in each crate's `tests/` directory. JS/TS tests in `packages/`.
- **Release**: Configure `tauri.conf.json` and run `pnpm tauri build`. Artifacts are in `./src-tauri/target/release`.

## Project Conventions
- **Configuration**: Main config is `tauri.conf.json` (read at compile time). Rust-side config in `Cargo.toml`.
- **Documentation**: Prefer inline Rust/JS doc comments. See `ARCHITECTURE.md` for high-level design.
- **Polyglot**: Rust for backend, JS/TS for frontend and tooling. No Node.js runtime is shipped in production.
- **Monorepo**: Each crate/package is self-contained. Use workspace-level `Cargo.toml` and `pnpm-workspace.yaml` for dependency management.
- **Cross-Component Communication**: Message passing between webview (frontend) and Rust backend via the JS API (`@tauri-apps/api`).

## Examples & References
- See `examples/` for sample apps and integration patterns.
- See `ARCHITECTURE.md` for detailed component roles and data flows.
- See each crate's `README.md` for crate-specific details.

## Special Notes
- **Plugins**: Author plugins by providing Rust code, JS glue, and API surface. See `crates/tauri-plugin`.
- **CI/CD**: Uses GitHub Actions (see `.github/workflows/`). Official [tauri-action](https://github.com/tauri-apps/tauri-action) for cross-platform builds.
- **Licensing**: MIT or Apache-2.0. See `LICENSE*` files for details.

---
For more, see [tauri.app](https://tauri.app) and the [official docs](https://v2.tauri.app/).
