# Design System: Array30 Typing Tutor

## 1. Current Visual Direction
The UI now follows a **clean, high-contrast DaisyUI light theme**:
- Bright surfaces with restrained shadows
- Clear hierarchy through spacing and typography
- Component polish driven by DaisyUI primitives

## 2. Design Tokens
Primary tokens are sourced from DaisyUI light theme variables:
- Base surfaces: `--b1`, `--b2`, `--b3`
- Content: `--bc`
- Accent roles: `--p`, `--s`, `--a`
- Status roles: `--su`, `--wa`, `--er`, `--in`

Custom styling is limited to the typing experience and Array30 key hints.

## 3. Typography
Imported families:
- `Space Grotesk` (UI chrome, headings, body)
- `JetBrains Mono` (Array30 code keycaps)

Usage rules:
- Global UI uses `Space Grotesk`
- Hint keycaps use `JetBrains Mono`

## 4. Component Implementation Notes
- **Tabs:** DaisyUI `tabs` with boxed styling; active tab uses primary highlight.
- **Metrics:** DaisyUI `stats` with `stat-title` / `stat-value` formatting.
- **Cards:** DaisyUI `card` with soft shadow and base surfaces.
- **Typing Exercise Text:** Custom styling for correct/incorrect/untyped characters.
- **Actions:** DaisyUI `btn` variants with consistent sizing.
- **Hint Row:** Dedicated hint card with keycap-styled Array30 code.

## 5. Hint and Keycap Behavior
Behavior in `src/components/practice_interface.rs`:
- Hint progression is based on **matched prefix count** (typed chars that match target in order), not raw input length.
- This keeps the hint anchored to the current expected character during temporary mismatch/composing states.
- If a code exists, it renders in a keycap style (`.code-hint-keycap`); if no mapping exists, only the target character is shown.
- Completion state displays a neutral "Exercise Complete" message.

## 6. Responsive Behavior
- `stats` stack vertically on small screens.
- Typing area and hint panel collapse to a single column on mobile.
- Hint/keycap font sizes scale down for small screens.
