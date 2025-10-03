import { app, BrowserWindow } from 'electron';

async function createWindow(url: string) {
  const win = new BrowserWindow({
    width: 1200,
    height: 800,
    webPreferences: {
      contextIsolation: true,
      nodeIntegration: false,
      sandbox: true,
      devTools: false,
    },
  });

  await win.loadURL(url);
}

app.whenReady().then(async () => {
  const url = process.env.ELECTRON_TARGET_URL || 'about:blank';
  await createWindow(url);
});
