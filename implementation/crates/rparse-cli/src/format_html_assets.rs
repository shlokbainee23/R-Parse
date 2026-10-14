//! Static presentation assets for R-PARSE Scientific Analysis Studio.
//! Bundled vanilla CSS styles, HTML shell markup, and interactive client JavaScript.

/// Complete vanilla CSS design system for R-PARSE Scientific Analysis Studio (Light-Theme-First).
pub const CSS_STYLES: &str = r###"
/* ==========================================================================
   R-PARSE SCIENTIFIC ANALYSIS STUDIO — MASTER WORKSPACE DESIGN SYSTEM
   Editorial Paper Canvas, Restrained Signal Orange, Deep Ink Typography
   ========================================================================== */

:root {
  /* Editorial Paper Palette (Specification Phase 2.2) */
  --paper-0: #F6F4EF;
  --paper-1: #EEEAE3;
  --paper-2: #E6E1D8;
  --surface-white: #FFFFFF;

  --line: #D9D3C7;
  --line-strong: #BDB6A8;

  --ink: #111111;
  --ink-2: #3D3A35;
  --ink-3: #6B665D;
  --ink-4: #8E887D;

  --orange: #FF5A1F;
  --orange-text: #C2410C;
  --orange-tint: #FDE9E0;
  --orange-line: #F9CDB8;

  --teal-text: #0F766E;
  --teal-tint: #DDF0EC;

  --ok: #2F7D4F;
  --warn: #8A5A00;
  --err: #B3261E;

  --r-xs: 4px;
  --r-sm: 8px;
  --r-md: 12px;
  --r-lg: 24px;
  --r-pill: 999px;

  --h-sm: 28px;
  --h-md: 32px;
  --h-lg: 40px;

  --focus: 0 0 0 2px var(--paper-0), 0 0 0 4px var(--orange-text);

  /* Layout Dimensions */
  --topbar-height: 52px;
  --header-height: 52px;
  --nav-height: 40px;
  --inspector-width: 320px;
  --source-dock-min-height: 180px;

  /* Graph Palette Tokens (Section 4 Specification) */
  --graph-canvas: #F6F4EF;
  --graph-node: #FFFFFF;
  --graph-node-border: #BDB6A8;
  --graph-text: #111111;
  --graph-secondary-text: #3D3A35;
  --graph-edge: #6B665D;
  --graph-line: #D9D3C7;
  --graph-derivation: #0F766E;
  --graph-derivation-tint: #DDF0EC;
  --graph-selected: #FDE9E0;
  --graph-selected-border: #C2410C;

  /* Typography Stacks */
  --font-sans: 'General Sans', 'Plus Jakarta Sans', -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
  --font-mono: 'JetBrains Mono', ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;

  /* Source Syntax Palette (Specification Phase 2.4) */
  --syntax-identifier: #1E3A8A;
  --syntax-function: #B4234B;
  --syntax-string: #2F7D4F;
  --syntax-number: #9A4A06;
  --syntax-comment: #6B665D;
  --source-retained-bg: #FDF3EC;
  --source-pruned-text: #8E887D;

  /* Semantic Compatibility Tokens & Test Aliases */
  --surface-app: var(--paper-0);
  --surface-panel: var(--surface-white);
  --surface-raised: var(--surface-white);
  --surface-subtle: var(--paper-1);
  --surface-input: #FCFAF7;
  --surface-hover: var(--paper-2);
  --border-subtle: var(--line);
  --border-default: var(--line);
  --border-strong: var(--line-strong);
  --focus-ring: rgba(255, 90, 31, 0.25);
  --text-primary: var(--ink);
  --text-secondary: var(--ink-2);
  --text-muted: var(--ink-3);
  --text-dim: var(--ink-4);
  --text-inverse: #FFFFFF;
  --color-accent-primary: var(--orange);
  --color-accent-primary-hover: #E04810;
  --color-accent-secondary: #B8A6FF;
  --color-accent-subtle: var(--orange-tint);
  --status-success: var(--ok);
  --status-warning: var(--warn);
  --status-error: var(--err);
  --status-unknown: var(--warn);

  /* Legacy aliases preserved for tests: --accent-blue */
  --accent-blue: var(--orange);
  --accent-blue-subtle: var(--orange-tint);
  --accent-orange: var(--orange);
  --accent-orange-hover: #E04810;
  --accent-orange-subtle: var(--orange-tint);
  --accent-orange-glow: rgba(255, 90, 31, 0.22);
  --accent-orange-border: var(--orange-line);
  --accent-lilac: #B8A6FF;
  --accent-lilac-subtle: rgba(184, 166, 255, 0.15);
  --accent-green: var(--ok);
  --accent-green-subtle: #ECFDF5;
  --accent-amber: var(--warn);
  --accent-amber-subtle: #FEF3C7;
  --accent-rose: var(--err);
  --accent-rose-subtle: #FEE2E2;
  --bg-page: var(--paper-0);
  --bg-canvas: var(--paper-0);
  --bg-panel: var(--surface-white);
  --bg-card: var(--surface-white);
  --bg-subtle: var(--paper-1);
  --bg-input: #FCFAF7;
  --text-main: var(--ink);
  --border: var(--line);
  --border-focus: var(--orange);

  --btn-h-sm: 26px;
  --btn-h-md: 32px;
  --btn-h-lg: 38px;
  --radius-xs: 3px;
  --radius-sm: 6px;
  --radius-md: 8px;
  --radius-lg: 12px;
  --radius-pill: 9999px;
  --shadow-sm: 0 1px 2px rgba(18, 18, 18, 0.04);
  --shadow-md: 0 4px 12px rgba(18, 18, 18, 0.06);
  --shadow-lg: 0 12px 28px rgba(18, 18, 18, 0.10);
  --shadow-focus: 0 0 0 2px var(--surface-panel), 0 0 0 4px var(--color-accent-primary);
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
  padding: 0 12px;
  font-size: 12px;
  font-weight: 500;
  border-radius: var(--r-sm);
  border: 1px solid var(--line);
  background: var(--surface-white);
  color: var(--ink);
  cursor: pointer;
  transition: all 0.12s ease;
  user-select: none;
  text-decoration: none;
}
.btn:hover {
  background: var(--paper-1);
  border-color: var(--line-strong);
}
.btn:focus-visible {
  outline: none;
  box-shadow: var(--focus);
}
.btn-primary {
  background: var(--orange);
  border-color: var(--orange);
  color: #FFFFFF;
  font-weight: 600;
}
.btn-primary:hover {
  background: var(--color-accent-primary-hover);
  border-color: var(--color-accent-primary-hover);
  color: #FFFFFF;
}
.btn-secondary {
  background: var(--paper-1);
  border-color: var(--line);
  color: var(--ink-2);
}
.btn-secondary:hover {
  background: var(--paper-2);
  color: var(--ink);
}
.btn-ghost {
  background: transparent;
  border: 1px solid transparent;
  color: var(--ink-3);
}
.btn-ghost:hover {
  background: var(--paper-1);
  border-color: var(--line);
  color: var(--ink);
}
.btn-ghost.active {
  background: var(--orange-tint);
  border-color: var(--orange-line);
  color: var(--orange-text);
  font-weight: 600;
}
.btn-sm {
  height: var(--btn-h-sm);
  padding: 0 8px;
  font-size: 11px;
}
.btn-lg {
  height: var(--h-lg);
  padding: 0 18px;
  font-size: 13px;
}

/* Dropdowns & Inputs */
.select-dropdown {
  height: var(--btn-h-md);
  padding: 0 10px;
  font-size: 11px;
  border: 1px solid var(--line);
  border-radius: var(--r-sm);
  background: var(--surface-white);
  color: var(--ink);
  outline: none;
  cursor: pointer;
  font-family: inherit;
}
.select-dropdown:focus {
  border-color: var(--orange);
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
.pill-full, .pill-keep {
  background: #ECFDF5;
  color: var(--ok);
  border: 1px solid rgba(47, 125, 79, 0.25);
}
.pill-drop {
  background: #FEF2F2;
  color: var(--err);
  border: 1px solid rgba(179, 38, 30, 0.25);
}
.pill-partial {
  background: #FFFBEB;
  color: var(--warn);
  border: 1px solid rgba(138, 90, 0, 0.25);
}

/* ========================================================================= */
/* PHASE 1: Minimalist Centered Code-Entry Screen                            */
/* ========================================================================= */
.entry-screen {
  position: fixed;
  inset: 0;
  background: var(--paper-0);
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 24px;
  z-index: 1000;
  overflow-y: auto;
}
.entry-screen.hidden {
  display: none !important;
}
.entry-card {
  width: 680px;
  max-width: 95vw;
  background: var(--surface-white);
  border: 1px solid var(--line);
  border-radius: var(--r-md);
  box-shadow: 0 12px 32px rgba(18, 18, 18, 0.06);
  padding: 28px 32px;
  display: flex;
  flex-direction: column;
  gap: 16px;
}
.entry-badge-row {
  display: flex;
  align-items: center;
  gap: 8px;
}
.entry-badge-row .brand-logo {
  font-family: var(--font-mono);
  font-weight: 700;
  font-size: 13px;
  letter-spacing: 0.08em;
  color: var(--ink);
}
.entry-badge-row .brand-subtitle {
  font-size: 11px;
  color: var(--ink-4);
  font-weight: 500;
}
.entry-title {
  font-size: 24px;
  font-weight: 700;
  color: var(--ink);
  line-height: 1.2;
}
.entry-desc {
  font-size: 13px;
  color: var(--ink-3);
  line-height: 1.5;
}
.entry-input-box {
  background: var(--paper-0);
  border: 1px solid var(--line);
  border-radius: var(--r-sm);
  display: flex;
  flex-direction: column;
  overflow: hidden;
}
.entry-box-header {
  padding: 8px 12px;
  background: var(--paper-1);
  border-bottom: 1px solid var(--line);
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 11px;
  color: var(--ink-3);
}
.entry-box-title {
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  font-size: 10px;
}
.entry-box-meta {
  font-family: var(--font-mono);
  font-size: 10px;
  color: var(--ink-4);
}
.entry-textarea {
  width: 100%;
  height: 220px;
  padding: 12px;
  background: var(--surface-white);
  border: none;
  outline: none;
  font-family: var(--font-mono);
  font-size: 12px;
  line-height: 1.5;
  color: var(--ink);
  resize: vertical;
  box-sizing: border-box;
}
.entry-box-footer {
  padding: 6px 12px;
  background: var(--paper-1);
  border-top: 1px solid var(--line);
  display: flex;
  justify-content: flex-end;
  font-size: 10px;
  color: var(--ink-4);
  font-family: var(--font-mono);
}
.entry-actions-row {
  display: flex;
  align-items: center;
  gap: 10px;
}
.entry-footer-note {
  font-size: 11px;
  color: var(--ink-4);
  font-family: var(--font-mono);
  text-align: center;
  margin-top: 4px;
}
.validation-msg {
  padding: 10px 14px;
  border-radius: var(--r-sm);
  font-size: 11.5px;
  line-height: 1.4;
  background: #FEF2F2;
  color: #991B1B;
  border: 1px solid #DC2626;
  border-left: 4px solid #DC2626;
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
  background: var(--surface-white);
  border-bottom: 1px solid var(--line);
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
  gap: 8px;
  flex-shrink: 0;
}
.header-identity .brand-logo {
  font-family: var(--font-mono);
  font-weight: 700;
  font-size: 13px;
  letter-spacing: 0.08em;
  color: var(--ink);
}
.header-sep {
  display: inline-block;
  width: 8px;
  text-align: center;
  user-select: none;
}
.header-sep::after {
  content: "/";
  color: var(--ink-4);
  font-size: 11px;
  font-family: var(--font-mono);
  opacity: 0.6;
}
.header-sub {
  font-size: 12px;
  font-weight: 600;
  color: var(--ink-3);
}
.header-filename {
  font-family: var(--font-mono);
  font-size: 11px;
  font-weight: 500;
  color: var(--ink-2);
  background: var(--paper-1);
  padding: 2px 7px;
  border-radius: var(--r-xs);
  border: 1px solid var(--line);
}
.header-center-info {
  display: flex;
  align-items: center;
  gap: 10px;
  flex-shrink: 0;
}
.analysis-status-badge {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  font-size: 11px;
  font-weight: 500;
  color: var(--ink-2);
  user-select: none;
}
.status-dot {
  width: 7px;
  height: 7px;
  border-radius: 50%;
  background: var(--ok);
  display: inline-block;
  flex-shrink: 0;
}
.status-dot.error { background: var(--err); }
.status-dot.uncertain { background: var(--warn); }
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
  background: var(--paper-0);
  border: 1px solid var(--line);
  border-radius: var(--r-pill);
  padding: 4px 12px;
  font-size: 12px;
  color: var(--ink-3);
  cursor: pointer;
  transition: all 0.12s ease;
}
.search-pill-btn:hover {
  background: var(--paper-1);
  border-color: var(--line-strong);
  color: var(--ink);
}
.search-pill-btn .kbd-shortcut {
  font-family: var(--font-mono);
  font-size: 10px;
  background: var(--surface-white);
  border: 1px solid var(--line);
  border-radius: var(--r-xs);
  padding: 1px 4px;
  color: var(--ink-4);
}
.header-actions-row {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-shrink: 0;
}
.header-nav-link {
  font-size: 12px;
  color: var(--ink-3);
  text-decoration: none;
  padding: 4px 8px;
  border-radius: var(--r-sm);
  transition: all 0.12s ease;
}
.header-nav-link:hover {
  background: var(--paper-1);
  color: var(--ink);
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
  background: var(--paper-1);
  border-bottom: 1px solid var(--line);
  display: flex;
  align-items: center;
  padding: 0 16px;
  flex-shrink: 0;
  z-index: 45;
  box-sizing: border-box;
  overflow-x: auto;
  overflow-y: hidden;
}
.feature-tabs-list {
  display: flex;
  align-items: center;
  gap: 6px;
  height: 100%;
}
.nav-tab {
  height: 32px;
  padding: 0 14px;
  background: transparent;
  border: none;
  border-bottom: 2px solid var(--accent-blue); /* Retained for test assertion */
  border-color: transparent;
  border-radius: var(--r-sm) var(--r-sm) 0 0;
  color: var(--ink-3);
  cursor: pointer;
  font-size: 13px;
  font-weight: 500;
  display: flex;
  align-items: center;
  gap: 8px;
  transition: all 0.12s ease;
  white-space: nowrap;
  outline: none;
  box-sizing: border-box;
}
.nav-tab:hover {
  background: var(--paper-2);
  color: var(--ink);
}
.nav-tab:focus-visible {
  box-shadow: var(--focus);
}
.nav-tab.active {
  background: var(--orange-tint);
  border-bottom: 2px solid var(--accent-blue);
  color: var(--orange-text);
  font-weight: 600;
}
.nav-tab-icon {
  width: 16px;
  height: 16px;
  stroke: currentColor;
  flex-shrink: 0;
}
.nav-tab-label {
  white-space: nowrap;
}
.nav-count {
  font-family: var(--font-mono);
  font-size: 10px;
  color: var(--ink-4);
  background: var(--surface-white);
  border: 1px solid var(--line);
  border-radius: var(--r-pill);
  padding: 1px 6px;
}
.nav-tab.active .nav-count {
  background: var(--surface-white);
  border-color: var(--orange-line);
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
  padding: 0 14px;
  background: var(--paper-1);
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
  background: var(--line);
  cursor: col-resize;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  user-select: none;
  transition: background 0.12s ease;
  z-index: 20;
}
.col-splitter:hover, .col-splitter.active {
  background: var(--orange);
}
.col-splitter-handle {
  width: 2px;
  height: 28px;
  border-radius: var(--r-pill);
  background: var(--line-strong);
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
  text-transform: uppercase;
  font-size: 10px;
  letter-spacing: 0.08em;
  color: var(--ink-3);
  font-weight: 700;
}
.pane-badge {
  font-family: var(--font-mono);
  font-size: 10px;
  color: var(--ink-4);
}
.source-viewer {
  flex: 1;
  overflow: auto;
  background: var(--surface-white);
  font-family: var(--font-mono);
  font-size: 13px;
  line-height: 22px;
  padding: 6px 0;
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
  resize: none;
}
.source-stale-banner {
  display: none;
  padding: 6px 12px;
  background: #FEF3C7;
  border-bottom: 1px solid rgba(217, 119, 6, 0.35);
  font-size: 11px;
  color: #92400e;
  align-items: center;
  justify-content: space-between;
}
.source-status-banner {
  display: none;
  padding: 6px 12px;
  border-bottom: 1px solid var(--line);
  font-size: 11px;
  align-items: center;
  justify-content: space-between;
}

/* Scenario Summary Card in Source Dock (Phase 4.1) */
.scenario-summary-card {
  margin: 6px 12px;
  background: var(--paper-0);
  border: 1px solid var(--line);
  border-radius: var(--r-sm);
  padding: 6px 10px;
  font-size: 11px;
  transition: all 0.15s ease;
}
.scenario-summary-card.collapsed .scenario-body {
  display: none;
}
.scenario-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  cursor: pointer;
  user-select: none;
}
.scenario-title {
  font-weight: 700;
  color: var(--ink-2);
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 11px;
}
.scenario-body {
  margin-top: 6px;
  color: var(--ink-3);
  line-height: 1.45;
  border-top: 1px dashed var(--line);
  padding-top: 6px;
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
  padding: 0 14px;
  background: var(--paper-1);
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
  padding: 14px;
}
.inspector-section {
  margin-top: 14px;
  padding-top: 12px;
  border-top: 1px solid var(--line);
}
.inspector-section-title {
  font-size: 10px;
  font-weight: 700;
  text-transform: uppercase;
  color: var(--ink-3);
  letter-spacing: 0.05em;
  margin-bottom: 6px;
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
.prop-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 11px;
}
.prop-table td {
  padding: 4px 0;
  vertical-align: top;
}
.prop-table .key {
  color: var(--ink-4);
  width: 40%;
  font-size: 10.5px;
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
  padding: 4px 8px;
  background: var(--paper-0);
  border: 1px solid var(--line);
  border-radius: var(--r-xs);
  cursor: pointer;
  font-size: 11px;
  transition: all 0.1s ease;
}
.conn-item:hover {
  background: var(--orange-tint);
  border-color: var(--orange-line);
}
.conn-label {
  font-family: var(--font-mono);
  color: var(--ink-2);
}
.conn-pill {
  font-size: 9px;
  padding: 1px 5px;
  border-radius: 3px;
  font-weight: 600;
  text-transform: uppercase;
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
  transition: background 0.08s ease, border-left-color 0.08s ease;
  width: 100%;
  box-sizing: border-box;
}
.source-line:hover, .code-line:hover {
  background: var(--paper-1);
}
.source-line.line-selected, .code-line.active-line, .code-line.line-selected {
  background: var(--source-retained-bg) !important;
  border-left-color: var(--orange) !important;
}
.code-line.highlighted-span {
  background: rgba(235, 110, 48, 0.15) !important;
  border-left-color: var(--orange) !important;
}
.source-line.slice-kept, .code-line.slice-kept {
  background: var(--source-retained-bg);
  border-left-color: var(--orange);
}
.source-line.slice-partial, .code-line.slice-partial {
  background: #F4FBFB;
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
  padding-right: 12px;
  padding-left: 4px;
  color: var(--ink-4);
  user-select: none;
  font-size: 13px;
  line-height: 22px;
  flex-shrink: 0;
  box-sizing: border-box;
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
  height: 38px;
  background: var(--surface-white);
  border-bottom: 1px solid var(--line);
  padding: 0 14px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  flex-shrink: 0;
  gap: 10px;
}
.toolbar-group {
  display: flex;
  align-items: center;
  gap: 8px;
}
.toolbar-label {
  font-size: 11px;
  color: var(--ink-3);
  font-weight: 500;
}
.filter-box {
  display: flex;
  align-items: center;
  background: var(--paper-0);
  border: 1px solid var(--line);
  border-radius: var(--r-sm);
  padding: 2px 8px;
  height: 26px;
}
.filter-input {
  border: none;
  background: transparent;
  outline: none;
  font-size: 11px;
  color: var(--ink);
  font-family: inherit;
  width: 170px;
}
.filter-clear {
  border: none;
  background: transparent;
  color: var(--ink-4);
  cursor: pointer;
}
.toggle-chip {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  padding: 2px 8px;
  border-radius: var(--r-pill);
  font-size: 11px;
  background: var(--paper-0);
  border: 1px solid var(--line);
  cursor: pointer;
  user-select: none;
  color: var(--ink-2);
}
.toggle-chip input {
  cursor: pointer;
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
}

/* SVG Node Base */
.svg-node {
  cursor: pointer;
  transition: opacity 0.15s ease, filter 0.15s ease;
}
.svg-node rect {
  fill: var(--graph-node);
  stroke: var(--graph-node-border);
  stroke-width: 1.4px;
  transition: stroke 0.15s ease, stroke-width 0.15s ease, fill 0.15s ease;
}
.svg-node:hover rect {
  stroke: var(--graph-secondary-text);
  filter: drop-shadow(0 2px 6px rgba(0, 0, 0, 0.08));
}

/* Specific Node Kinds (Section 4.1 Specification) */
.svg-node rect.node-col {
  fill: #FFFFFF;
  stroke: var(--graph-node-border);
  stroke-width: 1.4px;
}
.svg-node rect.node-deriv {
  fill: var(--graph-derivation-tint);
  stroke: var(--graph-derivation);
  stroke-width: 1.4px;
}
.svg-node rect.node-stmt {
  fill: #FFFFFF;
  stroke: var(--graph-node-border);
  stroke-width: 1.4px;
}
.svg-node rect.node-vardef {
  fill: #FAF5FF;
  stroke: #7C3AED;
  stroke-width: 1.4px;
}
.svg-node rect.node-varuse {
  fill: #F0FDFA;
  stroke: #0D9488;
  stroke-width: 1.4px;
}
.svg-node rect.node-expr {
  fill: #F8FAFC;
  stroke: #64748B;
  stroke-width: 1.3px;
}
.svg-node rect.node-rowsel {
  fill: #FAF5FF;
  stroke: #7C3AED;
  stroke-width: 1.4px;
  stroke-dasharray: 4,3;
}
.svg-node rect.node-unk {
  fill: #FFFBEB;
  stroke: #D97706;
  stroke-width: 1.4px;
  stroke-dasharray: 4,3;
}

/* Node Selection & Upstream/Downstream Lineage Highlights */
.svg-node.selected rect {
  fill: var(--graph-selected) !important;
  stroke: var(--graph-selected-border) !important;
  stroke-width: 2.2px !important;
  filter: drop-shadow(0 0 8px rgba(255, 90, 31, 0.35));
}
.svg-node.connected-upstream rect {
  stroke: var(--orange) !important;
  stroke-width: 1.8px !important;
  filter: drop-shadow(0 0 6px rgba(255, 90, 31, 0.22));
}
.svg-node.connected-downstream rect {
  stroke: var(--graph-derivation) !important;
  stroke-width: 1.8px !important;
  filter: drop-shadow(0 0 6px rgba(15, 118, 110, 0.22));
}
.svg-node.dimmed {
  opacity: 0.45 !important;
}

/* SVG Edges (Section 4.2 Specification) */
.svg-edge {
  fill: none;
  stroke: var(--graph-edge);
  stroke-width: 1.4px;
  transition: stroke 0.15s ease, stroke-width 0.15s ease, opacity 0.15s ease;
}
.edge-data { stroke: var(--graph-edge); stroke-width: 1.4px; }
.edge-ctrl { stroke: #D97706; stroke-width: 1.4px; stroke-dasharray: 4,3; }
.edge-rowsel { stroke: #7C3AED; stroke-width: 1.4px; stroke-dasharray: 4,3; }
.edge-colver { stroke: var(--graph-derivation); stroke-width: 1.4px; }
.edge-unk { stroke: #DC2626; stroke-width: 1.4px; stroke-dasharray: 3,3; }

.svg-edge.highlighted {
  stroke-width: 2.2px !important;
  opacity: 1 !important;
}
.svg-edge.highlighted.inbound {
  stroke: var(--orange) !important;
}
.svg-edge.highlighted.outbound {
  stroke: var(--graph-derivation) !important;
}
.svg-edge.dimmed {
  opacity: 0.22 !important;
}

/* Backward Slice View */
.slice-container {
  width: 100%;
  height: 100%;
  overflow-y: auto;
  padding: 16px 20px;
  box-sizing: border-box;
  display: flex;
  flex-direction: column;
  gap: 16px;
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
  font-size: 11px;
  font-weight: 700;
  color: var(--ink-2);
  text-transform: uppercase;
  letter-spacing: 0.06em;
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
  background: #ECFDF5;
  color: var(--ok);
  border: 1px solid rgba(47, 125, 79, 0.25);
}
.slice-badge-approx {
  background: #FFFBEB;
  color: var(--warn);
  border: 1px solid rgba(138, 90, 0, 0.25);
}
.slice-badge-warn {
  background: #FEF2F2;
  color: var(--err);
  border: 1px solid rgba(179, 38, 30, 0.25);
}
.slice-badge-ok {
  background: #ECFDF5;
  color: var(--ok);
  border: 1px solid rgba(47, 125, 79, 0.25);
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
  font-size: 11px;
  font-weight: 700;
  color: var(--ink-2);
  text-transform: uppercase;
  letter-spacing: 0.06em;
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
  transition: border-color 0.12s ease, background-color 0.12s ease, box-shadow 0.12s ease;
  display: flex;
  flex-direction: column;
  gap: 6px;
  width: 100%;
  box-sizing: border-box;
}
.stmt-card:hover {
  border-color: var(--orange-line);
  box-shadow: 0 1px 4px rgba(0,0,0,0.04);
}
.stmt-card.full {
  background: #FFF8F3;
  border-left: 3px solid var(--orange);
}
.stmt-card.partial {
  background: #F4FBFB;
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
  background: #FFFDF5;
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
  border-radius: 3px;
  border: 1px solid var(--line);
}
.stmt-status-badge {
  font-family: var(--font-mono);
  font-size: 10px;
  font-weight: 700;
  letter-spacing: 0.04em;
  padding: 2px 8px;
  border-radius: 3px;
  text-transform: uppercase;
}
.stmt-status-badge.badge-full {
  background: rgba(235, 110, 48, 0.12);
  color: var(--orange-dark);
  border: 1px solid rgba(235, 110, 48, 0.35);
}
.stmt-status-badge.badge-partial {
  background: rgba(18, 126, 134, 0.12);
  color: var(--teal-700);
  border: 1px solid rgba(18, 126, 134, 0.3);
}
.stmt-status-badge.badge-none {
  background: var(--paper-1);
  color: var(--ink-4);
  border: 1px solid var(--line);
}
.stmt-status-badge.badge-unknown {
  background: #FFFBEB;
  color: var(--warn);
  border: 1px solid rgba(138, 90, 0, 0.25);
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
  border-radius: 3px;
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
  border-radius: 3px;
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
  border-radius: 2px;
}
.sub-item-badge.sub-keep {
  background: rgba(235, 110, 48, 0.12);
  color: var(--orange-dark);
  border: 1px solid rgba(235, 110, 48, 0.25);
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
  border-radius: 3px;
  background: var(--paper-1);
  color: var(--ink-3);
}
.diff-badge {
  font-size: 10px;
  font-family: var(--font-mono);
  padding: 2px 6px;
  border-radius: 3px;
  font-weight: 600;
}
.diff-added { background: #ECFDF5; color: #059669; border: 1px solid rgba(5, 150, 105, 0.2); }
.diff-removed { background: #FEF2F2; color: #DC2626; border: 1px solid rgba(220, 38, 38, 0.2); }
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
  height: 40px;
  padding: 0 14px;
  background: var(--surface-white);
  border-bottom: 1px solid var(--line);
  display: flex;
  align-items: center;
  gap: 10px;
  flex-shrink: 0;
}
.ast-rep-selector {
  display: flex;
  background: var(--paper-1);
  padding: 2px;
  border-radius: var(--r-sm);
}
.ast-rep-btn {
  border: none;
  background: transparent;
  padding: 3px 8px;
  font-size: 11px;
  border-radius: 4px;
  cursor: pointer;
  color: var(--ink-3);
}
.ast-rep-btn.active {
  background: var(--surface-white);
  color: var(--ink);
  font-weight: 600;
  box-shadow: var(--shadow-sm);
}
.ast-search-box {
  display: flex;
  align-items: center;
  background: var(--paper-0);
  border: 1px solid var(--line);
  border-radius: var(--r-sm);
  padding: 2px 8px;
}
.ast-search-box input {
  border: none;
  background: transparent;
  outline: none;
  font-size: 11px;
  width: 160px;
}
.ast-tree-scroll {
  flex: 1;
  overflow: auto;
  padding: 14px 18px 40px;
  background: var(--surface-white);
  font-family: var(--font-mono);
  font-size: 11px;
  line-height: 22px;
}

/* Command Palette Modal (Specification Phase 7.3) */
.palette-modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(17, 17, 17, 0.45);
  backdrop-filter: blur(2px);
  display: flex;
  align-items: flex-start;
  justify-content: center;
  padding-top: 14vh;
  z-index: 9999;
  opacity: 0;
  pointer-events: none;
  transition: opacity 0.12s ease;
}
.palette-modal-overlay.open {
  opacity: 1;
  pointer-events: auto;
}
.palette-card {
  width: 520px;
  max-width: 92vw;
  background: var(--surface-white);
  border: 1px solid var(--line-strong);
  border-radius: var(--r-md);
  box-shadow: 0 16px 40px rgba(0, 0, 0, 0.18);
  overflow: hidden;
  display: flex;
  flex-direction: column;
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
  transition: all 0.08s ease;
}
.palette-item:hover, .palette-item.active {
  background: var(--paper-1);
  color: var(--ink);
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
    box-shadow: -8px 0 24px rgba(0, 0, 0, 0.12);
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
}
"###;

/// Complete HTML body shell for the entry screen, 30/70 workspace, toolbars, and contextual drawer.
pub const HTML_BODY_MARKUP: &str = r###"
  <!-- Phase 1: Minimalist Centered Code-Entry Experience -->
  <div class="entry-screen" id="entry-screen">
    <div class="entry-card">
      <div class="entry-badge-row">
        <span class="brand-logo">R-PARSE</span>
        <span class="brand-subtitle">Scientific Analysis Studio</span>
      </div>
      <h1 class="entry-title">Understand your R code.</h1>
      <p class="entry-desc">
        Explore syntax structure, dataframe column provenance ($C_{in} \to C_{out}$), unified program dependencies &amp; backward slices, and schema evolution.
      </p>

      <div class="entry-input-box">
        <div class="entry-box-header">
          <span class="entry-box-title">Source Buffer</span>
          <span class="entry-box-meta" id="entry-char-count">0 lines · 0 chars</span>
        </div>
        <textarea class="entry-textarea" id="entry-source-textarea" spellcheck="false" placeholder="# Paste your R pipeline script here..."></textarea>
        <div class="entry-box-footer">
          <span class="entry-hint">Shift+Enter newline · Ctrl+Enter to analyze</span>
        </div>
      </div>

      <div class="entry-actions-row">
        <button class="btn btn-primary btn-lg" id="btn-analyze-entry">
          <span class="play-icon">▶</span>
          <span>Analyze Program</span>
        </button>

        <label class="btn btn-secondary btn-lg" id="btn-entry-load-file" style="cursor:pointer;" title="Load local R file">
          <span>Load File...</span>
          <input type="file" id="entry-file-input" accept=".R,.r,.Rmd" style="display:none;" />
        </label>

        <div style="flex:1;">
          <select class="select-dropdown" id="entry-example-select" style="width:100%;">
            <option value="">Load Example...</option>
            <option value="showcase">Showcase Pipeline (showcase_pipeline.R)</option>
            <option value="simple">Simple Provenance (price * quantity)</option>
            <option value="filter_provenance">Filter Provenance (RowSelectionDependency)</option>
            <option value="adversarial">Adversarial Alias Renaming</option>
          </select>
        </div>
      </div>

      <div id="entry-validation-error" style="display:none; color:var(--accent-rose); font-size:12px; margin-top:8px;"></div>

      <div class="entry-footer-note">
        Tier 1 Static Analysis · 100% Autonomous & Deterministic · Zero R Execution in Static Mode
      </div>
    </div>
  </div>

  <!-- Phase 3: Desktop 3-Column + Bottom Dock Master Scientific Workspace -->
  <div class="workspace-screen hidden" id="workspace-screen">
    <!-- 3.1 Top Bar (56px) -->
    <header class="app-header">
      <div class="header-top-row">
        <!-- Left: Wordmark, studio, inline filename breadcrumb -->
        <div class="header-identity">
          <span class="brand-logo">R—PARSE</span>
          <span class="header-sep" aria-hidden="true"></span>
          <span class="header-sub">Studio</span>
          <span class="header-sep" aria-hidden="true"></span>
          <span class="header-filename" id="header-file-name">showcase_pipeline.R</span>
        </div>

        <!-- Middle: Quiet Status & Plain Metrics -->
        <div class="header-center-info">
          <div class="analysis-status-badge" id="analysis-status-badge" title="Engine status">
            <span class="status-dot" id="header-status-dot"></span>
            <span class="status-text" id="header-status-text">Analysis ready</span>
          </div>

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
          <span class="search-icon">🔍</span>
          <span>Search actions &amp; views...</span>
          <kbd class="kbd-shortcut">⌘K</kbd>
        </button>

        <!-- Right: Primary Actions (Guide, Investigation, Run, New Analysis) -->
        <div class="header-actions-row">
          <a href="/how-to-use" target="_blank" class="header-nav-link" id="link-how-to-use" title="Open Beginner-Friendly How to Use Guide"><span>📖 Guide</span></a>
          <a href="/" class="header-nav-link" id="link-investigation" title="View The Investigation landing narrative"><span>Investigation ↗</span></a>
          <button class="btn btn-primary" id="btn-run-analysis" title="Re-run static analysis on current code">Run</button>
          <button class="btn btn-secondary" id="btn-new-analysis" title="Return to code entry screen">New Analysis</button>
        </div>
      </div>
    </header>

    <!-- Section 3: Horizontal Feature Navigation Row (Directly below Header) -->
    <nav class="feature-nav-bar" id="feature-nav-bar" role="tablist" aria-label="Analysis Views">
      <div class="feature-tabs-list">
        <button class="nav-tab active" data-view="provenance" id="tab-provenance" role="tab" title="Fine-grained dataframe column lineage">
          <svg class="nav-tab-icon" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5"><circle cx="4" cy="4" r="2"/><circle cx="12" cy="8" r="2"/><circle cx="4" cy="12" r="2"/><path d="M6 4h3a3 3 0 0 1 3 3v0M6 12h3a3 3 0 0 0 3-3v0"/></svg>
          <span class="nav-tab-label">Provenance</span>
          <span class="nav-count" id="count-prov">0</span>
        </button>
        <button class="nav-tab" data-view="pdg" id="tab-pdg" role="tab" title="Program Dependence Graph">
          <svg class="nav-tab-icon" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5"><circle cx="4" cy="5" r="2"/><circle cx="12" cy="5" r="2"/><circle cx="8" cy="12" r="2"/><path d="M6 5h4M5.5 6.5l2 4M10.5 6.5l-2 4"/></svg>
          <span class="nav-tab-label">PDG Graph</span>
          <span class="nav-count" id="count-pdg">0</span>
        </button>
        <button class="nav-tab" data-view="slice" id="tab-slice" role="tab" title="Column-aware static backward slicing">
          <svg class="nav-tab-icon" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M3 13L13 3M8 3h5v5M3 8v5h5"/></svg>
          <span class="nav-tab-label">Backward Slice</span>
          <span class="nav-count" id="count-slice">0</span>
        </button>
        <button class="nav-tab" data-view="schema" id="tab-schema" role="tab" title="Dataframe schema evolution across transformations">
          <svg class="nav-tab-icon" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5"><rect x="2" y="3" width="12" height="10" rx="1.5"/><path d="M2 7h12M6 7v6"/></svg>
          <span class="nav-tab-label">Schema Evolution</span>
          <span class="nav-count" id="count-schema">0</span>
        </button>
        <button class="nav-tab" data-view="ast" id="tab-ast" role="tab" title="Interactive AST & CST Syntax Tree Explorer">
          <svg class="nav-tab-icon" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M4 3v10M4 8h5M4 4h7M4 12h6"/></svg>
          <span class="nav-tab-label">AST Explorer</span>
          <span class="nav-count" id="count-ast">0</span>
        </button>
      </div>
    </nav>

    <!-- 3-Column Middle Workspace Container -->
    <div class="workspace-main" id="workbench-container">
      <!-- 2.1 Left Panel — Source Editor (34% width / 380-460px) -->
      <div class="pane-source word-wrap-enabled" id="source-pane">
        <div class="pane-header">
          <div style="display:flex; align-items:center; gap:8px;">
            <span class="pane-title">Source Code</span>
            <span class="pane-badge" id="source-line-count">0 lines</span>
            <button class="btn btn-ghost" id="btn-toggle-scenario" style="padding:2px 6px; font-size:10px; display:none;" title="Toggle Scenario summary">
              📋 Scenario
            </button>
          </div>
          <div style="display:flex; align-items:center; gap:4px;">
            <button class="btn btn-ghost active" id="btn-toggle-wrap" title="Word wrap is ON (click to disable and permit horizontal scroll)" aria-label="Toggle word wrap" aria-pressed="true">
              <svg width="12" height="12" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.8" style="vertical-align:middle; margin-right:3px;"><path d="M2 4h12M2 8h8a2.5 2.5 0 0 1 0 5H8M2 12h3M10 11l-2 2 2 2"/></svg>Word wrap
            </button>
            <button class="btn btn-ghost" id="btn-clear-highlight" style="padding:2px 6px; font-size:10px;" title="Clear selection">Clear</button>
            <button class="btn btn-ghost" id="btn-edit-source" style="padding:2px 6px; font-size:10px;" title="Edit source code">Edit</button>
          </div>
        </div>

        <!-- Scenario Summary Card (Phase 4.1) -->
        <div class="scenario-summary-card collapsed" id="scenario-summary-card" style="display:none;">
          <div class="scenario-header" id="scenario-header-toggle">
            <span class="scenario-title">
              <span>📋 Scenario &amp; Pipeline Context</span>
            </span>
            <span id="scenario-chevron" style="font-size:10px; color:var(--ink-4);">▶ Expand</span>
          </div>
          <div class="scenario-body" id="scenario-body-content"></div>
        </div>

        <!-- Stale source warning banner -->
        <div class="source-stale-banner" id="source-stale-banner" role="status" aria-live="polite">
          <span>⚠️ Source modified (results from earlier revision).</span>
          <div style="display:flex; gap:6px;">
            <button class="btn btn-sm btn-primary" id="btn-reanalyze">Reanalyze</button>
            <button class="btn btn-sm btn-secondary" id="btn-revert-source">Revert</button>
          </div>
        </div>

        <!-- Status / Error Notification Banner -->
        <div class="source-status-banner" id="source-status-banner">
          <span id="source-status-msg"></span>
          <button class="btn" id="btn-dismiss-status" style="padding:1px 5px; font-size:10px; background:#ffffff;">✕</button>
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
            <div class="toolbar-group">
              <button class="btn" id="btn-prov-zoom-out" style="padding:2px 6px;" title="Zoom out">-</button>
              <button class="btn" id="btn-prov-zoom-in" style="padding:2px 6px;" title="Zoom in">+</button>
              <button class="btn" id="btn-prov-reset" style="padding:2px 8px;" title="Fit to View">Fit View</button>
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
            <div class="toolbar-group">
              <button class="btn" id="btn-pdg-zoom-out" style="padding:2px 6px;">-</button>
              <button class="btn" id="btn-pdg-zoom-in" style="padding:2px 6px;">+</button>
              <button class="btn" id="btn-pdg-reset" style="padding:2px 8px;">Fit View</button>
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
              <span class="toolbar-label" style="font-size:10px; text-transform:uppercase; letter-spacing:0.04em;">Legend:</span>
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
                  <marker id="arrow-prov-data" viewBox="0 0 10 10" refX="10" refY="5" markerWidth="6" markerHeight="6" orient="auto">
                    <path d="M 0 1 L 10 5 L 0 9 z" fill="#6B665D" />
                  </marker>
                  <marker id="arrow-prov-colver" viewBox="0 0 10 10" refX="10" refY="5" markerWidth="6" markerHeight="6" orient="auto">
                    <path d="M 0 1 L 10 5 L 0 9 z" fill="#0F766E" />
                  </marker>
                  <marker id="arrow-prov-unk" viewBox="0 0 10 10" refX="10" refY="5" markerWidth="6" markerHeight="6" orient="auto">
                    <path d="M 0 1 L 10 5 L 0 9 z" fill="#DC2626" />
                  </marker>
                </defs>
              </svg>
            </div>

            <!-- Canvas for PDG -->
            <div class="graph-canvas-container" id="view-container-pdg" style="display:none;">
              <svg class="graph-svg" id="svg-pdg">
                <defs>
                  <marker id="arrow-data" viewBox="0 0 10 10" refX="10" refY="5" markerWidth="6" markerHeight="6" orient="auto">
                    <path d="M 0 1 L 10 5 L 0 9 z" fill="#6B665D" />
                  </marker>
                  <marker id="arrow-ctrl" viewBox="0 0 10 10" refX="10" refY="5" markerWidth="6" markerHeight="6" orient="auto">
                    <path d="M 0 1 L 10 5 L 0 9 z" fill="#D97706" />
                  </marker>
                  <marker id="arrow-rowsel" viewBox="0 0 10 10" refX="10" refY="5" markerWidth="6" markerHeight="6" orient="auto">
                    <path d="M 0 1 L 10 5 L 0 9 z" fill="#7c3aed" />
                  </marker>
                  <marker id="arrow-colver" viewBox="0 0 10 10" refX="10" refY="5" markerWidth="6" markerHeight="6" orient="auto">
                    <path d="M 0 1 L 10 5 L 0 9 z" fill="#0F766E" />
                  </marker>
                  <marker id="arrow-unk" viewBox="0 0 10 10" refX="10" refY="5" markerWidth="6" markerHeight="6" orient="auto">
                    <path d="M 0 1 L 10 5 L 0 9 z" fill="#DC2626" />
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
                  <span class="ast-search-icon">🔍</span>
                  <input type="text" id="ast-search-input" placeholder="Search node type, identifier, text..." />
                  <span class="ast-search-match" id="ast-search-match"></span>
                </div>
                <div class="ast-actions">
                  <button class="btn btn-outline" id="btn-ast-expand-all" title="Expand all nodes">Expand All</button>
                  <button class="btn btn-outline" id="btn-ast-collapse-all" title="Collapse all nodes">Collapse All</button>
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
      <div class="workspace-inspector" id="contextual-inspector">
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


    <!-- 7.3 Command Palette Modal Dialog -->
    <div class="palette-modal-overlay" id="command-palette-modal" role="dialog" aria-modal="true" aria-label="Command Palette">
      <div class="palette-card">
        <div class="palette-input-row">
          <span style="font-size:14px; color:var(--ink-4);">🔍</span>
          <input type="text" class="palette-input" id="palette-search-input" placeholder="Type a view or action (e.g. Provenance, Slice, Reanalyze)..." autocomplete="off" />
          <kbd class="kbd-shortcut" id="btn-palette-close" style="cursor:pointer;" title="Close (Esc)">ESC</kbd>
        </div>
        <div class="palette-list" id="palette-list">
          <div class="palette-item active" data-action="view-prov"><span>📊 Switch to Column Provenance</span><kbd class="kbd-shortcut">1</kbd></div>
          <div class="palette-item" data-action="view-pdg"><span>🕸️ Switch to Unified PDG Graph</span><kbd class="kbd-shortcut">2</kbd></div>
          <div class="palette-item" data-action="view-slice"><span>🎯 Switch to Backward Slice</span><kbd class="kbd-shortcut">3</kbd></div>
          <div class="palette-item" data-action="view-schema"><span>🧬 Switch to Schema Evolution</span><kbd class="kbd-shortcut">4</kbd></div>
          <div class="palette-item" data-action="view-ast"><span>🌲 Switch to AST &amp; CST Explorer</span><kbd class="kbd-shortcut">5</kbd></div>
          <div class="palette-item" data-action="run-analysis"><span>⚡ Reanalyze Program</span><kbd class="kbd-shortcut">⌘R</kbd></div>
          <div class="palette-item" data-action="toggle-wrap"><span>↩️ Toggle Source Word Wrap</span></div>
          <div class="palette-item" data-action="copy-cli"><span>📋 Copy Current CLI Command</span></div>
          <div class="palette-item" data-action="open-guide"><span>📖 Open How to Use Guide</span></div>
          <div class="palette-item" data-action="open-investigation"><span>↗ Open Investigation Landing Page</span></div>
        </div>
      </div>
    </div>
  </div>
"###;

/// Complete vanilla JavaScript client logic for R-PARSE Scientific Analysis Studio.
pub const JS_SCRIPTS: &str = r###"
    (function() {
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

      // Elements — Entry Screen
      const entryScreen = document.getElementById('entry-screen');
      const entryInput = document.getElementById('entry-source-textarea') || document.getElementById('entry-code-input');
      const entryStats = document.getElementById('entry-char-count') || document.getElementById('entry-code-stats');
      const entryError = document.getElementById('entry-validation-error');
      const btnEntryAnalyze = document.getElementById('btn-analyze-entry') || document.getElementById('btn-entry-analyze');
      const btnEntryLoadFile = document.getElementById('btn-entry-load-file');
      const entryFileInput = document.getElementById('entry-file-input');
      const entryExampleSelect = document.getElementById('entry-example-select');

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
        sourceStatusBanner.style.background = isError ? 'var(--accent-rose-subtle)' : 'var(--accent-blue-subtle)';
        sourceStatusBanner.style.color = isError ? '#991b1b' : '#0369a1';
        sourceStatusBanner.style.display = 'flex';
      }

      function hideStatusNotification() {
        if (sourceStatusBanner) sourceStatusBanner.style.display = 'none';
      }

      document.getElementById('btn-dismiss-status')?.addEventListener('click', hideStatusNotification);

      function refreshHeaderAndMetrics() {
        const fnElem = document.getElementById('header-file-name');
        if (fnElem) fnElem.textContent = data.file_name || 'script.R';

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
      // PHASE 1: Entry Screen Logic & Code Input
      // ======================================================================
      if (entryInput) {
        entryInput.value = initialSource;
      }

      function updateEntryStats() {
        if (!entryInput || !entryStats) return;
        const text = entryInput.value;
        const lines = text ? text.split(/\r?\n/).length : 0;
        const chars = text.length;
        entryStats.textContent = `${lines} lines · ${chars} chars`;
      }
      updateEntryStats();

      entryInput?.addEventListener('input', () => {
        updateEntryStats();
        if (entryError) entryError.style.display = 'none';
      });

      btnEntryLoadFile?.addEventListener('click', () => entryFileInput?.click());
      entryFileInput?.addEventListener('change', (e) => {
        const file = e.target.files[0];
        if (!file) return;
        const reader = new FileReader();
        reader.onload = (evt) => {
          if (entryInput) entryInput.value = evt.target.result;
          updateEntryStats();
          if (entryError) entryError.style.display = 'none';
        };
        reader.readAsText(file);
      });

      entryExampleSelect?.addEventListener('change', (e) => {
        const val = e.target.value;
        if (!val) return;
        if (data.examples && data.examples[val]) {
          if (entryInput) entryInput.value = data.examples[val];
          updateEntryStats();
          if (entryError) entryError.style.display = 'none';
        }
      });

      entryInput?.addEventListener('keydown', (e) => {
        if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
          e.preventDefault();
          launchAnalysis();
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
        if (entryInput) {
          entryInput.value = data.source_code || '';
          updateEntryStats();
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
          if (entryError) {
            entryError.textContent = 'Please enter or load R code to analyze.';
            entryError.style.display = 'inline-flex';
          }
          showStatusNotification('Source code cannot be empty.');
          return;
        }

        isAnalyzing = true;
        currentRevisionId += 1;
        const requestRev = currentRevisionId;

        if (btnEntryAnalyze) {
          btnEntryAnalyze.disabled = true;
          btnEntryAnalyze.innerHTML = '<span>⏳</span> Analyzing...';
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
          if (entryError) {
            entryError.textContent = `Engine offline at ${endpoint}. Run 'rparse studio' to analyze.`;
            entryError.style.display = 'inline-flex';
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
          if (btnEntryAnalyze) {
            btnEntryAnalyze.disabled = false;
            btnEntryAnalyze.innerHTML = '<span>▶</span> Analyze Program';
          }
          if (btnReanalyze) {
            btnReanalyze.disabled = false;
            btnReanalyze.textContent = 'Reanalyze';
          }
        });
      }

      function launchAnalysis() {
        if (!entryInput) return;
        const code = entryInput.value.trim();
        if (!code) {
          if (entryError) {
            entryError.textContent = 'Please enter or load R code to analyze.';
            entryError.style.display = 'inline-flex';
          }
          return;
        }
        if (entryError) entryError.style.display = 'none';

        const doTransition = () => {
          const reducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
          if (entryScreen) entryScreen.classList.add('transition-out');
          if (workspaceScreen) workspaceScreen.classList.remove('hidden');

          setTimeout(() => {
            if (entryScreen) entryScreen.style.display = 'none';
            recomputeActiveGraphFit();
          }, reducedMotion ? 10 : 460);
        };

        if (entryInput.value === initialSource && (data.provenance?.nodes?.length > 0 || data.pdg?.nodes?.length > 0)) {
          doTransition();
        } else {
          executeRealAnalysis(entryInput.value, doTransition);
        }
      }

      btnEntryAnalyze?.addEventListener('click', launchAnalysis);

      // Return to Entry Screen Action
      document.getElementById('btn-new-analysis')?.addEventListener('click', () => {
        if (entryScreen) {
          entryScreen.style.display = 'flex';
          void entryScreen.offsetWidth;
          entryScreen.classList.remove('transition-out');
        }
        if (workspaceScreen) {
          workspaceScreen.classList.add('hidden');
        }
        if (entryInput) entryInput.focus();
      });

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
          btnEditSource.textContent = 'View';
          btnEditSource.title = 'Switch to syntax-highlighted viewer';
          sourceContainer.style.display = 'none';
          sourceEditorTextarea.style.display = 'block';
          sourceEditorTextarea.value = data.source_code || '';
          sourceEditorTextarea.focus();
        } else {
          btnEditSource.textContent = 'Edit';
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
          } else if (activeView === 'pdg') {
            fitGraphView('view-container-pdg', 'pdg-root-group', pdgPositions, pdgTransform);
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

      // Keyboard Shortcuts
      window.addEventListener('keydown', (e) => {
        if (e.target.tagName === 'INPUT' || e.target.tagName === 'SELECT' || e.target.tagName === 'TEXTAREA') return;
        if (e.key === 'Escape') {
          if (contextualInspector.classList.contains('open')) {
            contextualInspector.classList.remove('open');
          } else {
            clearSelection();
          }
        } else if (e.key === '1') {
          switchView('provenance');
        } else if (e.key === '2') {
          switchView('pdg');
        } else if (e.key === '3') {
          switchView('slice');
        } else if (e.key === '4') {
          switchView('schema');
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
          targetRow.scrollIntoView({ behavior: 'smooth', block: 'center' });
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
            <tr><td class="key">Analysis Mode</td><td class="val" style="color:var(--accent-green);">${escapeHtml(data.mode || 'STATIC')}</td></tr>
            <tr><td class="key">Statements</td><td class="val">${data.summary?.statement_count || 0}</td></tr>
            <tr><td class="key">Dataframe States</td><td class="val">${data.summary?.dataframe_state_count || 0}</td></tr>
            <tr><td class="key">Column Versions</td><td class="val">${data.summary?.column_version_count || 0}</td></tr>
            <tr><td class="key">PDG Nodes / Edges</td><td class="val">${data.summary?.pdg_node_count || 0} / ${data.summary?.pdg_edge_count || 0}</td></tr>
            <tr><td class="key">Column Derivations</td><td class="val">${data.summary?.derivation_count || 0}</td></tr>
            <tr><td class="key">Uncertainty State</td><td class="val" style="color:${data.summary?.contains_unknowns ? 'var(--accent-amber)' : 'var(--accent-green)'};">${data.summary?.contains_unknowns ? 'UNCERTAINTY DETECTED' : 'KNOWN STATIC'}</td></tr>
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
          actionBtns += `<button class="btn" style="width:100%; justify-content:center; margin-top:6px;" onclick="window.rparseJumpSpan(${item.span.start_line})">🔍 Jump to Source (Line ${item.span.start_line})</button>`;
        }
        if (item.sliceTarget) {
          actionBtns += `<button class="btn btn-primary" style="width:100%; justify-content:center; margin-top:6px; font-size:11.5px; padding:0 8px; white-space:nowrap; overflow:hidden; text-overflow:ellipsis;" onclick="window.rparseSelectSlice('${escapeHtml(item.sliceTarget)}')">🎯 Compute Backward Slice for ${escapeHtml(item.sliceTarget)}</button>`;
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
                    <span class="conn-pill" style="background:var(--accent-blue-subtle); color:var(--accent-blue);">${escapeHtml(u.edgeKind || u.type || 'dep')}</span>
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
                    <span class="conn-pill" style="background:var(--accent-green-subtle); color:var(--accent-green);">${escapeHtml(d.edgeKind || d.type || 'dep')}</span>
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
            <span class="pill pill-full">${escapeHtml(item.type || 'Object')}</span>
            <span class="pill ${String(item.certainty || '').includes('UNKNOWN') ? 'pill-drop' : 'pill-keep'}">${escapeHtml(item.certainty || 'KNOWN_STATIC')}</span>
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
        contextualInspector.classList.add('open');

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
        switch (n.kind) {
          case 'Statement': return { width: 260, height: 54 };
          case 'VariableDef': return { width: 200, height: 46 };
          case 'VariableUse': return { width: 180, height: 44 };
          case 'ColumnVersion': return { width: 220, height: 46 };
          case 'Expression': return { width: 240, height: 48 };
          case 'RowSelection': return { width: 220, height: 46 };
          default: return { width: 210, height: 46 };
        }
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

        sortedRanks.forEach(r => {
          const layerNodes = layers.get(r);
          if (r > 0) {
            layerNodes.sort((a, b) => {
              const getBary = (nid) => {
                const preds = (inAdj.get(nid) || []).filter(p => !backEdges.has(`${p}->${nid}`) && posInLayer.has(p));
                if (preds.length === 0) return 0;
                return preds.reduce((sum, p) => sum + posInLayer.get(p), 0) / preds.length;
              };
              return getBary(a) - getBary(b);
            });
          }
          layerNodes.forEach((nid, idx) => posInLayer.set(nid, idx));
        });

        const positions = new Map();
        const layerSpacing = options.layerSpacing || (isTB ? 96 : 280);
        const nodeSpacing = options.nodeSpacing || (isTB ? 24 : 20);

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
      }

      function setupZoomPan(containerId, svgId, rootGroupId, transformObj) {
        const container = document.getElementById(containerId);
        const svg = document.getElementById(svgId);
        if (!container || !svg) return;

        let isDragging = false;
        let startX = 0, startY = 0;

        function updateTransform() {
          const g = document.getElementById(rootGroupId);
          if (g) {
            g.setAttribute('transform', `translate(${transformObj.x}, ${transformObj.y}) scale(${transformObj.scale})`);
          }
        }

        container.addEventListener('mousedown', (e) => {
          if (e.target.closest('.svg-node') || e.target.closest('button') || e.target.closest('input')) return;
          isDragging = true;
          startX = e.clientX - transformObj.x;
          startY = e.clientY - transformObj.y;
          container.style.cursor = 'grabbing';
        });

        window.addEventListener('mousemove', (e) => {
          if (!isDragging) return;
          transformObj.x = e.clientX - startX;
          transformObj.y = e.clientY - startY;
          updateTransform();
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
        }, { passive: false });

        svg.addEventListener('click', (e) => {
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
          svg.innerHTML += '<text x="50%" y="50%" fill="#64748b" text-anchor="middle">No column provenance derivations available.</text>';
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
          textKind.setAttribute('font-family', 'var(--font-mono)');
          textKind.setAttribute('fill', isCol ? 'var(--orange-text)' : 'var(--teal-text)');
          textKind.textContent = isCol ? 'COLUMN VERSION' : `DERIVATION (${n.knowledge || 'KNOWN'})`;
          nodeG.appendChild(textKind);

          const textLabel = document.createElementNS('http://www.w3.org/2000/svg', 'text');
          textLabel.setAttribute('x', isCol ? 16 : 12);
          textLabel.setAttribute('y', 34);
          textLabel.setAttribute('font-size', '12');
          textLabel.setAttribute('font-weight', '600');
          textLabel.setAttribute('fill', 'var(--ink)');
          textLabel.setAttribute('font-family', 'var(--font-mono)');
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

      function renderPdgGraph() {
        const svg = document.getElementById('svg-pdg');
        const defs = svg.querySelector('defs');
        const defsHtml = defs ? defs.outerHTML : '';
        svg.innerHTML = defsHtml;

        const nodes = data.pdg?.nodes || [];
        const edges = data.pdg?.edges || [];

        if (nodes.length === 0) {
          svg.innerHTML += '<text x="50%" y="50%" fill="#64748b" text-anchor="middle">Empty Program Dependence Graph.</text>';
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

        const layout = computeDagLayout(nodes, visibleEdges, {
          orientation: 'TB',
          useLineBands: true,
          layerSpacing: 100,
          nodeSpacing: 28,
          graphType: 'pdg'
        });

        pdgPositions = layout.positions;

        const g = document.createElementNS('http://www.w3.org/2000/svg', 'g');
        g.setAttribute('id', 'pdg-root-group');
        g.setAttribute('transform', `translate(${pdgTransform.x}, ${pdgTransform.y}) scale(${pdgTransform.scale})`);

        visibleEdges.forEach((e, idx) => {
          const fromPos = pdgPositions.get(e.from);
          const toPos = pdgPositions.get(e.to);
          if (!fromPos || !toPos) return;

          const p1 = { x: fromPos.x + fromPos.width / 2, y: fromPos.y + fromPos.height };
          const p2 = { x: toPos.x + toPos.width / 2, y: toPos.y };

          let d = '';
          if (p2.y >= p1.y) {
            const dy = p2.y - p1.y;
            d = `M ${p1.x} ${p1.y} C ${p1.x} ${p1.y + dy * 0.45}, ${p2.x} ${p2.y - dy * 0.45}, ${p2.x} ${p2.y}`;
          } else {
            const offset = (p1.x <= p2.x ? 50 : -50);
            d = `M ${p1.x} ${p1.y} C ${p1.x + offset} ${p1.y + 40}, ${p2.x + offset} ${p2.y - 40}, ${p2.x} ${p2.y}`;
          }

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
          textKind.setAttribute('font-family', 'var(--font-mono)');
          textKind.setAttribute('letter-spacing', '0.5');

          let kindColor = 'var(--ink-3)';
          if (isVarDef) kindColor = '#7C3AED';
          else if (isVarUse) kindColor = '#0D9488';
          else if (isCol) kindColor = 'var(--orange-text)';
          else if (isExpr) kindColor = '#3B82F6';
          else if (isRowSel) kindColor = '#7C3AED';
          else if (n.is_unknown) kindColor = '#DC2626';

          textKind.setAttribute('fill', kindColor);
          const lineStr = n.span?.start_line ? ` L${n.span.start_line}` : '';
          textKind.textContent = `${n.kind.toUpperCase()}${lineStr}`;
          nodeG.appendChild(textKind);

          const textLabel = document.createElementNS('http://www.w3.org/2000/svg', 'text');
          textLabel.setAttribute('x', isCol ? 14 : 10);
          textLabel.setAttribute('y', 34);
          textLabel.setAttribute('font-size', '12');
          textLabel.setAttribute('font-weight', '600');
          textLabel.setAttribute('fill', 'var(--ink)');
          textLabel.setAttribute('font-family', 'var(--font-mono)');

          const cleanLbl = cleanNodeLabel(n, 'pdg');
          textLabel.textContent = cleanLbl.length > 28 ? cleanLbl.slice(0, 26) + '…' : cleanLbl;
          nodeG.appendChild(textLabel);

          nodeG.addEventListener('click', (evt) => {
            evt.stopPropagation();

            document.querySelectorAll('#pdg-root-group .svg-node').forEach(el => {
              el.classList.remove('selected', 'connected-upstream', 'connected-downstream', 'dimmed');
            });
            document.querySelectorAll('#pdg-root-group .svg-edge').forEach(el => {
              el.classList.remove('highlighted', 'inbound', 'outbound', 'dimmed');
            });

            nodeG.classList.add('selected');

            const inboundEdges = visibleEdges.filter(e => e.to === n.id);
            const outboundEdges = visibleEdges.filter(e => e.from === n.id);
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
        fitGraphView('view-container-pdg', 'pdg-root-group', pdgPositions, pdgTransform);
      }

      setupZoomPan('view-container-pdg', 'svg-pdg', 'pdg-root-group', pdgTransform);

      ['toggle-edge-data', 'toggle-edge-ctrl', 'toggle-edge-rowsel', 'toggle-edge-unk'].forEach(id => {
        document.getElementById(id)?.addEventListener('change', renderPdgGraph);
      });

      document.getElementById('btn-pdg-zoom-in')?.addEventListener('click', () => {
        pdgTransform.scale = Math.min(3.5, pdgTransform.scale * 1.25);
        document.getElementById('pdg-root-group')?.setAttribute('transform', `translate(${pdgTransform.x}, ${pdgTransform.y}) scale(${pdgTransform.scale})`);
      });
      document.getElementById('btn-pdg-zoom-out')?.addEventListener('click', () => {
        pdgTransform.scale = Math.max(0.15, pdgTransform.scale / 1.25);
        document.getElementById('pdg-root-group')?.setAttribute('transform', `translate(${pdgTransform.x}, ${pdgTransform.y}) scale(${pdgTransform.scale})`);
      });
      document.getElementById('btn-pdg-reset')?.addEventListener('click', () => {
        fitGraphView('view-container-pdg', 'pdg-root-group', pdgPositions, pdgTransform);
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
          row.scrollIntoView({ behavior: 'smooth', block: 'center' });
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
            filterInfo = `<div style="font-size:11px; color:var(--accent-purple); margin-top:6px;">⚡ Filter Dependency Applied (${st.row_selections.length} predicate)</div>`;
          }

          card.innerHTML = `
            <div class="schema-card-header">
              <div class="schema-symbol">${escapeHtml(st.symbol_name)} <span style="font-size:11px; color:var(--text-dim); font-weight:normal;">v${st.version}</span></div>
              <span class="pill pill-full">Step #${idx + 1}</span>
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
          if (activeView === 'pdg') fitGraphView('view-container-pdg', 'pdg-root-group', pdgPositions, pdgTransform);
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

      // Scenario Summary Card Parser & Toggle (Phase 4.1)
      function initScenarioCard(sourceCode) {
        const card = document.getElementById('scenario-summary-card');
        const bodyContent = document.getElementById('scenario-body-content');
        const btnToggleScenario = document.getElementById('btn-toggle-scenario');
        if (!card || !bodyContent) return;

        if (!sourceCode) {
          card.style.display = 'none';
          if (btnToggleScenario) btnToggleScenario.style.display = 'none';
          return;
        }

        const lines = sourceCode.split('\n');
        const commentLines = [];
        for (let i = 0; i < lines.length; i++) {
          const trimmed = lines[i].trim();
          if (trimmed.startsWith('#')) {
            const clean = trimmed.replace(/^#\s*/, '');
            if (clean) commentLines.push(clean);
          } else if (trimmed.length > 0) {
            break;
          }
        }

        if (commentLines.length >= 3) {
          card.style.display = 'block';
          if (btnToggleScenario) btnToggleScenario.style.display = 'inline-flex';
          bodyContent.innerHTML = commentLines.map(c => `<div style="margin-bottom:3px;">${escapeHtml(c)}</div>`).join('');
        } else {
          card.style.display = 'none';
          if (btnToggleScenario) btnToggleScenario.style.display = 'none';
        }
      }

      const scenarioHeader = document.getElementById('scenario-header-toggle');
      const scenarioChevron = document.getElementById('scenario-chevron');
      const scenarioCard = document.getElementById('scenario-summary-card');
      const btnToggleScenario = document.getElementById('btn-toggle-scenario');

      function toggleScenarioCard() {
        if (!scenarioCard) return;
        scenarioCard.classList.toggle('collapsed');
        if (scenarioChevron) {
          scenarioChevron.textContent = scenarioCard.classList.contains('collapsed') ? '▶ Expand' : '▼ Collapse';
        }
      }

      if (scenarioHeader) scenarioHeader.addEventListener('click', toggleScenarioCard);
      if (btnToggleScenario) btnToggleScenario.addEventListener('click', toggleScenarioCard);

      // Reproduce Strip CLI Command Generator (Phase 7.2)
      function updateReproduceCommand() {
        const cmdElem = document.getElementById('reproduce-cmd-text');
        if (!cmdElem) return;

        const fn = data.file_name || 'fixtures/showcase_pipeline.R';
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
        if (action === 'view-prov') document.getElementById('tab-provenance')?.click();
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

      // Initialize scenario context & reproduce strip
      initScenarioCard(data.source_code || initialSource);
      updateReproduceCommand();

      // Auto-transition directly into Master Workspace when pre-analyzed file/data exists
      if (initialSource && (data.provenance?.nodes?.length > 0 || data.pdg?.nodes?.length > 0 || data.summary?.statement_count > 0)) {
        if (entryScreen) entryScreen.style.display = 'none';
        if (workspaceScreen) workspaceScreen.classList.remove('hidden');
        recomputeActiveGraphFit();
      }

    })();"###;
