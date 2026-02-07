# Quick Start Guide

## Option 1: Using dx (Recommended)

### 1. Install dx (dioxus-cli) - one time only
```bash
cargo install dioxus-cli
```

### 2. Start the development server
```bash
cd /home/ccwu/workspace/array30-typing-tutor
dx serve
```

### 3. Open in browser
Visit `http://localhost:8080` in your web browser.

**Benefits:**
- Hot reload (changes auto-refresh)
- Better error messages
- Development server included

---

## Option 2: Manual WebAssembly Build

### 1. Add WebAssembly target (one time only)
```bash
rustup target add wasm32-unknown-unknown
```

### 2. Build the project
```bash
cd /home/ccwu/workspace/array30-typing-tutor
cargo build --target wasm32-unknown-unknown
```

### 3. Serve the dist folder
You can use Python's built-in server:
```bash
cd dist
python3 -m http.server 8000
```

Then visit `http://localhost:8000` in your browser.

---

## Troubleshooting

### Port already in use
If port 8080 (or 8000) is already in use:
- For dioxus: It will automatically try the next port
- For Python: `python3 -m http.server 9000` (use different port)

### WebAssembly not loading
- Make sure you're using the correct URL (http, not https for local dev)
- Check browser console (F12) for any error messages
- Clear browser cache and reload

### Still having issues?
Ensure you have:
- Rust 1.70+
- Latest Cargo
- WebAssembly target installed: `rustup target add wasm32-unknown-unknown`

