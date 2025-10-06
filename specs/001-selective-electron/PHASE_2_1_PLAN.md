# 🏗️ Phase 2.1: Foundation & Packaging - Detailed Implementation Plan

**Timeline**: Q1 2026 (3-4 months)
**Status**: Planning Phase
**Prerequisites**: Phase 1 deployed and stable in production

---

## 🎯 Phase 2.1 Objectives

Establish the foundational infrastructure for Pattern B modules, including secure packaging, code signing, and automated build pipelines. This phase creates the technical foundation that all future modules will build upon.

### Core Goals
- **Secure Module Framework**: Establish base security patterns for all modules
- **Cross-Platform Packaging**: Build pipeline for macOS, Windows, Linux
- **Code Signing Infrastructure**: Automated signing and verification
- **Update Server Foundation**: Basic infrastructure for module distribution

---

## 📋 Detailed Task Breakdown

### Task 2.1.1: Electron Builder Configuration 🔧
**Duration**: 2-3 weeks
**Owner**: Lead Developer + DevOps Engineer

#### Deliverables
- [ ] **Base Module Template**
  ```
  packages/electron-module-template/
  ├── electron-builder.yml          # Platform configs
  ├── package.json                  # Build scripts & deps
  ├── src/
  │   ├── main.ts                   # Secure entry point
  │   ├── preload.ts               # Minimal preload script
  │   ├── security/
  │   │   ├── flags.ts             # Security flag enforcement
  │   │   ├── csp.ts               # Content Security Policy
  │   │   └── ipc-auth.ts          # Token authentication
  │   └── renderer/
  │       ├── index.html           # Basic UI template
  │       └── style.css            # Minimal styling
  ├── assets/
  │   ├── icon.png                 # Module icon (1024x1024)
  │   └── background.png           # Installer background
  └── build/
      ├── entitlements.mac.plist   # macOS sandboxing
      └── notarize.js              # macOS notarization
  ```

- [ ] **Platform-Specific Configurations**
  ```yaml
  # electron-builder.yml
  appId: com.playa.module.template
  productName: Playa Module Template

  directories:
    output: dist

  files:
    - "!**/.vscode/*"
    - "!src/*"
    - "!electron-builder.yml"

  mac:
    category: public.app-category.utilities
    hardenedRuntime: true
    gatekeeperAssess: false
    entitlements: build/entitlements.mac.plist
    entitlementsInherit: build/entitlements.mac.plist
    target:
      - target: dmg
        arch: [x64, arm64]

  win:
    target: nsis
    certificateSubjectName: "Playa Technologies Inc"
    signingHashAlgorithms: ['sha256']

  linux:
    target:
      - target: deb
        arch: x64
      - target: rpm
        arch: x64
    category: Utility
  ```

#### Acceptance Criteria
- ✅ Template builds successfully on all platforms
- ✅ All security flags properly enforced
- ✅ Generated packages install correctly
- ✅ No warnings or errors in build process

---

### Task 2.1.2: Code Signing Infrastructure 🔐
**Duration**: 3-4 weeks
**Owner**: Security Engineer + DevOps Engineer

#### Certificate Acquisition Plan

##### macOS Code Signing
- [ ] **Apple Developer Account Setup**
  - Enroll in Apple Developer Program ($99/year)
  - Generate Developer ID Application certificate
  - Set up App Store Connect for notarization

- [ ] **Certificate Management**
  ```bash
  # Generate signing request
  security create-keypair \
    -a 2048 \
    -d "Playa Technologies Inc" \
    -D "Developer ID Application" \
    -t RSA \
    signing-key

  # Export for CI/CD
  security export \
    -k login.keychain \
    -t identities \
    -f p12 \
    -o developer-id.p12
  ```

##### Windows Code Signing
- [ ] **Certificate Authority Selection**
  - Research: DigiCert, GlobalSign, Sectigo
  - Consider EV certificate for immediate SmartScreen trust
  - Estimate: $300-800/year depending on CA and type

- [ ] **Azure SignTool Integration**
  ```yaml
  # GitHub Actions workflow
  - name: Sign Windows Binary
    uses: azure/azure-code-signing-action@v2
    with:
      azure-tenant-id: ${{ secrets.AZURE_TENANT_ID }}
      azure-client-id: ${{ secrets.AZURE_CLIENT_ID }}
      azure-client-secret: ${{ secrets.AZURE_CLIENT_SECRET }}
      endpoint: https://eus.codesigning.azure.net/
      code-signing-account-name: playa-codesigning
      certificate-profile-name: playa-modules
      files-folder: dist/
  ```

##### Linux Package Signing
- [ ] **GPG Key Generation**
  ```bash
  # Generate signing key
  gpg --full-generate-key
  # Select RSA, 4096 bits, no expiration
  # Real name: Playa Technologies Inc
  # Email: packages@playa.app

  # Export for package repositories
  gpg --armor --export packages@playa.app > playa-public.key
  ```

- [ ] **Repository Setup**
  ```bash
  # Debian repository structure
  dists/
  ├── stable/
  │   ├── Release
  │   ├── Release.gpg
  │   └── main/
  │       └── binary-amd64/
  │           └── Packages
  └── testing/
      └── ...
  ```

#### Automated Signing Pipeline
- [ ] **CI/CD Integration**
  ```yaml
  # .github/workflows/build-module.yml
  name: Build and Sign Module

  on:
    push:
      tags: ['v*']

  jobs:
    build-mac:
      runs-on: macos-latest
      steps:
        - name: Import certificates
          run: |
            security create-keychain -p "" build.keychain
            security import ${{ secrets.MAC_CERT_P12 }} -P ${{ secrets.MAC_CERT_PASSWORD }}

        - name: Build and sign
          run: |
            pnpm run build:mac

        - name: Notarize
          run: |
            xcrun notarytool submit dist/*.dmg \
              --apple-id ${{ secrets.APPLE_ID }} \
              --password ${{ secrets.APPLE_PASSWORD }} \
              --team-id ${{ secrets.TEAM_ID }}
  ```

#### Security Verification
- [ ] **Signature Validation Tools**
  ```typescript
  // src/security/verify-signature.ts
  export async function verifyModuleSignature(modulePath: string): Promise<boolean> {
    if (process.platform === 'darwin') {
      return verifyMacSignature(modulePath);
    } else if (process.platform === 'win32') {
      return verifyWindowsSignature(modulePath);
    } else {
      return verifyLinuxSignature(modulePath);
    }
  }

  async function verifyMacSignature(path: string): Promise<boolean> {
    // Use codesign --verify --deep --strict
    const result = await exec(`codesign --verify --deep --strict "${path}"`);
    return result.exitCode === 0;
  }
  ```

#### Acceptance Criteria
- ✅ All certificates acquired and properly configured
- ✅ Automated signing works in CI/CD pipeline
- ✅ Signature verification passes on all platforms
- ✅ Signed modules install without security warnings

---

### Task 2.1.3: Base Module Security Framework 🛡️
**Duration**: 2-3 weeks
**Owner**: Security Engineer + Lead Developer

#### Security Flag Enforcement
- [ ] **Mandatory Security Configuration**
  ```typescript
  // src/security/flags.ts
  export const REQUIRED_SECURITY_FLAGS = {
    webSecurity: true,
    contextIsolation: true,
    nodeIntegration: false,
    nodeIntegrationInWorker: false,
    nodeIntegrationInSubFrames: false,
    sandbox: true,
    enableRemoteModule: false,
    allowRunningInsecureContent: false,
    experimentalFeatures: false
  } as const;

  export function enforceSecurityFlags(options: BrowserWindowConstructorOptions) {
    const webPreferences = {
      ...REQUIRED_SECURITY_FLAGS,
      ...options.webPreferences
    };

    // Validate no security flags were overridden
    for (const [key, requiredValue] of Object.entries(REQUIRED_SECURITY_FLAGS)) {
      if (webPreferences[key] !== requiredValue) {
        throw new Error(`Security violation: ${key} must be ${requiredValue}`);
      }
    }

    return { ...options, webPreferences };
  }
  ```

#### Content Security Policy Framework
- [ ] **Strict CSP Implementation**
  ```typescript
  // src/security/csp.ts
  export const DEFAULT_CSP = [
    "default-src 'self'",
    "script-src 'self' 'unsafe-inline'", // Minimal inline scripts only
    "style-src 'self' 'unsafe-inline'",  // Allow inline styles
    "img-src 'self' data: https:",        // Images from self, data URIs, HTTPS
    "connect-src 'self' ws://localhost:*", // IPC communication only
    "font-src 'self'",
    "object-src 'none'",                  // No plugins
    "media-src 'self'",
    "frame-src 'none'",                   // No frames
    "base-uri 'self'",
    "form-action 'none'"                  // No forms
  ].join('; ');

  export function applyCSP(session: Electron.Session) {
    session.webRequest.onHeadersReceived((details, callback) => {
      callback({
        responseHeaders: {
          ...details.responseHeaders,
          'Content-Security-Policy': [DEFAULT_CSP]
        }
      });
    });
  }
  ```

#### IPC Authentication Framework
- [ ] **Token-Based Authentication**
  ```typescript
  // src/security/ipc-auth.ts
  export class IPCAuthenticator {
    private validTokens = new Set<string>();
    private tokenExpiry = new Map<string, number>();

    generateToken(): string {
      const token = crypto.randomBytes(32).toString('hex');
      const expiry = Date.now() + (30 * 60 * 1000); // 30 minutes

      this.validTokens.add(token);
      this.tokenExpiry.set(token, expiry);

      return token;
    }

    validateToken(token: string): boolean {
      if (!this.validTokens.has(token)) return false;

      const expiry = this.tokenExpiry.get(token);
      if (!expiry || Date.now() > expiry) {
        this.revokeToken(token);
        return false;
      }

      return true;
    }

    revokeToken(token: string): void {
      this.validTokens.delete(token);
      this.tokenExpiry.delete(token);
    }
  }
  ```

#### Acceptance Criteria
- ✅ Security flags cannot be overridden by module code
- ✅ CSP blocks unauthorized network requests
- ✅ IPC authentication prevents unauthorized access
- ✅ Security framework passes penetration testing

---

### Task 2.1.4: Update Server Infrastructure 🌐
**Duration**: 3-4 weeks
**Owner**: DevOps Engineer + Backend Developer

#### Server Architecture Design
- [ ] **Update Server Specification**
  ```
  update-server/
  ├── api/
  │   ├── modules/
  │   │   ├── list                 # GET /api/modules/list
  │   │   ├── {id}/latest          # GET /api/modules/{id}/latest
  │   │   ├── {id}/download        # GET /api/modules/{id}/download
  │   │   └── {id}/changelog       # GET /api/modules/{id}/changelog
  │   └── verification/
  │       └── signature            # POST /api/verification/signature
  ├── storage/
  │   ├── modules/
  │   │   └── {platform}/
  │   │       └── {module-id}/
  │   │           └── {version}/
  │   └── metadata/
  │       └── {module-id}.json
  └── cdn/
      └── cloudflare-config.json
  ```

- [ ] **Module Metadata Schema**
  ```typescript
  interface ModuleMetadata {
    id: string;
    name: string;
    description: string;
    version: string;
    minAppVersion: string;
    maxAppVersion?: string;
    platform: 'darwin' | 'win32' | 'linux';
    architecture: 'x64' | 'arm64' | 'universal';

    download: {
      url: string;
      size: number;
      checksum: string;
      signature: string;
    };

    capabilities: string[];
    permissions: string[];

    changelog: {
      version: string;
      date: string;
      changes: string[];
    }[];
  }
  ```

#### CDN and Distribution
- [ ] **Cloudflare Setup**
  ```javascript
  // cloudflare-worker.js
  export default {
    async fetch(request, env, ctx) {
      const url = new URL(request.url);

      // Module download endpoint
      if (url.pathname.startsWith('/download/')) {
        return handleModuleDownload(request, env);
      }

      // Signature verification
      if (url.pathname.startsWith('/verify/')) {
        return handleSignatureVerification(request, env);
      }

      return new Response('Not found', { status: 404 });
    }
  };

  async function handleModuleDownload(request, env) {
    // Verify request signature
    // Log download for analytics
    // Serve file from R2 storage
  }
  ```

- [ ] **Geographic Distribution**
  ```yaml
  # CDN configuration
  regions:
    - name: us-east
      primary: true
      storage: cloudflare-r2

    - name: eu-west
      storage: cloudflare-r2

    - name: asia-pacific
      storage: cloudflare-r2
  ```

#### Security and Monitoring
- [ ] **Access Logging**
  ```typescript
  interface DownloadLog {
    timestamp: string;
    moduleId: string;
    version: string;
    platform: string;
    userAgent: string;
    ipAddress: string; // Hashed for privacy
    success: boolean;
    errorCode?: string;
  }
  ```

- [ ] **Rate Limiting**
  ```javascript
  // Rate limiting per IP
  const RATE_LIMITS = {
    downloads: 10, // per hour
    verifications: 100 // per hour
  };
  ```

#### Acceptance Criteria
- ✅ Update server handles all module operations
- ✅ CDN provides fast global distribution
- ✅ All downloads are logged and monitored
- ✅ Rate limiting prevents abuse

---

### Task 2.1.5: CI/CD Pipeline Implementation 🔄
**Duration**: 2-3 weeks
**Owner**: DevOps Engineer + Lead Developer

#### Automated Build Pipeline
- [ ] **Multi-Platform Build Matrix**
  ```yaml
  # .github/workflows/build-modules.yml
  name: Build Modules

  on:
    push:
      paths: ['packages/modules/**']

  strategy:
    matrix:
      os: [macos-latest, windows-latest, ubuntu-latest]
      node: [18, 20]

  jobs:
    build:
      runs-on: ${{ matrix.os }}
      steps:
        - name: Checkout
          uses: actions/checkout@v4

        - name: Setup Node.js
          uses: actions/setup-node@v4
          with:
            node-version: ${{ matrix.node }}
            cache: pnpm

        - name: Install dependencies
          run: pnpm install --frozen-lockfile

        - name: Build module
          run: pnpm run build:module

        - name: Sign artifacts
          run: pnpm run sign

        - name: Upload to update server
          run: pnpm run upload
  ```

#### Quality Gates
- [ ] **Automated Testing**
  ```yaml
  test:
    runs-on: ubuntu-latest
    steps:
      - name: Unit tests
        run: pnpm test

      - name: Integration tests
        run: pnpm test:integration

      - name: Security scan
        uses: securecodewarrior/github-action-add-sarif@v1
        with:
          sarif-file: security-scan.sarif

      - name: License check
        run: pnpm run license-check
  ```

- [ ] **Deployment Gates**
  ```yaml
  deploy:
    needs: [build, test]
    if: github.ref == 'refs/heads/main'
    steps:
      - name: Security approval
        uses: trstringer/manual-approval@v1
        with:
          secret: ${{ github.TOKEN }}
          approvers: security-team

      - name: Deploy to staging
        run: pnpm run deploy:staging

      - name: Smoke tests
        run: pnpm run test:smoke

      - name: Deploy to production
        run: pnpm run deploy:production
  ```

#### Acceptance Criteria
- ✅ All platforms build automatically
- ✅ Quality gates prevent bad releases
- ✅ Signed artifacts uploaded to distribution
- ✅ Rollback capability for failed deployments

---

## 📊 Phase 2.1 Success Metrics

### Technical Metrics
- **Build Success Rate**: >98% successful builds across all platforms
- **Security Compliance**: 100% of modules pass security scans
- **Signing Success**: 100% of artifacts properly signed
- **Performance**: Module builds complete in <15 minutes

### Operational Metrics
- **Deployment Frequency**: Multiple deployments per day possible
- **Mean Time to Recovery**: <2 hours for critical issues
- **Certificate Validity**: All certificates valid for >6 months
- **CDN Performance**: <2 second download start time globally

### Quality Metrics
- **Zero Critical Bugs**: No P0 issues in production
- **Documentation Coverage**: 100% of APIs documented
- **Test Coverage**: >90% code coverage
- **Developer Experience**: Setup time <30 minutes for new developers

---

## 🗓️ Phase 2.1 Timeline

### Month 1: Foundation Setup
- **Week 1-2**: Certificate acquisition and setup
- **Week 3-4**: Electron Builder configuration and testing

### Month 2: Security & Infrastructure
- **Week 1-2**: Security framework implementation
- **Week 3-4**: Update server development and deployment

### Month 3: CI/CD & Testing
- **Week 1-2**: Automated pipeline implementation
- **Week 3-4**: End-to-end testing and security audit

### Month 4: Polish & Documentation
- **Week 1-2**: Performance optimization and bug fixes
- **Week 3-4**: Documentation, training, and knowledge transfer

---

## 🎯 Phase 2.1 Deliverables Checklist

### Core Infrastructure ✅
- [ ] Electron Builder template with all platforms
- [ ] Code signing certificates and automation
- [ ] Security framework with enforced flags
- [ ] Update server with CDN distribution
- [ ] CI/CD pipeline with quality gates

### Documentation 📚
- [ ] Module development guide
- [ ] Security best practices
- [ ] Build and deployment procedures
- [ ] Troubleshooting guide
- [ ] API documentation

### Testing & Validation 🧪
- [ ] Security audit report
- [ ] Performance benchmarks
- [ ] Cross-platform compatibility tests
- [ ] End-to-end integration tests
- [ ] Developer experience validation

---

## 🚀 Phase 2.1 Success Criteria

**Phase 2.1 is considered complete when:**

1. ✅ **Template Module**: Base module template builds and runs on all platforms
2. ✅ **Security**: All security requirements enforced and verified
3. ✅ **Signing**: Automated code signing works without manual intervention
4. ✅ **Distribution**: Update server delivers modules reliably
5. ✅ **Automation**: CI/CD pipeline handles full build-to-deployment cycle
6. ✅ **Documentation**: Complete documentation enables new developers
7. ✅ **Validation**: Independent security audit passes with no critical findings

**Ready to transition to Phase 2.2: Detection & Registry** 🎯

---

**Phase 2.1 Implementation Plan Complete** ✅
**Estimated Effort**: 120-160 developer hours
**Timeline**: 3-4 months
**Risk Level**: Medium (dependent on certificate acquisition)
