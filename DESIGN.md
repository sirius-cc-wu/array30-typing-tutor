# Design System: Array30 Typing Tutor
**Project ID:** 4643025004616796982

## 1. Visual Theme & Atmosphere
The design system follows a **Claymorphism** aesthetic—a soft, 3D, and tactile "toy-like" style. It aims to reduce practice fatigue by making the educational experience feel playful and engaging. The mood is professional yet approachable, utilizing "bubbly" geometry and soft depth.

## 2. Color Palette & Roles
*   **Lavender-Blue Base (#EEF2FF):** Primary background color. Provides a soft, low-strain canvas for long practice sessions.
*   **Indigo Deep (#4F46E5):** Primary action and accent color. Used for active keystrokes, pulsing highlights, and primary component states.
*   **Energetic Orange (#F97316):** Call-to-Action (CTA) color. Reserved for high-importance actions like "Restart" or "Next Lesson."
*   **Deep Navy (#1E1B4B):** Primary text color. Ensures maximum legibility (7:1+ contrast) against the lavender background.
*   **Vibrant Teal (#0D9488):** Success state color. Used for progress indicators and completed lessons.

## 3. Typography Rules
*   **Header & UI Font:** **Plus Jakarta Sans**. A modern, clean sans-serif with excellent readability at various sizes.
*   **Practice Text Font:** **Atkinson Hyperlegible** or **Fira Code**. High-distinction monospaced fonts to help users distinguish between similar characters (e.g., 'i', 'l', '1').
*   **Weights:** Headers use Bold (700); Body text uses Medium (500) or Regular (400).

## 4. Component Stylings
*   **Buttons:** Chunky, "pressable" appearance. Defined by a **3px solid border** and a **double shadow** (a dark outer shadow for depth and a light inner shadow for the 3D "clay" effect).
*   **Cards/Containers:** "ROUND_FULL" or generously rounded corners (24px+). Containers feature a subtle inner glow to enhance the 3D volume.
*   **Typing Area:** Elevated central card with a "pressed" look when characters are typed. Active characters pulse with the Indigo accent.

## 5. Layout Principles
*   **Whitespace:** Generous padding and margins to maintain the "airy" feel of the Claymorphic style.
*   **Grid Alignment:** Centered, focused layouts. The practice arena is always the focal point, with metrics docked in elevated cards above.
*   **Micro-interactions:** Smooth, slightly "bouncy" transitions (200-300ms) for hover and active states to reinforce the tactile nature of the UI.
