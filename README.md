# social-media-blocker-air

This project is a cross-browser extension and a companion desktop application.

## Development Setup

### Browser Extension Key

To ensure your extension has a consistent ID across development, you need to use a fixed public key in the `manifest.json`.

1. **Generate a private key**:
   ```bash
   openssl genrsa 2048 | openssl pkcs8 -topk8 -nocrypt -out key.pem
   ```
   *Note: `key.pem` is automatically added to `.gitignore` to prevent committing your private key.*

2. **Extract the public key** and format it for the `manifest.json`:
   ```bash
   openssl rsa -in key.pem -pubout -outform DER | openssl base64 -A
   ```

3. **Paste the resulting base64 string** into your `extension/manifest.json` under the `"key"` field:
   ```json
   {
     "key": "YOUR_BASE64_PUBLIC_KEY_HERE"
   }
   ```

   ## Building and Loading the Extension

   The browser extension works identically across Windows, Linux, and macOS.

   ### 1. Build the Extension
   Navigate to the `extension` directory and build the production files:
   ```bash
   cd extension
   npm install
   npm run build
   ```
   This will generate the built files in the `extension/dist` directory.

   ### 2. Load the Extension in Browser
   1. Open your browser's extensions management page (e.g., `chrome://extensions/` in Chrome/Edge, `about:debugging` in Firefox).
   2. Enable "Developer mode".
   3. Click "Load unpacked" and select the `extension` directory (or the `extension/dist` directory depending on your browser's requirements).

   *Note: The native messaging functionality requires the Tauri desktop application to be running and correctly registered on your OS.*
