import { app, BrowserWindow, session } from 'electron';

async function createWindow(url: string) {
  // Enforce Content Security Policy
  session.defaultSession.webRequest.onHeadersReceived((details, callback) => {
    callback({
      responseHeaders: {
        ...details.responseHeaders,
        'Content-Security-Policy': [
          "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https:; connect-src 'self' https:;"
        ]
      }
    });
  });

  const isProduction = process.env.NODE_ENV === 'production';

  const win = new BrowserWindow({
    width: 1200,
    height: 800,
    webPreferences: {
      contextIsolation: true,
      nodeIntegration: false,
      sandbox: true,
      devTools: !isProduction, // Disable devTools in production
    },
  });

  await win.loadURL(url);

  // Log security status
  console.log('[Electron Sidecar] Security flags enforced:');
  console.log(`  - contextIsolation: true`);
  console.log(`  - nodeIntegration: false`);
  console.log(`  - sandbox: true`);
  console.log(`  - devTools: ${!isProduction}`);
  console.log(`  - CSP: enforced`);
}

app.whenReady().then(async () => {
  const url = process.env.ELECTRON_TARGET_URL || 'about:blank';
  await createWindow(url);
});
