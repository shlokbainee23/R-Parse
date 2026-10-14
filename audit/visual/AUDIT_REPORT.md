# R-PARSE — Comprehensive Visual & Usability Forensic Audit Report

**Auditor:** Senior Product Designer, UI/UX Auditor, & Visual Quality Engineering  
**Project:** R-PARSE (Robust R Language Parsing and Program Analysis Engine)  
**Date:** October 12, 2026  
**Scope:** Phase 1 Visual Forensic Audit (Inspection & Architecture Review Only — Zero Code Modification)  
**Evidence Artifacts:** 53 Full-Fidelity Browser Screenshots in [`audit/visual/`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/) + [`contact_sheet.html`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/contact_sheet.html)

---

## Executive Summary & Core Diagnosis

R-PARSE contains a high-performance, deterministic Rust core (Phases 1–7) delivering fine-grained column provenance ($C_{in} \to C_{out}$), unified program dependence graphs (PDG with row selection distinctions), column-aware static backward slicing, and schema evolution tracking.

However, a visual and architectural forensic examination of the user-facing web layers reveals an acute **product fracture**:
1. **The Cinematic Landing Page** (`landing/index.html` & `landing/styles.css`) is an editorial, dark-canvas experience (`#16161A`, glowing Signal Orange `#FF5A1F`, refined JetBrains Mono typography, custom Three.js ribbons).
2. **The Analytical Studio** (`implementation/crates/rparse-cli/src/format_html_assets.rs`) is a light ivory/beige dashboard (`#F7F5F0`, stark white panels `#FFFFFF`, low-contrast borders) burdened by rectangular container gridlock, a bloated 76px two-row header, competing technical metadata, and critical responsive overflow defects.

When users transition from the landing page into the studio, the product breaks its visual promise. The workspace does not feel like an advanced scientific instrument; it feels like an un-themed, container-heavy utility.

Below is the complete, evidence-based log of all discovered defects, organized strictly by severity.

---

## Priority Defect Classification

```
┌────────────────────────────────────────────────────────────────────────┐
│ P0: CRITICAL (4 Issues) — Broken, misleading, or obstructive defects    │
│ P1: HIGH (6 Issues)     — Major hierarchy, layout & aesthetic schism   │
│ P2: MEDIUM (6 Issues)   — Typography, spacing, tokens & styling         │
│ P3: LOW (4 Issues)      — Refinements, micro-interactions & polish      │
└────────────────────────────────────────────────────────────────────────┘
```

---

## P0: Critical Defects (Broken, Misleading, or Obstructive)

### ISSUE-P0-01: Entry Screen Container Vertical Overflow on Validation Error (Action Buttons Clipped Offscreen)
- **Component:** `.entry-screen` / `.entry-card` / `#entry-validation-error`
- **Evidence Screenshot:** [`studio-02-entry-screen-validation-error.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-02-entry-screen-validation-error.png)
- **Source File & Lines:** [`format_html_assets.rs:L1732-L1778`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-cli/src/format_html_assets.rs#L1732-L1778), CSS [`format_html_assets.rs:L65-L105`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-cli/src/format_html_assets.rs#L65-L105)
- **Description:** `.entry-screen` is styled with `position: absolute; height: 100vh; overflow: hidden; justify-content: center;`. The inner `.entry-card` has fixed `padding: 36px 40px` and contains the textarea editor. When a validation error occurs (e.g., submitting empty code), the `#entry-validation-error` banner is injected, expanding the card height to over 910px. Because `.entry-screen` has no vertical scrolling enabled (`overflow-y: hidden`), the entire action bar (`Analyze Program`, `Load File`, `Load Example...`) is pushed past the bottom edge of the viewport.
- **Usability & Perceived Quality Harm:** The primary CTA is physically unreachable on standard 900px and 768px viewports. The user is trapped: they cannot scroll down to click "Analyze Program" after fixing their code, nor can they select an example from the dropdown. Playwright automated clicks timed out with `element is not visible` trying to interact with the button.
- **Severity:** **Critical (P0)**
- **Recommended Correction:** Add `overflow-y: auto; max-height: 100vh;` to `.entry-screen`, replace `height: 100vh` with `min-height: 100vh`, and ensure `.entry-card` uses compact vertical spacing with `margin: auto 0;`.
- **Presentation vs. Behavior:** Presentation fix only (CSS layout).
- **Dependencies:** None.

---

### ISSUE-P0-02: Studio Header Top-Row Horizontal Overflow at <=880px Viewports (Primary Actions Silently Pushed Off-Screen)
- **Component:** `.app-header` / `.header-top-row` / `.header-actions-row`
- **Evidence Screenshot:** [`studio-26-responsive-768x1024.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-26-responsive-768x1024.png) (and [`studio-27-responsive-390x844.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-27-responsive-390x844.png))
- **Source File & Lines:** [`format_html_assets.rs:L1783-L1820`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-cli/src/format_html_assets.rs#L1783-L1820), CSS [`format_html_assets.rs:L140-L195`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-cli/src/format_html_assets.rs#L140-L195)
- **Description:** `.header-top-row` has `display: flex; justify-content: space-between; gap: 16px;`. Its children are `.header-identity` and `.header-actions-row`. `.header-actions-row` contains the status badge, meta group, 3 summary metrics, the `Investigation ↗` link, the `Evidence` drawer button, and the `New Analysis` button. Total minimum width of this top bar is 878px. At viewports under 880px (e.g., tablet portrait 768px, or split-window desktop), `.header-top-row` scroll width is 878px while client width is 768px (`topRowScrollWidth: 878px > 768px`).
- **Usability & Perceived Quality Harm:** The header overflows horizontally without a scrollbar, slicing off the `New Analysis` and `Evidence` buttons into invisible off-screen space. A user operating in split-screen side-by-side with an R script editor cannot reset analysis or view diagnostic results.
- **Severity:** **Critical (P0)**
- **Recommended Correction:** Consolidate the 2-row header into a single streamlined 44px bar. Move secondary metrics (`Stmts`, `Cols`, `PDG`) into the contextual Evidence Drawer or status popover, allow file breadcrumbs to truncate, and apply responsive media queries that preserve primary action buttons at all screen widths.
- **Presentation vs. Behavior:** Presentation fix only (CSS & markup layout).
- **Dependencies:** Header unification (ISSUE-P1-02).

---

### ISSUE-P0-03: Contextual Inspector Overlay Obscures Active Graph Nodes (Click Target Occlusion)
- **Component:** `.contextual-inspector` inside `.canvas-viewport`
- **Evidence Screenshot:** [`studio-07-provenance-node-selected.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-07-provenance-node-selected.png), [`studio-11-pdg-node-selected.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-11-pdg-node-selected.png)
- **Source File & Lines:** [`format_html_assets.rs:L2030-L2046`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-cli/src/format_html_assets.rs#L2030-L2046), CSS [`format_html_assets.rs:L560-L615`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-cli/src/format_html_assets.rs#L560-L615)
- **Description:** `.contextual-inspector` is rendered as an overlay panel (`position: absolute; right: 0; top: 0; width: 320px; height: 100%; z-index: 10; box-shadow: var(--shadow-lg);`). In both the Column Provenance graph and the Unified PDG, output columns and downstream terminal statement nodes naturally lay out toward the right side of the SVG canvas. When a user clicks one of these right-aligned nodes to inspect its provenance or dependencies, the inspector slides out from the right margin and lands directly on top of the node the user just clicked!
- **Usability & Perceived Quality Harm:** The inspector literally covers the subject of inspection. The user cannot see the selected node's visual connections or highlighting without manually panning the SVG canvas to the left.
- **Severity:** **Critical (P0)**
- **Recommended Correction:** Either:
  1. Transition the inspector from an absolute floating overlay into a collapsible docked pane in the flex layout (`flex: 0 0 320px;`) that dynamically adjusts the SVG viewport width and auto-pans the selected node into clear canvas view; OR
  2. Implement an automatic canvas pan offset (`svg.panToNode(nodeId, offsetRight = 340)`) whenever the inspector is toggled.
- **Presentation vs. Behavior:** Presentation and canvas pan interaction; zero change to analysis semantics.
- **Dependencies:** None.

---

### ISSUE-P0-04: Divergent Studio Modal Markup and False Offline Handling (Misleading Status & Broken Links)
- **Component:** `#studio-modal` in `landing/index.html` vs `index.html` / `landing/investigation.js`
- **Evidence Screenshot:** [`landing-16-studio-modal-default.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/landing-16-studio-modal-default.png) vs [`landing-16b-root-modal-default.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/landing-16b-root-modal-default.png)
- **Source File & Lines:** [`index.html:L800-L840`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/index.html#L800-L840), [`landing/index.html:L740-L820`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/landing/index.html#L740-L820), [`landing/investigation.js:L353-L375`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/landing/investigation.js#L353-L375)
- **Description:** There is an architectural split between root `index.html` and `landing/index.html`:
  - In root `index.html`, the modal contains static text: "Probing local Studio daemon on 127.0.0.1:8080..." and a button linking to `http://127.0.0.1:8080/`. If the server is offline, clicking the button triggers a browser network failure (`ERR_CONNECTION_REFUSED`).
  - In `landing/investigation.js:L353`, `checkStudioDaemon()` attempts to `fetch('http://127.0.0.1:8080/api/health')`. If the fetch fails, it sets the status pill to: `"Offline: Studio server not detected on 127.0.0.1:8080 (start with \"rparse studio\")"`. But the launch button STILL says `"Enter Interactive Studio ↗"` and still links to `http://127.0.0.1:8080/`!
- **Usability & Perceived Quality Harm:** The modal promises live studio access even when the backend daemon is absent, offering no in-modal guidance or disabled state for the link. Furthermore, having two divergent HTML modal implementations across `index.html` and `landing/index.html` creates maintenance confusion.
- **Severity:** **Critical (P0)**
- **Recommended Correction:** Synchronize `index.html` and `landing/index.html`. In the modal, if the daemon is offline, disable the launch button (or change it to `"Start Daemon First"` with visual inactive state) and highlight the 1-click CLI launch command `rparse studio --port 8080 --open`. When online, show a clean, green verified status badge.
- **Presentation vs. Behavior:** Frontend modal logic and presentation.
- **Dependencies:** None.

---

## P1: High Priority (Hierarchy, Layout & Aesthetic Schism)

### ISSUE-P1-01: Aesthetic Schism Between Landing and Studio ("Two Different Products")
- **Component:** Global Design Tokens, Color Palette, Surfaces, Atmosphere
- **Evidence Screenshot:** Compare [`landing-01-hero-initial.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/landing-01-hero-initial.png) with [`studio-04-workspace-initial.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-04-workspace-initial.png)
- **Source Files:** [`landing/styles.css:L10-L65`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/landing/styles.css#L10-L65), [`format_html_assets.rs:L14-L70`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-cli/src/format_html_assets.rs#L14-L70)
- **Description:** The landing page immerses the user in an editorial investigation aesthetic: deep graphite canvas (`--window-deep: #16161A`), card surfaces (`#1E1E24`, `#25252D`), crisp white typography with subtle opacity steps, Signal Orange (`#FF5A1F`), and glowing Lilac (`#B8A6FF`). The Studio workspace abruptly switches to a warm ivory paper canvas (`--bg-page: #F7F5F0`), bright white panels (`--bg-panel: #FFFFFF`), and low-contrast dark gray ink (`#121212`).
- **Usability & Perceived Quality Harm:** The jarring shift shatters immersion. The landing page feels like an elite research-grade instrument, but the studio feels like a separate, hastily generated utility. The brand identity does not carry over.
- **Severity:** **High (P1)**
- **Recommended Correction:** Align the Studio with the landing page's design language: implement an engineering dark-mode palette (`#121216` canvas, `#1A1A20` panels, `#22222A` surfaces, `#FF5A1F` Signal Orange highlights, `#B8A6FF` dataflow lilac, and subtle precision borders `rgba(255,255,255,0.08)`).
- **Presentation vs. Behavior:** Presentation only (CSS tokens & styles).
- **Dependencies:** Shared design tokens (ISSUE-P2-01).

---

### ISSUE-P1-02: Redundant Two-Row Studio Header Consuming Permanent Canvas Real Estate
- **Component:** `.app-header` (`.header-top-row` + `.app-subnav`)
- **Evidence Screenshot:** [`studio-05-header-detail.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-05-header-detail.png)
- **Source File & Lines:** [`format_html_assets.rs:L1783-L1848`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-cli/src/format_html_assets.rs#L1783-L1848), CSS [`format_html_assets.rs:L140-L240`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-cli/src/format_html_assets.rs#L140-L240)
- **Description:** The Studio header is partitioned into two distinct rows:
  - Row 1 (42px): Wordmark `R—PARSE / Studio / showcase_pipeline.R`, Status badge, Meta items, Stats strip, Investigation link, Evidence button, New Analysis button.
  - Row 2 (34px): 5 subnav tabs (`Provenance`, `PDG Graph`, `Backward Slice`, `Schema Evolution`, `AST Explorer`).
  Together with 1px borders, this header permanently consumes 78px of vertical height.
- **Usability & Perceived Quality Harm:** In visual program analysis, canvas height is the most precious resource for reading graphs and tracing multi-line code pipelines. A 78px double header reduces visible canvas lines by 15–20% on 900px screens.
- **Severity:** **High (P1)**
- **Recommended Correction:** Consolidate into a single, elegant 44px top navigation bar. Place brand mark + filename on the left, subnav tabs in the center, and quiet status indicator + primary actions on the right.
- **Presentation vs. Behavior:** Presentation only.
- **Dependencies:** None.

---

### ISSUE-P1-03: Excessive Rectangular Containers & Border Gridlock
- **Component:** Global Workspace Layout Containers
- **Evidence Screenshot:** [`studio-04-workspace-initial.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-04-workspace-initial.png), [`studio-06-tab-provenance.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-06-tab-provenance.png)
- **Source File & Lines:** [`format_html_assets.rs:L1850-L2059`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-cli/src/format_html_assets.rs#L1850-L2059)
- **Description:** The user interface is boxed in by dozens of nested rectangular containers, each with its own 1px border:
  1. Header row 1 border-bottom
  2. Header row 2 border-bottom
  3. Source pane border-right
  4. Source pane header border-bottom
  5. Source viewer border
  6. Canvas toolbar border-bottom
  7. Canvas container border
  8. Inspector header border-bottom
  9. Inspector property tables with multiple borders
  10. Filter chips, search inputs, and stat badges with individual borders
- **Usability & Perceived Quality Harm:** "Border gridlock" fragments the interface into small visual cages, increasing cognitive friction and making the analytical canvas feel cramped. Modern developer tools use tonal surface shifts (e.g. 2–3% lightness contrast) rather than hard 1px strokes.
- **Severity:** **High (P1)**
- **Recommended Correction:** Remove redundant borders. Use subtle background contrast shifts (e.g., canvas `#121216` vs pane `#18181E`) and single border rules only where structural separation is necessary.
- **Presentation vs. Behavior:** Presentation only.
- **Dependencies:** ISSUE-P1-01.

---

### ISSUE-P1-04: Studio Session Modal Technical Over-Documentation & Hierarchy Inversion
- **Component:** `#studio-modal` in `landing/index.html`
- **Evidence Screenshot:** [`landing-17-studio-modal-cli-expanded.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/landing-17-studio-modal-cli-expanded.png)
- **Source File & Lines:** [`landing/index.html:L745-L815`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/landing/index.html#L745-L815)
- **Description:** The Studio Session modal contains:
  1. An explanatory lead paragraph
  2. A 4-bullet capabilities list (`Syntax & Semantics`, `Column Provenance`, `Dependencies & Slicing`, `Schema Evolution`)
  3. A live server status pill
  4. An expandable `<details>` disclosure with 4 complete terminal command boxes (Headless analysis, Targeted slice, Column provenance, Studio launch) with individual "Copy" buttons and hint paragraphs
  5. The actual launch button placed at the very bottom
- **Usability & Perceived Quality Harm:** The modal is suffering from an identity crisis: it is trying to be a landing page, a CLI documentation page, and a session launcher all at once. The primary user intent—launching the Studio session—is buried under 500px of documentation.
- **Severity:** **High (P1)**
- **Recommended Correction:** Strip out the capabilities list (which is already explained in Section 05). Focus the modal on session connection status:
  - If daemon is running: Large active status indicator + immediate "Open Studio Session ↗" CTA.
  - If daemon is offline: Clean 1-line copy command `rparse studio` with a one-click copy button.
- **Presentation vs. Behavior:** Presentation only.
- **Dependencies:** ISSUE-P0-04.

---

### ISSUE-P1-05: Schema Evolution Diff Legend Badges Resemble Clickable Interactive Filters (Dead UI)
- **Component:** `.diff-badge` in `#toolbar-schema`
- **Evidence Screenshot:** [`studio-14-tab-schema-evolution.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-14-tab-schema-evolution.png)
- **Source File & Lines:** [`format_html_assets.rs:L1951-L1956`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-cli/src/format_html_assets.rs#L1951-L1956), CSS [`format_html_assets.rs:L480-L510`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-cli/src/format_html_assets.rs#L480-L510)
- **Description:** In the Schema Evolution toolbar, four badges are displayed on the right: `+added`, `-removed`, `~modified`, `=retained`. They have rounded corners (`border-radius: var(--radius-sm)`), colored borders, and distinct background tints. Their visual styling is nearly indistinguishable from the interactive toggle chips in the PDG toolbar (`Data`, `Control`, `RowSelection`, `Unknown`). However, these badges have no event listeners and do nothing when clicked.
- **Usability & Perceived Quality Harm:** Users repeatedly click them expecting to filter the schema cards by mutation type. This violates the principle of visual affordance and creates user frustration (Dead UI).
- **Severity:** **High (P1)**
- **Recommended Correction:** Either:
  1. Prefix with a quiet label `Legend:` and style them as plain text indicators without button-like pill borders; OR
  2. Implement client-side filtering that highlights or filters schema cards matching the selected diff mutation.
- **Presentation vs. Behavior:** Presentation (or optional interactive filter).
- **Dependencies:** None.

---

### ISSUE-P1-06: Competing & Cluttered Status Metrics in Header Top Row
- **Component:** `.header-actions-row`
- **Evidence Screenshot:** [`studio-05-header-detail.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-05-header-detail.png)
- **Source File & Lines:** [`format_html_assets.rs:L1796-L1817`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-cli/src/format_html_assets.rs#L1796-L1817)
- **Description:** The right side of the top header row jams 9 items together:
  `[● Analysis ready] [Static analysis • Known static] [Stmts: 15] [Cols: 7] [PDG: 22] [Investigation ↗] [Evidence] [New Analysis]`.
  All items have similar font size (11px) and competing font weights.
- **Usability & Perceived Quality Harm:** The eye is overwhelmed by technical metadata that doesn't aid the immediate analytical task. Essential diagnostic information (`Known static` vs `UNKNOWN_STATIC`) is visually diluted by statement counters.
- **Severity:** **High (P1)**
- **Recommended Correction:** Retain only the primary status indicator (`● Static Analysis Ready` or `⚠️ Static Uncertainty Detected`) and primary actions (`Evidence`, `New Analysis`). Move statement/node counters into the Evidence Drawer where comprehensive compilation stats are already shown.
- **Presentation vs. Behavior:** Presentation only.
- **Dependencies:** ISSUE-P1-02.

---

## P2: Medium Priority (Typography, Spacing, Tokens & Styling)

### ISSUE-P2-01: Arbitrary Token Aliasing & Inconsistent Colorway (`--accent-blue: #FF5A1F`)
- **Component:** CSS `:root` token definitions in `format_html_assets.rs`
- **Source File & Lines:** [`format_html_assets.rs:L44-L65`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-cli/src/format_html_assets.rs#L44-L65)
- **Description:** Line 45 contains:
  ```css
  /* Required alias for test assertion: border-bottom: 2px solid var(--accent-blue) */
  --accent-blue: #FF5A1F;
  --accent-blue-subtle: #FFF0EB;
  ```
  The code aliases "blue" to Signal Orange (`#FF5A1F`) because a legacy test asserted that property! In addition, violet is defined as `--accent-purple: #7C3AED` in Studio, while Landing uses pale glowing `--accent-lilac: #B8A6FF`.
- **Usability & Perceived Quality Harm:** Confusing and error-prone CSS tokens that cause developer friction and visual drift between landing page and studio components.
- **Severity:** **Medium (P2)**
- **Recommended Correction:** Standardize tokens across both codebases (`--accent-primary: #FF5A1F`, `--accent-lilac: #B8A6FF`, `--accent-emerald: #059669`, `--accent-amber: #D97706`). Maintain legacy `--accent-blue` only as a documented deprecated fallback if required by tests.
- **Presentation vs. Behavior:** Presentation only.
- **Dependencies:** None.

---

### ISSUE-P2-02: Source Code Viewer Line Height, Wrapping Jitter, and Edit Mode Replacement
- **Component:** `.pane-source`, `.source-viewer`, `.source-editor-textarea`
- **Evidence Screenshot:** [`studio-19-source-pane-word-wrap-on.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-19-source-pane-word-wrap-on.png), [`studio-21-source-pane-edit-mode.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-21-source-pane-edit-mode.png)
- **Source File & Lines:** [`format_html_assets.rs:L300-L370`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-cli/src/format_html_assets.rs#L300-L370)
- **Description:** In the source viewer, line height is tightly set at `1.35` with no vertical padding between lines. When a user clicks "Edit", the read-only syntax-highlighted viewer with gutter line numbers is completely replaced by a bare `<textarea>` with zero line numbers and plain monospace font.
- **Usability & Perceived Quality Harm:** Switching to Edit mode causes jarring visual disruption and loss of positional reference (line numbers vanish).
- **Severity:** **Medium (P2)**
- **Recommended Correction:** Increase source code line-height to `1.55`. Provide a synchronized line gutter for the edit textarea, or integrate a lightweight syntax-aware editor wrapper.
- **Presentation vs. Behavior:** Presentation and editor UX.
- **Dependencies:** None.

---

### ISSUE-P2-03: Stale Source Banner Layout Shift and Clashing Amber Theme
- **Component:** `#source-stale-banner`
- **Evidence Screenshot:** [`studio-22-source-stale-banner.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-22-source-stale-banner.png)
- **Source File & Lines:** [`format_html_assets.rs:L1868-L1876`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-cli/src/format_html_assets.rs#L1868-L1876), CSS [`format_html_assets.rs:L380-L405`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-cli/src/format_html_assets.rs#L380-L405)
- **Description:** When the source code is modified in edit mode, `#source-stale-banner` appears directly above the source viewer, pushing all code lines downward by 36px. The banner uses a harsh yellow/amber background with an unstyled emoji `⚠️` and raw white buttons (`background:#ffffff;`) hardcoded in inline styles.
- **Usability & Perceived Quality Harm:** Layout shift disrupts reading context. Inline button styles bypass the design token system.
- **Severity:** **Medium (P2)**
- **Recommended Correction:** Render the stale indicator as a sleek non-shifting status pill in the source pane header (e.g. next to the line count badge), with clean icon buttons that use the shared design tokens.
- **Presentation vs. Behavior:** Presentation only.
- **Dependencies:** None.

---

### ISSUE-P2-04: Missing Technical Canvas Engineering Grid / Dot Matrix
- **Component:** `.canvas-viewport`, `#svg-provenance`, `#svg-pdg`
- **Evidence Screenshot:** [`studio-06-tab-provenance.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-06-tab-provenance.png), [`studio-09-tab-pdg.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-09-tab-pdg.png)
- **Source File & Lines:** CSS [`format_html_assets.rs:L515-L560`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-cli/src/format_html_assets.rs#L515-L560)
- **Description:** The canvas background in Provenance and PDG is a flat solid color (`#F7F5F0`). There is no spatial grid, coordinate markers, or subtle dot matrix.
- **Usability & Perceived Quality Harm:** The canvas feels empty and unpolished. High-end visual dev tools (e.g. Figma, Linear, Node-RED, Graphviz visualizers) use subtle dot matrices or graph grids to provide depth, motion anchors during pan/zoom, and an engineering-grade aesthetic.
- **Severity:** **Medium (P2)**
- **Recommended Correction:** Add an SVG/CSS background dot-matrix or engineering coordinate grid (`background-image: radial-gradient(rgba(255,255,255,0.08) 1px, transparent 1px); background-size: 24px 24px;`).
- **Presentation vs. Behavior:** Presentation only.
- **Dependencies:** ISSUE-P1-01.

---

### ISSUE-P2-05: Weak Contrast & Low Readability of Backward Slicing Retention Highlights
- **Component:** `.source-line.slice-keep`, `.source-line.slice-drop`
- **Evidence Screenshot:** [`studio-12-tab-backward-slice.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-12-tab-backward-slice.png)
- **Source File & Lines:** CSS [`format_html_assets.rs:L430-L465`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-cli/src/format_html_assets.rs#L430-L465)
- **Description:** In the Backward Slice view, retained code lines have a faint light-green background tint (`#ECFDF5`), and dropped lines have a pale red wash (`#FEE2E2`). Against the ivory background, the contrast ratio is under 1.4:1.
- **Usability & Perceived Quality Harm:** Users cannot easily distinguish between kept and dropped code fragments without intense scrutiny. Slicing attribution—one of the core capabilities of R-PARSE—lacks visual impact and clarity.
- **Severity:** **Medium (P2)**
- **Recommended Correction:** Use high-contrast gutter badges (`[KEEP]` in bold emerald `#059669`, `[DROP]` in muted strike-through `#DC2626` or 40% opacity) and solid left border markers (3px solid accent line), ensuring immediate scannability regardless of background color.
- **Presentation vs. Behavior:** Presentation only.
- **Dependencies:** None.

---

### ISSUE-P2-06: Graph Node Label Truncation and Overlapping Bezier Edges in PDG
- **Component:** `#svg-pdg`, `#svg-provenance`
- **Evidence Screenshot:** [`studio-09-tab-pdg.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-09-tab-pdg.png)
- **Source File & Lines:** [`format_html_assets.rs:L2550-L2700`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-cli/src/format_html_assets.rs#L2550-L2700)
- **Description:** PDG statement nodes with long expressions (e.g. `mutate(total = price * quantity, discount = total * 0.1)`) render raw text into SVG `<text>` elements. Text occasionally spills outside node rectangle borders. Furthermore, control and data dependency edges cross each other at steep angles without bridge hops or curvature offsets.
- **Usability & Perceived Quality Harm:** Visual clutter when graphs grow beyond 15 nodes, reducing graph comprehension.
- **Severity:** **Medium (P2)**
- **Recommended Correction:** Enforce character truncation (e.g. 28 chars with ellipsis `…`) on SVG node labels with full expression shown on hover/inspector. Add gentle edge separation offsets for parallel or crossing dependency links.
- **Presentation vs. Behavior:** SVG rendering presentation only.
- **Dependencies:** None.

---

## P3: Low Priority (Polish & Micro-Interactions)

### ISSUE-P3-01: Splitter Resize Handle Lacks Visual Affordance and Hover State
- **Component:** `.splitter` (`#splitter-source`)
- **Evidence Screenshot:** [`studio-04-workspace-initial.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-04-workspace-initial.png)
- **Source File & Lines:** CSS [`format_html_assets.rs:L285-L300`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-cli/src/format_html_assets.rs#L285-L300)
- **Description:** The pane splitter between source and canvas is an invisible 4px divider. It only reveals itself via the `cursor: col-resize`. It has no visual indicator or hover highlight.
- **Severity:** **Low (P3)**
- **Recommended Correction:** Add a subtle center grab pill (2 vertical dots or a 2px highlight rule) that lights up in Signal Orange (`#FF5A1F`) on hover.

---

### ISSUE-P3-02: Subnav Tab Count Badges Show Low-Value "0" or Single Digits
- **Component:** `.nav-count` in `.app-subnav`
- **Evidence Screenshot:** [`studio-05-header-detail.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-05-header-detail.png)
- **Source File & Lines:** [`format_html_assets.rs:L1821-L1847`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-cli/src/format_html_assets.rs#L1821-L1847)
- **Description:** Every tab displays a count badge: `Provenance: 0`, `PDG Graph: 0`, `Backward Slice: 0`, `Schema Evolution: 0`, `AST Explorer: 0`. Displaying "0" on tabs before selection creates an impression that the view is empty or broken.
- **Severity:** **Low (P3)**
- **Recommended Correction:** Hide count badges when count is 0, or replace with contextual badges only when a view has specific warnings or mutations to report.

---

### ISSUE-P3-03: Inconsistent Button Styles and Padding Across Panels
- **Component:** `.btn-ghost`, `.btn-secondary`, `.btn-primary`, `.btn-outline`
- **Evidence Screenshot:** [`studio-05-header-detail.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-05-header-detail.png), [`studio-16-tab-ast-explorer-cst.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-16-tab-ast-explorer-cst.png)
- **Source File & Lines:** CSS [`format_html_assets.rs:L105-L140`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-cli/src/format_html_assets.rs#L105-L140)
- **Description:** Button heights and paddings vary haphazardly: some use `padding: 2px 6px; font-size: 10px;`, others use `padding: 4px 10px; font-size: 11px;`, and AST buttons use `.btn-outline` with dark borders.
- **Severity:** **Low (P3)**
- **Recommended Correction:** Standardize into 3 strict button tiers: `.btn-sm` (24px height), `.btn-md` (30px height), and `.btn-lg` (38px height) using unified tokens.

---

### ISSUE-P3-04: Hardcoded Inline Styles on External Links in Navigation
- **Component:** `header-actions-row a[href="/landing"]`
- **Evidence Screenshot:** [`studio-05-header-detail.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-05-header-detail.png)
- **Source File & Lines:** [`format_html_assets.rs:L1814`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-cli/src/format_html_assets.rs#L1814)
- **Description:** The link back to the landing page uses inline CSS overrides:
  `style="text-decoration:none; display:inline-flex; align-items:center; gap:4px; font-weight:600; color:#FF5A1F; font-size:11px;"`.
- **Severity:** **Low (P3)**
- **Recommended Correction:** Move into a clean class `.header-nav-link` adhering to the design token stylesheet.

---

## Prioritized Remediation Summary

| Issue ID | Severity | Component | Key Impact |
| :--- | :--- | :--- | :--- |
| **ISSUE-P0-01** | Critical | Code Entry Screen | Actions pushed off-screen on validation errors |
| **ISSUE-P0-02** | Critical | Studio Header | Primary buttons cut off at <=880px viewports |
| **ISSUE-P0-03** | Critical | Contextual Inspector | Inspector slides out and covers active graph nodes |
| **ISSUE-P0-04** | Critical | Studio Session Modal | False offline handling & divergent modal code |
| **ISSUE-P1-01** | High | Global Brand & Tokens | Aesthetic schism: dark cinematic landing vs light ivory studio |
| **ISSUE-P1-02** | High | Header Architecture | Bloated 76px two-row header eats canvas height |
| **ISSUE-P1-03** | High | Workspace Containers | Container gridlock and border bloat creates visual fatigue |
| **ISSUE-P1-04** | High | Studio Launch Modal | Modal over-documentation buries launch CTA |
| **ISSUE-P1-05** | High | Schema Evolution Toolbar | Static diff badges look like filter buttons (Dead UI) |
| **ISSUE-P1-06** | High | Header Action Group | 9 competing status items clutter top navigation |
| **ISSUE-P2-01** | Medium | Design Tokens | `--accent-blue` aliased to orange; inconsistent colors |
| **ISSUE-P2-02** | Medium | Source Code Viewer | Cramped line height & jarring textarea replacement |
| **ISSUE-P2-03** | Medium | Stale Source Banner | Harsh amber banner injects vertical layout shift |
| **ISSUE-P2-04** | Medium | Analytical Canvas | Missing spatial engineering dot matrix / grid |
| **ISSUE-P2-05** | Medium | Backward Slice Highlights | Pale retention highlights suffer from low contrast (<1.4:1) |
| **ISSUE-P2-06** | Medium | Graph Visualization | Long node labels truncate or overflow SVG boxes |
| **ISSUE-P3-01** | Low | Workspace Splitter | Divider lacks hover state and grab affordance |
| **ISSUE-P3-02** | Low | Navigation Tab Counts | Empty "0" count badges add visual noise |
| **ISSUE-P3-03** | Low | Button Styles | Inconsistent padding and heights across toolbars |
| **ISSUE-P3-04** | Low | Header Links | Inline styling on navigation anchor tag |
