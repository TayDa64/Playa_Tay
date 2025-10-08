# IPC Security in Tauri

## Overview

The Inter-Process Communication (IPC) layer in Tauri provides secure communication between the frontend (JavaScript/WebView) and the backend (Rust). This document describes the security measures implemented to protect against various attack vectors.

## Security Validations

### 1. Invoke Key Validation

Every IPC request must include a valid invoke key that matches the server-generated key. This prevents unauthorized scripts from invoking backend commands.

**Implementation:**
- A random 128-bit Z85-encoded key is generated on application startup
- The key is injected into the frontend during initialization
- Every IPC request must include this key in the `Tauri-Invoke-Key` header
- The backend validates the key before processing any request

**Constraints:**
- Maximum length: 256 characters
- Cannot be empty
- Cannot contain null bytes

### 2. Command Name Validation

Command names are validated to prevent injection attacks and ensure only legitimate commands are processed.

**Validations:**
- Command name cannot be empty
- Maximum length: 512 characters
- Cannot contain null bytes (`\0`)
- Cannot contain control characters (except tab `\t`)

**Why these validations?**
- **Null bytes**: Can be used for string termination attacks in some contexts
- **Control characters**: Can be used to inject commands or manipulate output
- **Length limits**: Prevents resource exhaustion attacks

### 3. Payload Size Limits

To prevent Denial of Service (DoS) attacks through oversized payloads, all IPC request bodies are size-limited.

**Limit:**
- Maximum payload size: 10 MB (10,485,760 bytes)

**Rationale:**
- Prevents memory exhaustion attacks
- Ensures reasonable resource usage
- Most legitimate IPC payloads are much smaller than this limit
- For large file transfers, use dedicated file APIs instead of IPC

### 4. Access Control List (ACL)

Commands are subject to ACL checks based on:
- Command name and plugin
- Window/webview context
- Origin (local vs remote)
- User-defined capabilities

**How it works:**
1. Each command invocation is checked against the runtime authority
2. The authority resolves permissions based on capabilities defined in your application
3. Commands without proper permissions are rejected
4. Remote origins require explicit permission to invoke commands

### 5. Origin Validation

The origin of each IPC request is validated to ensure it comes from a trusted source.

**Validations:**
- Origin header must be present
- Origin must be a valid URL
- Remote origins must be explicitly allowed in capabilities

## Best Practices

### For Application Developers

1. **Minimize Command Surface Area**
   - Only expose commands that are absolutely necessary
   - Use the ACL system to restrict commands to specific windows/contexts

2. **Validate Command Arguments**
   - Always validate and sanitize input data in your command handlers
   - Use strong typing with Rust's type system
   - Don't trust any data coming from the frontend

3. **Use Scopes**
   - Leverage command-specific scopes to limit what operations can be performed
   - For file operations, use path scopes to restrict access to specific directories

4. **Avoid Sensitive Data in Frontend**
   - Never store sensitive data (API keys, credentials) in the frontend
   - Use the backend to securely store and access sensitive information

5. **Keep Payloads Small**
   - Design your IPC protocol to use small, efficient payloads
   - For large data transfers, consider alternative approaches (file APIs, streams)

### For Plugin Developers

1. **Follow Naming Conventions**
   - Use the `plugin:name|command` format for plugin commands
   - Choose clear, descriptive command names

2. **Define Clear Permissions**
   - Create granular permissions for each command
   - Document the security implications of each permission

3. **Implement Command Scopes**
   - Define scope objects for fine-grained control
   - Validate scope matches in command handlers

4. **Handle Errors Securely**
   - Don't leak sensitive information in error messages
   - Log security events for auditing

## Security Constants

The following constants define security limits in the IPC protocol:

```rust
// Maximum payload size (10 MB)
const MAX_PAYLOAD_SIZE: usize = 10 * 1024 * 1024;

// Maximum command name length
const MAX_COMMAND_LENGTH: usize = 512;

// Maximum invoke key length
const MAX_INVOKE_KEY_LENGTH: usize = 256;
```

These constants can be found in `crates/tauri/src/ipc/protocol.rs`.

## Testing

Comprehensive security tests are included in the protocol test suite:

- **Command validation tests**: Ensure invalid commands are rejected
- **Payload size tests**: Verify oversized payloads are blocked
- **Invoke key tests**: Validate key format requirements
- **Control character tests**: Ensure malicious characters are rejected

Run the tests with:
```bash
cargo test --package tauri --lib ipc::protocol::tests
```

## Common Attack Vectors and Mitigations

### 1. Command Injection
**Attack**: Attacker tries to inject malicious commands through command names
**Mitigation**: Command name validation rejects control characters and null bytes

### 2. Denial of Service
**Attack**: Attacker sends extremely large payloads to exhaust memory
**Mitigation**: 10MB payload size limit enforced before processing

### 3. Unauthorized Access
**Attack**: Malicious script tries to invoke privileged commands
**Mitigation**: Invoke key validation and ACL system

### 4. Remote Code Execution
**Attack**: Remote webpage tries to execute backend commands
**Mitigation**: Origin validation and remote domain restrictions in ACL

### 5. Path Traversal
**Attack**: Attacker uses relative paths to access restricted files
**Mitigation**: Path scopes in file system APIs (separate from IPC layer)

## Reporting Security Issues

If you discover a security vulnerability in the Tauri IPC system, please follow our responsible disclosure process outlined in [SECURITY.md](../../SECURITY.md).

**Do NOT:**
- Create public issues or pull requests
- Discuss vulnerabilities on Discord or forums
- Attempt to exploit vulnerabilities in production systems

## Future Enhancements

Potential future security enhancements being considered:

1. **Rate Limiting**: Limit the number of IPC calls per time period
2. **Request Signing**: Add cryptographic signatures to requests
3. **Audit Logging**: Comprehensive logging of security-relevant events
4. **Dynamic Payload Limits**: Configurable payload size limits per command
5. **Command Throttling**: Slow down repeated failed authentication attempts

## References

- [Tauri Security Documentation](https://tauri.app/security/)
- [IPC Protocol Implementation](./src/ipc/protocol.rs)
- [Runtime Authority Implementation](./src/ipc/authority.rs)
- [ACL Documentation](https://v2.tauri.app/security/capabilities/)
