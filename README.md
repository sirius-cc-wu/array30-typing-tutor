# Array30 Typing Tutor

Web-based typing tutor for the Array30 input method.

This branch rewrites the UI in Flutter Web and serves it from a Rust backend.

## Highlights

- Practice tab with live typing feedback
- Real-time WPM, accuracy, and elapsed time
- Session history persisted to browser `localStorage`
- Statistics tab with aggregate progress metrics
- Modern UI with Flutter theming and custom styling
- **Integrated Array30 code hints** sourced from [gontera/array30](https://github.com/gontera/array30)

## Tech Stack

- Flutter (web UI)
- Dart 3
- Rust (Axum static file backend)
- Browser storage via `shared_preferences` (web localStorage)

## Project Layout

```text
flutter_app/                  # Flutter web frontend
  lib/
    main.dart                  # App entry + UI
    models.dart                # Practice session + stats models
    storage.dart               # Local storage persistence
    array30_data.dart          # Array30 code mapping (auto-generated)
  web/                         # Flutter web host page
src/                           # Rust backend (Axum)
legacy/dioxus/                 # Previous Dioxus frontend (archived)
```

## Prerequisites

- Flutter SDK
- Rust toolchain (`rustup`, `cargo`)

## Run Locally (Dev)

### Flutter web dev server

```bash
cd flutter_app
flutter pub get
flutter run -d chrome
```

### Rust backend serving a web build

```bash
cd flutter_app
flutter build web

cd ..
cargo run
```

By default the Rust backend serves `flutter_app/build/web` at `http://127.0.0.1:8080`.
You can override the asset path or address:

```bash
ASSET_DIR=flutter_app/build/web ADDR=127.0.0.1:8080 cargo run
```

## Usage

1. Go to `Practice`.
2. Type the displayed Traditional Chinese sentence in the input area.
3. Watch live metrics:
   - `WPM`: `(typed_characters / 5) / minutes`
   - `Accuracy`: `correct_characters / total_typed * 100`
   - `Time`: elapsed seconds in the current session
4. Click `Save & Next Lesson` after completing an exercise.
5. Open `Statistics` to review cumulative performance.

## Legacy Dioxus Build

The prior Rust + Dioxus implementation has been archived under `legacy/dioxus/` for reference.

## License

MIT
