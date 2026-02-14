# Design System: Array30 Typing Tutor

## 1. Current Visual Direction
The implemented UI is a **claymorphic + soft-gradient hybrid**:
- Large rounded geometry (`--radius-shell: 2.5rem`, `--radius-card: 1.5rem`)
- High-contrast navy structure for action surfaces
- Soft layered backgrounds and inset highlights for depth

The app shell uses a lavender base plus radial accents, while controls use stronger borders/shadows to preserve tactile feedback.

## 2. Design Tokens (Implemented)
Defined in `assets/styles.css`:
- `--color-brand-primary: #4848e5`
- `--color-brand-secondary: #818cf8`
- `--color-brand-accent: #0d9488`
- `--color-brand-warm: #f97316`
- `--color-surface-base: #EEF2FF`
- `--color-content-strong: #1e1b4b`
- `--color-content-muted: #6b7280`

Shadow system:
- Clay-style shadows: `--shadow-clay`, `--shadow-clay-warm`, `--shadow-clay-navy`
- Hover/pressed variants: `--shadow-clay-hover`, `--shadow-clay-warm-hover`, `--shadow-clay-navy-hover`, `--shadow-clay-pressed`
- Focus ring: `--shadow-focus`

## 3. Typography
Imported families:
- `Plus Jakarta Sans` (UI chrome, labels, headings)
- `Atkinson Hyperlegible` (typing readability)
- `Fira Code` (code/keycap presentation)

Usage rules in implementation:
- Main UI text and navigation use `Plus Jakarta Sans`
- Typing content/input area uses `Atkinson Hyperlegible` with monospace fallback
- Input hints for Array30 codes use monospace keycaps (`Fira Code` first)

## 4. Component Implementation Notes
- **Tabs:** pill container with elevated active state and scale/translate micro-motion.
- **Metrics:** white rounded capsules, soft layered shadows, color-coded values:
  - WPM -> brand primary
  - Accuracy -> green
  - Progress/level -> brand warm
- **Typing Exercise Text:** framed white surface with muted-untyped, indigo-correct, and red-highlight-incorrect characters.
- **Actions:**
  - Primary CTA: wide warm pill with 3px border and warm clay shadow
  - Secondary CTA: circular navy shadow button
- **Hint Row:** dedicated hint box below typing area with:
  - next target character
  - arrow separator
  - **keycap-styled Array30 code** (`.code-hint-keycap`)

## 5. Hint and Keycap Behavior
Current behavior in `src/components/practice_interface.rs`:
- Hint progression is based on **matched prefix count** (typed chars that match target in order), not raw input length.
- This keeps the hint anchored to the current expected character during temporary mismatch/composing states.
- If a code exists, it renders in a keycap style (`.code-hint-keycap`); if no mapping exists, only the target character is shown.
- Completion state displays a neutral "Exercise Complete" message.

## 6. Responsive Behavior
Mobile breakpoints reduce hint/keycap sizing and spacing:
- Smaller `.code-hint-char` and `.code-hint-keycap`
- Maintains fixed hint box height to avoid layout shifts during typing
- Action buttons and text scale with `clamp()`-based sizing
