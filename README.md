# Array30 Typing Tutor

A web-based typing tutor application for practicing the Array30 input method, built with Rust and Dioxus.

## Features

- **Interactive Practice Interface**: Type along with Vietnamese text exercises
- **Real-time Statistics**: Track typing speed (WPM), accuracy, and elapsed time
- **Array30 Exercises**: Multiple lessons to practice with Array30 input method
- **Progress Tracking**: Visual progress bar showing typing completion
- **Reset Functionality**: Clear your input and start fresh

## Tech Stack

- **Language**: Rust
- **UI Framework**: Dioxus (web)
- **Styling**: Custom CSS with Tailwind-like utilities
- **Build System**: Cargo

## Project Structure

```
array30-typing-tutor/
├── src/
│   ├── main.rs              # Application entry point
│   ├── app.rs               # Root component
│   ├── components/
│   │   ├── mod.rs           # Component module and StatsDisplay
│   │   └── practice_interface.rs  # Main practice UI
│   └── logic.rs             # Typing logic and state management
├── assets/
│   └── styles.css           # Application styles
└── Cargo.toml               # Project configuration
```

## Getting Started

### Prerequisites

- Rust 1.70+
- Cargo
- dx (dioxus-cli) - install with: `cargo install dioxus-cli`

### Installation & Running

1. Navigate to the project directory:
   ```bash
   cd /home/ccwu/workspace/array30-typing-tutor
   ```

2. **Run with dx (Recommended)**
   ```bash
   dx serve
   ```
   This will start a development server with hot reload. Open `http://localhost:8080` in your browser.

### Development with Hot Reload

For the best development experience, use:
```bash
dx serve
```

This provides:
- Automatic recompilation on file changes
- Hot reload in the browser
- Development server on `http://localhost:8080`
- Better error messages

## Usage

1. Open the application in your web browser
2. Read the Traditional Chinese text shown on the screen
3. Type the text using Array30 input method in the textarea
4. View real-time statistics:
   - **WPM**: Words Per Minute (characters typed / 5 / minutes)
   - **Accuracy**: Percentage of correctly typed characters
   - **Seconds**: Elapsed time since you started typing
5. Click "Next Exercise" to move to the next lesson
6. Click "Reset" to clear everything and start over

## Features in Detail

### Practice Interface
- Clear display of the text to type
- Large textarea for comfortable typing
- Visual progress bar showing typing completion
- Real-time feedback on performance

### Statistics Dashboard
- WPM calculation based on standard typing metrics
- Accuracy calculation comparing correct vs. total characters typed
- Timer showing elapsed time

### Exercise Management
- Multiple Vietnamese Array30 practice exercises
- Cycle through different sentences for variety
- Quick progression to next exercise

## Future Enhancements

- [ ] Lesson progression system with difficulty levels
- [ ] Sound feedback for correct/incorrect typing
- [ ] Leaderboard and performance history
- [ ] Array30-specific keyboard layout visualization
- [ ] Custom lesson creation
- [ ] Export typing statistics

## Contributing

Feel free to contribute improvements to the typing tutor!

## License

MIT
