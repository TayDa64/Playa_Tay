# M1 Streaming Hub

A Tauri-based multi-view streaming application implementing the M1 (Streaming Hub) specification from VISION.md.

## Overview

M1 Streaming Hub is a production-ready streaming application that demonstrates:
- **Module Isolation**: Each stream runs in an isolated webview context
- **Security-by-Default**: Enforces contextIsolation, CSP, and sandbox mode per constitution.md P2
- **Tauri-First Architecture**: Pure Tauri/WebView implementation following constitution.md P1
- **Performance Optimization**: Custom streaming protocol with HTTP range request support
- **Multi-View Layout**: 2x2 grid layout inspired by Vue multi-view patterns

## Features

### Core Capabilities
- **4 Isolated Streaming Views**: Each stream operates independently in its own webview
- **Custom Streaming Protocol**: Efficient video streaming with range request support
- **Security Enforcement**: 
  - Content Security Policy (CSP) with strict allowlists
  - Sandbox mode enabled for all webviews
  - No eval or remote content execution
- **Performance Budget Compliance**:
  - Chunked streaming with 1MB max buffer (MAX_LEN)
  - Lazy loading of video content
  - Efficient memory management with streaming protocol

### Architecture

```
┌─────────────────────────────────────────────────┐
│          M1 Streaming Hub Window                │
│                 (1200x800)                      │
├─────────────────┬───────────────────────────────┤
│   Stream 1      │      Stream 2                 │
│   (Primary)     │      (Secondary)              │
│   Webview 1     │      Webview 2                │
│   Isolated      │      Isolated                 │
├─────────────────┼───────────────────────────────┤
│   Stream 3      │      Stream 4                 │
│   (Tertiary)    │      (Quaternary)             │
│   Webview 3     │      Webview 4                │
│   Isolated      │      Isolated                 │
└─────────────────┴───────────────────────────────┘
```

Each webview is completely isolated with its own:
- JavaScript context
- DOM tree
- Security policies
- Resource limits

## Constitution Compliance

### ✅ P1: Tauri-First with Selective Electron
- Uses pure Tauri/WebView implementation
- No Electron dependencies
- Module isolation through multiple webviews

### ✅ P2: Security-by-Default
- **CSP Enforcement**: `default-src 'self'; connect-src ipc: http://ipc.localhost; media-src stream: http://stream.localhost 'self'`
- **Context Isolation**: Each webview runs in isolated context
- **Sandbox Mode**: All webviews sandboxed by default
- **No Node Integration**: Pure web technologies
- **Asset Protocol Scoping**: Limited to specific video files

### ✅ P3: Reproducible Monorepo Builds
- Follows examples/ structure
- Self-contained with minimal dependencies
- Deterministic build process

### ✅ P4: Tests and Quality Gates
- Clean Rust code with proper error handling
- Comprehensive inline documentation
- Performance-optimized streaming protocol

### ✅ P5: Modularity and Packaging
- Standalone example that can be packaged independently
- Minimal resource footprint
- Clear separation of concerns

## Usage

### Development

To run the streaming hub in development mode:

```bash
# From repository root
cargo run --example streaming-hub
```

The application will:
1. Download a test video if not present (Big Buck Bunny)
2. Launch the streaming hub window with 4 views
3. Initialize the custom streaming protocol
4. Load the same video in all 4 streams (demonstrating isolation)

### Building

```bash
# Build the example
cargo build --example streaming-hub --release
```

### In Production

Each stream can be configured to point to different sources:
- Live streaming endpoints (HLS/DASH)
- Local media files
- Network streams
- DRM-protected content (via Electron Pattern A/B if needed)

## Technical Details

### Custom Streaming Protocol

The `stream://` protocol implements:
- **HTTP Range Requests**: Partial content delivery (206 responses)
- **Efficient Buffering**: 1MB chunks for optimal memory usage
- **Multi-range Support**: Handle complex range request scenarios
- **Error Handling**: Proper 404/416 responses

### Performance Budgets

- **Max Chunk Size**: 1000 KB (1MB) per request
- **Memory Efficiency**: Streaming reduces memory footprint vs. full file loading
- **Concurrent Streams**: Up to 4 streams with isolated contexts
- **Lazy Loading**: Video content loaded on-demand via range requests

### Security Model

```
Content Security Policy:
├─ default-src: 'self' only
├─ connect-src: IPC and localhost only
├─ media-src: Custom stream protocol + self
└─ style-src: Self + inline (for UI styling)

Asset Protocol:
└─ Scope: Only test video file accessible
```

## Extending the Hub

### Adding More Streams

Modify `main.rs` to add additional webviews:

```rust
let _webview5 = window.add_child(
  tauri::webview::WebviewBuilder::new("stream5", WebviewUrl::App(Default::default()))
    .auto_resize(),
  LogicalPosition::new(x, y),
  LogicalSize::new(width, height),
)?;
```

### Connecting Real Streams

Update the `index.html` video sources to point to actual stream URLs:

```javascript
source.src = 'https://your-streaming-server.com/stream.m3u8';
```

### DRM Support

For DRM-protected content, implement Electron Pattern A or B as per constitution.md:
- Pattern A: Sidecar process for Widevine support
- Pattern B: Optional module with signed updates

## Dependencies

- **Tauri**: Core framework
- **http**: HTTP types and utilities
- **http-range**: Range request parsing
- **getrandom**: Secure random boundary generation
- **percent-encoding**: URL decoding

## Notes

- The example uses a single test video file for all 4 streams to demonstrate isolation
- In production, each stream would connect to different sources
- The streaming protocol is optimized for local file streaming but can be adapted for network sources
- All webviews share the same HTML template but run in isolated contexts

## License

Copyright 2019-2024 Tauri Programme within The Commons Conservancy
SPDX-License-Identifier: Apache-2.0 OR MIT
