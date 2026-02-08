# Array30 Typing Tutor

Web-based typing tutor for the Array30 input method, built with Rust + Dioxus.

The current exercises are Traditional Chinese text samples.

## Highlights

- Practice tab with live typing feedback
- Real-time WPM, accuracy, and elapsed time
- Session history persisted to browser `localStorage`
- Statistics tab with aggregate progress metrics
- Modern UI with Tailwind-based styling and official DioxusLabs components

## Tech Stack

- Rust (edition 2021)
- Dioxus `0.7` (web target)
- `dx` (Dioxus CLI) for development workflow
- Browser storage via `wasm-bindgen` + `localStorage`

## Project Layout

```text
src/
  main.rs                        # App entry
  app.rs                         # Root layout + tab navigation
  logic.rs                       # Practice session state + typing stats
  storage.rs                     # Session persistence + statistics aggregation
  components/
    practice_interface.rs        # Practice workflow UI
    statistics.rs                # Statistics dashboard UI
    button/                      # Official DioxusLabs component (scaffolded)
    card/                        # Official DioxusLabs component (scaffolded)
assets/
  styles.css                     # App Tailwind/CSS styles
  dx-components-theme.css        # Global theme for DioxusLabs components
```

## Prerequisites

- Rust toolchain (`rustup`, `cargo`)
- Dioxus CLI:

```bash
cargo install dioxus-cli
```

## Run Locally

1. Clone and enter the repository.
2. Start the dev server:

```bash
dx serve
```

3. Open the local URL shown by `dx` (usually `http://localhost:8080`).

## Usage

1. Go to `Practice`.
2. Type the displayed Traditional Chinese sentence in the input area.
3. Watch live metrics:
   - `WPM`: `(typed_characters / 5) / minutes`
   - `Accuracy`: `correct_characters / total_typed * 100`
   - `Time`: elapsed seconds in the current session
4. Click `Save & Next Challenge` after completing an exercise.
5. Open `Statistics` to review cumulative performance.

## Development Notes

- List official UI components:

```bash
dx components list
```

- Add official UI components:

```bash
dx components add <component-name>
```

- Build/check:

```bash
cargo check
```

## Roadmap

- Improve CJK character-count correctness in UI progress/completion logic
- Add baseline unit tests for logic/statistics
- Remove existing compiler warnings in storage and style lints

## License

MIT
