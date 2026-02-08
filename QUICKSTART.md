# Quick Start

## Fast Path (Recommended)

1. Install Dioxus CLI once:

```bash
cargo install dioxus-cli
```

2. From the repository root, run:

```bash
dx serve
```

3. Open the local URL printed by `dx` (typically `http://localhost:8080`).

## Manual Build Path

1. Add WebAssembly target once:

```bash
rustup target add wasm32-unknown-unknown
```

2. Build the app:

```bash
cargo build --target wasm32-unknown-unknown
```

3. If you have build output to serve locally, use any static file server.
Example:

```bash
python3 -m http.server 8000
```

## Troubleshooting

### `dx` command not found

- Confirm `cargo install dioxus-cli` completed successfully.
- Ensure Cargo bin directory is on your `PATH` (often `~/.cargo/bin`).

### Port already in use

- `dx serve` usually picks another port automatically.
- For manual serving, choose another port:

```bash
python3 -m http.server 9000
```

### Browser loads but app is blank

- Check browser developer console for runtime errors.
- Restart `dx serve` after dependency changes.
- Run `cargo check` to confirm Rust-side compilation is clean.
