# Quick Start

## Flutter Web (Recommended)

1. Install Flutter SDK.
2. From the Flutter app directory, fetch packages and run:

```bash
cd flutter_app
flutter pub get
flutter run -d chrome
```

## Rust Backend (Serve Built Web Assets)

1. Build the Flutter web bundle:

```bash
cd flutter_app
flutter build web
```

2. From the repository root, run the Rust server:

```bash
cd ..
cargo run
```

The server will serve `flutter_app/build/web` on `http://127.0.0.1:8080`.

To customize:

```bash
ASSET_DIR=flutter_app/build/web ADDR=127.0.0.1:8080 cargo run
```

## Troubleshooting

### Flutter command not found

- Ensure Flutter is installed and on your `PATH`.

### Port already in use

- Change the `ADDR` environment variable:

```bash
ADDR=127.0.0.1:9090 cargo run
```
