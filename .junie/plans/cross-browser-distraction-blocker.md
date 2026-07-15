# Requirements

### Overview & Goals
The goal is to build a cross-browser extension and a companion desktop application (using Tauri and Svelte) to help users manage and block distracting websites, similar to the Freedom app.

### Scope
- **In Scope**:
    - Browser extension (Chrome, Firefox, Edge) with site-blocking capabilities using `declarativeNetRequest`.
    - Tauri companion desktop application for advanced management and scheduling.
    - Native messaging bridge for secure communication between the extension and the desktop app.
    - Svelte-based UI for both the extension popup and the desktop application.
- **Out of Scope**:
    - Mobile applications.
    - Complex synchronization server (focusing on local storage for this version).

### User Stories
- **As a user**, I want to quickly add a site to my block list from my browser.
- **As a user**, I want to manage my blocked sites from a centralized desktop application.
- **As a user**, I want the blocking to be performant and not slow down my browsing.

# Technical Design

### Current Implementation
This is a new project with no existing code.

### Key Decisions
- **Desktop Framework**: Tauri (Rust backend, native webview). Chosen for its lightweight footprint and performance.
- **Frontend Framework**: Svelte. Chosen for its efficiency and reactivity for UI components.
- **Browser Extension API**: `declarativeNetRequest`. Chosen for efficient, rule-based blocking that doesn't require inspecting every request in the background script.
- **Communication**: Native Messaging API. The standard and most secure way for a browser extension to interact with a native desktop application.

### Proposed Architecture
- **Browser Extension (`/extension`)**: Handles the actual web request blocking using manifest-defined rules. Communicates with the desktop app via `chrome.runtime.sendNativeMessage`.
- **Desktop App (`/desktop`)**: A Tauri application that acts as the source of truth for the blocked list and handles advanced scheduling or focus modes. Implements a native messaging host.
- **Data Flow**:
  - Desktop App <-> Browser Extension via Native Messaging.
  - Browser Extension <-> Web pages via `declarativeNetRequest`.

### File Structure
```text
/
├── extension/
│   ├── manifest.json
│   ├── background.js
│   └── src/ (Svelte UI)
├── desktop/
│   ├── src-tauri/ (Rust backend, including Native Messaging Host)
│   └── src/ (Svelte UI)
└── shared/ (Communication protocol definitions)
```

# Testing

### Validation Approach
- **Unit Tests**: Test the rule generation logic in Rust (for the desktop app) and JS (for the extension).
- **Integration Tests**:
    - Verify Native Messaging communication.
    - Verify that blocking rules are correctly applied by the browser when updated.
- **Manual Verification**:
    - Open the browser, enable the extension, and test if sites are blocked.
    - Use the desktop application to add a site and verify it's blocked in the browser.

# Delivery Steps

### ✓ Step 1: Setup Project Structure
Establish the project workspace and initialize the Tauri companion application.

- Initialize a new workspace.
- Create a `desktop` directory with a Tauri project scaffold.
- Create a `extension` directory with a basic manifest.json and directory structure.

### ✓ Step 2: Implement Browser Extension Blocking Logic
Implement the core site blocking logic in the browser extension using declarativeNetRequest.

- Define a `declarativeNetRequest` ruleset for blocking.
- Implement the background script to manage rules dynamically.
- Create a simple Svelte UI in the extension popup for toggling blocking on/off.

### * Step 3: Implement Native Messaging Bridge
Implement the Native Messaging host to enable communication between the browser extension and the Tauri application.

- Implement a basic Native Messaging host in Rust within the Tauri app.
- Configure the browser extension manifest to point to the native host.
- Establish a proof-of-concept communication bridge.

###   Step 4: Develop Desktop Companion UI and Syncing
Build the desktop application UI for managing blocked sites and integrate it with the extension.

- Build a Svelte-based UI within the Tauri desktop application.
- Implement functionality to add/remove blocked sites in the desktop app.
- Sync the blocked list between the desktop app and the extension via Native Messaging.
