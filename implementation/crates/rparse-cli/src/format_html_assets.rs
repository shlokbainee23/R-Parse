//! Static presentation assets for R-PARSE Scientific Analysis Studio.
//! Bundled vanilla CSS styles, HTML shell markup, and interactive client JavaScript.

/// Complete vanilla CSS design system for R-PARSE Scientific Analysis Studio (Light-Theme-First).
pub const CSS_STYLES: &str = r###"
/* ==========================================================================
   R-PARSE SCIENTIFIC ANALYSIS STUDIO — MASTER WORKSPACE DESIGN SYSTEM
   Editorial Paper Canvas, Restrained Signal Orange, Deep Ink Typography
   ========================================================================== */

:root {
  /* ========================================================================
     R-PARSE UNIFIED DESIGN SYSTEM — CANONICAL TOKENS (FROM LANDING PAGE)
     Editorial Warm Paper Canvas, Precision Ink Typography, Signal Orange
     ======================================================================== */

  /* Editorial Surfaces */
  --bg-bone: #EEEBE4;
  --bg-paper: #F7F5F0;
  --bg-card: #FFFFFF;
  --surface-white: #FFFFFF;

  --paper-0: var(--bg-paper);       /* #F7F5F0 (Master canvas & workspace) */
  --paper-1: var(--bg-bone);        /* #EEEBE4 (Subtle surface) */
  --paper-2: #E6E2D8;               /* Elevated hover & active state */

  /* Fine Rules & Borders */
  --rule-fine: rgba(18, 18, 18, 0.10);
  --rule-strong: rgba(18, 18, 18, 0.22);
  --line: #DFDCD5;                  /* Crisp opaque equivalent of rule-fine */
  --line-strong: #C5C2BA;           /* Crisp opaque equivalent of rule-strong */
  --line-soft: rgba(18, 18, 18, 0.07);

  /* Canonical Ink Typography */
  --text-ink: #121212;
  --text-ink-secondary: #3A3936;
  --text-muted: #77736B;
  --text-subtle: #A39E93;

  --ink: var(--text-ink);           /* #121212 */
  --ink-1: var(--text-ink);         /* #121212 */
  --ink-2: var(--text-ink-secondary);/* #3A3936 */
  --ink-3: var(--text-muted);       /* #77736B */
  --ink-4: var(--text-subtle);      /* #A39E93 */

  /* Canonical Brand & Accents */
  --accent-orange: #FF5A1F;
  --accent-orange-hover: #E04810;
  --accent-orange-subtle: rgba(255, 90, 31, 0.12);
  --accent-orange-glow: rgba(255, 90, 31, 0.35);

  --orange: var(--accent-orange);   /* #FF5A1F */
  --orange-hover: var(--accent-orange-hover); /* #E04810 */
  --orange-text: #C2410C;           /* High contrast text orange for readability */
  --orange-dark: #C2410C;           /* Semantic dark orange alias */
  --orange-tint: rgba(255, 90, 31, 0.12);
  --orange-line: rgba(255, 90, 31, 0.32);

  /* Secondary Accents */
  --accent-lilac: #B8A6FF;
  --accent-lilac-subtle: rgba(184, 166, 255, 0.15);
  --accent-lilac-text: #7B61FF;
  --accent-chartreuse: #D4FF3A;
  --accent-chartreuse-subtle: rgba(212, 255, 58, 0.16);
  --accent-amber: #F59E0B;
  --accent-amber-subtle: rgba(245, 158, 11, 0.15);

  /* Semantic Status Tokens */
  --status-ok: #2F7D4F;
  --status-ok-subtle: rgba(47, 125, 79, 0.12);
  --status-warn: #8A5A00;
  --status-warn-subtle: rgba(245, 158, 11, 0.15);
  --status-err: #B3261E;
  --status-err-subtle: rgba(179, 38, 30, 0.12);
  --status-ok-line: rgba(47, 125, 79, 0.25);
  --status-warn-line: rgba(138, 90, 0, 0.25);
  --status-err-line: rgba(179, 38, 30, 0.25);

  --ok: var(--status-ok);
  --warn: var(--status-warn);
  --err: var(--status-err);

  /* Derivation & Column Lineage */
  --teal-text: #0F766E;
  --teal-600: #0F766E;
  --teal-700: #0D635C;
  --teal-tint: rgba(15, 118, 110, 0.12);
  --teal-line: rgba(15, 118, 110, 0.30);

  /* Ink washes & overlays (translucent --text-ink, so they stay warm on paper and bone) */
  --ink-wash-1: rgba(18, 18, 18, 0.04);
  --ink-wash-2: rgba(18, 18, 18, 0.05);
  --ink-wash-3: rgba(18, 18, 18, 0.08);
  --scrim: rgba(18, 18, 18, 0.50);
  --shadow-overlay: 0 20px 48px rgba(18, 18, 18, 0.20);
  --shadow-drawer: -8px 0 24px rgba(18, 18, 18, 0.10);
  --shadow-raised: 0 1px 3px rgba(18, 18, 18, 0.10);
  --shadow-hover: 0 2px 8px rgba(18, 18, 18, 0.06);
  --orange-flash: rgba(255, 90, 31, 0.16);
  --text-selection: rgba(255, 90, 31, 0.22);
  --orange-tint-solid: #FFF1EA;     /* --orange-tint flattened onto white; opaque so edges never show through */

  /* Interactive Radii & Dimensions */
  --r-xs: 4px;
  --r-sm: 8px;
  --r-md: 12px;
  --r-lg: 24px;
  --r-pill: 9999px;

  --h-sm: 28px;
  --h-md: 32px;
  --h-lg: 40px;

  --btn-h-sm: 28px;
  --btn-h-md: 32px;
  --btn-h-lg: 40px;

  --focus: 0 0 0 2px var(--bg-paper), 0 0 0 4px var(--orange-text);
  --focus-ring: rgba(255, 90, 31, 0.25);

  /* Layout Dimensions */
  --topbar-height: 52px;
  --header-height: 52px;
  --nav-height: 40px;
  --inspector-width: 320px;
  --source-dock-min-height: 180px;

  /* Graph Palette Tokens */
  --graph-canvas: var(--bg-paper);
  --graph-node: var(--bg-card);
  --graph-node-border: var(--text-ink-secondary);
  --graph-text: var(--text-ink);
  --graph-secondary-text: var(--text-ink-secondary);
  --graph-edge: var(--text-ink-secondary);
  --graph-line: var(--line);
  --graph-derivation: var(--teal-700);
  --graph-derivation-tint: #D3EEEA;
  --graph-selected: var(--orange-tint-solid);
  --graph-selected-border: var(--accent-orange);
  --graph-selected-glow: rgba(255, 90, 31, 0.22);
  --graph-upstream: var(--accent-orange);
  --graph-downstream: var(--teal-text);
  --graph-edge-control: var(--accent-amber);
  --graph-edge-rowsel: var(--accent-lilac-text);
  --graph-edge-unknown: var(--status-err);
  --graph-unknown: var(--status-warn);
  --graph-unknown-tint: var(--status-warn-subtle);
  --graph-hover-shadow: rgba(18, 18, 18, 0.18);
  --graph-node-shadow: rgba(18, 18, 18, 0.16);
  --graph-lilac-tint: #EFEAFF;
  --graph-unknown-fill: #F8E7C4;

  /* Typography Stacks */
  --font-sans: 'General Sans', 'Plus Jakarta Sans', -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
  --font-mono: 'JetBrains Mono', ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;

  /* Source Syntax Palette */
  --syntax-identifier: #1E3A8A;
  --syntax-function: #9F1239;
  --syntax-string: var(--status-ok);
  --syntax-number: #9A4A06;
  --syntax-comment: var(--text-muted);
  --syntax-identifier-tint: rgba(30, 58, 138, 0.08);
  --syntax-function-tint: rgba(159, 18, 57, 0.08);
  --syntax-number-tint: rgba(154, 74, 6, 0.09);
  --source-retained-bg: rgba(255, 90, 31, 0.08);
  --source-pruned-text: var(--text-subtle);

  /* Semantic Compatibility Tokens & Test Aliases */
  --surface-app: var(--bg-paper);
  --surface-panel: var(--bg-card);
  --surface-raised: var(--bg-card);
  --surface-subtle: var(--bg-bone);
  --surface-input: #FCFAF7;
  --surface-hover: var(--paper-2);
  --border-subtle: var(--line);
  --border-default: var(--line);
  --border-strong: var(--line-strong);
  --text-primary: var(--text-ink);
  --text-secondary: var(--text-ink-secondary);
  --text-dim: var(--text-subtle);
  --text-inverse: #FFFFFF;
  --color-accent-primary: var(--accent-orange);
  --color-accent-primary-hover: var(--accent-orange-hover);
  --color-accent-secondary: var(--accent-lilac);
  --color-accent-subtle: rgba(255, 90, 31, 0.12);
  --status-success: var(--status-ok);
  --status-warning: var(--status-warn);
  --status-error: var(--status-err);
  --status-unknown: var(--status-warn);

  /* Legacy aliases preserved for tests */
  --accent-blue: var(--orange);
  --accent-blue-subtle: var(--orange-tint);
  --accent-orange-border: var(--orange-line);
  --accent-green: var(--ok);
  --accent-green-subtle: var(--status-ok-subtle);
  --accent-rose: var(--err);
  --accent-rose-subtle: var(--status-err-subtle);
  --bg-page: var(--paper-0);
  --bg-canvas: var(--paper-0);
  --bg-panel: var(--surface-white);
  --bg-subtle: var(--paper-1);
  --bg-input: #FCFAF7;
  --text-main: var(--ink);
  --border: var(--line);
  --border-focus: var(--orange);

  --radius-xs: 3px;
  --radius-sm: 6px;
  --radius-md: 8px;
  --radius-lg: 12px;
  --radius-pill: 9999px;
  --shadow-sm: 0 1px 2px rgba(18, 18, 18, 0.04);
  --shadow-md: 0 4px 12px rgba(18, 18, 18, 0.06);
  --shadow-lg: 0 12px 28px rgba(18, 18, 18, 0.10);
  --shadow-focus: 0 0 0 2px var(--surface-panel), 0 0 0 4px var(--color-accent-primary);

  /* Motion (curves shared with landing/styles.css; durations tuned for a dense workspace) */
  --ease-editorial: cubic-bezier(0.16, 1, 0.3, 1);
  --ease-out: cubic-bezier(0.22, 1, 0.36, 1);
  --motion-hover: 140ms;
  --motion-press: 120ms;
  --motion-view: 240ms;
  --motion-panel: 280ms;

  accent-color: var(--accent-orange);
}

/* Studio dark theme. Scoped to this document via html[data-theme="dark"]; the landing page does not load these styles. */
html[data-theme="dark"] {
  color-scheme: dark;

  --bg-bone: #141311;
  --bg-paper: #1B1A17;
  --bg-card: #242320;
  --surface-white: #242320;
  --paper-2: #2E2C28;

  --rule-fine: rgba(244, 241, 234, 0.10);
  --rule-strong: rgba(244, 241, 234, 0.18);
  --line: #34322C;
  --line-strong: #4E4A43;
  --line-soft: rgba(244, 241, 234, 0.07);

  --text-ink: #F4F1EA;
  --text-ink-secondary: #D2CDC3;
  --text-muted: #A8A296;
  --text-subtle: #8C877D;

  --orange-text: #FF8A5C;
  --orange-dark: #FF8A5C;
  --orange-tint: rgba(255, 90, 31, 0.16);
  --orange-line: rgba(255, 90, 31, 0.42);
  --accent-orange-subtle: rgba(255, 90, 31, 0.16);
  --accent-orange-glow: rgba(255, 90, 31, 0.28);
  --orange-tint-solid: #3A241C;
  --orange-flash: rgba(255, 90, 31, 0.22);
  --text-selection: rgba(255, 138, 92, 0.28);
  --color-accent-subtle: rgba(255, 90, 31, 0.16);

  --accent-lilac-text: #C4B5FD;
  --accent-lilac-subtle: rgba(196, 181, 253, 0.16);
  --accent-amber: #E0B15A;
  --accent-amber-subtle: rgba(224, 177, 90, 0.16);

  --status-ok: #6FCF97;
  --status-ok-subtle: rgba(111, 207, 151, 0.14);
  --status-warn: #E0B15A;
  --status-warn-subtle: rgba(224, 177, 90, 0.16);
  --status-err: #F07167;
  --status-err-subtle: rgba(240, 113, 103, 0.14);
  --status-ok-line: rgba(111, 207, 151, 0.32);
  --status-warn-line: rgba(224, 177, 90, 0.32);
  --status-err-line: rgba(240, 113, 103, 0.32);

  --teal-text: #5DCFC4;
  --teal-600: #5DCFC4;
  --teal-700: #3DB8AE;
  --teal-tint: rgba(93, 207, 196, 0.14);
  --teal-line: rgba(93, 207, 196, 0.36);

  --ink-wash-1: rgba(244, 241, 234, 0.04);
  --ink-wash-2: rgba(244, 241, 234, 0.07);
  --ink-wash-3: rgba(244, 241, 234, 0.11);
  --scrim: rgba(0, 0, 0, 0.62);
  --shadow-overlay: 0 20px 48px rgba(0, 0, 0, 0.45);
  --shadow-drawer: -8px 0 24px rgba(0, 0, 0, 0.35);
  --shadow-raised: 0 1px 3px rgba(0, 0, 0, 0.40);
  --shadow-hover: 0 2px 8px rgba(0, 0, 0, 0.35);
  --shadow-sm: 0 1px 2px rgba(0, 0, 0, 0.35);
  --shadow-md: 0 4px 12px rgba(0, 0, 0, 0.40);
  --shadow-lg: 0 12px 28px rgba(0, 0, 0, 0.50);

  --surface-input: #1E1D1A;
  --bg-input: #1E1D1A;

  --graph-canvas: #121110;
  --graph-node: #2C2B27;
  --graph-node-border: #B7B2A8;
  --graph-derivation-tint: #1A3330;
  --graph-selected-glow: rgba(255, 90, 31, 0.28);
  --graph-hover-shadow: rgba(0, 0, 0, 0.40);
  --graph-node-shadow: rgba(0, 0, 0, 0.45);
  --graph-lilac-tint: #2A2540;
  --graph-unknown-fill: #3A3020;

  --syntax-identifier: #9EBEF2;
  --syntax-function: #F0A0B4;
  --syntax-number: #E0B15A;
  --syntax-identifier-tint: rgba(158, 190, 242, 0.14);
  --syntax-function-tint: rgba(240, 160, 180, 0.14);
  --syntax-number-tint: rgba(224, 177, 90, 0.14);
  --source-retained-bg: rgba(255, 90, 31, 0.16);
}

html[data-theme="dark"] .brand-mark {
  background: #F4F1EA;
  border-radius: 4px;
  padding: 2px;
}

/* Experimental forced-black theme. Separate from the charcoal dark theme above. */
html[data-theme="black"] {
  color-scheme: dark;

  --bg-bone: #000000;
  --bg-paper: #000000;
  --bg-card: #000000;
  --surface-white: #000000;
  --paper-2: #161616;

  --rule-fine: rgba(255, 255, 255, 0.14);
  --rule-strong: rgba(255, 255, 255, 0.28);
  --line: #3A3A3A;
  --line-strong: #6A6A6A;
  --line-soft: rgba(255, 255, 255, 0.10);

  --text-ink: #F7F7F5;
  --text-ink-secondary: #E2E0DA;
  --text-muted: #B7B3AA;
  --text-subtle: #9A968E;

  --orange-text: #FF9A6A;
  --orange-dark: #FF9A6A;
  --orange-tint: rgba(255, 90, 31, 0.20);
  --orange-line: rgba(255, 154, 106, 0.55);
  --accent-orange-subtle: rgba(255, 90, 31, 0.20);
  --accent-orange-glow: rgba(255, 90, 31, 0.30);
  --orange-tint-solid: #2A120C;
  --orange-flash: rgba(255, 90, 31, 0.28);
  --text-selection: rgba(255, 154, 106, 0.32);
  --color-accent-subtle: rgba(255, 90, 31, 0.20);

  --accent-lilac-text: #D2C4FF;
  --accent-lilac-subtle: rgba(210, 196, 255, 0.18);
  --accent-amber: #F0C36A;
  --accent-amber-subtle: rgba(240, 195, 106, 0.18);

  --status-ok: #7DDBA4;
  --status-ok-subtle: rgba(125, 219, 164, 0.16);
  --status-warn: #F0C36A;
  --status-warn-subtle: rgba(240, 195, 106, 0.16);
  --status-err: #FF8B82;
  --status-err-subtle: rgba(255, 139, 130, 0.16);
  --status-ok-line: rgba(125, 219, 164, 0.40);
  --status-warn-line: rgba(240, 195, 106, 0.40);
  --status-err-line: rgba(255, 139, 130, 0.40);

  --teal-text: #7EE0D6;
  --teal-600: #7EE0D6;
  --teal-700: #5DCFC4;
  --teal-tint: rgba(126, 224, 214, 0.16);
  --teal-line: rgba(126, 224, 214, 0.42);

  --ink-wash-1: rgba(255, 255, 255, 0.05);
  --ink-wash-2: rgba(255, 255, 255, 0.08);
  --ink-wash-3: rgba(255, 255, 255, 0.12);
  --scrim: rgba(0, 0, 0, 0.72);
  --surface-input: #000000;
  --bg-input: #000000;

  --graph-canvas: #000000;
  --graph-node: #1A1A1A;
  --graph-node-border: #D4D4D4;
  --graph-edge: #E4E4E4;
  --graph-derivation-tint: #102E2C;
  --graph-selected-glow: rgba(255, 90, 31, 0.34);
  --graph-hover-shadow: rgba(0, 0, 0, 0.55);
  --graph-node-shadow: rgba(0, 0, 0, 0.65);
  --graph-lilac-tint: #241F38;
  --graph-unknown-fill: #3A2C12;

  --syntax-identifier: #B7D4FF;
  --syntax-function: #FFB3C4;
  --syntax-number: #F0C36A;
  --syntax-identifier-tint: rgba(183, 212, 255, 0.16);
  --syntax-function-tint: rgba(255, 179, 196, 0.16);
  --syntax-number-tint: rgba(240, 195, 106, 0.16);
  --source-retained-bg: rgba(255, 90, 31, 0.20);
}

html[data-theme="black"] .brand-mark {
  background: #F4F1EA;
  border-radius: 4px;
  padding: 2px;
}
html[data-theme="black"] .svg-node rect.node-col,
html[data-theme="black"] .svg-node rect.node-stmt,
html[data-theme="black"] .svg-node rect.node-expr {
  fill: var(--graph-node);
}

::selection {
  background-color: var(--text-selection);
  color: inherit;
}

/* Base Reset & Typography */
*, *::before, *::after {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
}

body {
  font-family: var(--font-sans);
  background: var(--paper-0);
  color: var(--ink);
  font-size: 13px;
  line-height: 1.45;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  overflow: hidden;
  height: 100vh;
  width: 100vw;
}

/* Buttons */
.btn {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  height: var(--btn-h-md);
  padding: 0 14px;
  font-family: var(--font-mono);
  font-size: 11.5px;
  font-weight: 600;
  letter-spacing: 0.02em;
  border-radius: var(--r-pill);
  border: 1px solid var(--line);
  background: var(--surface-white);
  color: var(--ink);
  cursor: pointer;
  transition: background-color var(--motion-hover) var(--ease-out),
              border-color var(--motion-hover) var(--ease-out),
              color var(--motion-hover) var(--ease-out),
              box-shadow var(--motion-hover) var(--ease-out),
              transform var(--motion-press) var(--ease-out);
  user-select: none;
  text-decoration: none;
  white-space: nowrap;
}
.btn:hover {
  background: var(--paper-1);
  border-color: var(--line-strong);
}
.btn:active:not(:disabled) {
  transform: scale(0.97);
}
.btn:focus-visible {
  outline: none;
  box-shadow: var(--focus);
}
.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
.btn-primary {
  background: var(--orange);
  border-color: var(--orange);
  color: var(--text-inverse);
}
.btn-primary:hover {
  background: var(--color-accent-primary-hover);
  border-color: var(--color-accent-primary-hover);
  color: var(--text-inverse);
  box-shadow: 0 3px 12px var(--accent-orange-glow);
}
.btn-secondary {
  background: transparent;
  border-color: var(--rule-strong);
  color: var(--ink);
}
.btn-secondary:hover {
  background: var(--ink-wash-2);
  border-color: var(--ink);
  color: var(--ink);
}
.btn-ghost {
  background: transparent;
  border: 1px solid transparent;
  color: var(--ink-3);
}
.btn-ghost:hover {
  background: var(--ink-wash-2);
  color: var(--ink);
}
.btn-ghost.active {
  background: var(--ink-wash-3);
  border-color: var(--rule-fine);
  color: var(--ink);
}
.btn-sm {
  height: var(--btn-h-sm);
  padding: 0 10px;
  font-size: 10.5px;
}
.btn-lg {
  height: var(--h-lg);
  padding: 0 20px;
  font-size: 12px;
  letter-spacing: 0.04em;
}

/* Dropdowns & Inputs */
.select-dropdown {
  height: var(--btn-h-md);
  padding: 0 30px 0 12px;
  font-size: 11.5px;
  border: 1px solid var(--line);
  border-radius: var(--r-sm);
  background-color: var(--surface-white);
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='10' height='6' viewBox='0 0 10 6'%3E%3Cpath d='M1 1l4 4 4-4' fill='none' stroke='%2377736B' stroke-width='1.5' stroke-linecap='round' stroke-linejoin='round'/%3E%3C/svg%3E");
  background-repeat: no-repeat;
  background-position: right 11px center;
  background-size: 10px 6px;
  -webkit-appearance: none;
  appearance: none;
  color: var(--ink);
  outline: none;
  cursor: pointer;
  font-family: var(--font-mono);
  transition: border-color var(--motion-hover) var(--ease-out), box-shadow var(--motion-hover) var(--ease-out);
}
.select-dropdown:hover {
  border-color: var(--line-strong);
}
.select-dropdown:focus-visible {
  border-color: var(--orange);
  box-shadow: 0 0 0 3px var(--focus-ring);
}
html[data-theme="dark"] .select-dropdown {
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='10' height='6' viewBox='0 0 10 6'%3E%3Cpath d='M1 1l4 4 4-4' fill='none' stroke='%23A8A296' stroke-width='1.5' stroke-linecap='round' stroke-linejoin='round'/%3E%3C/svg%3E");
}
html[data-theme="black"] .select-dropdown {
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='10' height='6' viewBox='0 0 10 6'%3E%3Cpath d='M1 1l4 4 4-4' fill='none' stroke='%23B7B3AA' stroke-width='1.5' stroke-linecap='round' stroke-linejoin='round'/%3E%3C/svg%3E");
}

/* Pills & Badges */
.pill {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  font-size: 10px;
  font-family: var(--font-mono);
  font-weight: 600;
  padding: 2px 7px;
  border-radius: var(--r-pill);
  text-transform: uppercase;
  letter-spacing: 0.04em;
}
.pill-neutral {
  background: var(--ink-wash-2);
  color: var(--ink-2);
  border: 1px solid var(--rule-fine);
}
.pill-full, .pill-keep {
  background: var(--status-ok-subtle);
  color: var(--ok);
  border: 1px solid var(--status-ok-line);
}
.pill-drop {
  background: var(--status-err-subtle);
  color: var(--err);
  border: 1px solid var(--status-err-line);
}
.pill-partial {
  background: var(--status-warn-subtle);
  color: var(--warn);
  border: 1px solid var(--status-warn-line);
}

/* ========================================================================= */
/* CANONICAL LOAD PROGRAM MODAL DIALOG                                       */
/* ========================================================================= */
.load-modal-backdrop {
  position: fixed;
  inset: 0;
  z-index: 9500;
  background: var(--scrim);
  backdrop-filter: blur(4px);
  -webkit-backdrop-filter: blur(4px);
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 24px;
  opacity: 0;
  pointer-events: none;
  transition: opacity var(--motion-view) var(--ease-out);
}
.load-modal-backdrop.open {
  opacity: 1;
  pointer-events: auto;
}
.load-program-dialog {
  background: var(--surface-white);
  border: 1px solid var(--line);
  border-radius: var(--r-md);
  box-shadow: var(--shadow-overlay);
  width: 100%;
  max-width: 780px;
  max-height: 90vh;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  opacity: 0;
  transform: translateY(10px);
  transition: transform var(--motion-panel) var(--ease-editorial), opacity var(--motion-view) var(--ease-out);
}
.load-modal-backdrop.open .load-program-dialog {
  opacity: 1;
  transform: translateY(0);
}
.load-modal-header {
  padding: 14px 20px;
  border-bottom: 1px solid var(--line);
  background: var(--bg-paper);
  display: flex;
  align-items: center;
  justify-content: space-between;
}
.load-header-brand {
  display: flex;
  align-items: center;
  gap: 10px;
}
.load-wordmark {
  font-family: var(--font-sans);
  font-weight: 700;
  font-size: 15px;
  letter-spacing: -0.02em;
  color: var(--ink);
}
.load-badge-label {
  font-family: var(--font-mono);
  font-size: 10px;
  font-weight: 600;
  letter-spacing: 0.08em;
  color: var(--orange-text);
  background: var(--orange-tint);
  border: 1px solid var(--orange-line);
  padding: 2px 7px;
  border-radius: var(--r-xs);
  text-transform: uppercase;
}
.load-close-btn {
  width: 30px;
  height: 30px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: 1px solid var(--rule-fine);
  border-radius: var(--r-pill);
  color: var(--ink-3);
  font-size: 13px;
  line-height: 1;
  cursor: pointer;
  transition: background-color var(--motion-hover) var(--ease-out),
              color var(--motion-hover) var(--ease-out),
              border-color var(--motion-hover) var(--ease-out);
}
.load-close-btn:hover {
  background: var(--ink-wash-2);
  color: var(--ink);
  border-color: var(--rule-strong);
}
.load-close-btn:focus-visible {
  outline: none;
  box-shadow: var(--focus);
}
.load-modal-body {
  padding: 24px 24px 28px;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 14px;
  background: var(--surface-white);
}
.load-dialog-intro {
  display: flex;
  flex-direction: column;
  gap: 4px;
}
.load-program-title {
  font-family: var(--font-sans);
  font-size: 18px;
  font-weight: 700;
  color: var(--ink);
  margin: 0;
  letter-spacing: -0.01em;
}
.load-program-subtitle {
  font-size: 12.5px;
  color: var(--ink-3);
  margin: 0;
  line-height: 1.5;
}
.load-code-box {
  background: var(--paper-0);
  border: 1px solid var(--line);
  border-radius: var(--r-sm);
  display: flex;
  flex-direction: column;
  overflow: hidden;
  transition: border-color var(--motion-hover) var(--ease-out), box-shadow var(--motion-hover) var(--ease-out);
}
.load-code-box:focus-within {
  border-color: var(--orange);
  box-shadow: 0 0 0 3px var(--focus-ring);
}
.load-code-topbar {
  padding: 8px 12px;
  background: var(--bg-paper);
  border-bottom: 1px solid var(--line);
  display: flex;
  justify-content: space-between;
  align-items: center;
}
.load-buffer-title {
  font-family: var(--font-mono);
  font-size: 10px;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.06em;
  color: var(--ink-2);
}
.load-buffer-stats {
  font-family: var(--font-mono);
  font-size: 10.5px;
  color: var(--ink-3);
}
.load-code-textarea {
  width: 100%;
  height: 220px;
  padding: 12px 14px;
  background: var(--surface-white);
  border: none;
  outline: none;
  font-family: var(--font-mono);
  font-size: 12.5px;
  line-height: 1.6;
  color: var(--ink);
  caret-color: var(--orange);
  resize: vertical;
  tab-size: 2;
  box-sizing: border-box;
}
.load-code-textarea::placeholder {
  color: var(--ink-4);
}
.play-icon {
  font-size: 9px;
}
.load-code-footer {
  padding: 6px 12px;
  background: var(--bg-paper);
  border-top: 1px solid var(--line);
  display: flex;
  justify-content: flex-end;
}
.load-hint {
  font-family: var(--font-mono);
  font-size: 10.5px;
  color: var(--ink-3);
}
.load-hint kbd {
  background: var(--paper-2);
  border: 1px solid var(--line);
  border-radius: var(--r-xs);
  padding: 1px 4px;
  font-size: 10px;
  color: var(--ink-2);
}
.load-actions-row {
  display: flex;
  align-items: center;
  gap: 10px;
}
.validation-msg {
  padding: 10px 14px;
  border-radius: var(--r-sm);
  font-size: 11.5px;
  line-height: 1.4;
  background: var(--status-err-subtle);
  color: var(--status-err);
  border: 1px solid var(--status-err-line);
  border-left: 4px solid var(--status-err);
  display: flex;
  align-items: center;
  gap: 8px;
}

/* ========================================================================= */
/* PHASE 2 & 3: Consolidated Unified Master Workspace Layout                */
/* ========================================================================= */
.workspace-screen {
  display: flex;
  flex-direction: column;
  width: 100vw;
  height: 100vh;
  overflow: hidden;
  position: relative;
  background: var(--paper-0);
  color: var(--ink);
  font-family: var(--font-sans);
}
.workspace-screen.hidden {
  display: none !important;
}

/* 3.1 Top Bar (56px) */
.app-header {
  height: var(--topbar-height);
  background: var(--bg-bone);
  border-bottom: 1px solid var(--rule-fine);
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 16px;
  flex-shrink: 0;
  z-index: 50;
  box-sizing: border-box;
}
.header-top-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
  height: 100%;
  gap: 12px;
}
.header-identity {
  display: flex;
  align-items: center;
  gap: 10px;
  flex-shrink: 0;
}
.header-identity .brand-mark {
  width: 22px;
  height: 22px;
  display: block;
  object-fit: contain;
  flex-shrink: 0;
  margin-right: -2px;
}
.header-identity .brand-logo {
  font-family: var(--font-sans);
  font-weight: 700;
  font-size: 16px;
  letter-spacing: -0.02em;
  color: var(--ink);
}
.header-center-info {
  display: flex;
  align-items: center;
  gap: 10px;
  flex-shrink: 0;
}
.status-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: var(--ok);
  display: inline-block;
  flex-shrink: 0;
  transition: background-color var(--motion-view) var(--ease-out);
}
.status-dot.error { background: var(--err); }
.status-dot.uncertain { background: var(--warn); }
.status-dot.analyzing {
  background: var(--orange);
  animation: rp-status-busy 1s ease-in-out infinite alternate;
}
@keyframes rp-status-busy {
  from { opacity: 1; }
  to { opacity: 0.35; }
}
.header-meta-group {
  display: none;
}
.stats-strip {
  display: none;
}

/* Command Palette Trigger in Top Bar */
.search-pill-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  height: var(--btn-h-md);
  background: var(--ink-wash-1);
  border: 1px solid var(--rule-fine);
  border-radius: var(--r-pill);
  padding: 0 14px 0 12px;
  font-family: var(--font-sans);
  font-size: 12px;
  color: var(--ink-3);
  cursor: pointer;
  transition: background-color var(--motion-hover) var(--ease-out),
              border-color var(--motion-hover) var(--ease-out),
              color var(--motion-hover) var(--ease-out);
}
.search-pill-btn:hover {
  background: var(--surface-white);
  border-color: var(--rule-strong);
  color: var(--ink);
}
.search-pill-btn:focus-visible {
  outline: none;
  box-shadow: var(--focus);
}
.search-pill-btn .search-icon {
  display: inline-flex;
  color: var(--ink-4);
}
.header-actions-row {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-shrink: 0;
}
.header-nav-link {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  font-family: var(--font-mono);
  font-size: 11px;
  font-weight: 500;
  letter-spacing: 0.04em;
  color: var(--ink-3);
  text-decoration: none;
  padding: 6px 10px;
  border-radius: var(--r-pill);
  transition: background-color var(--motion-hover) var(--ease-out), color var(--motion-hover) var(--ease-out);
}
.header-nav-link:hover {
  background: var(--ink-wash-2);
  color: var(--ink);
}
.header-nav-link:focus-visible {
  outline: none;
  box-shadow: var(--focus);
}
.theme-toggle {
  width: 32px;
  height: 32px;
  padding: 0;
  justify-content: center;
  color: var(--ink-3);
  flex-shrink: 0;
}
.theme-toggle .theme-icon-sun { display: none; }
html[data-theme="dark"] .theme-toggle .theme-icon-moon { display: none; }
html[data-theme="dark"] .theme-toggle .theme-icon-sun { display: block; }
.theme-toggle.active {
  color: var(--ink);
  border-color: var(--rule-strong);
}

/* 3-Column Working Workspace Container */
.workspace-main {
  display: flex;
  flex-direction: row;
  height: calc(100vh - var(--topbar-height) - var(--nav-height));
  width: 100%;
  overflow: hidden;
  position: relative;
}

/* Horizontal Feature Navigation Row (Section 3: Directly below Header) */
.feature-nav-bar {
  height: var(--nav-height);
  background: var(--bg-bone);
  border-bottom: 1px solid var(--rule-fine);
  display: flex;
  align-items: stretch;
  padding: 0 12px;
  flex-shrink: 0;
  z-index: 45;
  box-sizing: border-box;
  overflow-x: auto;
  overflow-y: hidden;
}
.feature-tabs-list {
  display: flex;
  align-items: stretch;
  gap: 2px;
  height: 100%;
}
.nav-tab {
  position: relative;
  height: 100%;
  padding: 0 14px;
  background: transparent;
  border: none;
  border-bottom: 2px solid var(--accent-blue); /* Retained for test assertion */
  border-color: transparent;
  border-radius: 0;
  color: var(--ink-3);
  cursor: pointer;
  font-family: var(--font-sans);
  font-size: 12.5px;
  font-weight: 600;
  display: flex;
  align-items: center;
  gap: 8px;
  transition: color var(--motion-hover) var(--ease-out), background-color var(--motion-hover) var(--ease-out);
  white-space: nowrap;
  outline: none;
  box-sizing: border-box;
}
.nav-tab::after {
  content: "";
  position: absolute;
  left: 10px;
  right: 10px;
  bottom: -2px;
  height: 2px;
  border-radius: 2px 2px 0 0;
  background: var(--accent-orange);
  transform: scaleX(0);
  transition: transform var(--motion-view) var(--ease-editorial);
}
.nav-tab:hover {
  background: var(--ink-wash-1);
  color: var(--ink);
}
.nav-tab:focus-visible {
  box-shadow: inset 0 0 0 2px var(--orange-line);
}
.nav-tab.active {
  background: transparent;
  color: var(--ink);
}
.nav-tab.active::after {
  transform: scaleX(1);
}
.nav-tab-icon {
  width: 15px;
  height: 15px;
  stroke: currentColor;
  flex-shrink: 0;
  transition: color var(--motion-hover) var(--ease-out);
}
.nav-tab.active .nav-tab-icon {
  color: var(--accent-orange);
}
.nav-tab-label {
  white-space: nowrap;
}
.nav-count {
  font-family: var(--font-mono);
  font-size: 10px;
  font-weight: 500;
  color: var(--ink-4);
  background: var(--ink-wash-1);
  border: 1px solid transparent;
  border-radius: var(--r-pill);
  padding: 1px 6px;
  transition: color var(--motion-hover) var(--ease-out), background-color var(--motion-hover) var(--ease-out);
}
.nav-tab.active .nav-count {
  background: var(--orange-tint);
  color: var(--orange-text);
}
.nav-count.count-zero, .nav-count:empty {
  display: none !important;
}

/* 2.1 Left Panel — Source Editor (34% width / 380-460px) */
.pane-source {
  width: 34%;
  min-width: 320px;
  max-width: 600px;
  height: 100%;
  background: var(--surface-white);
  border-right: 1px solid var(--line);
  display: flex;
  flex-direction: column;
  position: relative;
  overflow: hidden;
  flex-shrink: 0;
  box-sizing: border-box;
}
.pane-header {
  height: 38px;
  padding: 0 10px 0 14px;
  background: var(--bg-paper);
  border-bottom: 1px solid var(--line);
  display: flex;
  align-items: center;
  justify-content: space-between;
  font-size: 11px;
  font-weight: 600;
  color: var(--ink-3);
  flex-shrink: 0;
}

/* Resizable Column Splitter between Source and Center */
.col-splitter {
  width: 6px;
  margin-left: -3px;
  margin-right: -3px;
  background: transparent;
  cursor: col-resize;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  user-select: none;
  transition: background-color var(--motion-hover) var(--ease-out);
  z-index: 20;
  outline: none;
}
.col-splitter:hover, .col-splitter.active, .col-splitter:focus-visible {
  background: var(--orange-line);
}
.col-splitter-handle {
  width: 2px;
  height: 28px;
  border-radius: var(--r-pill);
  background: var(--line-strong);
  transition: background-color var(--motion-hover) var(--ease-out);
}
.col-splitter:hover .col-splitter-handle, .col-splitter.active .col-splitter-handle, .col-splitter:focus-visible .col-splitter-handle {
  background: var(--orange);
}

/* 2.2 Center Panel — Active Analysis View */
.central-stage {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-width: 0;
  height: 100%;
  overflow: hidden;
  background: var(--graph-canvas);
  position: relative;
}
.analysis-view-stage {
  flex: 1;
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow: hidden;
  position: relative;
}
.pane-title {
  font-family: var(--font-mono);
  text-transform: uppercase;
  font-size: 10.5px;
  letter-spacing: 0.08em;
  color: var(--ink-2);
  font-weight: 600;
}
.pane-title-toggle {
  appearance: none;
  background: transparent;
  border: none;
  padding: 0;
  margin: 0;
  cursor: pointer;
  font: inherit;
  letter-spacing: inherit;
  text-transform: inherit;
  color: inherit;
}
.pane-title-toggle:hover {
  color: var(--ink);
}
.pane-title-toggle:focus-visible {
  outline: none;
  box-shadow: var(--focus);
  border-radius: var(--r-xs);
}
.pane-icon-btn {
  width: 28px;
  height: 28px;
  padding: 0;
  justify-content: center;
  gap: 0;
}
.pane-icon-btn .icon-view {
  display: none;
}
.pane-icon-btn.is-editing .icon-edit {
  display: none;
}
.pane-icon-btn.is-editing .icon-view {
  display: block;
}
.pane-source.source-collapsed {
  width: 36px;
  min-width: 36px;
  max-width: 36px;
}
.pane-source.source-collapsed .pane-header {
  height: 100%;
  padding: 14px 0;
  justify-content: flex-start;
}
.pane-source.source-collapsed .pane-header > div:first-child {
  height: 100%;
  width: 100%;
  justify-content: center;
}
.source-rail-chevron {
  display: none;
  width: 12px;
  height: 12px;
  flex-shrink: 0;
}
.pane-source.source-collapsed .source-rail-chevron {
  display: block;
}
.pane-source.source-collapsed .pane-title-toggle {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 12px;
  height: 100%;
  width: 100%;
}
.pane-source.source-collapsed .pane-title-label {
  writing-mode: vertical-rl;
  transform: rotate(180deg);
}
.pane-source.source-collapsed .pane-badge,
.pane-source.source-collapsed .pane-header > div:last-child,
.pane-source.source-collapsed .source-stale-banner,
.pane-source.source-collapsed .source-status-banner,
.pane-source.source-collapsed .source-viewer,
.pane-source.source-collapsed .source-editor-textarea {
  display: none !important;
}
.workspace-main:has(.pane-source.source-collapsed) .col-splitter {
  display: none;
}
.pane-badge {
  font-family: var(--font-mono);
  font-size: 10.5px;
  color: var(--ink-4);
}
.source-viewer {
  flex: 1;
  overflow: auto;
  scrollbar-width: none;
  background: linear-gradient(to right, var(--bg-paper) 47px, var(--surface-white) 47px) local;
  font-family: var(--font-mono);
  font-size: 13px;
  line-height: 22px;
  padding: 6px 0;
}
.source-viewer::-webkit-scrollbar,
.source-editor-textarea::-webkit-scrollbar {
  width: 0;
  height: 0;
  display: none;
}
.source-editor-textarea {
  display: none;
  flex: 1;
  border: none;
  outline: none;
  background: var(--surface-white);
  padding: 10px 14px;
  font-family: var(--font-mono);
  font-size: 13px;
  line-height: 22px;
  color: var(--ink);
  caret-color: var(--orange);
  resize: none;
  scrollbar-width: none;
  box-shadow: inset 2px 0 0 var(--orange);
}
.source-stale-banner {
  display: none;
  padding: 6px 12px;
  background: var(--status-warn-subtle);
  border-bottom: 1px solid var(--status-warn-line);
  font-size: 11px;
  color: var(--status-warn);
  align-items: center;
  justify-content: space-between;
}
.stale-banner-label {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  min-width: 0;
}
.source-status-banner {
  display: none;
  padding: 6px 12px;
  border-bottom: 1px solid var(--line);
  font-size: 11px;
  align-items: center;
  justify-content: space-between;
}

/* 3.4 Right Inspector (320px column) */
.workspace-inspector {
  width: var(--inspector-width);
  min-width: 280px;
  max-width: 420px;
  background: var(--surface-white);
  border-left: 1px solid var(--line);
  display: flex;
  flex-direction: column;
  flex-shrink: 0;
  overflow: hidden;
  z-index: 10;
  transition: width 0.16s ease;
}
.workspace-inspector.collapsed {
  width: 0 !important;
  min-width: 0 !important;
  display: none !important;
}
.inspector-header {
  height: 38px;
  padding: 0 10px 0 14px;
  background: var(--bg-paper);
  border-bottom: 1px solid var(--line);
  display: flex;
  align-items: center;
  justify-content: space-between;
  flex-shrink: 0;
}
.inspector-title-group {
  display: flex;
  align-items: center;
  gap: 6px;
}
.inspector-content {
  flex: 1;
  overflow-y: auto;
  padding: 16px;
}
.inspector-content > * {
  animation: rp-reveal var(--motion-panel) var(--ease-editorial);
}
.inspector-section {
  margin-top: 16px;
  padding-top: 14px;
  border-top: 1px solid var(--line);
}
.inspector-section-title {
  font-family: var(--font-mono);
  font-size: 10px;
  font-weight: 600;
  text-transform: uppercase;
  color: var(--ink-3);
  letter-spacing: 0.08em;
  margin-bottom: 8px;
  display: flex;
  justify-content: space-between;
  align-items: center;
}
.card {
  background: var(--surface-white);
  border: 1px solid var(--line);
  border-radius: var(--r-sm);
  padding: 12px;
  margin-bottom: 10px;
}
.inspector-content > .card {
  border: none;
  border-radius: 0;
  padding: 0;
  margin-bottom: 0;
  background: transparent;
}
.prop-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 11px;
}
.prop-table td {
  padding: 5px 0;
  vertical-align: top;
  border-bottom: 1px solid var(--line-soft);
}
.prop-table tr:last-child td {
  border-bottom: none;
}
.prop-table .key {
  color: var(--ink-3);
  width: 40%;
  font-size: 11px;
  padding-right: 8px;
}
.prop-table .val {
  color: var(--ink);
  font-family: var(--font-mono);
  word-break: break-all;
}
.conn-list {
  display: flex;
  flex-direction: column;
  gap: 4px;
}
.conn-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
  padding: 5px 8px;
  background: var(--paper-0);
  border: 1px solid transparent;
  border-radius: var(--r-xs);
  cursor: pointer;
  font-size: 11px;
  transition: background-color var(--motion-hover) var(--ease-out),
              border-color var(--motion-hover) var(--ease-out),
              transform var(--motion-hover) var(--ease-out);
}
.conn-item:hover {
  background: var(--orange-tint);
  border-color: var(--orange-line);
  transform: translateX(2px);
}
.conn-label {
  font-family: var(--font-mono);
  color: var(--ink-2);
}
.conn-pill {
  font-family: var(--font-mono);
  font-size: 9.5px;
  padding: 1px 6px;
  border-radius: var(--r-pill);
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.04em;
  flex-shrink: 0;
}
.inspector-content details > summary {
  list-style-position: inside;
  font-family: var(--font-mono);
  letter-spacing: 0.08em;
  transition: color var(--motion-hover) var(--ease-out);
}
.inspector-content details > summary:hover {
  color: var(--ink) !important;
}
.inspector-content details[open] > .raw-json {
  animation: rp-reveal var(--motion-view) var(--ease-editorial);
}
.badge-count {
  font-family: var(--font-mono);
  font-size: 10px;
  color: var(--ink-4);
}
.raw-json {
  font-family: var(--font-mono);
  font-size: 10px;
  background: var(--paper-0);
  border: 1px solid var(--line);
  border-radius: var(--r-xs);
  padding: 8px;
  overflow: auto;
  max-height: 180px;
  color: var(--ink-2);
}

/* Word Wrap Test Requirements (Preserved exactly) */
#source-pane.word-wrap-enabled .source-viewer {
  white-space: normal;
}
#source-pane.word-wrap-enabled .line-text {
  white-space: pre-wrap;
  overflow-wrap: anywhere;
  word-break: break-word;
}
#source-pane:not(.word-wrap-enabled) .source-viewer {
  white-space: nowrap;
}
#source-pane:not(.word-wrap-enabled) .line-text {
  white-space: pre;
  overflow-wrap: normal;
  word-break: normal;
}

/* Source Line Gutter & Retained Slicing Colors */
.source-line, .code-line {
  display: flex;
  align-items: flex-start;
  font-family: var(--font-mono);
  font-size: 13px;
  line-height: 22px;
  min-height: 22px;
  cursor: pointer;
  border-left: 3px solid transparent;
  transition: background-color var(--motion-hover) var(--ease-out), border-left-color var(--motion-hover) var(--ease-out);
  width: 100%;
  box-sizing: border-box;
}
.source-line:hover, .code-line:hover {
  background: var(--bg-paper);
}
.source-line.line-selected, .code-line.active-line, .code-line.line-selected {
  background: var(--source-retained-bg) !important;
  border-left-color: var(--orange) !important;
}
.code-line.highlighted-span {
  background: var(--accent-orange-subtle) !important;
  border-left-color: var(--orange) !important;
}
.source-line.line-selected, .code-line.line-selected, .code-line.highlighted-span {
  animation: rp-line-flash 700ms var(--ease-out);
}
.source-line.line-selected .line-num, .code-line.line-selected .line-num,
.code-line.active-line .line-num, .code-line.highlighted-span .line-num {
  color: var(--orange-text);
  font-weight: 600;
}
@keyframes rp-line-flash {
  from { box-shadow: inset 0 0 0 100vmax var(--orange-flash); }
  to { box-shadow: inset 0 0 0 100vmax transparent; }
}
.source-line.slice-kept, .code-line.slice-kept {
  background: var(--source-retained-bg);
  border-left-color: var(--orange);
}
.source-line.slice-partial, .code-line.slice-partial {
  background: var(--teal-tint);
  border-left-color: var(--teal-600);
}
.source-line.slice-dropped, .code-line.slice-dropped {
  color: var(--source-pruned-text);
  opacity: 0.75;
}
.line-num {
  width: 44px;
  min-width: 44px;
  max-width: 44px;
  text-align: right;
  padding-right: 10px;
  padding-left: 4px;
  margin-right: 10px;
  align-self: stretch;
  color: var(--ink-4);
  background: var(--bg-paper);
  border-right: 1px solid var(--line-soft);
  user-select: none;
  font-size: 12px;
  line-height: 22px;
  flex-shrink: 0;
  box-sizing: border-box;
  transition: color var(--motion-hover) var(--ease-out);
}
.line-text {
  flex: 1;
  min-width: 0;
  font-size: 13px;
  line-height: 22px;
  font-family: var(--font-mono);
  color: var(--ink-1);
  padding-right: 12px;
  box-sizing: border-box;
}

/* Syntax Highlighting */
.syn-ident { color: var(--syntax-identifier); }
.syn-func, .fn, .syn-fn  { color: var(--syntax-function); font-weight: 600; }
.syn-str, .str   { color: var(--syntax-string); }
.syn-num, .num   { color: var(--syntax-number); }
.syn-com, .cmt   { color: var(--syntax-comment); font-style: italic; }
.kw, .syn-kw     { color: var(--orange-dark); font-weight: 600; }

/* Toolbars */
.view-toolbar {
  min-height: 38px;
  height: auto;
  background: var(--bg-paper);
  border-bottom: 1px solid var(--line);
  padding: 5px 12px;
  display: flex;
  align-items: center;
  align-content: center;
  justify-content: space-between;
  flex-shrink: 0;
  flex-wrap: wrap;
  column-gap: 10px;
  row-gap: 6px;
}
.toolbar-group {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;
  min-width: 0;
  row-gap: 6px;
}
.view-toolbar > .toolbar-group:first-child {
  flex: 1 1 220px;
}
.view-toolbar > .toolbar-group:last-child,
.view-toolbar > .zoom-cluster {
  flex: 0 1 auto;
  max-width: 100%;
}
.toolbar-actions {
  flex-wrap: nowrap;
  flex-shrink: 0;
}
.pdg-neighborhood-btn {
  height: 28px;
  padding: 0 10px;
  font-size: 10.5px;
  letter-spacing: 0.04em;
}
.zoom-cluster {
  display: inline-flex;
  align-items: center;
  height: 28px;
  padding: 2px;
  gap: 1px;
  flex-shrink: 0;
  background: var(--surface-white);
  border: 1px solid var(--line);
  border-radius: var(--r-pill);
}
.zoom-btn,
.zoom-fit {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  height: 22px;
  border: none;
  background: transparent;
  color: var(--ink-3);
  border-radius: var(--r-pill);
  cursor: pointer;
  font-family: var(--font-mono);
  font-size: 10.5px;
  font-weight: 600;
  letter-spacing: 0.06em;
  text-transform: uppercase;
}
.zoom-btn {
  width: 24px;
  color: var(--ink-2);
}
.zoom-fit {
  gap: 5px;
  padding: 0 9px 0 7px;
}
.zoom-btn:hover,
.zoom-fit:hover {
  background: var(--ink-wash-2);
  color: var(--ink);
}
.zoom-btn:focus-visible,
.zoom-fit:focus-visible {
  outline: none;
  box-shadow: var(--focus);
}
.zoom-cluster-rule {
  width: 1px;
  height: 12px;
  background: var(--line);
  margin: 0 2px;
  flex-shrink: 0;
}
.toolbar-label {
  font-size: 11px;
  color: var(--ink-3);
  font-weight: 500;
  white-space: nowrap;
}
.filter-box {
  display: flex;
  align-items: center;
  flex: 1 1 140px;
  min-width: 120px;
  max-width: 240px;
  background: var(--surface-white);
  border: 1px solid var(--line);
  border-radius: var(--r-pill);
  padding: 2px 10px;
  height: var(--btn-h-sm);
  transition: border-color var(--motion-hover) var(--ease-out), box-shadow var(--motion-hover) var(--ease-out);
}
.filter-box:hover {
  border-color: var(--line-strong);
}
.filter-box:focus-within {
  border-color: var(--orange);
  box-shadow: 0 0 0 3px var(--focus-ring);
}
.filter-input {
  border: none;
  background: transparent;
  outline: none;
  font-size: 11.5px;
  color: var(--ink);
  font-family: inherit;
  width: 100%;
  min-width: 0;
}
.filter-input::placeholder, .ast-search-box input::placeholder, .palette-input::placeholder {
  color: var(--ink-4);
}
.filter-clear {
  border: none;
  background: transparent;
  color: var(--ink-4);
  cursor: pointer;
  transition: color var(--motion-hover) var(--ease-out);
}
.filter-clear:hover {
  color: var(--ink);
}
.toggle-chip {
  display: inline-flex;
  align-items: center;
  gap: 5px;
  height: 26px;
  padding: 0 10px 0 8px;
  border-radius: var(--r-pill);
  font-size: 11px;
  font-weight: 500;
  background: transparent;
  border: 1px solid var(--line);
  cursor: pointer;
  user-select: none;
  color: var(--ink-3);
  white-space: nowrap;
  flex-shrink: 0;
  transition: background-color var(--motion-hover) var(--ease-out),
              border-color var(--motion-hover) var(--ease-out),
              color var(--motion-hover) var(--ease-out);
}
.toggle-chip:hover {
  border-color: var(--line-strong);
  color: var(--ink);
}
.toggle-chip:has(input:checked) {
  background: var(--surface-white);
  color: var(--ink);
}
.toggle-chip:has(input:focus-visible) {
  box-shadow: var(--focus);
}
.toggle-chip input {
  cursor: pointer;
  margin: 0;
}

/* Canvases */
.canvas-viewport {
  flex: 1;
  position: relative;
  overflow: hidden;
  background: var(--paper-0);
}
.graph-canvas-container {
  width: 100%;
  height: 100%;
  position: absolute;
  inset: 0;
  overflow: hidden;
  background: var(--paper-0);
}
.graph-svg {
  width: 100%;
  height: 100%;
  cursor: grab;
}
.graph-svg:active {
  cursor: grabbing;
}

/* ========================================================================= */
/* GRAPH RENDERING & VISUALIZATION STYLES (Restrained Editorial Light Theme) */
/* ========================================================================= */
svg.graph-svg {
  width: 100%;
  height: 100%;
  display: block;
  background: var(--graph-canvas);
  fill: var(--graph-text);
}

/* SVG Node Base */
.svg-node {
  cursor: pointer;
  transition: opacity var(--motion-view) var(--ease-out);
}
.svg-node text {
  font-family: var(--font-mono);
  pointer-events: none;
}
.svg-node rect {
  fill: var(--graph-node);
  stroke: var(--graph-node-border);
  stroke-width: 1.75px;
  filter: drop-shadow(0 1px 2px var(--graph-node-shadow));
  transition: stroke var(--motion-hover) var(--ease-out),
              stroke-width var(--motion-hover) var(--ease-out),
              fill var(--motion-hover) var(--ease-out);
}
.svg-node:hover rect {
  stroke: var(--ink);
  filter: drop-shadow(0 2px 6px var(--graph-hover-shadow));
}
.svg-graph-empty {
  fill: var(--text-muted);
  font-family: var(--font-mono);
  font-size: 12px;
}

/* Specific Node Kinds (Section 4.1 Specification) */
.svg-node rect.node-col {
  fill: var(--bg-card);
  stroke: var(--graph-node-border);
  stroke-width: 1.75px;
}
.svg-node rect.node-deriv {
  fill: var(--graph-derivation-tint);
  stroke: var(--graph-derivation);
  stroke-width: 1.75px;
}
.svg-node rect.node-stmt {
  fill: var(--bg-card);
  stroke: var(--graph-node-border);
  stroke-width: 1.75px;
}
.svg-node rect.node-vardef {
  fill: var(--graph-lilac-tint);
  stroke: var(--accent-lilac-text);
  stroke-width: 1.75px;
}
.svg-node rect.node-varuse {
  fill: var(--graph-derivation-tint);
  stroke: var(--graph-derivation);
  stroke-width: 1.75px;
}
.svg-node rect.node-expr {
  fill: var(--bg-card);
  stroke: var(--graph-node-border);
  stroke-width: 1.75px;
}
.svg-node rect.node-rowsel {
  fill: var(--graph-lilac-tint);
  stroke: var(--accent-lilac-text);
  stroke-width: 1.75px;
  stroke-dasharray: 4,3;
}
.svg-node rect.node-unk {
  fill: var(--graph-unknown-fill);
  stroke: var(--graph-unknown);
  stroke-width: 1.75px;
  stroke-dasharray: 4,3;
}

/* Node Selection & Upstream/Downstream Lineage Highlights */
.svg-node.selected rect {
  fill: var(--graph-selected) !important;
  stroke: var(--accent-orange) !important;
  stroke-width: 2.5px !important;
  filter: drop-shadow(0 2px 10px var(--graph-selected-glow));
}
.svg-node.connected-upstream rect {
  stroke: var(--graph-upstream) !important;
  stroke-width: 2.25px !important;
}
.svg-node.connected-downstream rect {
  stroke: var(--graph-downstream) !important;
  stroke-width: 2.25px !important;
}
.svg-node.dimmed {
  opacity: 0.4 !important;
}

/* SVG Edges (Section 4.2 Specification) */
.svg-edge {
  fill: none;
  stroke: var(--graph-edge);
  stroke-width: 2px;
  stroke-opacity: 1;
  transition: stroke var(--motion-hover) var(--ease-out),
              stroke-width var(--motion-hover) var(--ease-out),
              opacity var(--motion-view) var(--ease-out);
}
.edge-data { stroke: var(--graph-edge); stroke-width: 2px; }
.edge-ctrl { stroke: var(--graph-edge-control); stroke-width: 2px; stroke-dasharray: 5,3; }
.edge-rowsel { stroke: var(--graph-edge-rowsel); stroke-width: 2px; stroke-dasharray: 5,3; }
.edge-colver { stroke: var(--graph-derivation); stroke-width: 2px; }
.edge-unk { stroke: var(--graph-edge-unknown); stroke-width: 2px; stroke-dasharray: 4,3; }

.svg-edge.highlighted {
  stroke-width: 2px !important;
  stroke-opacity: 1;
  opacity: 1 !important;
}
.svg-edge.highlighted.inbound {
  stroke: var(--graph-upstream) !important;
}
.svg-edge.highlighted.outbound {
  stroke: var(--graph-downstream) !important;
}
#arrow-data path, #arrow-prov-data path { fill: var(--graph-edge); }
#arrow-ctrl path { fill: var(--graph-edge-control); }
#arrow-rowsel path { fill: var(--graph-edge-rowsel); }
#arrow-colver path, #arrow-prov-colver path { fill: var(--graph-derivation); }
#arrow-unk path, #arrow-prov-unk path { fill: var(--graph-edge-unknown); }
.svg-edge.dimmed {
  opacity: 0.22 !important;
}

/* Backward Slice View */
.slice-container {
  width: 100%;
  height: 100%;
  overflow-y: auto;
  scrollbar-width: none;
  padding: 16px 20px;
  box-sizing: border-box;
  display: flex;
  flex-direction: column;
  gap: 16px;
}
.slice-container::-webkit-scrollbar {
  width: 0;
  height: 0;
  display: none;
}
.slice-summary-panel {
  background: var(--surface-white);
  border: 1px solid var(--line);
  border-radius: var(--r-sm);
  padding: 14px 16px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}
.slice-summary-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-wrap: wrap;
  gap: 8px;
}
.slice-summary-title {
  font-family: var(--font-mono);
  font-size: 10.5px;
  font-weight: 600;
  color: var(--ink-2);
  text-transform: uppercase;
  letter-spacing: 0.08em;
  display: flex;
  align-items: center;
  gap: 6px;
}
.slice-summary-badges {
  display: flex;
  align-items: center;
  gap: 8px;
}
.slice-badge {
  font-size: 10px;
  font-family: var(--font-mono);
  font-weight: 600;
  padding: 2px 8px;
  border-radius: var(--r-xs);
  text-transform: uppercase;
  letter-spacing: 0.03em;
}
.slice-badge-sound {
  background: var(--status-ok-subtle);
  color: var(--ok);
  border: 1px solid var(--status-ok-line);
}
.slice-badge-approx {
  background: var(--status-warn-subtle);
  color: var(--warn);
  border: 1px solid var(--status-warn-line);
}
.slice-badge-warn {
  background: var(--status-err-subtle);
  color: var(--err);
  border: 1px solid var(--status-err-line);
}
.slice-badge-ok {
  background: var(--status-ok-subtle);
  color: var(--ok);
  border: 1px solid var(--status-ok-line);
}
.slice-stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
  gap: 10px;
}
.slice-stat-card {
  background: var(--paper-0);
  border: 1px solid var(--line);
  border-radius: var(--r-xs);
  padding: 10px 12px;
  display: flex;
  flex-direction: column;
  gap: 4px;
}
.slice-stat-label {
  font-size: 11px;
  font-weight: 600;
  color: var(--ink-3);
}
.slice-stat-val {
  font-family: var(--font-mono);
  font-size: 18px;
  font-weight: 700;
  color: var(--ink-1);
  line-height: 1.2;
}
.slice-stat-val.val-full {
  color: var(--orange-dark);
}
.slice-stat-val.val-partial {
  color: var(--teal-700);
}
.slice-stat-val.val-none {
  color: var(--ink-4);
}
.slice-stat-val.val-unknown {
  color: var(--warn);
}
.slice-stat-val.val-elements {
  color: var(--ink-2);
}
.slice-section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 4px 2px 2px 2px;
  border-bottom: 1px solid var(--line);
}
.slice-section-title {
  font-family: var(--font-mono);
  font-size: 10.5px;
  font-weight: 600;
  color: var(--ink-2);
  text-transform: uppercase;
  letter-spacing: 0.08em;
}
.slice-section-meta {
  font-size: 11px;
  color: var(--ink-4);
  font-family: var(--font-mono);
}
.stmt-retention-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
  width: 100%;
}
.stmt-card {
  background: var(--surface-white);
  border: 1px solid var(--line);
  border-radius: var(--r-xs);
  padding: 10px 14px;
  cursor: pointer;
  transition: border-color var(--motion-hover) var(--ease-out),
              background-color var(--motion-hover) var(--ease-out),
              box-shadow var(--motion-hover) var(--ease-out),
              opacity var(--motion-hover) var(--ease-out);
  display: flex;
  flex-direction: column;
  gap: 6px;
  width: 100%;
  box-sizing: border-box;
}
.stmt-card:hover {
  border-color: var(--orange-line);
  box-shadow: var(--shadow-hover);
}
.stmt-card.full {
  background: var(--accent-orange-subtle);
  border-left: 3px solid var(--orange);
}
.stmt-card.partial {
  background: var(--teal-tint);
  border-left: 3px solid var(--teal-600);
}
.stmt-card.none {
  background: var(--surface-white);
  border-left: 3px solid var(--line);
  opacity: 0.85;
}
.stmt-card.none:hover {
  opacity: 1;
}
.stmt-card.unknown {
  background: var(--status-warn-subtle);
  border-left: 3px solid var(--warn);
}
.stmt-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 12px;
}
.stmt-header-left {
  display: flex;
  align-items: center;
  gap: 8px;
}
.stmt-header-right {
  display: flex;
  align-items: center;
}
.stmt-line-badge {
  font-family: var(--font-mono);
  font-size: 11px;
  font-weight: 700;
  color: var(--ink-2);
  background: var(--paper-1);
  padding: 2px 6px;
  border-radius: var(--r-xs);
  border: 1px solid var(--line);
}
.stmt-status-badge {
  font-family: var(--font-mono);
  font-size: 10px;
  font-weight: 700;
  letter-spacing: 0.04em;
  padding: 2px 8px;
  border-radius: var(--r-xs);
  text-transform: uppercase;
}
.stmt-status-badge.badge-full {
  background: var(--accent-orange-subtle);
  color: var(--orange-dark);
  border: 1px solid var(--orange-line);
}
.stmt-status-badge.badge-partial {
  background: var(--teal-tint);
  color: var(--teal-700);
  border: 1px solid var(--teal-line);
}
.stmt-status-badge.badge-none {
  background: var(--paper-1);
  color: var(--ink-4);
  border: 1px solid var(--line);
}
.stmt-status-badge.badge-unknown {
  background: var(--status-warn-subtle);
  color: var(--warn);
  border: 1px solid var(--status-warn-line);
}
.stmt-code {
  font-family: var(--font-mono);
  font-size: 12.5px;
  line-height: 20px;
  color: var(--ink-1);
  white-space: pre-wrap;
  overflow-wrap: anywhere;
  word-break: break-word;
  padding: 6px 8px;
  background: var(--paper-0);
  border-radius: var(--r-xs);
  border: 1px solid var(--line-soft);
}
.sub-elements-list {
  margin-top: 4px;
  padding-top: 6px;
  border-top: 1px dashed var(--line);
  display: flex;
  flex-direction: column;
  gap: 4px;
}
.sub-elements-heading {
  font-size: 10px;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.04em;
  color: var(--ink-4);
  margin-bottom: 2px;
}
.sub-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 3px 6px;
  border-radius: var(--r-xs);
  background: var(--paper-0);
  font-size: 11px;
}
.sub-item-label {
  font-family: var(--font-mono);
  color: var(--ink-2);
  word-break: break-all;
}
.sub-item-badge {
  font-family: var(--font-mono);
  font-size: 10px;
  font-weight: 700;
  padding: 1px 6px;
  border-radius: var(--r-xs);
}
.sub-item-badge.sub-keep {
  background: var(--accent-orange-subtle);
  color: var(--orange-dark);
  border: 1px solid var(--orange-line);
}
.sub-item-badge.sub-drop {
  background: var(--paper-1);
  color: var(--ink-4);
  border: 1px solid var(--line);
}

/* Schema Evolution */
.schema-timeline {
  width: 100%;
  height: 100%;
  overflow-y: auto;
  padding: 16px;
  box-sizing: border-box;
  display: flex;
  flex-direction: column;
  gap: 12px;
}
.schema-card {
  background: var(--surface-white);
  border: 1px solid var(--line);
  border-radius: var(--r-sm);
  padding: 14px;
  transition: border-color var(--motion-hover) var(--ease-out), box-shadow var(--motion-hover) var(--ease-out);
}
.schema-card:hover {
  border-color: var(--line-strong);
}
.schema-symbol {
  font-family: var(--font-mono);
  font-size: 13px;
  font-weight: 600;
  color: var(--ink);
}
.schema-diff-summary {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  margin-bottom: 8px;
}
.schema-diff-summary:empty {
  display: none;
}
.schema-cols-grid {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
}
.col-chip {
  display: inline-flex;
  align-items: baseline;
  gap: 4px;
  font-family: var(--font-mono);
  font-size: 11px;
  color: var(--ink-2);
  background: var(--bg-paper);
  border: 1px solid var(--line);
  border-radius: var(--r-pill);
  padding: 2px 9px;
}
.col-ver {
  font-size: 10px;
  color: var(--ink-4);
}
.schema-legend-group {
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  justify-content: flex-end;
  gap: 6px;
  row-gap: 4px;
  max-width: 100%;
}
#slice-target-select {
  max-width: min(280px, 100%);
  min-width: 0;
}
.schema-card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}
.schema-step-badge {
  font-family: var(--font-mono);
  font-size: 10px;
  padding: 2px 6px;
  border-radius: var(--r-xs);
  background: var(--paper-1);
  color: var(--ink-3);
}
.diff-badge {
  font-size: 10px;
  font-family: var(--font-mono);
  padding: 2px 6px;
  border-radius: var(--r-xs);
  font-weight: 600;
  white-space: nowrap;
  flex-shrink: 0;
}
.diff-added { background: var(--status-ok-subtle); color: var(--status-ok); border: 1px solid var(--status-ok-line); }
.diff-removed { background: var(--status-err-subtle); color: var(--status-err); border: 1px solid var(--status-err-line); }
.diff-mod { background: var(--orange-tint); color: var(--orange-text); border: 1px solid var(--orange-line); }
.diff-ret { background: var(--paper-1); color: var(--ink-3); border: 1px solid var(--line); }

/* AST Explorer */
.ast-explorer-container {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
}
.ast-toolbar {
  min-height: 40px;
  height: auto;
  padding: 6px 12px;
  background: var(--bg-paper);
  border-bottom: 1px solid var(--line);
  display: flex;
  align-items: center;
  align-content: center;
  flex-wrap: wrap;
  column-gap: 10px;
  row-gap: 6px;
  flex-shrink: 0;
}
.ast-rep-selector {
  display: flex;
  background: var(--ink-wash-2);
  padding: 2px;
  border-radius: var(--r-pill);
  flex-shrink: 0;
}
.ast-rep-btn {
  border: none;
  background: transparent;
  height: 24px;
  padding: 0 10px;
  font-family: var(--font-sans);
  font-size: 11px;
  font-weight: 600;
  border-radius: var(--r-pill);
  cursor: pointer;
  color: var(--ink-3);
  white-space: nowrap;
  transition: background-color var(--motion-hover) var(--ease-out),
              color var(--motion-hover) var(--ease-out),
              box-shadow var(--motion-hover) var(--ease-out);
}
.ast-rep-btn:hover {
  color: var(--ink);
}
.ast-rep-btn:focus-visible {
  outline: none;
  box-shadow: var(--focus);
}
.ast-rep-btn.active {
  background: var(--surface-white);
  color: var(--ink);
  box-shadow: var(--shadow-raised);
}
.ast-search-box {
  display: flex;
  align-items: center;
  gap: 6px;
  height: var(--btn-h-sm);
  background: var(--surface-white);
  border: 1px solid var(--line);
  border-radius: var(--r-pill);
  padding: 0 10px;
  flex: 1 1 160px;
  min-width: 140px;
  max-width: 280px;
  transition: border-color var(--motion-hover) var(--ease-out), box-shadow var(--motion-hover) var(--ease-out);
}
.ast-search-box:hover {
  border-color: var(--line-strong);
}
.ast-search-box:focus-within {
  border-color: var(--orange);
  box-shadow: 0 0 0 3px var(--focus-ring);
}
.ast-search-box input {
  border: none;
  background: transparent;
  outline: none;
  font-family: inherit;
  font-size: 11.5px;
  color: var(--ink);
  width: 100%;
  min-width: 0;
}
.ui-icon {
  width: 15px;
  height: 15px;
  flex-shrink: 0;
  display: block;
}
.ast-search-icon {
  display: inline-flex;
  align-items: center;
  color: var(--ink-4);
  flex-shrink: 0;
}
.schema-filter-note {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 11px;
  color: var(--graph-edge-rowsel);
  margin-top: 6px;
}
.ast-search-match {
  font-family: var(--font-mono);
  font-size: 10px;
  color: var(--orange-text);
  background: var(--orange-tint);
  border-radius: var(--r-pill);
  padding: 1px 6px;
  white-space: nowrap;
}
.ast-actions {
  display: flex;
  gap: 6px;
  flex-shrink: 0;
  flex-wrap: nowrap;
}
.ast-actions .btn {
  height: var(--btn-h-sm);
  padding: 0 10px;
  font-size: 10.5px;
}
.ast-node-stats {
  margin-left: auto;
  font-family: var(--font-mono);
  font-size: 10.5px;
  color: var(--ink-4);
  white-space: nowrap;
  flex-shrink: 0;
}
.ast-tree-scroll {
  flex: 1;
  overflow: auto;
  padding: 14px 18px 40px;
  background: var(--surface-white);
  font-family: var(--font-mono);
  font-size: 11.5px;
  line-height: 22px;
}
.ast-node-row {
  display: flex;
  align-items: center;
  gap: 8px;
  min-height: 26px;
  padding: 0 8px 0 4px;
  border-radius: var(--r-xs);
  border-left: 2px solid transparent;
  cursor: pointer;
  white-space: nowrap;
  transition: background-color var(--motion-hover) var(--ease-out), border-color var(--motion-hover) var(--ease-out);
}
.ast-node-row:hover {
  background: var(--bg-paper);
}
.ast-node-row.selected {
  background: var(--orange-tint);
  border-left-color: var(--accent-orange);
}
.ast-node-row.has-error .ast-node-label {
  color: var(--status-err);
}
.ast-node-children {
  margin-left: 13px;
  padding-left: 10px;
  border-left: 1px solid var(--line);
}
.ast-node-children.collapsed {
  display: none;
}
.ast-node-chevron {
  width: 14px;
  flex-shrink: 0;
  display: inline-flex;
  justify-content: center;
  font-size: 8px;
  color: var(--ink-4);
  transition: transform var(--motion-view) var(--ease-editorial), color var(--motion-hover) var(--ease-out);
}
.ast-node-chevron.expanded {
  transform: rotate(90deg);
}
.ast-node-chevron.empty {
  visibility: hidden;
}
.ast-node-row:hover .ast-node-chevron:not(.empty) {
  color: var(--ink);
}
.ast-node-tag {
  flex-shrink: 0;
  min-width: 38px;
  text-align: center;
  font-size: 9.5px;
  font-weight: 600;
  letter-spacing: 0.04em;
  line-height: 16px;
  padding: 0 5px;
  border-radius: var(--r-pill);
  color: var(--ink-3);
  background: var(--ink-wash-2);
}
.ast-node-tag.ast-tag-stmt { color: var(--ink); background: var(--ink-wash-3); }
.ast-node-tag.ast-tag-id   { color: var(--syntax-identifier); background: var(--syntax-identifier-tint); }
.ast-node-tag.ast-tag-lit  { color: var(--syntax-number); background: var(--syntax-number-tint); }
.ast-node-tag.ast-tag-call { color: var(--syntax-function); background: var(--syntax-function-tint); }
.ast-node-tag.ast-tag-op   { color: var(--teal-text); background: var(--teal-tint); }
.ast-node-tag.ast-tag-err  { color: var(--status-err); background: var(--status-err-subtle); }
.ast-node-label {
  color: var(--ink);
  font-weight: 500;
}
.ast-node-detail {
  color: var(--ink-3);
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 40ch;
}
.ast-node-span {
  margin-left: auto;
  padding-left: 12px;
  font-size: 10px;
  color: var(--ink-4);
}
.ast-match-highlight {
  background: var(--orange-tint);
  color: var(--orange-text);
  border-radius: 2px;
  padding: 0 2px;
}

/* Command Palette Modal (Specification Phase 7.3) */
.palette-modal-overlay {
  position: fixed;
  inset: 0;
  background: var(--scrim);
  backdrop-filter: blur(2px);
  display: flex;
  align-items: flex-start;
  justify-content: center;
  padding-top: 14vh;
  z-index: 9999;
  opacity: 0;
  pointer-events: none;
  transition: opacity var(--motion-view) var(--ease-out);
}
.palette-modal-overlay.open {
  opacity: 1;
  pointer-events: auto;
}
.palette-card {
  width: 520px;
  max-width: 92vw;
  background: var(--surface-white);
  border: 1px solid var(--line);
  border-radius: var(--r-md);
  box-shadow: var(--shadow-overlay);
  overflow: hidden;
  display: flex;
  flex-direction: column;
  transform: translateY(-8px);
  transition: transform var(--motion-panel) var(--ease-editorial);
}
.palette-modal-overlay.open .palette-card {
  transform: translateY(0);
}
.palette-input-row {
  display: flex;
  align-items: center;
  padding: 12px 16px;
  border-bottom: 1px solid var(--line);
  gap: 10px;
}
.palette-input {
  flex: 1;
  border: none;
  outline: none;
  font-size: 14px;
  color: var(--ink);
  background: transparent;
  font-family: var(--font-sans);
}
.palette-list {
  max-height: 320px;
  overflow-y: auto;
  padding: 6px;
}
.palette-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 12px;
  border-radius: var(--r-sm);
  cursor: pointer;
  font-size: 13px;
  color: var(--ink-2);
  border-left: 2px solid transparent;
  transition: background-color var(--motion-hover) var(--ease-out),
              color var(--motion-hover) var(--ease-out),
              border-color var(--motion-hover) var(--ease-out);
}
.palette-item:hover, .palette-item.active {
  background: var(--bg-paper);
  color: var(--ink);
}
.palette-item.active {
  border-left-color: var(--accent-orange);
}
.palette-search-icon,
.palette-item-icon {
  width: 15px;
  height: 15px;
  flex-shrink: 0;
  stroke: currentColor;
  color: var(--ink-4);
}
.palette-item-label {
  display: inline-flex;
  align-items: center;
  gap: 10px;
  min-width: 0;
}
.palette-item:hover .palette-item-icon,
.palette-item.active .palette-item-icon {
  color: var(--ink);
}
.kbd-shortcut {
  font-family: var(--font-mono);
  font-size: 10px;
  color: var(--ink-4);
  background: var(--bg-paper);
  border: 1px solid var(--line);
  border-radius: var(--r-xs);
  padding: 1px 5px;
}

/* ========================================================================= */
/* SHARED MOTION — view switches, panel content reveals                      */
/* ========================================================================= */
.view-animate-in {
  animation: rp-view-in var(--motion-view) var(--ease-editorial);
}
@keyframes rp-view-in {
  from { opacity: 0; transform: translateY(4px); }
  to { opacity: 1; transform: none; }
}
@keyframes rp-reveal {
  from { opacity: 0; transform: translateY(3px); }
  to { opacity: 1; transform: none; }
}



/* Responsive Media Queries (Section 8 Specification) */
@media (max-width: 1200px) {
  .header-center-info .stats-strip {
    display: none;
  }
}
@media (max-width: 1024px) {
  .header-meta-group {
    display: none;
  }
  .search-pill-btn span:not(.search-icon) {
    display: none;
  }
  .search-pill-btn {
    padding: 4px 8px;
  }
  .pane-source {
    min-width: 280px;
    width: 32%;
  }
  .workspace-inspector {
    width: 280px;
  }
}
@media (max-width: 860px) {
  .workspace-inspector {
    position: absolute;
    right: 0;
    top: 0;
    bottom: 0;
    width: 320px;
    z-index: 60;
    box-shadow: var(--shadow-drawer);
  }
  .workspace-inspector.collapsed {
    transform: translateX(100%);
    display: flex !important;
  }
}

@media (prefers-reduced-motion: reduce) {
  * {
    animation-duration: 0.001s !important;
    transition-duration: 0.001s !important;
  }
  .view-animate-in,
  .inspector-content > *,
  .status-dot.analyzing,
  .source-line.line-selected, .code-line.line-selected, .code-line.highlighted-span {
    animation: none !important;
  }
  .btn:active:not(:disabled),
  .conn-item:hover {
    transform: none;
  }
}
"###;

/// Complete HTML body shell for the 30/70 workspace, toolbars, modal dialogs, and contextual drawer.
pub const HTML_BODY_MARKUP: &str = r###"
  <!-- Phase 3: Desktop 3-Column + Bottom Dock Master Scientific Workspace -->
  <div class="workspace-screen" id="workspace-screen">
    <!-- 3.1 Top Bar (56px) -->
    <header class="app-header">
      <div class="header-top-row">
        <!-- Left: Monogram and wordmark -->
        <div class="header-identity">
          <img class="brand-mark" src="/brand/rparse-mark-96.png" width="22" height="22" alt="" aria-hidden="true">
          <span class="brand-logo">R—PARSE</span>
        </div>

        <div class="header-center-info">
          <div class="header-meta-group">
            <span class="header-meta-item" id="header-mode">Static analysis</span>
            <span class="header-meta-sep">•</span>
            <span class="header-meta-item" id="stat-unk">Known static</span>
          </div>

          <div class="stats-strip" id="stats-strip">
            <div class="stat-item" title="Analyzed R Statements"><span class="stat-val" id="stat-stmts">0</span> <span class="stat-lbl">stmts</span></div>
            <span class="stat-divider">•</span>
            <div class="stat-item" title="Tracked Column Versions"><span class="stat-val" id="stat-cols">0</span> <span class="stat-lbl">cols</span></div>
            <span class="stat-divider">•</span>
            <div class="stat-item" title="Unified PDG Nodes"><span class="stat-val" id="stat-nodes">0</span> <span class="stat-lbl">pdg nodes</span></div>
          </div>
        </div>

        <!-- Center-Right: Search Command Palette Trigger -->
        <button class="search-pill-btn" id="btn-command-palette-trigger" title="Open Command Palette (⌘K / Ctrl+K)">
          <span class="search-icon" aria-hidden="true"><svg width="13" height="13" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round"><circle cx="7" cy="7" r="4.5"/><path d="M10.5 10.5L14 14"/></svg></span>
          <span>Search actions &amp; views...</span>
        </button>

        <!-- Right: Primary Actions (Guide, Investigation, Run, New Analysis) -->
        <div class="header-actions-row">
          <button class="btn btn-ghost theme-toggle" id="btn-theme-toggle" type="button" aria-pressed="false" aria-label="Switch to dark mode" title="Switch to dark mode">
            <svg class="theme-icon theme-icon-moon" width="14" height="14" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M13.2 10.4A5.2 5.2 0 0 1 5.6 2.8 5.2 5.2 0 1 0 13.2 10.4z"/></svg>
            <svg class="theme-icon theme-icon-sun" width="14" height="14" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" aria-hidden="true"><circle cx="8" cy="8" r="2.5"/><path d="M8 1.6v1.5M8 12.9v1.5M1.6 8h1.5M12.9 8h1.5M3.5 3.5l1.1 1.1M11.4 11.4l1.1 1.1M3.5 12.5l1.1-1.1M11.4 4.6l1.1-1.1"/></svg>
          </button>
          <button class="btn btn-ghost theme-toggle" id="btn-forced-black" type="button" aria-pressed="false" aria-label="Forced black" title="Forced black">
            <svg width="14" height="14" viewBox="0 0 16 16" aria-hidden="true"><circle cx="8" cy="8" r="5.2" fill="currentColor"/></svg>
          </button>
          <a href="/how-to-use" target="_blank" class="header-nav-link" id="link-how-to-use" title="Open Beginner-Friendly How to Use Guide"><svg class="ui-icon" width="15" height="15" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linejoin="round" aria-hidden="true"><path d="M3 3.2h4a2 2 0 0 1 2 2V13a1.8 1.8 0 0 0-1.8-1.5H3z"/><path d="M13 3.2H9a2 2 0 0 0-2 2V13a1.8 1.8 0 0 1 1.8-1.5H13z"/></svg><span>Guide</span></a>
          <a href="/" class="header-nav-link" id="link-investigation" title="View The Investigation landing narrative"><span>Investigation ↗</span></a>
          <button class="btn btn-primary" id="btn-run-analysis" title="Re-run static analysis on current code">Run</button>
          <button class="btn btn-secondary" id="btn-new-analysis" title="Return to code entry screen">New Analysis</button>
        </div>
      </div>
    </header>

    <!-- Section 3: Horizontal Feature Navigation Row (Directly below Header) -->
    <nav class="feature-nav-bar" id="feature-nav-bar" role="tablist" aria-label="Analysis Views">
      <div class="feature-tabs-list">
        <button class="nav-tab" data-view="ast" id="tab-ast" role="tab" title="Interactive AST & CST Syntax Tree Explorer">
          <svg class="nav-tab-icon" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M4 3v10M4 8h5M4 4h7M4 12h6"/></svg>
          <span class="nav-tab-label">AST Explorer</span>
          <span class="nav-count" id="count-ast">0</span>
        </button>
        <button class="nav-tab" data-view="pdg" id="tab-pdg" role="tab" title="Program Dependence Graph">
          <svg class="nav-tab-icon" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5"><circle cx="4" cy="5" r="2"/><circle cx="12" cy="5" r="2"/><circle cx="8" cy="12" r="2"/><path d="M6 5h4M5.5 6.5l2 4M10.5 6.5l-2 4"/></svg>
          <span class="nav-tab-label">PDG Graph</span>
          <span class="nav-count" id="count-pdg">0</span>
        </button>
        <button class="nav-tab" data-view="schema" id="tab-schema" role="tab" title="Dataframe schema evolution across transformations">
          <svg class="nav-tab-icon" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5"><rect x="2" y="3" width="12" height="10" rx="1.5"/><path d="M2 7h12M6 7v6"/></svg>
          <span class="nav-tab-label">Schema Evolution</span>
          <span class="nav-count" id="count-schema">0</span>
        </button>
        <button class="nav-tab active" data-view="provenance" id="tab-provenance" role="tab" title="Fine-grained dataframe column lineage">
          <svg class="nav-tab-icon" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5"><circle cx="4" cy="4" r="2"/><circle cx="12" cy="8" r="2"/><circle cx="4" cy="12" r="2"/><path d="M6 4h3a3 3 0 0 1 3 3v0M6 12h3a3 3 0 0 0 3-3v0"/></svg>
          <span class="nav-tab-label">Provenance</span>
          <span class="nav-count" id="count-prov">0</span>
        </button>
        <button class="nav-tab" data-view="slice" id="tab-slice" role="tab" title="Column-aware static backward slicing">
          <svg class="nav-tab-icon" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M3 13L13 3M8 3h5v5M3 8v5h5"/></svg>
          <span class="nav-tab-label">Backward Slice</span>
          <span class="nav-count" id="count-slice">0</span>
        </button>
      </div>
    </nav>

    <!-- 3-Column Middle Workspace Container -->
    <div class="workspace-main" id="workbench-container">
      <!-- 2.1 Left Panel — Source Editor (34% width / 380-460px) -->
      <div class="pane-source word-wrap-enabled" id="source-pane">
        <div class="pane-header">
          <div style="display:flex; align-items:center; gap:8px;">
            <button type="button" class="pane-title pane-title-toggle" id="btn-toggle-source" aria-pressed="false" title="Hide source code"><svg class="source-rail-chevron" width="12" height="12" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M6 3.5 10.5 8 6 12.5"/></svg><span class="pane-title-label">Source Code</span></button>
            <span class="pane-badge" id="source-line-count">0 lines</span>
          </div>
          <div style="display:flex; align-items:center; gap:4px;">
            <button class="btn btn-ghost pane-icon-btn active" id="btn-toggle-wrap" title="Word wrap is ON (click to disable and permit horizontal scroll)" aria-label="Toggle word wrap" aria-pressed="true">
              <svg class="ui-icon" width="15" height="15" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M2.5 4h11M2.5 8h7.2a2.2 2.2 0 0 1 0 4.4H7.2"/><path d="M9.2 10.6 7.2 12.4l2 1.8"/></svg>
            </button>
            <button class="btn btn-ghost pane-icon-btn" id="btn-clear-highlight" title="Clear selection" aria-label="Clear selection">
              <svg class="ui-icon" width="15" height="15" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" aria-hidden="true"><circle cx="8" cy="8" r="5.2"/><path d="M5.3 10.7 10.7 5.3"/></svg>
            </button>
            <button class="btn btn-ghost pane-icon-btn" id="btn-edit-source" title="Edit source code" aria-label="Edit source code" aria-pressed="false">
              <svg class="ui-icon icon-edit" width="15" height="15" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M10.2 2.8 13.2 5.8 5.4 13.6H2.4v-3z"/><path d="M8.6 4.4 11.6 7.4"/></svg>
              <svg class="ui-icon icon-view" width="15" height="15" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M1.6 8s2.4-4 6.4-4 6.4 4 6.4 4-2.4 4-6.4 4-6.4-4-6.4-4z"/><circle cx="8" cy="8" r="1.7"/></svg>
            </button>
          </div>
        </div>

        <!-- Stale source warning banner -->
        <div class="source-stale-banner" id="source-stale-banner" role="status" aria-live="polite">
          <span class="stale-banner-label"><svg class="ui-icon" width="15" height="15" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M8 2.4 14.2 13.4H1.8z"/><path d="M8 6.4v3.2"/><path d="M8 11.5h.01"/></svg>Source modified (results from earlier revision).</span>
          <div style="display:flex; gap:6px;">
            <button class="btn btn-sm btn-primary" id="btn-reanalyze">Reanalyze</button>
            <button class="btn btn-sm btn-secondary" id="btn-revert-source">Revert</button>
          </div>
        </div>

        <!-- Status / Error Notification Banner -->
        <div class="source-status-banner" id="source-status-banner">
          <span id="source-status-msg"></span>
          <button class="btn btn-ghost btn-sm" id="btn-dismiss-status" aria-label="Dismiss notification">✕</button>
        </div>

        <div class="source-viewer" id="source-container"></div>
        <textarea class="source-editor-textarea" id="source-editor-textarea" spellcheck="false" placeholder="Edit R program source here..."></textarea>
      </div>

      <!-- Resizable Column Splitter between Source Editor and Central Stage -->
      <div class="col-splitter" id="splitter-source" tabindex="0" role="separator" aria-orientation="vertical" aria-label="Resize source editor" title="Drag to resize source editor">
        <div class="col-splitter-handle"></div>
      </div>

      <!-- 2.2 Center Panel — Active Analysis Stage -->
      <div class="central-stage" id="central-stage">
        <!-- Active Analysis View -->
        <div class="analysis-view-stage" id="main-view-pane">
          <!-- View A: Provenance Toolbar -->
          <div class="view-toolbar" id="toolbar-provenance">
            <div class="toolbar-group">
              <div class="filter-box">
                <input type="text" class="filter-input" id="prov-search" placeholder="Filter column or derivation..." autocomplete="off" />
                <button class="filter-clear" id="btn-prov-clear-search" style="display:none;" title="Clear filter">×</button>
              </div>
              <label class="toggle-chip" id="btn-prov-toggle-grouping" title="Group multi-input derivations">
                <input type="checkbox" id="check-prov-grouping" checked /> Grouped Multi-Input Derivations
              </label>
            </div>
            <div class="zoom-cluster" role="group" aria-label="Graph zoom">
              <button class="zoom-btn" id="btn-prov-zoom-out" type="button" title="Zoom out" aria-label="Zoom out"><svg width="12" height="12" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" aria-hidden="true"><path d="M3.5 8h9"/></svg></button>
              <button class="zoom-btn" id="btn-prov-zoom-in" type="button" title="Zoom in" aria-label="Zoom in"><svg width="12" height="12" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" aria-hidden="true"><path d="M8 3.5v9M3.5 8h9"/></svg></button>
              <span class="zoom-cluster-rule" aria-hidden="true"></span>
              <button class="zoom-fit" id="btn-prov-reset" type="button" title="Fit graph to view"><svg width="12" height="12" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M3 6V3.5h2.5M10.5 3.5H13V6M13 10v2.5h-2.5M5.5 12.5H3V10"/></svg>Fit</button>
            </div>
          </div>

          <!-- View B: PDG Toolbar -->
          <div class="view-toolbar" id="toolbar-pdg" style="display:none;">
            <div class="toolbar-group edge-filter-group">
              <span class="toolbar-label">Edges:</span>
              <label class="toggle-chip" title="Data Dependencies & Column Derivations">
                <input type="checkbox" id="toggle-edge-data" checked /> Data
              </label>
              <label class="toggle-chip chip-ctrl" title="CFG Branch Control Dependencies">
                <input type="checkbox" id="toggle-edge-ctrl" checked /> Control
              </label>
              <label class="toggle-chip chip-rowsel" title="DataFrame Row Selection Dependencies">
                <input type="checkbox" id="toggle-edge-rowsel" checked /> RowSelection
              </label>
              <label class="toggle-chip chip-unk" title="Static Uncertainty Edges">
                <input type="checkbox" id="toggle-edge-unk" checked /> Unknown
              </label>
            </div>
            <div class="toolbar-group toolbar-actions">
              <button class="btn btn-ghost btn-sm pdg-neighborhood-btn" id="btn-pdg-neighborhood" type="button" aria-pressed="false" title="Show only the selected node and its direct dependencies. This is one hop, not every transitive dependency.">Neighborhood</button>
            <div class="zoom-cluster" role="group" aria-label="Graph zoom">
              <button class="zoom-btn" id="btn-pdg-zoom-out" type="button" title="Zoom out" aria-label="Zoom out"><svg width="12" height="12" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" aria-hidden="true"><path d="M3.5 8h9"/></svg></button>
              <button class="zoom-btn" id="btn-pdg-zoom-in" type="button" title="Zoom in" aria-label="Zoom in"><svg width="12" height="12" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" aria-hidden="true"><path d="M8 3.5v9M3.5 8h9"/></svg></button>
              <span class="zoom-cluster-rule" aria-hidden="true"></span>
              <button class="zoom-fit" id="btn-pdg-reset" type="button" title="Fit graph to view"><svg width="12" height="12" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M3 6V3.5h2.5M10.5 3.5H13V6M13 10v2.5h-2.5M5.5 12.5H3V10"/></svg>Fit</button>
            </div>
            </div>
          </div>

          <!-- View C: Backward Slice Toolbar -->
          <div class="view-toolbar" id="toolbar-slice" style="display:none;">
            <div class="toolbar-group">
              <span class="toolbar-label">Criterion:</span>
              <select class="select-dropdown" id="slice-target-select"></select>
            </div>
            <div class="toolbar-group">
              <span id="slice-conf-badge" class="pill pill-full">COMPLETE</span>
              <span id="slice-unk-badge" class="pill pill-keep">KNOWN</span>
            </div>
          </div>

          <!-- View D: Schema Evolution Toolbar -->
          <div class="view-toolbar" id="toolbar-schema" style="display:none;">
            <div class="toolbar-group">
              <span class="toolbar-label">Dataframe Transitions</span>
              <span class="pane-badge" id="schema-step-count">0 states</span>
            </div>
            <div class="toolbar-group schema-legend-group" role="note" aria-label="Schema diff legend">
              <span class="toolbar-label">Legend:</span>
              <span class="diff-badge diff-added" title="Newly derived column">+added</span>
              <span class="diff-badge diff-removed" title="Removed column">-removed</span>
              <span class="diff-badge diff-mod" title="Modified column">~modified</span>
              <span class="diff-badge diff-ret" title="Retained column">=retained</span>
            </div>
          </div>

          <!-- Canvas Viewport -->
          <div class="canvas-viewport" id="canvas-viewport">
            <!-- Canvas for Provenance -->
            <div class="graph-canvas-container" id="view-container-provenance">
              <svg class="graph-svg" id="svg-provenance">
                <defs>
                  <marker id="arrow-prov-data" viewBox="0 0 10 10" refX="10" refY="5" markerWidth="8" markerHeight="8" orient="auto">
                    <path d="M 0 1 L 10 5 L 0 9 z" />
                  </marker>
                  <marker id="arrow-prov-colver" viewBox="0 0 10 10" refX="10" refY="5" markerWidth="8" markerHeight="8" orient="auto">
                    <path d="M 0 1 L 10 5 L 0 9 z" />
                  </marker>
                  <marker id="arrow-prov-unk" viewBox="0 0 10 10" refX="10" refY="5" markerWidth="8" markerHeight="8" orient="auto">
                    <path d="M 0 1 L 10 5 L 0 9 z" />
                  </marker>
                </defs>
              </svg>
            </div>

            <!-- Canvas for PDG -->
            <div class="graph-canvas-container" id="view-container-pdg" style="display:none;">
              <svg class="graph-svg" id="svg-pdg">
                <defs>
                  <marker id="arrow-data" viewBox="0 0 10 10" refX="10" refY="5" markerWidth="8" markerHeight="8" orient="auto">
                    <path d="M 0 1 L 10 5 L 0 9 z" />
                  </marker>
                  <marker id="arrow-ctrl" viewBox="0 0 10 10" refX="10" refY="5" markerWidth="8" markerHeight="8" orient="auto">
                    <path d="M 0 1 L 10 5 L 0 9 z" />
                  </marker>
                  <marker id="arrow-rowsel" viewBox="0 0 10 10" refX="10" refY="5" markerWidth="8" markerHeight="8" orient="auto">
                    <path d="M 0 1 L 10 5 L 0 9 z" />
                  </marker>
                  <marker id="arrow-colver" viewBox="0 0 10 10" refX="10" refY="5" markerWidth="8" markerHeight="8" orient="auto">
                    <path d="M 0 1 L 10 5 L 0 9 z" />
                  </marker>
                  <marker id="arrow-unk" viewBox="0 0 10 10" refX="10" refY="5" markerWidth="8" markerHeight="8" orient="auto">
                    <path d="M 0 1 L 10 5 L 0 9 z" />
                  </marker>
                </defs>
              </svg>
            </div>

            <!-- Container for Slice -->
            <div class="slice-container" id="view-container-slice" style="display:none;"></div>

            <!-- Container for Schema -->
            <div class="schema-timeline" id="view-container-schema" style="display:none;"></div>

            <!-- Container for AST & CST Explorer (View E) -->
            <div class="ast-explorer-container" id="view-container-ast" style="display:none;">
              <div class="ast-toolbar">
                <div class="ast-rep-selector" role="radiogroup" aria-label="Tree Representation">
                  <button class="ast-rep-btn active" id="btn-ast-rep-cst" data-rep="cst" title="Tree-sitter Concrete Syntax Tree (with exact tokens & error nodes)">Concrete Syntax (CST)</button>
                  <button class="ast-rep-btn" id="btn-ast-rep-ir" data-rep="ir" title="Canonical Normalized Semantic IR / AST">Semantic IR / AST</button>
                </div>
                <div class="ast-search-box">
                  <span class="ast-search-icon" aria-hidden="true"><svg class="ui-icon" width="15" height="15" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round"><circle cx="7" cy="7" r="4.5"/><path d="M10.5 10.5L14 14"/></svg></span>
                  <input type="text" id="ast-search-input" placeholder="Search node type, identifier, text..." />
                  <span class="ast-search-match" id="ast-search-match"></span>
                </div>
                <div class="ast-actions">
                  <button class="btn btn-secondary" id="btn-ast-expand-all" title="Expand all nodes">Expand All</button>
                  <button class="btn btn-secondary" id="btn-ast-collapse-all" title="Collapse all nodes">Collapse All</button>
                </div>
                <div class="ast-node-stats" id="ast-node-stats">
                  <span id="ast-stat-total">Total: 0</span> · <span id="ast-stat-visible">Visible: 0</span>
                </div>
              </div>
              <div class="ast-tree-scroll" id="ast-tree-root"></div>
            </div>
          </div>
        </div>
      </div>

      <!-- 2.3 Right Panel — Inspector (320px column) -->
      <div class="workspace-inspector collapsed" id="contextual-inspector">
        <div class="inspector-header">
          <div class="inspector-title-group">
            <span class="pane-title">Inspector</span>
            <span class="pane-badge" id="inspector-status-badge">Overview</span>
          </div>
          <div style="display:flex; align-items:center; gap:4px;">
            <button class="btn btn-ghost" id="btn-jump-source" style="padding:2px 6px; font-size:10px;" title="Jump to source position">Jump</button>
            <button class="btn btn-ghost" id="btn-close-inspector" style="padding:2px 6px; font-size:10px;" title="Collapse inspector">✕</button>
          </div>
        </div>
        <div class="inspector-content" id="inspector-container"></div>
      </div>
    </div>


    <!-- Canonical Load Program Modal Dialog -->
    <div class="load-modal-backdrop" id="load-program-modal" role="dialog" aria-modal="true" aria-labelledby="load-program-title" style="display:none;">
      <div class="load-program-dialog">
        <!-- Modal Header -->
        <header class="load-modal-header">
          <div class="load-header-brand">
            <span class="load-wordmark">R—PARSE</span>
            <span class="load-badge-label">LOAD PROGRAM</span>
          </div>
          <button class="load-close-btn" id="btn-close-load-program" type="button" aria-label="Close dialog" title="Close (Esc)">✕</button>
        </header>

        <!-- Modal Body Content -->
        <div class="load-modal-body">
          <div class="load-dialog-intro">
            <h2 class="load-program-title" id="load-program-title">Load or Enter R Program</h2>
            <p class="load-program-subtitle">Paste R pipeline source code, open a local script, or load a canonical example for deterministic Tier 1 static analysis.</p>
          </div>

          <div class="load-code-box">
            <div class="load-code-topbar">
              <span class="load-buffer-title">Source Buffer</span>
              <span class="load-buffer-stats" id="load-program-stats">0 lines · 0 chars</span>
            </div>
            <textarea class="load-code-textarea" id="load-program-textarea" spellcheck="false" placeholder="# Paste your R pipeline script here...&#10;df &lt;- data.frame(price = 100, quantity = 2, age = 30)&#10;filtered &lt;- filter(df, price &gt; 0)&#10;df2 &lt;- mutate(filtered, total = price * quantity, discount = age * 0.05)&#10;df3 &lt;- mutate(df2, tax = total * 0.18)"></textarea>
            <div class="load-code-footer">
              <span class="load-hint"><kbd>Shift+Enter</kbd> newline · <kbd>Ctrl+Enter</kbd> / <kbd>⌘+Enter</kbd> to analyze</span>
            </div>
          </div>

          <div class="load-actions-row">
            <button class="btn btn-primary btn-lg" id="btn-load-program-analyze">
              <span class="play-icon">▶</span>
              <span id="btn-load-analyze-text">Analyze Program</span>
            </button>

            <label class="btn btn-secondary btn-lg" id="btn-load-program-file" style="cursor:pointer;" title="Load local R file (.R, .r, .Rmd)">
              <span>Load File...</span>
              <input type="file" id="load-program-file-input" accept=".R,.r,.Rmd" style="display:none;" />
            </label>

            <div style="flex:1;">
              <select class="select-dropdown" id="load-program-example-select" style="width:100%;">
                <option value="">Load Example...</option>
                <option value="showcase_pipeline">Showcase Pipeline</option>
                <option value="canonical">Simple Provenance</option>
                <option value="filter_provenance">Filter Provenance</option>
                <option value="partial_retention">Partial Retention</option>
                <option value="uncertainty">Uncertainty</option>
              </select>
            </div>

            <button class="btn btn-ghost" id="btn-cancel-load-program" type="button">Cancel</button>
          </div>

          <div class="validation-msg" id="load-program-validation-error" style="display:none;"></div>
        </div>
      </div>
    </div>

    <!-- 7.3 Command Palette Modal Dialog -->
    <div class="palette-modal-overlay" id="command-palette-modal" role="dialog" aria-modal="true" aria-label="Command Palette">
      <div class="palette-card">
        <div class="palette-input-row">
          <svg class="palette-search-icon" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" aria-hidden="true"><circle cx="7" cy="7" r="4.5"/><path d="M10.5 10.5L14 14"/></svg>
          <input type="text" class="palette-input" id="palette-search-input" placeholder="Type a view or action (e.g. Load, Provenance, Slice)..." autocomplete="off" />
          <kbd class="kbd-shortcut" id="btn-palette-close" style="cursor:pointer;" title="Close (Esc)">ESC</kbd>
        </div>
        <div class="palette-list" id="palette-list">
          <div class="palette-item active" data-action="new-analysis"><span class="palette-item-label"><svg class="palette-item-icon" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M4 2.5h5.5L13 6v7.5H4z"/><path d="M9.5 2.5V6H13"/></svg>Load Program / New Analysis</span><kbd class="kbd-shortcut">⌘N</kbd></div>
          <div class="palette-item" data-action="view-ast"><span class="palette-item-label"><svg class="palette-item-icon" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M4 3v10M4 8h5M4 4h7M4 12h6"/></svg>Switch to AST Explorer</span><kbd class="kbd-shortcut">1</kbd></div>
          <div class="palette-item" data-action="view-pdg"><span class="palette-item-label"><svg class="palette-item-icon" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5"><circle cx="4" cy="5" r="2"/><circle cx="12" cy="5" r="2"/><circle cx="8" cy="12" r="2"/><path d="M6 5h4M5.5 6.5l2 4M10.5 6.5l-2 4"/></svg>Switch to PDG Graph</span><kbd class="kbd-shortcut">2</kbd></div>
          <div class="palette-item" data-action="view-schema"><span class="palette-item-label"><svg class="palette-item-icon" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5"><rect x="2" y="3" width="12" height="10" rx="1.5"/><path d="M2 7h12M6 7v6"/></svg>Switch to Schema Evolution</span><kbd class="kbd-shortcut">3</kbd></div>
          <div class="palette-item" data-action="view-prov"><span class="palette-item-label"><svg class="palette-item-icon" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5"><circle cx="4" cy="4" r="2"/><circle cx="12" cy="8" r="2"/><circle cx="4" cy="12" r="2"/><path d="M6 4h3a3 3 0 0 1 3 3v0M6 12h3a3 3 0 0 0 3-3v0"/></svg>Switch to Provenance</span><kbd class="kbd-shortcut">4</kbd></div>
          <div class="palette-item" data-action="view-slice"><span class="palette-item-label"><svg class="palette-item-icon" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M3 13L13 3M8 3h5v5M3 8v5h5"/></svg>Switch to Backward Slice</span><kbd class="kbd-shortcut">5</kbd></div>
          <div class="palette-item" data-action="run-analysis"><span class="palette-item-label"><svg class="palette-item-icon" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M13.2 8A5.2 5.2 0 1 1 12 4.6"/><path d="M12 2.2V5H9.2"/></svg>Reanalyze Program</span><kbd class="kbd-shortcut">⌘R</kbd></div>
          <div class="palette-item" data-action="toggle-wrap"><span class="palette-item-label"><svg class="palette-item-icon" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M2 4h12M2 8h8a2.5 2.5 0 0 1 0 5H8M2 12h3M10 11l-2 2 2 2"/></svg>Toggle Source Word Wrap</span></div>
          <div class="palette-item" data-action="copy-cli"><span class="palette-item-label"><svg class="palette-item-icon" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><rect x="4.5" y="3.5" width="8" height="10" rx="1.2"/><path d="M6.2 3.5V2.6h4.2v.9"/></svg>Copy Current CLI Command</span></div>
          <div class="palette-item" data-action="open-guide"><span class="palette-item-label"><svg class="palette-item-icon" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linejoin="round" aria-hidden="true"><path d="M3 3.2h4a2 2 0 0 1 2 2V13a1.8 1.8 0 0 0-1.8-1.5H3z"/><path d="M13 3.2H9a2 2 0 0 0-2 2V13a1.8 1.8 0 0 1 1.8-1.5H13z"/></svg>Open How to Use Guide</span></div>
          <div class="palette-item" data-action="open-investigation"><span class="palette-item-label"><svg class="palette-item-icon" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M5 11L11 5"/><path d="M6.5 5H11v4.5"/></svg>Open Investigation Landing Page</span></div>
        </div>
      </div>
    </div>
  </div>
"###;

/// Complete vanilla JavaScript client logic for R-PARSE Scientific Analysis Studio.
pub const JS_SCRIPTS: &str = r###"
    (function() {
      const themeStorageKey = 'rparse-studio-theme';
      const themeBaseKey = 'rparse-studio-theme-base';
      const themeToggle = document.getElementById('btn-theme-toggle');
      const forcedBlackToggle = document.getElementById('btn-forced-black');
      function applyStudioTheme(theme) {
        const dark = theme === 'dark';
        const black = theme === 'black';
        if (dark || black) document.documentElement.setAttribute('data-theme', theme);
        else document.documentElement.removeAttribute('data-theme');
        if (themeToggle) {
          themeToggle.setAttribute('aria-pressed', dark ? 'true' : 'false');
          const label = dark ? 'Switch to light mode' : 'Switch to dark mode';
          themeToggle.setAttribute('aria-label', label);
          themeToggle.title = label;
        }
        if (forcedBlackToggle) {
          forcedBlackToggle.setAttribute('aria-pressed', black ? 'true' : 'false');
          forcedBlackToggle.classList.toggle('active', black);
          const label = black ? 'Turn off forced black' : 'Forced black';
          forcedBlackToggle.setAttribute('aria-label', label);
          forcedBlackToggle.title = label;
        }
      }
      const initialTheme = document.documentElement.getAttribute('data-theme');
      applyStudioTheme(initialTheme === 'dark' || initialTheme === 'black' ? initialTheme : 'light');
      themeToggle?.addEventListener('click', () => {
        const next = document.documentElement.getAttribute('data-theme') === 'dark' ? 'light' : 'dark';
        try { localStorage.setItem(themeStorageKey, next); } catch (e) {}
        applyStudioTheme(next);
      });
      forcedBlackToggle?.addEventListener('click', () => {
        const current = document.documentElement.getAttribute('data-theme');
        let next = 'black';
        if (current === 'black') {
          next = 'light';
          try {
            const saved = localStorage.getItem(themeBaseKey);
            if (saved === 'dark' || saved === 'light') next = saved;
          } catch (e) {}
        } else {
          try { localStorage.setItem(themeBaseKey, current === 'dark' ? 'dark' : 'light'); } catch (e) {}
        }
        try { localStorage.setItem(themeStorageKey, next); } catch (e) {}
        applyStudioTheme(next);
      });

      const rawDataElement = document.getElementById('rparse-report-data');
      if (!rawDataElement) return;
      let data = {};
      try {
        data = JSON.parse(rawDataElement.textContent);
      } catch (e) {
        console.error("Failed to parse report dataset", e);
        return;
      }

      const initialSource = data.source_code || '';

      // State
      let activeView = 'provenance';
      let selectedItem = null;
      let activeSliceTarget = data.initial_selection?.criterion || Object.keys(data.slices || {})[0] || '';
      let currentRevisionId = 0;
      let isAnalyzing = false;
      let isSourceEditMode = false;

      const reducedMotionQuery = window.matchMedia ? window.matchMedia('(prefers-reduced-motion: reduce)') : null;
      function prefersReducedMotion() {
        return !!(reducedMotionQuery && reducedMotionQuery.matches);
      }
      function scrollBehavior() {
        return prefersReducedMotion() ? 'auto' : 'smooth';
      }

      // Elements — Canonical Load Program Modal Dialog
      const loadProgramModal = document.getElementById('load-program-modal');
      const loadProgramInput = document.getElementById('load-program-textarea') || document.getElementById('entry-source-textarea');
      const loadProgramStats = document.getElementById('load-program-stats') || document.getElementById('entry-char-count');
      const loadProgramError = document.getElementById('load-program-validation-error') || document.getElementById('entry-validation-error');
      const btnLoadProgramAnalyze = document.getElementById('btn-load-program-analyze') || document.getElementById('btn-analyze-entry');
      const btnLoadProgramFile = document.getElementById('btn-load-program-file') || document.getElementById('btn-entry-load-file');
      const loadProgramFileInput = document.getElementById('load-program-file-input') || document.getElementById('entry-file-input');
      const loadProgramExampleSelect = document.getElementById('load-program-example-select') || document.getElementById('entry-example-select');
      const btnCloseLoadProgram = document.getElementById('btn-close-load-program');
      const btnCancelLoadProgram = document.getElementById('btn-cancel-load-program');

      // Elements — Workspace Screen
      const workspaceScreen = document.getElementById('workspace-screen');
      const sourcePane = document.getElementById('source-pane');
      const splitterSource = document.getElementById('splitter-source-dock') || document.getElementById('splitter-source');
      const sourceContainer = document.getElementById('source-container');
      const sourceEditorTextarea = document.getElementById('source-editor-textarea');
      const sourceStaleBanner = document.getElementById('source-stale-banner');
      const sourceStatusBanner = document.getElementById('source-status-banner');
      const sourceStatusMsg = document.getElementById('source-status-msg');
      const btnEditSource = document.getElementById('btn-edit-source');
      const btnReanalyze = document.getElementById('btn-reanalyze');
      const btnRevertSource = document.getElementById('btn-revert-source');
      const contextualInspector = document.getElementById('contextual-inspector');
      const inspectorContainer = document.getElementById('inspector-container');
      const navTabs = document.querySelectorAll('.nav-tab');

      function getApiEndpoint() {
        if (window.location.protocol.startsWith('http')) {
          return `${window.location.origin}/api/analyze`;
        }
        return 'http://127.0.0.1:8080/api/analyze';
      }

      function showStatusNotification(msg, isError = true) {
        if (!sourceStatusBanner || !sourceStatusMsg) return;
        sourceStatusMsg.textContent = msg;
        sourceStatusBanner.style.background = isError ? 'var(--status-err-subtle)' : 'var(--orange-tint)';
        sourceStatusBanner.style.color = isError ? 'var(--status-err)' : 'var(--orange-text)';
        sourceStatusBanner.style.display = 'flex';
      }

      function hideStatusNotification() {
        if (sourceStatusBanner) sourceStatusBanner.style.display = 'none';
      }

      document.getElementById('btn-dismiss-status')?.addEventListener('click', hideStatusNotification);

      function refreshHeaderAndMetrics() {
        const modeElem = document.getElementById('header-mode');
        if (modeElem) {
          const m = (data.mode || 'static').toLowerCase();
          modeElem.textContent = m === 'static' ? 'Static analysis' : `${m.charAt(0).toUpperCase() + m.slice(1)} analysis`;
        }

        const stmtsElem = document.getElementById('stat-stmts');
        if (stmtsElem) stmtsElem.textContent = data.summary?.statement_count ?? 0;
        const colsElem = document.getElementById('stat-cols');
        if (colsElem) colsElem.textContent = data.summary?.column_version_count ?? 0;
        const nodesElem = document.getElementById('stat-nodes');
        if (nodesElem) nodesElem.textContent = data.summary?.pdg_node_count ?? 0;

        function setNavBadge(elem, count) {
          if (!elem) return;
          const num = Number(count);
          if (!num || num <= 0 || isNaN(num)) {
            elem.textContent = '';
            elem.classList.add('count-zero');
          } else {
            elem.textContent = String(num);
            elem.classList.remove('count-zero');
          }
        }

        setNavBadge(document.getElementById('count-prov'), data.provenance?.nodes?.length);
        setNavBadge(document.getElementById('count-pdg'), data.pdg?.nodes?.length);
        setNavBadge(document.getElementById('count-slice'), Object.keys(data.slices || {}).length);
        setNavBadge(document.getElementById('count-schema'), data.schema_evolution?.length);

        const astSummary = data.ast_explorer?.summary;
        const astCount = astSummary?.cst_node_count ?? data.ast_explorer?.cst?.child_count ?? 0;
        setNavBadge(document.getElementById('count-ast'), astCount);

        const unkElem = document.getElementById('stat-unk');
        if (unkElem) {
          if (data.summary?.contains_unknowns || data.analysis_status === 'uncertain') {
            unkElem.textContent = 'Uncertainty detected';
            unkElem.className = 'header-meta-item stat-unk-text';
          } else {
            unkElem.textContent = 'Known static';
            unkElem.className = 'header-meta-item stat-known-text';
          }
        }

        const statusDot = document.getElementById('header-status-dot');
        const statusText = document.getElementById('header-status-text');
        if (statusDot && statusText) {
          if (data.analysis_status === 'error') {
            statusDot.className = 'status-dot error';
            statusText.textContent = 'Analysis error';
          } else if (data.analysis_status === 'syntax_error') {
            statusDot.className = 'status-dot error';
            statusText.textContent = 'Syntax error';
          } else if (data.summary?.contains_unknowns || data.analysis_status === 'uncertain') {
            statusDot.className = 'status-dot uncertain';
            statusText.textContent = 'Analysis ready (uncertain)';
          } else {
            statusDot.className = 'status-dot';
            statusText.textContent = 'Analysis ready';
          }
        }
      }
      refreshHeaderAndMetrics();


      // ======================================================================
      // CANONICAL LOAD PROGRAM MODAL CONTROLLER
      // ======================================================================
      function updateLoadProgramStats() {
        if (!loadProgramInput || !loadProgramStats) return;
        const text = loadProgramInput.value;
        const lines = text ? text.split(/\r?\n/).length : 0;
        const chars = text.length;
        loadProgramStats.textContent = `${lines} lines · ${chars} chars`;
      }

      let loadModalHideTimer = null;
      function openLoadProgramModal(blank) {
        if (!loadProgramModal) return;
        if (loadProgramInput) {
          loadProgramInput.value = blank ? '' : (data.source_code || initialSource || '');
          updateLoadProgramStats();
        }
        if (loadProgramError) loadProgramError.style.display = 'none';
        clearTimeout(loadModalHideTimer);
        loadProgramModal.style.display = 'flex';
        void loadProgramModal.offsetWidth;
        loadProgramModal.classList.add('open');
        setTimeout(() => {
          loadProgramInput?.focus();
        }, 50);
      }

      function closeLoadProgramModal() {
        if (!loadProgramModal) return;
        loadProgramModal.classList.remove('open');
        clearTimeout(loadModalHideTimer);
        loadModalHideTimer = setTimeout(() => {
          if (!loadProgramModal.classList.contains('open')) loadProgramModal.style.display = 'none';
        }, prefersReducedMotion() ? 0 : 260);
        if (loadProgramError) loadProgramError.style.display = 'none';
      }

      if (loadProgramInput) {
        loadProgramInput.value = initialSource;
        updateLoadProgramStats();
      }

      loadProgramInput?.addEventListener('input', () => {
        updateLoadProgramStats();
        if (loadProgramError) loadProgramError.style.display = 'none';
      });

      btnCloseLoadProgram?.addEventListener('click', closeLoadProgramModal);
      btnCancelLoadProgram?.addEventListener('click', closeLoadProgramModal);

      loadProgramModal?.addEventListener('click', (e) => {
        if (e.target === loadProgramModal) {
          closeLoadProgramModal();
        }
      });

      btnLoadProgramFile?.addEventListener('click', () => loadProgramFileInput?.click());
      loadProgramFileInput?.addEventListener('change', (e) => {
        const file = e.target.files[0];
        if (!file) return;
        const reader = new FileReader();
        reader.onload = (evt) => {
          if (loadProgramInput) {
            loadProgramInput.value = evt.target.result;
            updateLoadProgramStats();
            if (loadProgramError) loadProgramError.style.display = 'none';
          }
        };
        reader.readAsText(file);
      });

      loadProgramExampleSelect?.addEventListener('change', (e) => {
        const val = e.target.value;
        if (!val) return;
        if (data.examples && data.examples[val]) {
          if (loadProgramInput) {
            loadProgramInput.value = data.examples[val];
            updateLoadProgramStats();
            if (loadProgramError) loadProgramError.style.display = 'none';
          }
        }
      });

      loadProgramInput?.addEventListener('keydown', (e) => {
        if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
          e.preventDefault();
          launchLoadProgramAnalysis();
        }
        if (e.key === 'Escape') {
          e.preventDefault();
          closeLoadProgramModal();
        }
      });

      function launchLoadProgramAnalysis() {
        if (!loadProgramInput) return;
        const code = loadProgramInput.value.trim();
        if (!code) {
          if (loadProgramError) {
            loadProgramError.textContent = 'Please enter or load R code to analyze.';
            loadProgramError.style.display = 'flex';
          }
          return;
        }
        if (loadProgramError) loadProgramError.style.display = 'none';

        if (btnLoadProgramAnalyze) {
          btnLoadProgramAnalyze.disabled = true;
          const txt = document.getElementById('btn-load-analyze-text');
          if (txt) txt.textContent = 'Analyzing...';
        }

        executeRealAnalysis(loadProgramInput.value, () => {
          closeLoadProgramModal();
          recomputeActiveGraphFit();
        });
      }

      btnLoadProgramAnalyze?.addEventListener('click', launchLoadProgramAnalysis);

      // Primary "New Analysis" Action in Header
      document.getElementById('btn-new-analysis')?.addEventListener('click', openLoadProgramModal);

      // Global shortcut ⌘N / Ctrl+N to open Load Program modal
      window.addEventListener('keydown', (e) => {
        if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'n') {
          e.preventDefault();
          openLoadProgramModal();
        }
      });

      // ======================================================================
      // PHASE 2: Real Analysis Engine Bridge & View Synchronization
      // ======================================================================
      function updateAllViewsAndMetrics(newData) {
        data = newData;

        if (sourceEditorTextarea) {
          sourceEditorTextarea.value = data.source_code || '';
        }
        if (loadProgramInput) {
          loadProgramInput.value = data.source_code || '';
          updateLoadProgramStats();
        }

        refreshHeaderAndMetrics();
        clearSelection();
        if (contextualInspector) {
          contextualInspector.classList.remove('open');
        }
        astInitializedReps = { cst: false, ir: false };
        astCollapsedNodeIds.clear();

        renderSourceViewer();

        const availableSlices = Object.keys(data.slices || {});
        if (!availableSlices.includes(activeSliceTarget)) {
          activeSliceTarget = availableSlices[0] || '';
        }
        const sliceSelect = document.getElementById('slice-target-select');
        if (sliceSelect) {
          sliceSelect.innerHTML = '';
          availableSlices.forEach(k => {
            const opt = document.createElement('option');
            opt.value = k;
            opt.textContent = k;
            if (k === activeSliceTarget) opt.selected = true;
            sliceSelect.appendChild(opt);
          });
        }

        if (activeView === 'provenance' && (!data.provenance?.nodes || data.provenance.nodes.length === 0) && data.pdg?.nodes?.length > 0) {
          selectDefaultView(data);
        } else {
          switchView(activeView);
        }

        recomputeActiveGraphFit();
        if (sourceStaleBanner) {
          sourceStaleBanner.style.display = 'none';
        }
      }

      function executeRealAnalysis(sourceCode, onCompleteTransition) {
        if (isAnalyzing) return;
        const trimmedCode = sourceCode.trim();
        if (!trimmedCode) {
          if (loadProgramError) {
            loadProgramError.textContent = 'Please enter or load R code to analyze.';
            loadProgramError.style.display = 'flex';
          }
          showStatusNotification('Source code cannot be empty.');
          return;
        }

        isAnalyzing = true;
        currentRevisionId += 1;
        const requestRev = currentRevisionId;

        if (btnLoadProgramAnalyze) {
          btnLoadProgramAnalyze.disabled = true;
          const txt = document.getElementById('btn-load-analyze-text');
          if (txt) txt.textContent = 'Analyzing...';
        }
        if (btnReanalyze) {
          btnReanalyze.disabled = true;
          btnReanalyze.textContent = 'Analyzing...';
        }
        const statusDot = document.getElementById('header-status-dot');
        const statusText = document.getElementById('header-status-text');
        if (statusDot && statusText) {
          statusDot.className = 'status-dot analyzing';
          statusText.textContent = 'Analyzing...';
        }
        hideStatusNotification();

        const endpoint = getApiEndpoint();
        const payload = {
          source: sourceCode,
          file_name: data.file_name || 'script.R',
          mode: (data.mode || 'static').toLowerCase(),
          criterion: activeSliceTarget || undefined,
          revision: requestRev
        };

        fetch(endpoint, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(payload)
        })
        .then(res => {
          if (!res.ok) {
            throw new Error(`HTTP ${res.status}: ${res.statusText}`);
          }
          return res.json();
        })
        .then(freshData => {
          if (freshData.revision && freshData.revision !== currentRevisionId) {
            return;
          }

          if (freshData.analysis_status === 'error') {
            data.source_code = sourceCode;
            data.source_lines = sourceCode.split(/\r?\n/);
            data.diagnostics = freshData.diagnostics || [];
            renderSourceViewer();
            showStatusNotification(`Analysis failed: ${freshData.error_message || 'Syntax/normalization error'}`);
          } else if (freshData.analysis_status === 'syntax_error') {
            updateAllViewsAndMetrics(freshData);
            showStatusNotification('Syntax error detected in R source.');
          } else {
            updateAllViewsAndMetrics(freshData);
          }

          if (onCompleteTransition) {
            onCompleteTransition();
          }
        })
        .catch(err => {
          showStatusNotification(
            `R-PARSE engine offline at ${endpoint}. Run 'rparse studio' to connect live analysis.`,
            true
          );
          if (loadProgramError) {
            loadProgramError.textContent = `Engine offline at ${endpoint}. Run 'rparse studio' to analyze.`;
            loadProgramError.style.display = 'flex';
          }
          data.source_code = sourceCode;
          data.source_lines = sourceCode.split(/\r?\n/);
          renderSourceViewer();
          if (sourceStaleBanner) {
            sourceStaleBanner.style.display = 'flex';
          }
          if (onCompleteTransition) {
            onCompleteTransition();
          }
        })
        .finally(() => {
          isAnalyzing = false;
          if (btnLoadProgramAnalyze) {
            btnLoadProgramAnalyze.disabled = false;
            const txt = document.getElementById('btn-load-analyze-text');
            if (txt) txt.textContent = 'Analyze Program';
          }
          if (btnReanalyze) {
            btnReanalyze.disabled = false;
            btnReanalyze.textContent = 'Reanalyze';
          }
        });
      }

      // ======================================================================
      // PHASE 3: 30/70 Workspace Layout & Splitter Resizing
      // ======================================================================
      let isResizingSource = false;
      splitterSource?.addEventListener('mousedown', (e) => {
        e.preventDefault();
        isResizingSource = true;
        splitterSource.classList.add('active');
        document.body.style.cursor = 'col-resize';
        document.body.style.userSelect = 'none';
      });

      window.addEventListener('mousemove', (e) => {
        if (isResizingSource) {
          const totalW = window.innerWidth;
          const pct = Math.max(22, Math.min(45, (e.clientX / totalW) * 100));
          sourcePane.style.width = pct + '%';
        }
      });

      window.addEventListener('mouseup', () => {
        if (isResizingSource) {
          isResizingSource = false;
          splitterSource?.classList.remove('active');
          document.body.style.cursor = '';
          document.body.style.userSelect = '';
          recomputeActiveGraphFit();
        }
      });

      // Splitter Keyboard & Touch Accessibility (ISSUE-P3-01)
      splitterSource?.addEventListener('keydown', (e) => {
        if (e.key === 'ArrowLeft' || e.key === 'ArrowRight') {
          e.preventDefault();
          const currentPct = parseFloat(sourcePane.style.width) || 30;
          const delta = e.key === 'ArrowLeft' ? -2 : 2;
          const nextPct = Math.max(18, Math.min(50, currentPct + delta));
          sourcePane.style.width = nextPct + '%';
          splitterSource.setAttribute('aria-valuenow', Math.round(nextPct));
          recomputeActiveGraphFit();
        }
      });

      splitterSource?.addEventListener('touchstart', (e) => {
        if (e.touches.length === 1) {
          isResizingSource = true;
          splitterSource.classList.add('active');
        }
      }, { passive: true });

      window.addEventListener('touchmove', (e) => {
        if (isResizingSource && e.touches.length === 1) {
          const totalW = window.innerWidth;
          const pct = Math.max(18, Math.min(50, (e.touches[0].clientX / totalW) * 100));
          sourcePane.style.width = pct + '%';
        }
      }, { passive: true });

      window.addEventListener('touchend', () => {
        if (isResizingSource) {
          isResizingSource = false;
          splitterSource?.classList.remove('active');
          recomputeActiveGraphFit();
        }
      });

      document.getElementById('btn-toggle-source')?.addEventListener('click', () => {
        if (!sourcePane) return;
        const collapsed = sourcePane.classList.toggle('source-collapsed');
        const toggle = document.getElementById('btn-toggle-source');
        if (toggle) {
          toggle.setAttribute('aria-pressed', collapsed ? 'true' : 'false');
          toggle.title = collapsed ? 'Show source code' : 'Hide source code';
        }
        window.dispatchEvent(new Event('resize'));
      });

      // Source Word Wrap Feature (Defaults to ON)
      let isWordWrapEnabled = true;
      const btnToggleWrap = document.getElementById('btn-toggle-wrap');

      function applyWordWrapState() {
        if (sourcePane) {
          sourcePane.classList.toggle('word-wrap-enabled', isWordWrapEnabled);
        }
        if (sourceEditorTextarea) {
          sourceEditorTextarea.setAttribute('wrap', isWordWrapEnabled ? 'soft' : 'off');
          sourceEditorTextarea.wrap = isWordWrapEnabled ? 'soft' : 'off';
        }
        if (btnToggleWrap) {
          btnToggleWrap.classList.toggle('active', isWordWrapEnabled);
          btnToggleWrap.setAttribute('aria-pressed', isWordWrapEnabled ? 'true' : 'false');
          btnToggleWrap.title = isWordWrapEnabled
            ? 'Word wrap is ON (click to disable and permit horizontal scroll)'
            : 'Word wrap is OFF (click to enable word wrap)';
        }
      }

      btnToggleWrap?.addEventListener('click', () => {
        isWordWrapEnabled = !isWordWrapEnabled;
        applyWordWrapState();
      });

      applyWordWrapState();

      // Source In-Place Editing & Stale Handling
      btnEditSource?.addEventListener('click', () => {
        isSourceEditMode = !isSourceEditMode;
        if (isSourceEditMode) {
          btnEditSource.classList.add('is-editing');
          btnEditSource.setAttribute('aria-pressed', 'true');
          btnEditSource.setAttribute('aria-label', 'View source code');
          btnEditSource.title = 'Switch to syntax-highlighted viewer';
          sourceContainer.style.display = 'none';
          sourceEditorTextarea.style.display = 'block';
          sourceEditorTextarea.value = data.source_code || '';
          sourceEditorTextarea.focus();
        } else {
          btnEditSource.classList.remove('is-editing');
          btnEditSource.setAttribute('aria-pressed', 'false');
          btnEditSource.setAttribute('aria-label', 'Edit source code');
          btnEditSource.title = 'Edit source code in place';
          sourceEditorTextarea.style.display = 'none';
          sourceContainer.style.display = 'block';
          renderSourceViewer();
        }
      });

      sourceEditorTextarea?.addEventListener('input', () => {
        const text = sourceEditorTextarea.value;
        const lineCount = text ? text.split(/\r?\n/).length : 0;
        const lcElem = document.getElementById('source-line-count');
        if (lcElem) lcElem.textContent = `${lineCount} lines`;

        if (text !== data.source_code) {
          sourceStaleBanner.style.display = 'flex';
        } else {
          sourceStaleBanner.style.display = 'none';
        }
      });

      sourceEditorTextarea?.addEventListener('keydown', (e) => {
        if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
          e.preventDefault();
          executeRealAnalysis(sourceEditorTextarea.value);
        }
      });

      btnReanalyze?.addEventListener('click', () => {
        const currentCode = isSourceEditMode ? sourceEditorTextarea.value : (data.source_code || entryInput.value);
        executeRealAnalysis(currentCode);
      });

      btnRevertSource?.addEventListener('click', () => {
        if (sourceEditorTextarea) sourceEditorTextarea.value = data.source_code || '';
        if (entryInput) entryInput.value = data.source_code || '';
        sourceStaleBanner.style.display = 'none';
        hideStatusNotification();
        renderSourceViewer();
      });

      // ======================================================================
      // PHASE 4: Intelligent Default View Selection
      // ======================================================================
      function selectDefaultView(data) {
        if (data.initial_selection?.view) {
          const v = data.initial_selection.view;
          if (v === 'provenance' && data.provenance?.nodes?.length) { switchView('provenance'); return; }
          if (v === 'pdg' && data.pdg?.nodes?.length) { switchView('pdg'); return; }
          if (v === 'slice' && Object.keys(data.slices || {}).length) { switchView('slice'); return; }
          if (v === 'schema' && data.schema_evolution?.length) { switchView('schema'); return; }
        }
        if (data.provenance?.nodes?.length > 0) {
          switchView('provenance');
        } else if (data.pdg?.nodes?.length > 0) {
          switchView('pdg');
        } else if (Object.keys(data.slices || {}).length > 0) {
          switchView('slice');
        } else if (data.schema_evolution?.length > 0) {
          switchView('schema');
        } else {
          switchView('provenance');
        }
      }

      // ======================================================================
      // PHASE 5: Contextual Slide-Out Inspector Overlay (NOT a 3rd Panel)
      // ======================================================================
      document.getElementById('btn-close-inspector')?.addEventListener('click', () => {
        contextualInspector.classList.remove('open');
        recomputeActiveGraphFit();
      });

      function recomputeActiveGraphFit() {
        setTimeout(() => {
          if (activeView === 'provenance') {
            fitGraphView('view-container-provenance', 'prov-root-group', provPositions, provTransform);
          }
        }, 60);
      }

      // Auto-pan on selection to ensure node is not occluded by docked inspector (ISSUE-P0-03)
      function ensureNodeVisibleOnCanvas(nodeId, graphType) {
        setTimeout(() => {
          const isPdg = graphType === 'pdg';
          const containerId = isPdg ? 'view-container-pdg' : 'view-container-provenance';
          const rootId = isPdg ? 'pdg-root-group' : 'prov-root-group';
          const posMap = isPdg ? pdgPositions : provPositions;
          const transformObj = isPdg ? pdgTransform : provTransform;
          const container = document.getElementById(containerId);
          const rootG = document.getElementById(rootId);
          if (!container || !rootG || !posMap) return;

          const parsedId = isNaN(Number(nodeId)) ? nodeId : Number(nodeId);
          const pos = posMap.get(parsedId) || posMap.get(nodeId);
          if (!pos) return;

          const cw = container.clientWidth;
          const ch = container.clientHeight;
          if (cw <= 0 || ch <= 0) return;

          const screenX = transformObj.x + pos.x * transformObj.scale;
          const screenY = transformObj.y + pos.y * transformObj.scale;
          const nodeW = pos.width * transformObj.scale;
          const nodeH = pos.height * transformObj.scale;

          let adjusted = false;
          const rightMargin = 40;
          const leftMargin = 30;
          const topMargin = 30;
          const bottomMargin = 30;

          if (screenX + nodeW > cw - rightMargin) {
            transformObj.x = Math.round(cw - rightMargin - (pos.x + pos.width) * transformObj.scale);
            adjusted = true;
          } else if (screenX < leftMargin) {
            transformObj.x = Math.round(leftMargin - pos.x * transformObj.scale);
            adjusted = true;
          }

          if (screenY + nodeH > ch - bottomMargin) {
            transformObj.y = Math.round(ch - bottomMargin - (pos.y + pos.height) * transformObj.scale);
            adjusted = true;
          } else if (screenY < topMargin) {
            transformObj.y = Math.round(topMargin - pos.y * transformObj.scale);
            adjusted = true;
          }

          if (adjusted) {
            rootG.setAttribute('transform', `translate(${transformObj.x}, ${transformObj.y}) scale(${transformObj.scale})`);
          }
        }, 30);
      }

      // Keyboard shortcuts follow the analysis tab order:
      // 1 AST Explorer, 2 PDG Graph, 3 Schema Evolution, 4 Provenance, 5 Backward Slice.
      window.addEventListener('keydown', (e) => {
        if (e.target.tagName === 'INPUT' || e.target.tagName === 'SELECT' || e.target.tagName === 'TEXTAREA') return;
        if (e.key === 'Escape') {
          if (contextualInspector.classList.contains('open')) {
            contextualInspector.classList.remove('open');
          } else {
            clearSelection();
          }
        } else if (e.key === '1') {
          switchView('ast');
        } else if (e.key === '2') {
          switchView('pdg');
        } else if (e.key === '3') {
          switchView('schema');
        } else if (e.key === '4') {
          switchView('provenance');
        } else if (e.key === '5') {
          switchView('slice');
        }
      });

      // Render Source Viewer
      function renderSourceViewer() {
        sourceContainer.innerHTML = '';
        const lines = data.source_lines || [];
        const lineCountElem = document.getElementById('source-line-count');
        if (lineCountElem) lineCountElem.textContent = `${lines.length} lines`;

        lines.forEach((lineText, idx) => {
          const lineNum = idx + 1;
          const row = document.createElement('div');
          row.className = 'code-line source-line';
          row.dataset.line = lineNum;
          row.setAttribute('data-line', lineNum);

          const numCell = document.createElement('div');
          numCell.className = 'line-num';
          numCell.textContent = lineNum;

          const textCell = document.createElement('div');
          textCell.className = 'line-text';
          textCell.innerHTML = syntaxHighlight(lineText) || '&nbsp;';

          row.appendChild(numCell);
          row.appendChild(textCell);

          row.addEventListener('click', () => {
            selectSourceLine(lineNum);
          });

          sourceContainer.appendChild(row);
        });

        updateSliceSourceHighlighting();
      }

      function updateSliceSourceHighlighting() {
        const isSlice = (activeView === 'slice');
        const curSlice = data.slices?.[activeSliceTarget];
        const keptLines = new Set();
        const partialLines = new Set();
        const dropLines = new Set();

        if (isSlice && curSlice && curSlice.statements) {
          curSlice.statements.forEach(st => {
            if (st.state === 'fully_retained') {
              keptLines.add(st.start_line);
            } else if (st.state === 'partially_retained') {
              partialLines.add(st.start_line);
            } else {
              dropLines.add(st.start_line);
            }
          });
        }

        document.querySelectorAll('.code-line, .source-line').forEach(el => {
          el.classList.remove('slice-kept', 'slice-partial', 'slice-dropped');
          if (isSlice && curSlice) {
            const lineNum = parseInt(el.dataset.line || el.getAttribute('data-line'), 10);
            if (keptLines.has(lineNum)) {
              el.classList.add('slice-kept');
            } else if (partialLines.has(lineNum)) {
              el.classList.add('slice-partial');
            } else if (dropLines.has(lineNum)) {
              el.classList.add('slice-dropped');
            }
          }
        });
      }

      function syntaxHighlight(code) {
        if (!code) return '';
        const escaped = code.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
        return escaped.replace(
          /(#.*$)|("(?:\\.|[^"\\])*"|'(?:\\.|[^'\\])*')|\b(function|if|else|for|while|in|return|repeat|break|next)\b|\b(data\.frame|filter|mutate|select|transmute|rename|summarize|arrange)\b|\b(\d+(?:\.\d+)?)\b/g,
          (match, cmt, str, kw, fn, num) => {
            if (cmt) return `<span class="cmt">${cmt}</span>`;
            if (str) return `<span class="str">${str}</span>`;
            if (kw) return `<span class="kw">${kw}</span>`;
            if (fn) return `<span class="fn">${fn}</span>`;
            if (num) return `<span class="num">${num}</span>`;
            return match;
          }
        );
      }

      function highlightSourceSpan(span) {
        if (!span || !span.start_line) return;
        document.querySelectorAll('.code-line, .source-line').forEach(el => el.classList.remove('active-line', 'line-selected', 'highlighted-span'));
        const startLine = span.start_line;
        const targetRow = document.querySelector(`.code-line[data-line="${startLine}"], .source-line[data-line="${startLine}"]`);
        if (targetRow) {
          targetRow.classList.add('highlighted-span');
          targetRow.scrollIntoView({ behavior: scrollBehavior(), block: 'center' });
        }
      }

      function selectSourceLine(lineNum) {
        document.querySelectorAll('.code-line, .source-line').forEach(el => el.classList.remove('active-line', 'line-selected'));
        const row = document.querySelector(`.code-line[data-line="${lineNum}"], .source-line[data-line="${lineNum}"]`);
        if (row) row.classList.add('active-line', 'line-selected');

        // Find PDG nodes or schema states on this line
        const matchingNode = data.pdg?.nodes?.find(n => n.span && n.span.start_line === lineNum);
        const matchingSchema = data.schema_evolution?.find(s => s.span && s.span.start_line === lineNum);

        if (matchingNode) {
          showInspector({
            title: matchingNode.label,
            type: matchingNode.kind,
            id: matchingNode.node_id,
            span: matchingNode.span,
            certainty: matchingNode.is_unknown ? 'UNKNOWN_STATIC' : 'KNOWN_STATIC',
            details: matchingNode.details
          });
        } else if (matchingSchema) {
          showInspector({
            title: `${matchingSchema.symbol_name}@v${matchingSchema.version}`,
            type: 'DataFrameState',
            id: `df_${matchingSchema.state_id}`,
            span: matchingSchema.span,
            certainty: matchingSchema.knowledge,
            details: matchingSchema
          });
        }
      }

      // Inspector Overview and Node Inspector
      function renderOverviewCard() {
        inspectorContainer.innerHTML = '';
        const badge = document.getElementById('inspector-status-badge');
        if (badge) badge.textContent = 'Overview';

        const card = document.createElement('div');
        card.className = 'card';
        card.innerHTML = `
          <div style="font-size:13px; font-weight:700; color:var(--text-main); margin-bottom:2px;">Pipeline Overview</div>
          <div style="font-size:11px; color:var(--text-dim); margin-bottom:10px;">Deterministic Static Analysis Model</div>
          
          <table class="prop-table" style="margin-bottom:12px;">
            <tr><td class="key">Target File</td><td class="val">${escapeHtml(data.file_name || 'script.R')}</td></tr>
            <tr><td class="key">Analysis Mode</td><td class="val">${escapeHtml(data.mode || 'STATIC')}</td></tr>
            <tr><td class="key">Statements</td><td class="val">${data.summary?.statement_count || 0}</td></tr>
            <tr><td class="key">Dataframe States</td><td class="val">${data.summary?.dataframe_state_count || 0}</td></tr>
            <tr><td class="key">Column Versions</td><td class="val">${data.summary?.column_version_count || 0}</td></tr>
            <tr><td class="key">PDG Nodes / Edges</td><td class="val">${data.summary?.pdg_node_count || 0} / ${data.summary?.pdg_edge_count || 0}</td></tr>
            <tr><td class="key">Column Derivations</td><td class="val">${data.summary?.derivation_count || 0}</td></tr>
            <tr><td class="key">Uncertainty State</td><td class="val" style="color:${data.summary?.contains_unknowns ? 'var(--status-warn)' : 'var(--status-ok)'};">${data.summary?.contains_unknowns ? 'UNCERTAINTY DETECTED' : 'KNOWN STATIC'}</td></tr>
          </table>

          <div style="padding-top:8px; border-top:1px solid var(--border);">
            <div style="font-size:10px; font-weight:700; color:var(--text-dim); text-transform:uppercase; margin-bottom:4px;">Workspace Navigation</div>
            <div style="font-size:11px; color:var(--text-muted); line-height:1.4;">
              Click any node in the graph, statement in the slice view, or step in schema evolution to inspect lineage, certainty, and dependencies.
            </div>
          </div>
        `;
        inspectorContainer.appendChild(card);
      }

      function showInspector(item) {
        selectedItem = item;
        inspectorContainer.innerHTML = '';
        if (!item) {
          renderOverviewCard();
          return;
        }

        const badge = document.getElementById('inspector-status-badge');
        if (badge) badge.textContent = item.type || 'Selected';

        const card = document.createElement('div');
        card.className = 'card';

        let actionBtns = '';
        if (item.span && item.span.start_line) {
          actionBtns += `<button class="btn" style="width:100%; justify-content:center; margin-top:6px;" onclick="window.rparseJumpSpan(${item.span.start_line})"><svg class="ui-icon" width="15" height="15" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M2.5 4h5.5M2.5 8h3.5M2.5 12h5.5"/><path d="M13.5 8H9.2"/><path d="M11 5.8 8.8 8 11 10.2"/></svg>Jump to Source (Line ${item.span.start_line})</button>`;
        }
        if (item.sliceTarget) {
          actionBtns += `<button class="btn btn-primary" style="width:100%; justify-content:center; margin-top:6px; font-size:11.5px; padding:0 8px; white-space:nowrap; overflow:hidden; text-overflow:ellipsis;" onclick="window.rparseSelectSlice('${escapeHtml(item.sliceTarget)}')"><svg class="ui-icon" width="15" height="15" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" aria-hidden="true"><path d="M3 13L13 3M8 3h5v5M3 8v5h5"/></svg>Compute Backward Slice for ${escapeHtml(item.sliceTarget)}</button>`;
        }

        let upstreamHtml = '';
        if (item.upstream && item.upstream.length > 0) {
          upstreamHtml = `
            <div class="inspector-section">
              <div class="inspector-section-title">
                <span>Upstream Dependencies (Incoming)</span>
                <span class="badge-count">${item.upstream.length}</span>
              </div>
              <div class="conn-list">
                ${item.upstream.map(u => `
                  <div class="conn-item" onclick="window.rparseSelectNode('${escapeHtml(String(u.id))}', '${activeView}')" title="Click to inspect ${escapeHtml(u.label)}">
                    <span class="conn-label">${escapeHtml(u.label)}</span>
                    <span class="conn-pill" style="background:var(--orange-tint); color:var(--orange-text);">${escapeHtml(u.edgeKind || u.type || 'dep')}</span>
                  </div>
                `).join('')}
              </div>
            </div>
          `;
        } else if (item.graphNode) {
          upstreamHtml = `
            <div class="inspector-section">
              <div class="inspector-section-title">Upstream Dependencies</div>
              <div style="font-size:11px; color:var(--text-dim); font-style:italic; padding:4px 0;">(None — Root Origin Node)</div>
            </div>
          `;
        }

        let downstreamHtml = '';
        if (item.downstream && item.downstream.length > 0) {
          downstreamHtml = `
            <div class="inspector-section">
              <div class="inspector-section-title">
                <span>Downstream Dependents (Outgoing)</span>
                <span class="badge-count">${item.downstream.length}</span>
              </div>
              <div class="conn-list">
                ${item.downstream.map(d => `
                  <div class="conn-item" onclick="window.rparseSelectNode('${escapeHtml(String(d.id))}', '${activeView}')" title="Click to inspect ${escapeHtml(d.label)}">
                    <span class="conn-label">${escapeHtml(d.label)}</span>
                    <span class="conn-pill" style="background:var(--teal-tint); color:var(--teal-text);">${escapeHtml(d.edgeKind || d.type || 'dep')}</span>
                  </div>
                `).join('')}
              </div>
            </div>
          `;
        } else if (item.graphNode) {
          downstreamHtml = `
            <div class="inspector-section">
              <div class="inspector-section-title">Downstream Dependents</div>
              <div style="font-size:11px; color:var(--text-dim); font-style:italic; padding:4px 0;">(None — Terminal Consumer Node)</div>
            </div>
          `;
        }

        let propRows = `<tr><td class="key">Identifier</td><td class="val">${escapeHtml(String(item.id ?? '—'))}</td></tr>`;
        if (item.span) {
          propRows += `<tr><td class="key">Source Span</td><td class="val">Line ${item.span.start_line}:${item.span.start_col} – Line ${item.span.end_line}:${item.span.end_col}</td></tr>`;
          propRows += `<tr><td class="key">Byte Offset</td><td class="val">${item.span.start_byte} .. ${item.span.end_byte}</td></tr>`;
        }

        if (item.details) {
          for (const [k, v] of Object.entries(item.details)) {
            if (typeof v === 'object' && v !== null) continue;
            const lk = k.toLowerCase();
            if (lk === 'id' || lk === 'node_id' || lk === 'label' || (lk === 'name' && item.title === String(v))) continue;
            propRows += `<tr><td class="key">${escapeHtml(k)}</td><td class="val">${escapeHtml(String(v))}</td></tr>`;
          }
        }

        card.innerHTML = `
          <div style="font-size:13px; font-weight:700; color:var(--text-main); margin-bottom:4px; word-break:break-all;">${escapeHtml(item.title || item.label || 'Details')}</div>
          <div style="display:flex; gap:6px; margin-bottom:8px; flex-wrap:wrap;">
            <span class="pill pill-neutral">${escapeHtml(item.type || 'Object')}</span>
            <span class="pill ${String(item.certainty || '').includes('UNKNOWN') ? 'pill-partial' : 'pill-keep'}">${escapeHtml(item.certainty || 'KNOWN_STATIC')}</span>
          </div>

          ${actionBtns}

          <div class="inspector-section">
            <div class="inspector-section-title">Node Metadata</div>
            <table class="prop-table">
              ${propRows}
            </table>
          </div>

          ${upstreamHtml}
          ${downstreamHtml}

          <div class="inspector-section">
            <details>
              <summary style="font-size:10px; color:var(--text-dim); text-transform:uppercase; cursor:pointer; font-weight:700;">Raw JSON</summary>
              <pre class="raw-json" style="margin-top:6px;">${escapeHtml(JSON.stringify(item, null, 2))}</pre>
            </details>
          </div>
        `;

        inspectorContainer.appendChild(card);
        contextualInspector.classList.remove('collapsed');
        contextualInspector.classList.add('open');
        recomputeActiveGraphFit();

        if (item.span) {
          highlightSourceSpan(item.span);
        }
      }

      function escapeHtml(s) {
        return String(s).replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;');
      }

      // Graph Layout Helpers
      function cleanNodeLabel(n, graphType) {
        if (graphType === 'provenance') {
          return n.label || n.transform_type || n.id;
        }
        if (n.kind === 'ColumnVersion') {
          const m = (n.label || '').match(/ColumnPort\((.*?)\)/);
          return m ? m[1] : (n.label || n.node_id);
        }
        if (n.kind === 'VariableDef') {
          return n.details?.name ? `def ${n.details.name}` : (n.label || n.node_id);
        }
        if (n.kind === 'VariableUse') {
          return n.details?.name ? `use ${n.details.name}` : (n.label || n.node_id);
        }
        if (n.kind === 'Statement') {
          return n.details?.description ? n.details.description : (n.label || n.node_id);
        }
        if (n.kind === 'Expression') {
          return n.details?.description ? n.details.description : (n.label || n.node_id);
        }
        return n.label || n.node_id;
      }

      function getNodeSize(n, graphType) {
        if (graphType === 'provenance') {
          if (n.type === 'column' || n.type === 'column_version') {
            return { width: 220, height: 46 };
          }
          return { width: 240, height: 50 };
        }
        const label = cleanNodeLabel(n, 'pdg');
        const shown = label.length > 32 ? label.slice(0, 30) + '…' : label;
        const lineStr = n.span?.start_line ? ` L${n.span.start_line}` : '';
        const kindLine = `${String(n.kind || 'NODE').toUpperCase()}${lineStr}`;
        const chars = Math.max(kindLine.length, shown.length);
        const width = Math.max(168, Math.min(252, Math.round(40 + chars * 7.1)));
        const height = n.kind === 'Statement' ? 52 : 46;
        return { width, height };
      }

      function computeDagLayout(nodes, edges, options) {
        const orientation = options.orientation || 'TB';
        const isTB = orientation === 'TB';
        const graphType = options.graphType || 'pdg';

        if (!nodes || nodes.length === 0) {
          return { positions: new Map(), bounds: { minX: 0, minY: 0, maxX: 0, maxY: 0 }, backEdges: new Set() };
        }

        const nodeIds = nodes.map(n => n.id);
        const nodeMap = new Map();
        nodes.forEach(n => nodeMap.set(n.id, n));

        const adj = new Map();
        const inAdj = new Map();
        nodeIds.forEach(id => {
          adj.set(id, []);
          inAdj.set(id, []);
        });

        edges.forEach(e => {
          if (adj.has(e.from) && adj.has(e.to)) {
            adj.get(e.from).push(e.to);
            inAdj.get(e.to).push(e.from);
          }
        });

        const visited = new Map();
        const backEdges = new Set();
        function dfs(u) {
          visited.set(u, 1);
          const nbrs = adj.get(u) || [];
          for (const v of nbrs) {
            if (visited.get(v) === 1) {
              backEdges.add(`${u}->${v}`);
            } else if (!visited.has(v)) {
              dfs(v);
            }
          }
          visited.set(u, 2);
        }

        nodeIds.forEach(id => {
          if (!visited.has(id)) dfs(id);
        });

        const dagAdj = new Map();
        const dagInDeg = new Map();
        nodeIds.forEach(id => {
          dagAdj.set(id, []);
          dagInDeg.set(id, 0);
        });

        edges.forEach(e => {
          if (adj.has(e.from) && adj.has(e.to)) {
            if (!backEdges.has(`${e.from}->${e.to}`)) {
              dagAdj.get(e.from).push(e.to);
              dagInDeg.set(e.to, dagInDeg.get(e.to) + 1);
            }
          }
        });

        const ranks = new Map();
        const useLineBands = options.useLineBands && nodes.some(n => n.kind === 'Statement');

        if (useLineBands) {
          const lines = Array.from(new Set(nodes.map(n => n.span?.start_line).filter(l => typeof l === 'number'))).sort((a, b) => a - b);
          const lineRankMap = new Map();
          lines.forEach((l, idx) => lineRankMap.set(l, idx * 3));

          nodeIds.forEach(id => {
            const n = nodeMap.get(id);
            const sl = n.span?.start_line;
            const baseR = (typeof sl === 'number' && lineRankMap.has(sl)) ? lineRankMap.get(sl) : 0;
            if (n.kind === 'Statement') {
              ranks.set(id, baseR + 1);
            } else if (n.kind === 'VariableUse') {
              ranks.set(id, baseR);
            } else {
              ranks.set(id, baseR + 2);
            }
          });

          let changed = true;
          let iters = 0;
          while (changed && iters < 80) {
            changed = false;
            iters++;
            for (const u of nodeIds) {
              const uR = ranks.get(u);
              for (const v of dagAdj.get(u)) {
                if (ranks.get(v) <= uR) {
                  ranks.set(v, uR + 1);
                  changed = true;
                }
              }
            }
          }
        } else {
          const queue = [];
          nodeIds.forEach(id => {
            if (dagInDeg.get(id) === 0) {
              queue.push(id);
              ranks.set(id, 0);
            }
          });

          const inDegCopy = new Map(dagInDeg);
          while (queue.length > 0) {
            const u = queue.shift();
            const uR = ranks.get(u) || 0;
            for (const v of dagAdj.get(u)) {
              ranks.set(v, Math.max(ranks.get(v) || 0, uR + 1));
              inDegCopy.set(v, inDegCopy.get(v) - 1);
              if (inDegCopy.get(v) === 0) {
                queue.push(v);
              }
            }
          }

          nodeIds.forEach(id => {
            if (!ranks.has(id)) ranks.set(id, 0);
          });
        }

        const uniqueRanks = Array.from(new Set(ranks.values())).sort((a, b) => a - b);
        const rankCompressMap = new Map();
        uniqueRanks.forEach((r, idx) => rankCompressMap.set(r, idx));
        nodeIds.forEach(id => ranks.set(id, rankCompressMap.get(ranks.get(id))));

        const layers = new Map();
        nodeIds.forEach(id => {
          const r = ranks.get(id);
          if (!layers.has(r)) layers.set(r, []);
          layers.get(r).push(id);
        });

        const posInLayer = new Map();
        const sortedRanks = Array.from(layers.keys()).sort((a, b) => a - b);
        const orderingPasses = Math.max(1, Math.min(6, options.orderingPasses || 1));

        function orderLayer(r, direction) {
          const layerNodes = layers.get(r);
          if (direction === 'down' && r === 0) {
            layerNodes.forEach((nid, idx) => posInLayer.set(nid, idx));
            return;
          }
          layerNodes.sort((a, b) => {
            const score = (nid) => {
              const raw = direction === 'down' ? (inAdj.get(nid) || []) : (adj.get(nid) || []);
              const linked = raw.filter(p => {
                const key = direction === 'down' ? `${p}->${nid}` : `${nid}->${p}`;
                return !backEdges.has(key) && posInLayer.has(p);
              });
              if (linked.length === 0) return 0;
              return linked.reduce((sum, p) => sum + posInLayer.get(p), 0) / linked.length;
            };
            const da = score(a);
            const db = score(b);
            if (da === db && orderingPasses > 1) {
              return String(a) < String(b) ? -1 : String(a) > String(b) ? 1 : 0;
            }
            return da - db;
          });
          layerNodes.forEach((nid, idx) => posInLayer.set(nid, idx));
        }

        sortedRanks.forEach(r => orderLayer(r, 'down'));
        if (orderingPasses > 1) {
          for (let pass = 1; pass < orderingPasses; pass++) {
            const dir = pass % 2 === 1 ? 'up' : 'down';
            const seq = dir === 'down' ? sortedRanks : sortedRanks.slice().reverse();
            seq.forEach(r => orderLayer(r, dir));
          }
        }

        const positions = new Map();
        const requestedSpacing = options.layerSpacing || (isTB ? 96 : 280);
        const nodeSpacing = options.nodeSpacing || (isTB ? 24 : 20);
        let maxMainExtent = 0;
        nodeIds.forEach(id => {
          const sz = getNodeSize(nodeMap.get(id), graphType);
          maxMainExtent = Math.max(maxMainExtent, isTB ? sz.height : sz.width);
        });
        const rankGutter = isTB ? 64 : 56;
        const layerSpacing = requestedSpacing > maxMainExtent ? requestedSpacing : (maxMainExtent + rankGutter);

        let maxCrossSpan = 0;
        const layerSpans = new Map();

        sortedRanks.forEach(r => {
          const layerNodes = layers.get(r);
          let span = 0;
          layerNodes.forEach((nid, idx) => {
            const n = nodeMap.get(nid);
            const sz = getNodeSize(n, graphType);
            span += (isTB ? sz.width : sz.height);
            if (idx > 0) span += nodeSpacing;
          });
          layerSpans.set(r, span);
          if (span > maxCrossSpan) maxCrossSpan = span;
        });

        sortedRanks.forEach(r => {
          const layerNodes = layers.get(r);
          const totalSpan = layerSpans.get(r) || 0;
          const crossOffset = Math.max(0, (maxCrossSpan - totalSpan) / 2);

          let currentCross = crossOffset;
          const mainCoord = r * layerSpacing;

          layerNodes.forEach(nid => {
            const n = nodeMap.get(nid);
            const sz = getNodeSize(n, graphType);

            let x, y;
            if (isTB) {
              x = currentCross;
              y = mainCoord;
              currentCross += sz.width + nodeSpacing;
            } else {
              x = mainCoord;
              y = currentCross;
              currentCross += sz.height + nodeSpacing;
            }

            positions.set(nid, {
              x,
              y,
              width: sz.width,
              height: sz.height,
              data: n,
              rank: r
            });
          });
        });

        let minX = Infinity, minY = Infinity, maxX = -Infinity, maxY = -Infinity;
        positions.forEach(p => {
          if (p.x < minX) minX = p.x;
          if (p.y < minY) minY = p.y;
          if (p.x + p.width > maxX) maxX = p.x + p.width;
          if (p.y + p.height > maxY) maxY = p.y + p.height;
        });

        return { positions, bounds: { minX, minY, maxX, maxY }, backEdges };
      }

      function fitGraphView(containerId, rootGroupId, nodePositions, transformObj) {
        const container = document.getElementById(containerId);
        const rootGroup = document.getElementById(rootGroupId);
        if (!container || !rootGroup || !nodePositions || nodePositions.size === 0) return;

        const cw = container.clientWidth || 800;
        const ch = container.clientHeight || 600;

        let minX = Infinity, minY = Infinity, maxX = -Infinity, maxY = -Infinity;
        nodePositions.forEach(p => {
          if (p.x < minX) minX = p.x;
          if (p.y < minY) minY = p.y;
          if (p.x + p.width > maxX) maxX = p.x + p.width;
          if (p.y + p.height > maxY) maxY = p.y + p.height;
        });

        const padding = 48;
        const gw = Math.max(80, (maxX - minX) + padding * 2);
        const gh = Math.max(80, (maxY - minY) + padding * 2);

        const scaleX = (cw - 32) / gw;
        const scaleY = (ch - 32) / gh;
        const optimalScale = Math.min(1.15, Math.max(0.18, Math.min(scaleX, scaleY)));

        const graphRenderW = (maxX - minX) * optimalScale;
        const graphRenderH = (maxY - minY) * optimalScale;

        transformObj.scale = optimalScale;
        transformObj.x = Math.round((cw - graphRenderW) / 2 - minX * optimalScale);
        transformObj.y = Math.round((ch - graphRenderH) / 2 - minY * optimalScale);

        rootGroup.setAttribute('transform', `translate(${transformObj.x}, ${transformObj.y}) scale(${transformObj.scale})`);
      }

      function clearSelection() {
        document.querySelectorAll('.svg-node').forEach(el => {
          el.classList.remove('selected', 'connected-upstream', 'connected-downstream', 'dimmed');
        });
        document.querySelectorAll('.svg-edge').forEach(el => {
          el.classList.remove('highlighted', 'inbound', 'outbound', 'dimmed');
        });
        document.querySelectorAll('.code-line').forEach(el => {
          el.classList.remove('active-line', 'highlighted-span');
        });
        const searchInput = document.getElementById('prov-search');
        if (searchInput && searchInput.value) {
          searchInput.value = '';
          const clearBtn = document.getElementById('btn-prov-clear-search');
          if (clearBtn) clearBtn.style.display = 'none';
        }
        selectedItem = null;
        renderOverviewCard();
        const leaveNeighborhood = pdgNeighborhood;
        pdgSelectedId = null;
        if (leaveNeighborhood) {
          pdgNeighborhood = false;
          syncPdgNeighborhoodButton();
          if (activeView === 'pdg') renderPdgGraph();
        }
      }

      function setupZoomPan(containerId, svgId, rootGroupId, transformObj, hooks) {
        const container = document.getElementById(containerId);
        const svg = document.getElementById(svgId);
        if (!container || !svg) return;

        let isDragging = false;
        let dragMoved = false;
        let startX = 0, startY = 0;
        let downX = 0, downY = 0;

        function updateTransform() {
          const g = document.getElementById(rootGroupId);
          if (g) {
            g.setAttribute('transform', `translate(${transformObj.x}, ${transformObj.y}) scale(${transformObj.scale})`);
          }
        }

        container.addEventListener('mousedown', (e) => {
          if (e.target.closest('.svg-node') || e.target.closest('button') || e.target.closest('input')) return;
          isDragging = true;
          dragMoved = false;
          downX = e.clientX;
          downY = e.clientY;
          startX = e.clientX - transformObj.x;
          startY = e.clientY - transformObj.y;
          container.style.cursor = 'grabbing';
        });

        window.addEventListener('mousemove', (e) => {
          if (!isDragging) return;
          if (Math.hypot(e.clientX - downX, e.clientY - downY) > 4) dragMoved = true;
          transformObj.x = e.clientX - startX;
          transformObj.y = e.clientY - startY;
          updateTransform();
          if (hooks && hooks.onTransform) hooks.onTransform();
        });

        window.addEventListener('mouseup', () => {
          if (isDragging) {
            isDragging = false;
            container.style.cursor = 'grab';
          }
        });

        container.addEventListener('wheel', (e) => {
          e.preventDefault();
          const rect = container.getBoundingClientRect();
          const mouseX = e.clientX - rect.left;
          const mouseY = e.clientY - rect.top;

          const zoomFactor = e.deltaY < 0 ? 1.15 : 0.85;
          const newScale = Math.max(0.15, Math.min(3.5, transformObj.scale * zoomFactor));

          transformObj.x = mouseX - (mouseX - transformObj.x) * (newScale / transformObj.scale);
          transformObj.y = mouseY - (mouseY - transformObj.y) * (newScale / transformObj.scale);
          transformObj.scale = newScale;

          updateTransform();
          if (hooks && hooks.onTransform) hooks.onTransform();
        }, { passive: false });

        svg.addEventListener('click', (e) => {
          if (dragMoved) {
            dragMoved = false;
            return;
          }
          if (!e.target.closest('.svg-node')) {
            clearSelection();
          }
        });

        return updateTransform;
      }

      // Tab Navigation
      navTabs.forEach(tab => {
        tab.addEventListener('click', () => {
          const view = tab.dataset.view;
          switchView(view);
        });
      });

      function switchView(view) {
        activeView = view;
        navTabs.forEach(t => t.classList.toggle('active', t.dataset.view === view));

        ['provenance', 'pdg', 'slice', 'schema', 'ast'].forEach(v => {
          const tb = document.getElementById(`toolbar-${v}`);
          const vc = document.getElementById(`view-container-${v}`);
          if (tb) {
            tb.style.display = (v === view) ? 'flex' : 'none';
            if (v === view) tb.classList.add('view-animate-in');
          }
          if (vc) {
            vc.style.display = (v === view) ? (v === 'provenance' || v === 'pdg' ? 'block' : 'flex') : 'none';
            if (v === view) {
              vc.classList.remove('view-animate-in');
              void vc.offsetWidth;
              vc.classList.add('view-animate-in');
            }
          }
        });

        if (view === 'provenance') {
          renderProvenanceGraph();
        } else if (view === 'pdg') {
          renderPdgGraph();
        } else if (view === 'slice') {
          renderSliceInspector();
        } else if (view === 'schema') {
          renderSchemaEvolution();
        } else if (view === 'ast') {
          renderAstExplorer();
        }
        updateSliceSourceHighlighting();
      }

      // ======================================================================
      // View A: Column Provenance Explorer (SVG Layered Graph)
      // ======================================================================
      let provTransform = { x: 40, y: 40, scale: 1 };
      let provPositions = new Map();

      function renderProvenanceGraph() {
        const svg = document.getElementById('svg-provenance');
        const defs = svg.querySelector('defs');
        const defsHtml = defs ? defs.outerHTML : '';
        svg.innerHTML = defsHtml;

        const nodes = data.provenance?.nodes || [];
        const edges = data.provenance?.edges || [];

        if (nodes.length === 0) {
          svg.innerHTML += '<text class="svg-graph-empty" x="50%" y="50%" text-anchor="middle">No column provenance derivations available.</text>';
          return;
        }

        const layout = computeDagLayout(nodes, edges, {
          orientation: 'LR',
          useLineBands: false,
          layerSpacing: 280,
          nodeSpacing: 24,
          graphType: 'provenance'
        });

        provPositions = layout.positions;

        const g = document.createElementNS('http://www.w3.org/2000/svg', 'g');
        g.setAttribute('id', 'prov-root-group');
        g.setAttribute('transform', `translate(${provTransform.x}, ${provTransform.y}) scale(${provTransform.scale})`);

        edges.forEach((e, idx) => {
          const fromPos = provPositions.get(e.from);
          const toPos = provPositions.get(e.to);
          if (!fromPos || !toPos) return;

          const p1 = { x: fromPos.x + fromPos.width, y: fromPos.y + fromPos.height / 2 };
          const p2 = { x: toPos.x, y: toPos.y + toPos.height / 2 };

          const dx = Math.max(30, (p2.x - p1.x) / 2);
          const path = document.createElementNS('http://www.w3.org/2000/svg', 'path');
          path.setAttribute('d', `M ${p1.x} ${p1.y} C ${p1.x + dx} ${p1.y}, ${p2.x - dx} ${p2.y}, ${p2.x} ${p2.y}`);
          path.dataset.from = e.from;
          path.dataset.to = e.to;
          path.dataset.edgeId = idx;

          const isInput = (e.type === 'input_column');
          path.setAttribute('class', isInput ? 'svg-edge edge-data' : 'svg-edge edge-colver');
          path.setAttribute('marker-end', isInput ? 'url(#arrow-prov-data)' : 'url(#arrow-prov-colver)');
          g.appendChild(path);
        });

        provPositions.forEach((pos, id) => {
          const n = pos.data;
          const nodeG = document.createElementNS('http://www.w3.org/2000/svg', 'g');
          nodeG.setAttribute('class', `svg-node`);
          nodeG.setAttribute('transform', `translate(${pos.x}, ${pos.y})`);
          nodeG.dataset.nodeId = n.id;

          const isCol = (n.type === 'column' || n.type === 'column_version');

          const title = document.createElementNS('http://www.w3.org/2000/svg', 'title');
          title.textContent = `${isCol ? 'Column' : 'Derivation'}: ${n.label || n.transform_type || n.id}`;
          nodeG.appendChild(title);

          const rect = document.createElementNS('http://www.w3.org/2000/svg', 'rect');
          rect.setAttribute('width', pos.width);
          rect.setAttribute('height', pos.height);
          rect.setAttribute('rx', isCol ? 23 : 6);
          rect.setAttribute('class', isCol ? 'node-col' : 'node-deriv');
          nodeG.appendChild(rect);

          const textKind = document.createElementNS('http://www.w3.org/2000/svg', 'text');
          textKind.setAttribute('x', isCol ? 16 : 12);
          textKind.setAttribute('y', 17);
          textKind.setAttribute('font-size', '11');
          textKind.setAttribute('font-weight', '700');
          textKind.style.fill = isCol ? 'var(--orange-text)' : 'var(--teal-text)';
          textKind.textContent = isCol ? 'COLUMN VERSION' : `DERIVATION (${n.knowledge || 'KNOWN'})`;
          nodeG.appendChild(textKind);

          const textLabel = document.createElementNS('http://www.w3.org/2000/svg', 'text');
          textLabel.setAttribute('x', isCol ? 16 : 12);
          textLabel.setAttribute('y', 34);
          textLabel.setAttribute('font-size', '12');
          textLabel.setAttribute('font-weight', '600');
          textLabel.style.fill = 'var(--ink)';
          const lbl = n.label || n.transform_type || id;
          textLabel.textContent = lbl.length > 25 ? lbl.slice(0, 23) + '…' : lbl;
          nodeG.appendChild(textLabel);

          nodeG.addEventListener('click', (evt) => {
            evt.stopPropagation();

            document.querySelectorAll('#prov-root-group .svg-node').forEach(el => {
              el.classList.remove('selected', 'connected-upstream', 'connected-downstream', 'dimmed');
            });
            document.querySelectorAll('#prov-root-group .svg-edge').forEach(el => {
              el.classList.remove('highlighted', 'inbound', 'outbound', 'dimmed');
            });

            nodeG.classList.add('selected');

            const inboundEdges = edges.filter(e => e.to === n.id);
            const outboundEdges = edges.filter(e => e.from === n.id);
            const upstreamNodeIds = new Set(inboundEdges.map(e => e.from));
            const downstreamNodeIds = new Set(outboundEdges.map(e => e.to));

            document.querySelectorAll('#prov-root-group .svg-edge').forEach(el => {
              const f = el.dataset.from;
              const t = el.dataset.to;
              if (t === n.id) el.classList.add('highlighted', 'inbound');
              else if (f === n.id) el.classList.add('highlighted', 'outbound');
              else el.classList.add('dimmed');
            });

            document.querySelectorAll('#prov-root-group .svg-node').forEach(el => {
              const nid = el.dataset.nodeId;
              if (nid === n.id) {
                // selected
              } else if (upstreamNodeIds.has(nid)) {
                el.classList.add('connected-upstream');
              } else if (downstreamNodeIds.has(nid)) {
                el.classList.add('connected-downstream');
              } else {
                el.classList.add('dimmed');
              }
            });

            const upstreamList = inboundEdges.map(e => {
              const parentNode = nodes.find(x => x.id === e.from);
              return {
                id: e.from,
                label: parentNode?.label || parentNode?.transform_type || e.from,
                type: parentNode?.type,
                edgeKind: e.type
              };
            });

            const downstreamList = outboundEdges.map(e => {
              const childNode = nodes.find(x => x.id === e.to);
              return {
                id: e.to,
                label: childNode?.label || childNode?.transform_type || e.to,
                type: childNode?.type,
                edgeKind: e.type
              };
            });

            showInspector({
              title: n.label || n.transform_type,
              type: isCol ? 'ColumnVersion' : 'DerivationStep',
              id: n.id,
              span: n.span,
              certainty: n.knowledge || (n.is_unknown ? 'UNKNOWN_STATIC' : 'KNOWN_STATIC'),
              details: n,
              upstream: upstreamList,
              downstream: downstreamList,
              sliceTarget: isCol ? n.label : null,
              graphNode: true
            });

            ensureNodeVisibleOnCanvas(n.id, 'provenance');
          });

          g.appendChild(nodeG);
        });

        svg.appendChild(g);
        fitGraphView('view-container-provenance', 'prov-root-group', provPositions, provTransform);
      }

      setupZoomPan('view-container-provenance', 'svg-provenance', 'prov-root-group', provTransform);

      // Provenance Filter Search Input
      const provSearch = document.getElementById('prov-search');
      const provClearSearch = document.getElementById('btn-prov-clear-search');
      if (provSearch) {
        provSearch.addEventListener('input', (e) => {
          const q = e.target.value.trim().toLowerCase();
          if (provClearSearch) provClearSearch.style.display = q ? 'block' : 'none';
          if (!q) {
            if (!selectedItem) clearSelection();
            return;
          }
          document.querySelectorAll('#prov-root-group .svg-node').forEach(el => {
            const nid = el.dataset.nodeId;
            const node = data.provenance?.nodes?.find(n => n.id === nid);
            const text = ((node?.label || '') + ' ' + (node?.transform_type || '') + ' ' + (nid || '')).toLowerCase();
            if (text.includes(q)) {
              el.classList.remove('dimmed');
            } else {
              el.classList.add('dimmed');
            }
          });
        });
      }
      if (provClearSearch) {
        provClearSearch.addEventListener('click', () => {
          provSearch.value = '';
          provClearSearch.style.display = 'none';
          clearSelection();
        });
      }

      document.getElementById('btn-prov-zoom-in')?.addEventListener('click', () => {
        provTransform.scale = Math.min(3.5, provTransform.scale * 1.25);
        document.getElementById('prov-root-group')?.setAttribute('transform', `translate(${provTransform.x}, ${provTransform.y}) scale(${provTransform.scale})`);
      });
      document.getElementById('btn-prov-zoom-out')?.addEventListener('click', () => {
        provTransform.scale = Math.max(0.15, provTransform.scale / 1.25);
        document.getElementById('prov-root-group')?.setAttribute('transform', `translate(${provTransform.x}, ${provTransform.y}) scale(${provTransform.scale})`);
      });
      document.getElementById('btn-prov-reset')?.addEventListener('click', () => {
        fitGraphView('view-container-provenance', 'prov-root-group', provPositions, provTransform);
      });

      // ======================================================================
      // View B: Program Dependence Graph (PDG) Explorer
      // ======================================================================
      let pdgTransform = { x: 40, y: 40, scale: 1 };
      let pdgPositions = new Map();
      let pdgNeighborhood = false;
      let pdgSelectedId = null;
      let pdgOrientation = 'TB';
      let pdgViewportMode = 'readable';
      let pdgContentBounds = null;
      let pdgApplyingSelection = false;

      function syncPdgNeighborhoodButton() {
        const btn = document.getElementById('btn-pdg-neighborhood');
        if (!btn) return;
        btn.setAttribute('aria-pressed', pdgNeighborhood ? 'true' : 'false');
        btn.classList.toggle('active', pdgNeighborhood);
        btn.textContent = pdgNeighborhood ? 'Full graph' : 'Neighborhood';
        btn.title = pdgNeighborhood
          ? 'Return to the complete PDG'
          : 'Show only the selected node and its direct dependencies. This is one hop, not every transitive dependency.';
      }

      function pdgLayoutOptions(orientation) {
        const lr = orientation === 'LR';
        return {
          orientation: orientation,
          useLineBands: true,
          layerSpacing: lr ? 168 : 118,
          nodeSpacing: lr ? 20 : 34,
          graphType: 'pdg',
          orderingPasses: 4
        };
      }

      function choosePdgOrientation(layoutNodes, layoutEdges) {
        const tb = computeDagLayout(layoutNodes, layoutEdges, pdgLayoutOptions('TB'));
        const lr = computeDagLayout(layoutNodes, layoutEdges, pdgLayoutOptions('LR'));
        const tbW = Math.max(1, tb.bounds.maxX - tb.bounds.minX);
        const tbH = Math.max(1, tb.bounds.maxY - tb.bounds.minY);
        if (tbW > tbH * 1.7 && tbW > 980) {
          pdgOrientation = 'LR';
          return lr;
        }
        pdgOrientation = 'TB';
        return tb;
      }

      function pdgLane(edge, bounds, orientation, forward) {
        const key = `${edge.from}:${edge.to}:${edge.kind || ''}`;
        let hash = 0;
        for (let i = 0; i < key.length; i++) hash = (hash * 33 + key.charCodeAt(i)) >>> 0;
        const slot = hash % 6;
        if (orientation === 'LR') {
          return forward ? (bounds.maxY + 18 + slot * 12) : (bounds.minY - 18 - slot * 12);
        }
        return forward ? (bounds.maxX + 18 + slot * 12) : (bounds.minX - 18 - slot * 12);
      }

      function pdgEdgePath(fromPos, toPos, outIndex, outCount, inIndex, inCount, orientation, lane) {
        const span = (toPos.rank ?? 0) - (fromPos.rank ?? 0);
        if (orientation === 'LR') {
          const x1 = fromPos.x + fromPos.width;
          const y1 = fromPos.y + fromPos.height * ((outIndex + 1) / (outCount + 1));
          const x2 = toPos.x;
          const y2 = toPos.y + toPos.height * ((inIndex + 1) / (inCount + 1));
          if (span === 1 && x2 >= x1) {
            const mid = (x1 + x2) / 2;
            return `M ${x1} ${y1} C ${mid} ${y1}, ${mid} ${y2}, ${x2} ${y2}`;
          }
          const xOut = x1 + 14;
          const xIn = x2 - 14;
          return `M ${x1} ${y1} L ${xOut} ${y1} L ${xOut} ${lane} L ${xIn} ${lane} L ${xIn} ${y2} L ${x2} ${y2}`;
        }
        const x1 = fromPos.x + fromPos.width * ((outIndex + 1) / (outCount + 1));
        const y1 = fromPos.y + fromPos.height;
        const x2 = toPos.x + toPos.width * ((inIndex + 1) / (inCount + 1));
        const y2 = toPos.y;
        if (span === 1 && y2 >= y1) {
          const mid = (y1 + y2) / 2;
          return `M ${x1} ${y1} C ${x1} ${mid}, ${x2} ${mid}, ${x2} ${y2}`;
        }
        const yOut = y1 + 14;
        const yIn = y2 - 14;
        return `M ${x1} ${y1} L ${x1} ${yOut} L ${lane} ${yOut} L ${lane} ${yIn} L ${x2} ${yIn} L ${x2} ${y2}`;
      }

      function readPdgNodeBounds() {
        let minX = Infinity, minY = Infinity, maxX = -Infinity, maxY = -Infinity;
        pdgPositions.forEach(p => {
          minX = Math.min(minX, p.x);
          minY = Math.min(minY, p.y);
          maxX = Math.max(maxX, p.x + p.width);
          maxY = Math.max(maxY, p.y + p.height);
        });
        if (!isFinite(minX)) return null;
        return { minX, minY, maxX, maxY };
      }

      function readPdgBounds(root) {
        try {
          const box = root.getBBox();
          if (box && box.width > 0 && box.height > 0) {
            return { minX: box.x, minY: box.y, maxX: box.x + box.width, maxY: box.y + box.height };
          }
        } catch (err) { /* fall through to node bounds */ }
        let minX = Infinity, minY = Infinity, maxX = -Infinity, maxY = -Infinity;
        pdgPositions.forEach(p => {
          minX = Math.min(minX, p.x);
          minY = Math.min(minY, p.y);
          maxX = Math.max(maxX, p.x + p.width);
          maxY = Math.max(maxY, p.y + p.height);
        });
        if (!isFinite(minX)) return { minX: 0, minY: 0, maxX: 80, maxY: 80 };
        return { minX, minY, maxX, maxY };
      }

      function placePdgViewport(mode) {
        const container = document.getElementById('view-container-pdg');
        const root = document.getElementById('pdg-root-group');
        if (!container || !root) return;
        const bounds = readPdgBounds(root);
        const nodeBounds = readPdgNodeBounds() || bounds;
        pdgContentBounds = bounds;
        const cw = container.clientWidth || 800;
        const ch = container.clientHeight || 600;
        const gw = Math.max(80, (bounds.maxX - bounds.minX) + 72);
        const gh = Math.max(80, (bounds.maxY - bounds.minY) + 72);
        const fit = Math.min(1.12, Math.max(0.2, Math.min((cw - 16) / gw, (ch - 16) / gh)));
        const alignStart = mode !== 'fit' && fit < 0.78;
        const scale = alignStart ? 0.85 : fit;
        const anchor = alignStart ? nodeBounds : bounds;
        const graphW = (anchor.maxX - anchor.minX) * scale;
        const graphH = (anchor.maxY - anchor.minY) * scale;
        pdgTransform.scale = scale;
        if (!alignStart) {
          pdgTransform.x = Math.round((cw - graphW) / 2 - anchor.minX * scale);
          pdgTransform.y = Math.round((ch - graphH) / 2 - anchor.minY * scale);
        } else if (pdgOrientation === 'LR') {
          pdgTransform.x = Math.round(32 - anchor.minX * scale);
          pdgTransform.y = Math.round(Math.max(32, (ch - graphH) / 2) - anchor.minY * scale);
        } else {
          pdgTransform.x = Math.round(Math.max(32, (cw - graphW) / 2) - anchor.minX * scale);
          pdgTransform.y = Math.round(32 - anchor.minY * scale);
        }
        root.setAttribute('transform', `translate(${pdgTransform.x}, ${pdgTransform.y}) scale(${pdgTransform.scale})`);
        pdgViewportMode = mode === 'fit' ? 'fit' : 'readable';
      }

      function clampPdgPan() {
        const container = document.getElementById('view-container-pdg');
        const bounds = pdgContentBounds;
        if (!container || !bounds) return;
        const scale = pdgTransform.scale;
        const cw = container.clientWidth || 800;
        const ch = container.clientHeight || 600;
        const contentW = (bounds.maxX - bounds.minX) * scale;
        const contentH = (bounds.maxY - bounds.minY) * scale;
        if (contentW > cw - 80) {
          const minTx = 72 - bounds.maxX * scale;
          const maxTx = cw - 72 - bounds.minX * scale;
          pdgTransform.x = Math.min(maxTx, Math.max(minTx, pdgTransform.x));
        }
        if (contentH > ch - 80) {
          const minTy = 72 - bounds.maxY * scale;
          const maxTy = ch - 72 - bounds.minY * scale;
          pdgTransform.y = Math.min(maxTy, Math.max(minTy, pdgTransform.y));
        }
        const root = document.getElementById('pdg-root-group');
        if (root) root.setAttribute('transform', `translate(${pdgTransform.x}, ${pdgTransform.y}) scale(${pdgTransform.scale})`);
      }

      function renderPdgGraph() {
        const svg = document.getElementById('svg-pdg');
        const defs = svg.querySelector('defs');
        const defsHtml = defs ? defs.outerHTML : '';
        svg.innerHTML = defsHtml;

        const nodes = data.pdg?.nodes || [];
        const edges = data.pdg?.edges || [];

        if (nodes.length === 0) {
          svg.dataset.viewMode = 'full';
          svg.dataset.orientation = 'TB';
          svg.innerHTML += '<text class="svg-graph-empty" x="50%" y="50%" text-anchor="middle">Empty Program Dependence Graph.</text>';
          return;
        }

        const filterData = document.getElementById('toggle-edge-data')?.checked ?? true;
        const filterCtrl = document.getElementById('toggle-edge-ctrl')?.checked ?? true;
        const filterRowSel = document.getElementById('toggle-edge-rowsel')?.checked ?? true;
        const filterUnk = document.getElementById('toggle-edge-unk')?.checked ?? true;

        const visibleEdges = edges.filter(e => {
          const k = e.kind;
          const isData = (k === 'VariableDataDependency' || k === 'ColumnDerivation' || k === 'ColumnStatementBinding' || k === 'DataDependency');
          const isCtrl = (k === 'ControlDependency');
          const isRowSel = (k === 'RowSelectionDependency');
          const isUnk = (k === 'UnknownDependency' || e.is_unknown === true);

          if (isData && !filterData) return false;
          if (isCtrl && !filterCtrl) return false;
          if (isRowSel && !filterRowSel) return false;
          if (isUnk && !filterUnk) return false;
          return true;
        });

        if (pdgNeighborhood && pdgSelectedId != null && !nodes.some(n => n.id === pdgSelectedId)) {
          pdgNeighborhood = false;
          pdgSelectedId = null;
          syncPdgNeighborhoodButton();
        }

        let drawNodes = nodes;
        let drawEdges = visibleEdges;
        if (pdgNeighborhood && pdgSelectedId != null) {
          const hop = visibleEdges.filter(e => e.from === pdgSelectedId || e.to === pdgSelectedId);
          const ids = new Set([pdgSelectedId]);
          hop.forEach(e => { ids.add(e.from); ids.add(e.to); });
          drawNodes = nodes.filter(n => ids.has(n.id));
          drawEdges = hop;
        }
        svg.dataset.viewMode = (pdgNeighborhood && pdgSelectedId != null) ? 'neighborhood' : 'full';

        const layout = choosePdgOrientation(drawNodes, drawEdges);
        svg.dataset.orientation = pdgOrientation;

        pdgPositions = layout.positions;
        const nodeBounds = layout.bounds;

        const g = document.createElementNS('http://www.w3.org/2000/svg', 'g');
        g.setAttribute('id', 'pdg-root-group');
        g.setAttribute('transform', `translate(${pdgTransform.x}, ${pdgTransform.y}) scale(${pdgTransform.scale})`);

        const outGroups = new Map();
        const inGroups = new Map();
        drawEdges.forEach(e => {
          if (!outGroups.has(e.from)) outGroups.set(e.from, []);
          if (!inGroups.has(e.to)) inGroups.set(e.to, []);
          outGroups.get(e.from).push(e);
          inGroups.get(e.to).push(e);
        });
        outGroups.forEach(list => list.sort((a, b) => String(a.to).localeCompare(String(b.to))));
        inGroups.forEach(list => list.sort((a, b) => String(a.from).localeCompare(String(b.from))));

        drawEdges.forEach((e, idx) => {
          const fromPos = pdgPositions.get(e.from);
          const toPos = pdgPositions.get(e.to);
          if (!fromPos || !toPos) return;

          const outs = outGroups.get(e.from) || [e];
          const ins = inGroups.get(e.to) || [e];
          const span = (toPos.rank ?? 0) - (fromPos.rank ?? 0);
          const forward = span > 0;
          const lane = pdgLane(e, nodeBounds, pdgOrientation, forward);
          const d = pdgEdgePath(
            fromPos, toPos,
            Math.max(0, outs.indexOf(e)), outs.length,
            Math.max(0, ins.indexOf(e)), ins.length,
            pdgOrientation, lane
          );

          const path = document.createElementNS('http://www.w3.org/2000/svg', 'path');
          path.setAttribute('d', d);
          path.dataset.from = e.from;
          path.dataset.to = e.to;
          path.dataset.edgeId = e.id ?? idx;

          let edgeClass = 'svg-edge edge-data';
          let markerUrl = 'url(#arrow-data)';

          if (e.kind === 'ControlDependency') {
            edgeClass = 'svg-edge edge-ctrl';
            markerUrl = 'url(#arrow-ctrl)';
          } else if (e.kind === 'RowSelectionDependency') {
            edgeClass = 'svg-edge edge-rowsel';
            markerUrl = 'url(#arrow-rowsel)';
          } else if (e.kind === 'ColumnStatementBinding' || e.kind === 'ColumnDerivation') {
            edgeClass = 'svg-edge edge-colver';
            markerUrl = 'url(#arrow-colver)';
          } else if (e.kind === 'UnknownDependency' || e.is_unknown) {
            edgeClass = 'svg-edge edge-unk';
            markerUrl = 'url(#arrow-unk)';
          }

          path.setAttribute('class', edgeClass);
          path.setAttribute('marker-end', markerUrl);
          g.appendChild(path);
        });

        pdgPositions.forEach((pos, id) => {
          const n = pos.data;
          const nodeG = document.createElementNS('http://www.w3.org/2000/svg', 'g');
          nodeG.setAttribute('class', `svg-node`);
          nodeG.setAttribute('transform', `translate(${pos.x}, ${pos.y})`);
          nodeG.dataset.nodeId = n.id;

          const title = document.createElementNS('http://www.w3.org/2000/svg', 'title');
          title.textContent = `${n.kind} (#${n.id}): ${n.label}\nSpan: Line ${n.span?.start_line || '?'}`;
          nodeG.appendChild(title);

          const isCol = n.kind === 'ColumnVersion';
          const isStmt = n.kind === 'Statement';
          const isRowSel = n.kind === 'RowSelection';
          const isExpr = n.kind === 'Expression';
          const isVarDef = n.kind === 'VariableDef';
          const isVarUse = n.kind === 'VariableUse';

          let rectClass = 'node-stmt';
          if (isVarDef) rectClass = 'node-vardef';
          else if (isVarUse) rectClass = 'node-varuse';
          else if (isCol) rectClass = 'node-col';
          else if (isExpr) rectClass = 'node-expr';
          else if (isRowSel) rectClass = 'node-rowsel';
          else if (n.is_unknown) rectClass = 'node-unk';

          const rect = document.createElementNS('http://www.w3.org/2000/svg', 'rect');
          rect.setAttribute('width', pos.width);
          rect.setAttribute('height', pos.height);
          rect.setAttribute('rx', isCol ? 23 : 6);
          rect.setAttribute('class', rectClass);
          nodeG.appendChild(rect);

          const textKind = document.createElementNS('http://www.w3.org/2000/svg', 'text');
          textKind.setAttribute('x', isCol ? 14 : 10);
          textKind.setAttribute('y', 17);
          textKind.setAttribute('font-size', '11');
          textKind.setAttribute('font-weight', '700');
          textKind.setAttribute('letter-spacing', '0.5');

          let kindColor = 'var(--text-muted)';
          if (isVarDef) kindColor = 'var(--accent-lilac-text)';
          else if (isVarUse) kindColor = 'var(--teal-text)';
          else if (isCol) kindColor = 'var(--orange-text)';
          else if (isExpr) kindColor = 'var(--text-ink-secondary)';
          else if (isRowSel) kindColor = 'var(--accent-lilac-text)';
          else if (n.is_unknown) kindColor = 'var(--graph-unknown)';

          textKind.style.fill = kindColor;
          const lineStr = n.span?.start_line ? ` L${n.span.start_line}` : '';
          textKind.textContent = `${n.kind.toUpperCase()}${lineStr}`;
          nodeG.appendChild(textKind);

          const textLabel = document.createElementNS('http://www.w3.org/2000/svg', 'text');
          textLabel.setAttribute('x', isCol ? 14 : 10);
          textLabel.setAttribute('y', 34);
          textLabel.setAttribute('font-size', '12');
          textLabel.setAttribute('font-weight', '600');
          textLabel.style.fill = 'var(--ink)';

          const cleanLbl = cleanNodeLabel(n, 'pdg');
          textLabel.textContent = cleanLbl.length > 32 ? cleanLbl.slice(0, 30) + '…' : cleanLbl;
          nodeG.appendChild(textLabel);

          nodeG.addEventListener('click', (evt) => {
            evt.stopPropagation();
            const previousId = pdgSelectedId;
            pdgSelectedId = n.id;
            if (pdgNeighborhood && !pdgApplyingSelection && previousId !== n.id) {
              renderPdgGraph();
              return;
            }

            document.querySelectorAll('#pdg-root-group .svg-node').forEach(el => {
              el.classList.remove('selected', 'connected-upstream', 'connected-downstream', 'dimmed');
            });
            document.querySelectorAll('#pdg-root-group .svg-edge').forEach(el => {
              el.classList.remove('highlighted', 'inbound', 'outbound', 'dimmed');
            });

            nodeG.classList.add('selected');

            const inboundEdges = drawEdges.filter(e => e.to === n.id);
            const outboundEdges = drawEdges.filter(e => e.from === n.id);
            const upstreamNodeIds = new Set(inboundEdges.map(e => e.from));
            const downstreamNodeIds = new Set(outboundEdges.map(e => e.to));

            document.querySelectorAll('#pdg-root-group .svg-edge').forEach(el => {
              const f = Number(el.dataset.from);
              const t = Number(el.dataset.to);
              if (t === n.id) {
                el.classList.add('highlighted', 'inbound');
              } else if (f === n.id) {
                el.classList.add('highlighted', 'outbound');
              } else {
                el.classList.add('dimmed');
              }
            });

            document.querySelectorAll('#pdg-root-group .svg-node').forEach(el => {
              const nid = Number(el.dataset.nodeId);
              if (nid === n.id) {
                // selected
              } else if (upstreamNodeIds.has(nid)) {
                el.classList.add('connected-upstream');
              } else if (downstreamNodeIds.has(nid)) {
                el.classList.add('connected-downstream');
              } else {
                el.classList.add('dimmed');
              }
            });

            const upstreamList = inboundEdges.map(e => {
              const parentNode = nodes.find(x => x.id === e.from);
              return {
                id: e.from,
                label: cleanNodeLabel(parentNode, 'pdg'),
                type: parentNode?.kind,
                edgeKind: e.kind
              };
            });

            const downstreamList = outboundEdges.map(e => {
              const childNode = nodes.find(x => x.id === e.to);
              return {
                id: e.to,
                label: cleanNodeLabel(childNode, 'pdg'),
                type: childNode?.kind,
                edgeKind: e.kind
              };
            });

            let sliceTarget = null;
            if (n.kind === 'ColumnVersion') {
              const match = (n.label || '').match(/([a-zA-Z0-9._]+)\$([a-zA-Z0-9._]+)/);
              if (match) sliceTarget = `${match[1]}$${match[2]}`;
            }

            showInspector({
              title: cleanLbl,
              type: `PDG Node (${n.kind})`,
              id: n.node_id || `node#${n.id}`,
              span: n.span,
              certainty: n.is_unknown ? 'UNKNOWN_STATIC' : 'KNOWN_STATIC',
              details: n.details,
              upstream: upstreamList,
              downstream: downstreamList,
              sliceTarget,
              graphNode: true
            });

            ensureNodeVisibleOnCanvas(n.id, 'pdg');
          });

          g.appendChild(nodeG);
        });

        svg.appendChild(g);
        placePdgViewport('readable');
        syncPdgNeighborhoodButton();
        if (pdgSelectedId != null) {
          const selectedEl = document.querySelector(`#pdg-root-group .svg-node[data-node-id="${pdgSelectedId}"]`);
          if (selectedEl) {
            pdgApplyingSelection = true;
            selectedEl.dispatchEvent(new MouseEvent('click', { bubbles: true }));
            pdgApplyingSelection = false;
          }
        }
      }

      setupZoomPan('view-container-pdg', 'svg-pdg', 'pdg-root-group', pdgTransform, {
        onTransform() {
          pdgViewportMode = 'manual';
          clampPdgPan();
        }
      });

      ['toggle-edge-data', 'toggle-edge-ctrl', 'toggle-edge-rowsel', 'toggle-edge-unk'].forEach(id => {
        document.getElementById(id)?.addEventListener('change', renderPdgGraph);
      });

      document.getElementById('btn-pdg-neighborhood')?.addEventListener('click', () => {
        if (pdgNeighborhood) {
          pdgNeighborhood = false;
          renderPdgGraph();
          return;
        }
        if (pdgSelectedId == null) {
          showStatusNotification('Select a node to show its direct neighborhood.', false);
          return;
        }
        pdgNeighborhood = true;
        renderPdgGraph();
      });

      document.getElementById('btn-pdg-zoom-in')?.addEventListener('click', () => {
        pdgViewportMode = 'manual';
        pdgTransform.scale = Math.min(3.5, pdgTransform.scale * 1.25);
        clampPdgPan();
      });
      document.getElementById('btn-pdg-zoom-out')?.addEventListener('click', () => {
        pdgViewportMode = 'manual';
        pdgTransform.scale = Math.max(0.15, pdgTransform.scale / 1.25);
        clampPdgPan();
      });
      document.getElementById('btn-pdg-reset')?.addEventListener('click', () => {
        placePdgViewport('fit');
      });

      window.rparseSelectNode = function(nodeId, targetView) {
        if (targetView && targetView !== activeView) {
          switchView(targetView);
        }
        setTimeout(() => {
          const rootId = (activeView === 'pdg') ? 'pdg-root-group' : 'prov-root-group';
          const nodeElem = document.querySelector(`#${rootId} .svg-node[data-node-id="${nodeId}"]`);
          if (nodeElem) {
            nodeElem.dispatchEvent(new MouseEvent('click', { bubbles: true }));
            const posMap = (activeView === 'pdg') ? pdgPositions : provPositions;
            const parsedId = isNaN(Number(nodeId)) ? nodeId : Number(nodeId);
            const pos = posMap.get(parsedId) || posMap.get(nodeId);
            const containerId = (activeView === 'pdg') ? 'view-container-pdg' : 'view-container-provenance';
            const container = document.getElementById(containerId);
            const transformObj = (activeView === 'pdg') ? pdgTransform : provTransform;
            if (pos && container) {
              const cw = container.clientWidth || 800;
              const ch = container.clientHeight || 600;
              transformObj.x = Math.round(cw / 2 - (pos.x + pos.width / 2) * transformObj.scale);
              transformObj.y = Math.round(ch / 2 - (pos.y + pos.height / 2) * transformObj.scale);
              const rootG = document.getElementById(rootId);
              if (rootG) rootG.setAttribute('transform', `translate(${transformObj.x}, ${transformObj.y}) scale(${transformObj.scale})`);
            }
          }
        }, 50);
      };

      window.rparseJumpSpan = function(lineNum) {
        selectSourceLine(lineNum);
        const row = document.querySelector(`.code-line[data-line="${lineNum}"]`);
        if (row) {
          row.classList.add('highlighted-span');
          row.scrollIntoView({ behavior: scrollBehavior(), block: 'center' });
        }
      };

      // ======================================================================
      // View C: Backward Slice Inspector
      // ======================================================================
      function renderSliceInspector() {
        const container = document.getElementById('view-container-slice');
        const select = document.getElementById('slice-target-select');

        // Populate Target Dropdown
        const sliceKeys = Object.keys(data.slices || {});
        if (!sliceKeys.includes(activeSliceTarget)) {
          activeSliceTarget = sliceKeys[0] || '';
        }
        if (select) {
          select.innerHTML = '';
          sliceKeys.forEach(k => {
            const opt = document.createElement('option');
            opt.value = k;
            opt.textContent = k;
            if (k === activeSliceTarget) opt.selected = true;
            select.appendChild(opt);
          });
          select.onchange = (e) => {
            activeSliceTarget = e.target.value;
            renderSliceInspector();
          };
        }

        const curSlice = data.slices?.[activeSliceTarget];
        if (!curSlice) {
          container.innerHTML = '<div class="card" style="text-align:center;color:var(--text-dim);padding:24px;">No slice computed for selected target.</div>';
          updateSliceSourceHighlighting();
          return;
        }

        // Update Header Badges
        const confBadge = document.getElementById('slice-conf-badge');
        if (confBadge) {
          confBadge.textContent = curSlice.confidence || 'COMPLETE';
          confBadge.className = `pill ${curSlice.confidence === 'COMPLETE' ? 'pill-full' : 'pill-drop'}`;
        }

        const unkBadge = document.getElementById('slice-unk-badge');
        if (unkBadge) {
          unkBadge.textContent = curSlice.contains_unknowns ? 'CONTAINS UNKNOWNS: YES' : 'NO UNKNOWNS';
          unkBadge.className = `pill ${curSlice.contains_unknowns ? 'pill-drop' : 'pill-keep'}`;
        }

        const stmts = curSlice.statements || [];
        const totalStmts = stmts.length;
        let fullyRetained = 0;
        let partiallyRetained = 0;
        let notRetained = 0;
        let unknownCount = 0;

        stmts.forEach(st => {
          if (st.state === 'fully_retained') fullyRetained++;
          else if (st.state === 'partially_retained') partiallyRetained++;
          else if (st.state === 'unknown') unknownCount++;
          else notRetained++;
        });

        container.innerHTML = `
          <div class="slice-summary-panel">
            <div class="slice-summary-header">
              <div class="slice-summary-title">Program Statement Retention</div>
              <div class="slice-summary-badges">
                <span class="slice-badge ${curSlice.confidence === 'COMPLETE' ? 'slice-badge-sound' : 'slice-badge-approx'}">${escapeHtml(curSlice.confidence || 'COMPLETE')}</span>
                <span class="slice-badge ${curSlice.contains_unknowns ? 'slice-badge-warn' : 'slice-badge-ok'}">${curSlice.contains_unknowns ? 'CONTAINS UNKNOWNS' : 'NO UNKNOWNS'}</span>
              </div>
            </div>
            <div class="slice-stats-grid">
              <div class="slice-stat-card">
                <div class="slice-stat-label">Total Statements</div>
                <div class="slice-stat-val">${totalStmts}</div>
              </div>
              <div class="slice-stat-card">
                <div class="slice-stat-label">Fully Retained</div>
                <div class="slice-stat-val val-full">${fullyRetained}</div>
              </div>
              <div class="slice-stat-card">
                <div class="slice-stat-label">Partially Retained</div>
                <div class="slice-stat-val val-partial">${partiallyRetained}</div>
              </div>
              <div class="slice-stat-card">
                <div class="slice-stat-label">Not Retained</div>
                <div class="slice-stat-val val-none">${notRetained}</div>
              </div>
              ${unknownCount > 0 ? `
              <div class="slice-stat-card">
                <div class="slice-stat-label">Unknown</div>
                <div class="slice-stat-val val-unknown">${unknownCount}</div>
              </div>` : `
              <div class="slice-stat-card">
                <div class="slice-stat-label">Retained Elements</div>
                <div class="slice-stat-val val-elements">${curSlice.retained_element_count || 0}</div>
              </div>`}
            </div>
          </div>

          <div class="slice-section-header">
            <span class="slice-section-title">Statement Breakdown</span>
            <span class="slice-section-meta">${totalStmts} statements evaluated</span>
          </div>
          <div class="stmt-retention-list" id="slice-stmt-list"></div>
        `;

        const stmtListContainer = document.getElementById('slice-stmt-list');

        stmts.forEach(st => {
          const card = document.createElement('div');
          const state = st.state;
          let cardClass = 'none';
          let statusBadgeClass = 'badge-none';
          let statusText = 'NOT RETAINED';

          if (state === 'fully_retained') {
            cardClass = 'full';
            statusBadgeClass = 'badge-full';
            statusText = 'FULLY RETAINED';
          } else if (state === 'partially_retained') {
            cardClass = 'partial';
            statusBadgeClass = 'badge-partial';
            statusText = 'PARTIALLY RETAINED';
          } else if (state === 'unknown') {
            cardClass = 'unknown';
            statusBadgeClass = 'badge-unknown';
            statusText = 'UNKNOWN';
          }

          card.className = `stmt-card ${cardClass}`;
          card.dataset.line = st.start_line;

          const lineIdx = st.start_line - 1;
          const codeSnippet = (data.source_lines && data.source_lines[lineIdx]) ? data.source_lines[lineIdx] : `Statement at Line ${st.start_line}`;

          let html = `
            <div class="stmt-header">
              <div class="stmt-header-left">
                <span class="stmt-line-badge">Line ${st.start_line}</span>
              </div>
              <div class="stmt-header-right">
                <span class="stmt-status-badge ${statusBadgeClass}">${statusText}</span>
              </div>
            </div>
            <div class="stmt-code">${syntaxHighlight(codeSnippet) || escapeHtml(codeSnippet)}</div>
          `;

          if (st.sub_elements && st.sub_elements.length > 0) {
            html += `<div class="sub-elements-list">
              <div class="sub-elements-heading">Expressions &amp; Columns:</div>`;
            st.sub_elements.forEach(sub => {
              const isKeep = sub.state === 'retained';
              html += `
                <div class="sub-item">
                  <span class="sub-item-label">${escapeHtml(sub.label)}</span>
                  <span class="sub-item-badge ${isKeep ? 'sub-keep' : 'sub-drop'}">${isKeep ? '[KEEP]' : '[DROP]'}</span>
                </div>
              `;
            });
            html += `</div>`;
          }

          card.innerHTML = html;

          card.addEventListener('click', () => {
            selectSourceLine(st.start_line);
            showInspector({
              title: `Statement (Line ${st.start_line})`,
              type: 'StatementSlice',
              id: `stmt_line_${st.start_line}`,
              span: st.span,
              certainty: curSlice.contains_unknowns ? 'UNKNOWN_STATIC' : 'KNOWN_STATIC',
              details: {
                retention_state: st.state,
                criterion: curSlice.criterion,
                sub_element_count: st.sub_elements?.length || 0
              }
            });
          });

          stmtListContainer.appendChild(card);
        });

        updateSliceSourceHighlighting();
      }

      window.rparseSelectSlice = function(target) {
        activeSliceTarget = target;
        switchView('slice');
        const sel = document.getElementById('slice-target-select');
        if (sel) sel.value = target;
        renderSliceInspector();
      };

      // ======================================================================
      // View D: Dataframe Schema Evolution Inspector
      // ======================================================================
      function renderSchemaEvolution() {
        const container = document.getElementById('view-container-schema');
        const steps = data.schema_evolution || [];
        document.getElementById('schema-step-count').textContent = `${steps.length} states recorded`;

        if (steps.length === 0) {
          container.innerHTML = '<div class="card" style="text-align:center;color:var(--text-dim);">No dataframe transformations recorded.</div>';
          return;
        }

        container.innerHTML = '';
        steps.forEach((st, idx) => {
          const card = document.createElement('div');
          card.className = 'schema-card';

          const lineIdx = (st.span?.start_line || 1) - 1;
          const codeSnippet = (data.source_lines && data.source_lines[lineIdx]) ? data.source_lines[lineIdx].trim() : 'Dataframe transformation';

          let diffSummaryHtml = '';
          if (st.diff) {
            if (st.diff.added?.length) diffSummaryHtml += `<span class="diff-badge diff-added">+${st.diff.added.length} added</span>`;
            if (st.diff.removed?.length) diffSummaryHtml += `<span class="diff-badge diff-removed">-${st.diff.removed.length} removed</span>`;
            if (st.diff.modified?.length) diffSummaryHtml += `<span class="diff-badge diff-mod">~${st.diff.modified.length} modified</span>`;
            if (st.diff.retained?.length) diffSummaryHtml += `<span class="diff-badge diff-ret">=${st.diff.retained.length} retained</span>`;
          }

          let colsHtml = '';
          (st.columns || []).forEach(col => {
            colsHtml += `
              <span class="col-chip">
                <span>${escapeHtml(col.name)}</span>
                <span class="col-ver">@v${st.version}</span>
              </span>
            `;
          });

          let filterInfo = '';
          if (st.row_selections && st.row_selections.length > 0) {
            filterInfo = `<div class="schema-filter-note"><svg class="ui-icon" width="15" height="15" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M2.2 3.2h11.6L9.4 8.4v3.2l-2.8 1.4V8.4z"/></svg><span>Filter Dependency Applied (${st.row_selections.length} predicate)</span></div>`;
          }

          card.innerHTML = `
            <div class="schema-card-header">
              <div class="schema-symbol">${escapeHtml(st.symbol_name)} <span style="font-size:11px; color:var(--text-dim); font-weight:normal;">v${st.version}</span></div>
              <span class="pill pill-neutral">Step #${idx + 1}</span>
            </div>
            <div class="stmt-code" style="margin-bottom:8px;">${escapeHtml(codeSnippet)}</div>
            <div class="schema-diff-summary">${diffSummaryHtml}</div>
            <div class="schema-cols-grid">${colsHtml}</div>
            ${filterInfo}
          `;

          card.addEventListener('click', () => {
            showInspector({
              title: `${st.symbol_name}@v${st.version}`,
              type: 'DataFrameState',
              id: `state_${st.state_id}`,
              span: st.span,
              certainty: st.knowledge,
              details: {
                symbol_name: st.symbol_name,
                version: st.version,
                parent_state: st.parent_state ?? 'root',
                column_count: st.columns?.length || 0,
                row_selection_count: st.row_selections?.length || 0
              }
            });
          });

          container.appendChild(card);
        });
      }

      // ======================================================================
      // View E: Interactive AST & CST Syntax Tree Explorer
      // ======================================================================
      let astActiveRep = 'cst'; // 'cst' | 'ir'
      let astCollapsedNodeIds = new Set();
      let astSearchQuery = '';
      let astSelectedNodeId = null;
      let astInitialized = false;
      let astInitializedReps = { cst: false, ir: false };

      function updateAstVisibleCount() {
        const treeRoot = document.getElementById('ast-tree-root');
        const vElem = document.getElementById('ast-stat-visible');
        if (!treeRoot || !vElem) return;
        const visibleRows = Array.from(treeRoot.querySelectorAll('.ast-node-row'))
          .filter(el => el.offsetParent !== null).length;
        vElem.textContent = `Visible: ${visibleRows}`;
      }

      function initAstExplorerEvents() {
        if (astInitialized) return;
        astInitialized = true;

        const btnCst = document.getElementById('btn-ast-rep-cst');
        const btnIr = document.getElementById('btn-ast-rep-ir');
        const searchInput = document.getElementById('ast-search-input');
        const btnExpandAll = document.getElementById('btn-ast-expand-all');
        const btnCollapseAll = document.getElementById('btn-ast-collapse-all');

        btnCst?.addEventListener('click', () => {
          if (astActiveRep !== 'cst') {
            astActiveRep = 'cst';
            btnCst.classList.add('active');
            btnIr?.classList.remove('active');
            renderAstExplorer();
          }
        });

        btnIr?.addEventListener('click', () => {
          if (astActiveRep !== 'ir') {
            astActiveRep = 'ir';
            btnIr.classList.add('active');
            btnCst?.classList.remove('active');
            renderAstExplorer();
          }
        });

        searchInput?.addEventListener('input', (e) => {
          astSearchQuery = e.target.value.trim().toLowerCase();
          renderAstExplorer();
        });

        btnExpandAll?.addEventListener('click', () => {
          const repPrefix = astActiveRep === 'cst' ? 'cst-' : 'ir-';
          Array.from(astCollapsedNodeIds).forEach(id => {
            if (typeof id === 'string' && id.startsWith(repPrefix)) {
              astCollapsedNodeIds.delete(id);
            }
          });
          const treeRoot = document.getElementById('ast-tree-root');
          if (treeRoot) {
            treeRoot.querySelectorAll('.ast-node-children').forEach(el => el.classList.remove('collapsed'));
            treeRoot.querySelectorAll('.ast-node-chevron:not(.empty)').forEach(el => {
              el.classList.add('expanded');
              el.title = 'Collapse node';
            });
            updateAstVisibleCount();
          }
        });

        btnCollapseAll?.addEventListener('click', () => {
          const treeData = astActiveRep === 'cst' ? data.ast_explorer?.cst : data.ast_explorer?.ir;
          if (treeData) {
            collectExpandableIds(treeData, astCollapsedNodeIds);
            if (treeData.id) astCollapsedNodeIds.delete(treeData.id);
          }
          renderAstExplorer();
        });
      }

      function collectExpandableIds(node, set) {
        if (!node) return;
        if (node.children && node.children.length > 0) {
          set.add(node.id);
          node.children.forEach(c => collectExpandableIds(c, set));
        }
      }

      function getNodeCategoryTag(kind, isError, isMissing) {
        if (isError || isMissing) return { tag: 'ERROR', cls: 'ast-tag-err' };
        const k = (kind || '').toLowerCase();
        if (k.includes('assign') || k.includes('stmt') || k.includes('if') || k.includes('for') || k.includes('while') || k.includes('return')) {
          return { tag: 'STMT', cls: 'ast-tag-stmt' };
        }
        if (k.includes('ident') || k.includes('symbol') || k.includes('param') || k.includes('name')) {
          return { tag: 'ID', cls: 'ast-tag-id' };
        }
        if (k.includes('lit') || k.includes('float') || k.includes('int') || k.includes('str') || k.includes('bool') || k.includes('null') || k.includes('na')) {
          return { tag: 'LIT', cls: 'ast-tag-lit' };
        }
        if (k.includes('call') || k.includes('func') || k.includes('lambda')) {
          return { tag: 'CALL', cls: 'ast-tag-call' };
        }
        if (k.includes('op') || k.includes('pipe') || k.includes('extract') || k.includes('binary') || k.includes('unary')) {
          return { tag: 'OP', cls: 'ast-tag-op' };
        }
        return { tag: 'NODE', cls: 'ast-tag-def' };
      }

      function doesSubtreeMatchSearch(node, query) {
        if (!query) return true;
        const text = `${node.kind || ''} ${node.label || ''} ${node.detail || ''} ${node.text || ''} ${node.field_name || ''}`.toLowerCase();
        if (text.includes(query)) return true;
        if (node.children && node.children.length > 0) {
          return node.children.some(c => doesSubtreeMatchSearch(c, query));
        }
        return false;
      }

      function renderAstExplorer() {
        initAstExplorerEvents();
        const treeRoot = document.getElementById('ast-tree-root');
        if (!treeRoot) return;

        treeRoot.innerHTML = '';
        const astPayload = data.ast_explorer;
        if (!astPayload) {
          treeRoot.innerHTML = '<div style="color:var(--text-dim); text-align:center; margin-top:40px;">No AST data available. Reanalyze source code to inspect syntax tree.</div>';
          return;
        }

        const rootNode = astActiveRep === 'cst' ? astPayload.cst : astPayload.ir;
        if (!rootNode || rootNode.kind === 'empty') {
          treeRoot.innerHTML = '<div style="color:var(--text-dim); text-align:center; margin-top:40px;">Source code is empty. Enter R code to explore its syntax tree.</div>';
          const tElem = document.getElementById('ast-stat-total');
          const vElem = document.getElementById('ast-stat-visible');
          if (tElem) tElem.textContent = 'Total: 0';
          if (vElem) vElem.textContent = 'Visible: 0';
          return;
        }

        // On first render of this rep, expand root & depth <= 2
        if (!astInitializedReps[astActiveRep] && !astSearchQuery) {
          function autoCollapseDeeper(node, depth) {
            if (!node || !node.children || node.children.length === 0) return;
            if (depth >= 2) {
              astCollapsedNodeIds.add(node.id);
            }
            node.children.forEach(c => autoCollapseDeeper(c, depth + 1));
          }
          autoCollapseDeeper(rootNode, 0);
          astInitializedReps[astActiveRep] = true;
        }

        let totalNodes = 0;
        let visibleNodes = 0;
        let searchMatches = 0;

        function buildNodeElement(node, parentNode, depth) {
          totalNodes++;
          const matchesThis = astSearchQuery && `${node.kind || ''} ${node.label || ''} ${node.detail || ''} ${node.text || ''} ${node.field_name || ''}`.toLowerCase().includes(astSearchQuery);
          if (matchesThis) searchMatches++;

          const hasChildren = node.children && node.children.length > 0;
          const isCollapsed = astCollapsedNodeIds.has(node.id) && !astSearchQuery;

          visibleNodes++;

          const nodeContainer = document.createElement('div');
          nodeContainer.className = 'ast-tree-node';
          nodeContainer.dataset.nodeId = node.id;

          const row = document.createElement('div');
          row.className = 'ast-node-row';
          if (node.id === astSelectedNodeId) row.classList.add('selected');
          if (node.is_error || node.is_missing || node.has_error) row.classList.add('has-error');

          let childrenContainer = null;
          if (hasChildren) {
            childrenContainer = document.createElement('div');
            childrenContainer.className = `ast-node-children ${isCollapsed ? 'collapsed' : ''}`;
          }

          // Chevron icon
          const chevron = document.createElement('span');
          chevron.className = `ast-node-chevron ${hasChildren ? (!isCollapsed ? 'expanded' : '') : 'empty'}`;
          chevron.textContent = '▶';
          if (hasChildren) {
            chevron.title = isCollapsed ? 'Expand node' : 'Collapse node';
            chevron.addEventListener('click', (e) => {
              e.stopPropagation();
              e.preventDefault();
              const isCurrentlyCollapsed = astCollapsedNodeIds.has(node.id);
              if (isCurrentlyCollapsed) {
                astCollapsedNodeIds.delete(node.id);
                chevron.classList.add('expanded');
                chevron.title = 'Collapse node';
                if (childrenContainer) childrenContainer.classList.remove('collapsed');
              } else {
                astCollapsedNodeIds.add(node.id);
                chevron.classList.remove('expanded');
                chevron.title = 'Expand node';
                if (childrenContainer) childrenContainer.classList.add('collapsed');
              }
              updateAstVisibleCount();
            });
          }
          row.appendChild(chevron);

          // Category Badge Tag
          const cat = getNodeCategoryTag(node.kind, node.is_error, node.is_missing);
          const tagSpan = document.createElement('span');
          tagSpan.className = `ast-node-tag ${cat.cls}`;
          tagSpan.textContent = cat.tag;
          row.appendChild(tagSpan);

          // Node Label
          const labelSpan = document.createElement('span');
          labelSpan.className = 'ast-node-label';
          if (matchesThis) {
            labelSpan.innerHTML = `<mark class="ast-match-highlight">${escapeHtml(node.label || node.kind)}</mark>`;
          } else {
            labelSpan.textContent = node.label || node.kind;
          }
          row.appendChild(labelSpan);

          // Snippet detail
          if (node.detail && node.detail !== node.label) {
            const detailSpan = document.createElement('span');
            detailSpan.className = 'ast-node-detail';
            detailSpan.textContent = `(${node.detail})`;
            row.appendChild(detailSpan);
          }

          // Span badge if available
          if (node.span && node.span.start_line) {
            const spanBadge = document.createElement('span');
            spanBadge.className = 'ast-node-span';
            spanBadge.textContent = `L${node.span.start_line}:${node.span.start_col}–${node.span.end_line}:${node.span.end_col}`;
            spanBadge.title = `Bytes [${node.span.start_byte}, ${node.span.end_byte})`;
            row.appendChild(spanBadge);
          }

          // Row selection event -> opens inspector & highlights editor
          row.addEventListener('click', () => {
            astSelectedNodeId = node.id;
            document.querySelectorAll('.ast-node-row.selected').forEach(el => el.classList.remove('selected'));
            row.classList.add('selected');

            const inspectorDetails = {
              'Representation': astActiveRep === 'cst' ? 'Tree-sitter Concrete Syntax Tree (CST)' : 'Canonical Semantic IR / AST',
              'Node Kind': node.kind,
              'Node ID': node.id,
              'Preview Text': node.text || '(empty)',
            };

            if (node.field_name) {
              inspectorDetails['Grammar Field'] = node.field_name;
            }
            if (node.is_named !== undefined) {
              inspectorDetails['Named Syntax Rule'] = node.is_named ? 'Yes' : 'No (Anonymous Token)';
            }
            if (node.is_error || node.is_missing) {
              inspectorDetails['Syntax Status'] = node.is_missing ? 'Missing expected syntax token' : 'Syntax error recovery node';
            }
            if (node.span && node.span.start_line) {
              inspectorDetails['Source Lines'] = `Line ${node.span.start_line}, Col ${node.span.start_col} to Line ${node.span.end_line}, Col ${node.span.end_col}`;
              inspectorDetails['Byte Offsets'] = `[${node.span.start_byte}, ${node.span.end_byte}) (${node.span.end_byte - node.span.start_byte} bytes)`;
            }

            const upstream = parentNode ? [{
              id: parentNode.id,
              label: parentNode.label || parentNode.kind,
              type: 'Parent Node',
              edgeKind: 'parent'
            }] : [];

            const downstream = (node.children || []).map(c => ({
              id: c.id,
              label: c.label || c.kind,
              type: 'Child Node',
              edgeKind: c.field_name || 'child'
            }));

            showInspector({
              title: `${node.kind} (${node.label || ''})`,
              type: astActiveRep === 'cst' ? 'CST Node' : 'Semantic IR Node',
              id: node.id,
              span: node.span,
              certainty: (node.is_error || node.has_error) ? 'SYNTAX_ERROR' : 'KNOWN_STATIC',
              details: inspectorDetails,
              upstream,
              downstream,
              graphNode: false
            });

            if (contextualInspector) {
              contextualInspector.classList.add('open');
            }

            if (node.span && node.span.start_line) {
              window.rparseJumpSpan(node.span.start_line);
            }
          });

          nodeContainer.appendChild(row);

          if (hasChildren && childrenContainer) {
            node.children.forEach(child => {
              if (doesSubtreeMatchSearch(child, astSearchQuery)) {
                childrenContainer.appendChild(buildNodeElement(child, node, depth + 1));
              }
            });
            nodeContainer.appendChild(childrenContainer);
          }

          return nodeContainer;
        }

        if (doesSubtreeMatchSearch(rootNode, astSearchQuery)) {
          treeRoot.appendChild(buildNodeElement(rootNode, null, 0));
        } else {
          treeRoot.innerHTML = `<div style="color:var(--text-dim); text-align:center; margin-top:40px;">No nodes match search query "<em>${escapeHtml(astSearchQuery)}</em>".</div>`;
        }

        const totalElem = document.getElementById('ast-stat-total');
        const visibleElem = document.getElementById('ast-stat-visible');
        const matchPill = document.getElementById('ast-search-match');
        if (totalElem) totalElem.textContent = `Total: ${totalNodes}`;
        if (visibleElem) updateAstVisibleCount();
        if (matchPill) {
          if (astSearchQuery) {
            matchPill.style.display = 'inline-block';
            matchPill.textContent = `${searchMatches} match${searchMatches === 1 ? '' : 'es'}`;
          } else {
            matchPill.style.display = 'none';
          }
        }
      }

      // Clear & Jump Actions
      document.getElementById('btn-clear-highlight')?.addEventListener('click', clearSelection);

      document.getElementById('btn-jump-source')?.addEventListener('click', () => {
        if (selectedItem?.span) {
          highlightSourceSpan(selectedItem.span);
        }
      });

      // Auto-fit on Window Resize
      let resizeTimer = null;
      window.addEventListener('resize', () => {
        clearTimeout(resizeTimer);
        resizeTimer = setTimeout(() => {
          if (activeView === 'pdg') {
            if (pdgViewportMode === 'manual') clampPdgPan();
            else {
              placePdgViewport(pdgViewportMode === 'fit' ? 'fit' : 'readable');
              if (pdgViewportMode !== 'fit' && pdgSelectedId != null) {
                ensureNodeVisibleOnCanvas(pdgSelectedId, 'pdg');
              }
            }
          }
          if (activeView === 'provenance') fitGraphView('view-container-provenance', 'prov-root-group', provPositions, provTransform);
        }, 100);
      });

      // Initial Boot: Render source viewer, overview card, select default view
      renderSourceViewer();
      renderOverviewCard();
      selectDefaultView(data);

      if (data.initial_selection?.criterion) {
        window.rparseSelectSlice(data.initial_selection.criterion);
      }
    

      // =========================================================================
      // MASTER WORKSPACE ENHANCEMENTS (Specification Phases 3, 4, 5, 7)
      // =========================================================================

      // Inspector Toggle
      const btnCloseInspector = document.getElementById('btn-close-inspector');
      if (btnCloseInspector && contextualInspector) {
        btnCloseInspector.addEventListener('click', () => {
          contextualInspector.classList.toggle('collapsed');
          setTimeout(() => {
            if (typeof fitGraphToViewport === 'function') fitGraphToViewport();
          }, 150);
        });
      }

      // Reproduce Strip CLI Command Generator (Phase 7.2)
      function updateReproduceCommand() {
        const cmdElem = document.getElementById('reproduce-cmd-text');
        if (!cmdElem) return;

        const fn = data.file_name || 'program.R';
        let cmd = `cargo run -p rparse-cli -- analyze ${fn}`;

        if (activeView === 'provenance') {
          const col = (selectedItem && selectedItem.sliceTarget) ? selectedItem.sliceTarget : 'df2$total';
          cmd = `cargo run -p rparse-cli -- provenance ${fn} --column '${col}'`;
        } else if (activeView === 'slice') {
          const sel = document.getElementById('slice-target-select');
          const crit = (sel && sel.value) ? sel.value : 'df2$total';
          cmd = `cargo run -p rparse-cli -- slice ${fn} --criterion '${crit}'`;
        } else if (activeView === 'pdg') {
          cmd = `cargo run -p rparse-cli -- analyze ${fn} --graph pdg --format dot`;
        }

        cmdElem.textContent = cmd;
      }

      const btnCopyCli = document.getElementById('btn-copy-cli');
      if (btnCopyCli) {
        btnCopyCli.addEventListener('click', () => {
          const cmdElem = document.getElementById('reproduce-cmd-text');
          if (!cmdElem) return;
          navigator.clipboard.writeText(cmdElem.textContent).then(() => {
            const orig = btnCopyCli.textContent;
            btnCopyCli.textContent = 'Copied!';
            btnCopyCli.style.color = 'var(--ok)';
            setTimeout(() => {
              btnCopyCli.textContent = orig;
              btnCopyCli.style.color = '';
            }, 1600);
          }).catch(() => {
            btnCopyCli.textContent = 'Error';
          });
        });
      }

      // Command Palette Modal Dialog (Phase 7.3)
      const commandPaletteModal = document.getElementById('command-palette-modal');
      const btnPaletteTrigger = document.getElementById('btn-command-palette-trigger');
      const btnPaletteClose = document.getElementById('btn-palette-close');
      const paletteSearchInput = document.getElementById('palette-search-input');
      const paletteList = document.getElementById('palette-list');

      function openCommandPalette() {
        if (!commandPaletteModal) return;
        commandPaletteModal.classList.add('open');
        if (paletteSearchInput) {
          paletteSearchInput.value = '';
          paletteSearchInput.focus();
        }
        filterPaletteItems('');
      }

      function closeCommandPalette() {
        if (!commandPaletteModal) return;
        commandPaletteModal.classList.remove('open');
      }

      if (btnPaletteTrigger) btnPaletteTrigger.addEventListener('click', openCommandPalette);
      if (btnPaletteClose) btnPaletteClose.addEventListener('click', closeCommandPalette);

      if (commandPaletteModal) {
        commandPaletteModal.addEventListener('click', (e) => {
          if (e.target === commandPaletteModal) closeCommandPalette();
        });
      }

      window.addEventListener('keydown', (e) => {
        if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === 'k') {
          e.preventDefault();
          if (commandPaletteModal && commandPaletteModal.classList.contains('open')) {
            closeCommandPalette();
          } else {
            openCommandPalette();
          }
        } else if (e.key === 'Escape') {
          closeCommandPalette();
        }
      });

      function filterPaletteItems(query) {
        if (!paletteList) return;
        const q = query.toLowerCase().trim();
        const items = paletteList.querySelectorAll('.palette-item');
        items.forEach(it => {
          const text = it.textContent.toLowerCase();
          it.style.display = text.includes(q) ? 'flex' : 'none';
        });
      }

      if (paletteSearchInput) {
        paletteSearchInput.addEventListener('input', (e) => {
          filterPaletteItems(e.target.value);
        });

        paletteSearchInput.addEventListener('keydown', (e) => {
          if (e.key === 'Enter') {
            const firstVisible = paletteList?.querySelector('.palette-item:not([style*="display: none"])');
            if (firstVisible) {
              executePaletteAction(firstVisible.getAttribute('data-action'));
            }
          }
        });
      }

      function executePaletteAction(action) {
        closeCommandPalette();
        if (action === 'new-analysis') openLoadProgramModal();
        else if (action === 'view-prov') document.getElementById('tab-provenance')?.click();
        else if (action === 'view-pdg') document.getElementById('tab-pdg')?.click();
        else if (action === 'view-slice') document.getElementById('tab-slice')?.click();
        else if (action === 'view-schema') document.getElementById('tab-schema')?.click();
        else if (action === 'view-ast') document.getElementById('tab-ast')?.click();
        else if (action === 'run-analysis') document.getElementById('btn-reanalyze')?.click();
        else if (action === 'toggle-wrap') document.getElementById('btn-toggle-wrap')?.click();
        else if (action === 'copy-cli') document.getElementById('btn-copy-cli')?.click();
        else if (action === 'open-guide') window.open('/how-to-use', '_blank');
        else if (action === 'open-investigation') window.open('/landing', '_blank');
      }

      if (paletteList) {
        paletteList.addEventListener('click', (e) => {
          const item = e.target.closest('.palette-item');
          if (item) {
            executePaletteAction(item.getAttribute('data-action'));
          }
        });
      }

      // Bidirectional Source Line Selection (Phase 5)
      if (sourceContainer) {
        sourceContainer.addEventListener('click', (e) => {
          const lineElem = e.target.closest('.source-line');
          if (!lineElem) return;

          const lineNum = parseInt(lineElem.getAttribute('data-line'), 10);
          if (!lineNum || isNaN(lineNum)) return;

          // Clear previous selection
          sourceContainer.querySelectorAll('.source-line.line-selected').forEach(el => el.classList.remove('line-selected'));
          lineElem.classList.add('line-selected');

          // Find associated node or statement in current data
          if (activeView === 'provenance' && data.provenance?.nodes) {
            const provNode = data.provenance.nodes.find(n => n.span && n.span.start_line === lineNum);
            if (provNode) {
              selectNode(provNode.id, 'provenance');
              return;
            }
          }

          if (activeView === 'pdg' && data.pdg?.nodes) {
            const pdgNode = data.pdg.nodes.find(n => n.span && n.span.start_line === lineNum);
            if (pdgNode) {
              selectNode(pdgNode.id, 'pdg');
              return;
            }
          }

          // Otherwise show basic line details in inspector
          showInspector({
            id: `L${lineNum}`,
            title: `Source Line ${lineNum}`,
            type: 'Source Location',
            certainty: 'KNOWN_STATIC',
            span: { start_line: lineNum, start_col: 1, end_line: lineNum, end_col: lineElem.textContent.length }
          });
        });
      }

      // Top bar "Run" button triggers reanalyze
      const btnRunTop = document.getElementById('btn-run-analysis');
      if (btnRunTop) {
        btnRunTop.addEventListener('click', () => {
          if (btnReanalyze) btnReanalyze.click();
        });
      }

      // Initialize reproduce strip
      updateReproduceCommand();

      // Check query parameter or empty source to auto-open Load Program modal
      const studioParams = new URLSearchParams(window.location.search);
      const startBlank = studioParams.get('new') === '1' || studioParams.get('new') === 'true' || !initialSource;
      if (startBlank || !data.summary || data.summary.statement_count === 0) {
        openLoadProgramModal(true);
      }
      recomputeActiveGraphFit();

    })();"###;
