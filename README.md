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
