# Architecture Overview

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                        Playa Tay Application                 │
└─────────────────────────────────────────────────────────────┘
                              │
                              │
        ┌─────────────────────┴─────────────────────┐
        │                                           │
        ▼                                           ▼
┌──────────────────┐                      ┌──────────────────┐
│  Tauri Core      │                      │  Electron        │
│  (Main Process)  │◄────────IPC─────────►│  (Sidecar)       │
└──────────────────┘                      └──────────────────┘
        │                                           │
        │                                           │
        ▼                                           ▼
┌──────────────────┐                      ┌──────────────────┐
│  Native WebView  │                      │  Electron        │
│  (WKWebView/     │                      │  BrowserWindow   │
│   WebView2/      │                      │                  │
│   WebKitGTK)     │                      │                  │
└──────────────────┘                      └──────────────────┘
        │                                           │
        │                                           │
        └─────────────────┬─────────────────────────┘
                          │
                          ▼
                ┌──────────────────┐
                │   Frontend UI    │
                │   (Svelte/React/ │
                │    Vue/etc)      │
                └──────────────────┘
```

## Component Interaction

### 1. Application Startup

```
User launches app
    │
    ▼
Tauri initializes
    │
    ├──► Load configuration
    │
    ├──► Check for Electron binary
    │    │
    │    ├──► If found: Initialize Electron sidecar
    │    │
    │    └──► If not found: Enable fallback mode
    │
    ├──► Start native webview
    │
    └──► Load frontend UI
```

### 2. Feature Invocation Flow

```
User clicks "Launch Electron Feature" button
    │
    ▼
Frontend calls invoke('launch_electron_feature')
    │
    ▼
Tauri command handler receives request
    │
    ├──► Validate request
    │
    ├──► Check if Electron is available
    │    │
    │    ├──► Yes: Send IPC message to Electron
    │    │         │
    │    │         ▼
    │    │    Electron receives message
    │    │         │
    │    │         ▼
    │    │    Execute feature
    │    │         │
    │    │         ▼
    │    │    Send response back to Tauri
    │    │
    │    └──► No: Return error or fallback
    │
    └──► Return result to frontend
```

### 3. IPC Communication

```
┌─────────────┐                    ┌──────────────┐
│    Tauri    │                    │   Electron   │
│   Process   │                    │   Process    │
└─────────────┘                    └──────────────┘
      │                                    │
      │  1. Send command request           │
      ├───────────────────────────────────►│
      │     {id, command, payload}         │
      │                                    │
      │                                    │ 2. Process
      │                                    │    command
      │                                    │
      │  3. Send response                  │
      │◄───────────────────────────────────┤
      │     {id, success, data/error}      │
      │                                    │
```

## Data Flow

### Tauri → Electron

1. Frontend invokes Tauri command
2. Tauri validates and processes request
3. Tauri sends IPC message to Electron
4. Electron executes requested operation
5. Electron sends response back
6. Tauri forwards response to frontend

### Electron → Tauri

1. Electron detects event/condition
2. Electron sends notification to Tauri
3. Tauri processes notification
4. Tauri emits event to frontend
5. Frontend updates UI accordingly

## Security Boundaries

```
┌────────────────────────────────────────────┐
│  Untrusted Zone                            │
│  ┌──────────────────────────────────────┐  │
│  │  Frontend UI (JavaScript)            │  │
│  └──────────────────────────────────────┘  │
└────────────────┬───────────────────────────┘
                 │ Tauri IPC
                 │ (validated commands only)
                 ▼
┌────────────────────────────────────────────┐
│  Trusted Zone                              │
│  ┌──────────────────────────────────────┐  │
│  │  Tauri Core (Rust)                   │  │
│  └──────────────────────────────────────┘  │
│                                            │
│  ┌──────────────────────────────────────┐  │
│  │  Electron Sidecar (Node.js)          │  │
│  └──────────────────────────────────────┘  │
└────────────────────────────────────────────┘
                 │
                 ▼
        System Resources
```

## Module Responsibilities

### Tauri Core
- Application lifecycle management
- Native webview rendering
- IPC coordination
- Configuration management
- Security enforcement

### Electron Sidecar
- Node.js-specific features
- Electron API access
- Legacy module support
- Additional UI windows (if needed)

### Frontend UI
- User interface rendering
- User interaction handling
- State management
- Feature orchestration

## Deployment Architecture

### Development Environment
```
src/
├── frontend/          # Frontend code
├── tauri/            # Tauri Rust code
└── electron/         # Electron code (if using micro-shell)

Separate processes running concurrently
```

### Production Bundle
```
app.app (or app.exe/AppImage)
├── main binary       # Tauri application
├── resources/
│   ├── frontend/     # Bundled web assets
│   └── electron/     # Electron binary (if included)
└── config/          # Configuration files
```

## Scalability Considerations

- **Process isolation**: Each component runs in its own process
- **Resource management**: Monitor and limit resource usage
- **Load balancing**: Distribute work between Tauri and Electron
- **Caching**: Cache results to minimize IPC overhead
- **Lazy loading**: Start components only when needed
