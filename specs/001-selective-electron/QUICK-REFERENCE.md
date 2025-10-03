# Quick Reference Card

## 🎯 Where to Start

| Your Situation | Start Here | Then Read | Time Needed |
|----------------|------------|-----------|-------------|
| **New to this concept** | INDEX.md | README.md → spec.md | 1 hour |
| **Ready to implement** | implementation-guide.md | configuration.md | 2-3 hours |
| **Existing Tauri app** | migration-guide.md | implementation-guide.md | 2-4 hours |
| **Architecture review** | architecture.md | spec.md → configuration.md | 1-2 hours |
| **Troubleshooting** | faq.md | implementation-guide.md | 30 min |

## 📋 Integration Patterns at a Glance

### Sidecar Pattern
```
Tauri App (Main) ←→ Electron (Separate Process)
```
**When to use**: Occasional Electron features  
**Pros**: Isolated, secure, simple  
**Cons**: Larger bundle size  
**Bundle size**: +50-100MB  

### Micro-Shell Pattern
```
Tauri App ⊃ Electron Features
```
**When to use**: Heavy Electron usage  
**Pros**: Full integration, shared state  
**Cons**: More complex  
**Bundle size**: +50-100MB  

## 💻 Code Snippets

### Check Electron Availability
```typescript
import { electronAPI } from './lib/electron';
await electronAPI.initialize();
if (electronAPI.isAvailable()) {
    // Use Electron
} else {
    // Use fallback
}
```

### Tauri Command (Rust)
```rust
#[tauri::command]
async fn launch_electron_feature(
    feature: String
) -> Result<String, String> {
    // Implementation
    Ok(format!("Launched {}", feature))
}
```

### Frontend Invoke (TypeScript)
```typescript
import { invoke } from '@tauri-apps/api/core';

const result = await invoke('launch_electron_feature', {
    feature: 'my-feature'
});
```

## 🔧 Common Commands

```bash
# Development
pnpm tauri dev

# Build
pnpm tauri build

# Test
cargo test          # Rust tests
pnpm test          # Frontend tests

# Check Electron
electron --version
```

## ⚡ Performance Targets

| Metric | Target | Critical |
|--------|--------|----------|
| Startup Time | < +100ms | < +200ms |
| Memory Usage | < +50MB | < +100MB |
| Binary Size | < +20MB | < +50MB |
| IPC Latency | < 10ms | < 50ms |

## 🔐 Security Checklist

- [ ] Validate all IPC messages
- [ ] Never enable `nodeIntegration`
- [ ] Use `contextBridge` for safe APIs
- [ ] Sanitize all inputs
- [ ] Follow principle of least privilege
- [ ] Keep dependencies updated
- [ ] Regular security audits

## 🐛 Troubleshooting Quick Fixes

| Problem | Quick Fix |
|---------|-----------|
| Electron not found | `pnpm add -D electron` |
| Command not found | Check `generate_handler!` registration |
| High memory usage | Profile and check for leaks |
| Slow IPC | Batch messages, cache results |
| Build failures | Check system dependencies |

## 📞 Getting Help

- **Documentation**: Start with INDEX.md
- **FAQ**: See faq.md for 40+ answers
- **Community**: [Tauri Discord](https://discord.com/invite/tauri)
- **Issues**: [GitHub](https://github.com/tauri-apps/tauri/issues)

## 🎓 Implementation Phases

### Phase 1: Planning (1-2 days)
- [ ] Read documentation
- [ ] Choose pattern
- [ ] Identify requirements

### Phase 2: Development (2-3 days)
- [ ] Create commands
- [ ] Build frontend
- [ ] Implement IPC

### Phase 3: Testing (1-2 days)
- [ ] Unit tests
- [ ] Integration tests
- [ ] Performance tests

### Phase 4: Deployment (1 day)
- [ ] Build packages
- [ ] Test on all platforms
- [ ] Deploy

## 🔑 Key Files

### Must Read
- `INDEX.md` - Navigation
- `spec.md` - Specification
- `implementation-guide.md` - How to implement

### Reference
- `architecture.md` - Design patterns
- `configuration.md` - Setup details
- `faq.md` - Troubleshooting

### Migration
- `migration-guide.md` - Adding to existing app
- `README.md` - Quick overview

## 📊 Decision Matrix

### Should I use Electron with Tauri?

| Requirement | Use Electron? |
|-------------|---------------|
| Need Node.js modules | ✅ Yes |
| Need Electron APIs | ✅ Yes |
| Performance critical | ❌ No (use Tauri) |
| Small bundle size | ❌ No (use Tauri) |
| Multiple window contexts | ✅ Maybe |
| Legacy code | ✅ Maybe |

## 🎯 Success Criteria

✅ All existing features still work  
✅ Electron features work when available  
✅ Graceful degradation without Electron  
✅ Performance targets met  
✅ Security audit passed  
✅ Tests pass  
✅ Documentation complete  

## 📝 Quick Notes

- **Always test without Electron** - Ensure graceful degradation
- **Use feature flags** - Make Electron optional
- **Profile performance** - Monitor resource usage
- **Document everything** - Future you will thank you
- **Start small** - Migrate one feature at a time
- **Security first** - Never trust cross-process data

---

**Need more details?** See the full documentation in this directory!
