import os
import re

target_path = os.path.abspath('implementation/crates/rparse-cli/src/format_html_assets.rs')

css_content = r'''
/* ==========================================================================
   R-PARSE SCIENTIFIC ANALYSIS STUDIO — EDITORIAL DESIGN SYSTEM
   Theme: Warm Bone & Paper Canvas, Near-Black Ink, Signal Orange Accents,
          Fine Subtle Rules, Restrained Lilac Dataflow Highlights
   ========================================================================== */

:root {
  /* Surfaces & Canvas (Editorial Paper Foundation) */
  --bg-page: #F7F5F0;
  --bg-canvas: #F7F5F0;
  --bg-panel: #FFFFFF;
  --bg-card: #FFFFFF;
  --bg-subtle: #EEEBE4;
  --bg-input: #FCFAF7;
  
  /* Borders & Dividers */
  --border: rgba(18, 18, 18, 0.08);
  --border-strong: rgba(18, 18, 18, 0.16);
  --border-focus: #FF5A1F;
  --border-subtle: rgba(18, 18, 18, 0.04);

  /* Typography Colors (Near-Black Ink & Warm Neutrals) */
  --text-main: #121212;
  --text-secondary: #3A3936;
  --text-muted: #77736B;
  --text-dim: #9C978E;
  --text-inverse: #FFFFFF;

  /* Accent Colorway */
  --accent-orange: #FF5A1F;
  --accent-orange-hover: #E04810;
  --accent-orange-subtle: rgba(255, 90, 31, 0.08);
  --accent-orange-glow: rgba(255, 90, 31, 0.22);
  --accent-orange-border: rgba(255, 90, 31, 0.25);

  /* Required alias for test assertion: border-bottom: 2px solid var(--accent-blue) */
  --accent-blue: #FF5A1F;
  --accent-blue-subtle: rgba(255, 90, 31, 0.08);

  /* Restrained Lilac (RowSelection, AST, Partial Retention) */
  --accent-purple: #7C3AED;
  --accent-purple-subtle: #F5F3FF;
  --accent-lilac: #7C3AED;
  --accent-lilac-subtle: #F5F3FF;

  /* Emerald (Known Static, Kept Tokens, Fully Retained) */
  --accent-green: #059669;
  --accent-green-subtle: #ECFDF5;

  /* Amber (Uncertainty, Warnings, Filter Dependencies) */
  --accent-amber: #D97706;
  --accent-amber-subtle: #FEF3C7;

  /* Rose (Errors, Dropped Tokens, Removed Columns) */
  --accent-rose: #DC2626;
  --accent-rose-subtle: #FEE2E2;

  /* Typography Stack */
  --font-mono: "JetBrains Mono", ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
  --font-sans: "Plus Jakarta Sans", -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;

  /* Elevation Shadows */
  --shadow-xs: 0 1px 2px rgba(18, 18, 18, 0.03);
  --shadow-sm: 0 2px 5px rgba(18, 18, 18, 0.04);
  --shadow-md: 0 4px 14px rgba(18, 18, 18, 0.06);
  --shadow-lg: 0 14px 32px rgba(18, 18, 18, 0.08);
  --shadow-focus: 0 0 0 3px rgba(255, 90, 31, 0.16);

  /* Radii */
  --radius-xs: 2px;
  --radius-sm: 4px;
  --radius-md: 6px;
  --radius-lg: 10px;
  --radius-pill: 9999px;
}

* { box-sizing: border-box; margin: 0; padding: 0; }

body {
  background: var(--bg-page);
  color: var(--text-main);
  font-family: var(--font-sans);
  font-size: 12px;
  line-height: 1.5;
  height: 100vh;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  user-select: none;
  -webkit-font-smoothing: antialiased;
}

/* ========================================================================= */
/* MOTION SYSTEM KEYFRAMES & UTILITIES                                       */
/* ========================================================================= */
@keyframes viewFadeIn {
  from { opacity: 0; transform: translateY(4px); }
  to { opacity: 1; transform: translateY(0); }
}

@keyframes nodeEntrance {
  from { opacity: 0; }
  to { opacity: 1; }
}

@keyframes edgeEntrance {
  from { opacity: 0; }
  to { opacity: 1; }
}

@keyframes cardEntrance {
  from { opacity: 0; transform: translateY(6px); }
  to { opacity: 1; transform: translateY(0); }
}

@keyframes pulseGlow {
  0%, 100% { box-shadow: 0 0 0 0 rgba(5, 150, 105, 0.35); }
  50% { box-shadow: 0 0 0 4px rgba(5, 150, 105, 0.0); }
}

@keyframes pulseAnalyzing {
  0%, 100% { box-shadow: 0 0 0 0 rgba(255, 90, 31, 0.4); }
  50% { box-shadow: 0 0 0 5px rgba(255, 90, 31, 0.0); }
}

.view-animate-in {
  animation: viewFadeIn 0.16s cubic-bezier(0.16, 1, 0.3, 1) both;
}

/* ========================================================================= */
/* PHASE 1: Centered Code-Entry Experience                                   */
/* ========================================================================= */
.entry-screen {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100vh;
  z-index: 100;
  background: var(--bg-page);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 24px;
  transition: opacity 0.35s cubic-bezier(0.16, 1, 0.3, 1), transform 0.35s cubic-bezier(0.16, 1, 0.3, 1);
}
.entry-screen.transition-out {
  opacity: 0;
  transform: scale(0.98) translateY(-12px);
  pointer-events: none;
}
.entry-card {
  max-width: 780px;
  width: 100%;
  display: flex;
  flex-direction: column;
  gap: 16px;
  background: #FFFFFF;
  padding: 36px 40px;
  border-radius: var(--radius-lg);
  border: 1px solid var(--border);
  box-shadow: var(--shadow-lg);
}
.entry-badge-row {
  display: flex;
  align-items: center;
  gap: 8px;
}
.brand-logo {
  font-family: var(--font-mono);
  font-weight: 700;
  font-size: 12px;
  letter-spacing: 0.06em;
  color: var(--text-main);
  background: transparent;
  padding: 0;
  border: none;
}
.brand-subtitle {
  font-size: 11px;
  color: var(--text-muted);
  font-weight: 500;
  letter-spacing: 0.02em;
}
.entry-title {
  font-size: 26px;
  font-weight: 700;
  color: var(--text-main);
  letter-spacing: -0.03em;
  margin-top: 2px;
}
.entry-desc {
  font-size: 13px;
  color: var(--text-muted);
  line-height: 1.55;
}
.entry-editor-wrapper {
  border: 1px solid var(--border-strong);
  border-radius: var(--radius-md);
  background: var(--bg-input);
  box-shadow: var(--shadow-xs);
  display: flex;
  flex-direction: column;
  overflow: hidden;
  transition: border-color 0.15s ease, box-shadow 0.15s ease;
}
.entry-editor-wrapper:focus-within {
  border-color: var(--accent-orange);
  box-shadow: var(--shadow-focus);
}
.entry-code-input {
  width: 100%;
  height: 270px;
  min-height: 180px;
  max-height: 480px;
  border: none;
  padding: 14px 16px;
  font-family: var(--font-mono);
  font-size: 12px;
  line-height: 21px;
  color: var(--text-main);
  resize: vertical;
  outline: none;
  background: transparent;
  white-space: pre;
}
.entry-editor-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 7px 14px;
  background: var(--bg-subtle);
  border-top: 1px solid var(--border);
  font-size: 11px;
  color: var(--text-dim);
  font-family: var(--font-mono);
}
.entry-actions-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  flex-wrap: wrap;
}
.entry-actions-left {
  display: flex;
  align-items: center;
  gap: 8px;
}
.btn {
  padding: 5px 12px;
  font-size: 11px;
  background: #FFFFFF;
  border: 1px solid var(--border-strong);
  color: var(--text-secondary);
  border-radius: var(--radius-sm);
  cursor: pointer;
  display: inline-flex;
  align-items: center;
  gap: 5px;
  font-weight: 500;
  transition: all 0.12s ease;
  font-family: inherit;
}
.btn:hover {
  background: var(--bg-subtle);
  color: var(--text-main);
  border-color: var(--text-secondary);
}
.btn:active {
  transform: scale(0.98);
}
.btn-primary {
  background: var(--accent-orange);
  color: #FFFFFF;
  border: 1px solid var(--accent-orange-hover);
  font-weight: 600;
  padding: 6px 16px;
  box-shadow: 0 1px 3px rgba(255, 90, 31, 0.25);
}
.btn-primary:hover {
  background: var(--accent-orange-hover);
  color: #FFFFFF;
  box-shadow: 0 3px 8px rgba(255, 90, 31, 0.35);
}
.btn-primary:active {
  transform: scale(0.98);
}
.btn-outline {
  background: transparent;
  border: 1px solid var(--border-strong);
  color: var(--text-secondary);
}
.btn-outline:hover {
  background: var(--bg-subtle);
  color: var(--text-main);
  border-color: var(--text-main);
}
.select-dropdown {
  padding: 5px 10px;
  font-size: 11px;
  border: 1px solid var(--border-strong);
  border-radius: var(--radius-sm);
  background: #FFFFFF;
  color: var(--text-main);
  outline: none;
  cursor: pointer;
  font-family: inherit;
}
.select-dropdown:focus {
  border-color: var(--accent-orange);
}
.entry-footer-note {
  font-size: 11px;
  color: var(--text-dim);
  font-family: var(--font-mono);
  text-align: center;
  margin-top: 4px;
}
.validation-msg {
  padding: 8px 12px;
  border-radius: var(--radius-sm);
  font-size: 11px;
  background: var(--accent-rose-subtle);
  color: #991b1b;
  border: 1px solid rgba(220, 38, 38, 0.3);
  display: flex;
  align-items: center;
  gap: 8px;
}

/* ========================================================================= */
/* PHASE 2: Application Header & Analysis Navigation                         */
/* ========================================================================= */
.app-header {
  background: var(--bg-panel);
  border-bottom: 1px solid var(--border);
  display: flex;
  flex-direction: column;
  flex-shrink: 0;
  z-index: 50;
}

/* Row 1: Identity, Quiet Status/Metrics, Primary Actions */
.header-top-row {
  height: 42px;
  padding: 0 16px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  border-bottom: 1px solid var(--border-subtle);
  background: var(--bg-panel);
  gap: 16px;
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
  font-size: 12px;
  letter-spacing: 0.08em;
  color: var(--text-main);
  background: transparent;
  border: none;
  padding: 0;
}

.header-sep {
  display: inline-block;
  width: 8px;
  text-align: center;
  user-select: none;
}
.header-sep::after {
  content: "/";
  color: var(--text-dim);
  font-size: 11px;
  font-family: var(--font-mono);
  opacity: 0.6;
}

.header-sub {
  font-size: 11px;
  font-weight: 600;
  color: var(--text-muted);
  letter-spacing: 0.02em;
}

.header-filename {
  font-family: var(--font-mono);
  font-size: 11px;
  font-weight: 500;
  color: var(--text-main);
  background: transparent;
  border: none;
  padding: 0;
  max-width: 240px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

/* Center Editorial Status & Plain Metrics (NO Individual Boxed Badges) */
.header-center-info {
  display: flex;
  align-items: center;
  gap: 12px;
  flex-wrap: nowrap;
}

.analysis-status-badge {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  font-size: 11px;
  color: var(--text-muted);
  user-select: none;
}

.status-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: var(--accent-green);
  display: inline-block;
  flex-shrink: 0;
  animation: pulseGlow 2.5s infinite;
  transition: background 0.15s ease;
}

.status-dot.analyzing {
  background: var(--accent-orange);
  animation: pulseAnalyzing 1s infinite;
}

.status-dot.error {
  background: var(--accent-rose);
  animation: none;
}

.status-dot.uncertain {
  background: var(--accent-amber);
  animation: none;
}

.header-meta-group {
  display: flex;
  align-items: center;
  gap: 5px;
  font-size: 11px;
  color: var(--text-dim);
}

.header-meta-item {
  font-size: 11px;
  color: var(--text-dim);
  font-weight: 400;
}

.header-meta-item.stat-unk-text {
  color: var(--accent-amber);
  font-weight: 500;
}

.header-meta-item.stat-known-text {
  color: var(--text-muted);
}

.header-meta-sep, .stat-divider {
  color: var(--text-dim);
  font-size: 10px;
  user-select: none;
  opacity: 0.5;
}

/* Header Summary Stats — Plain Aligned Text, NO Boxed Container */
.stats-strip {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 11px;
  color: var(--text-muted);
  background: transparent;
  border: none;
  padding: 0;
}

.stat-item {
  display: flex;
  align-items: baseline;
  gap: 3px;
}

.stat-val {
  color: var(--text-main);
  font-weight: 600;
  font-family: var(--font-mono);
  font-size: 11px;
}

.stat-lbl {
  color: var(--text-dim);
  font-size: 10px;
  font-weight: 400;
}

/* Header Actions */
.header-actions-row {
  display: flex;
  align-items: center;
  gap: 10px;
  flex-shrink: 0;
}

.btn-editorial-link {
  font-size: 11px;
  font-weight: 500;
  color: var(--accent-orange);
  text-decoration: none;
  padding: 4px 8px;
  border-radius: var(--radius-xs);
  transition: all 0.12s ease;
  display: inline-flex;
  align-items: center;
  gap: 4px;
}
.btn-editorial-link:hover {
  background: var(--accent-orange-subtle);
}

.header-actions-row .btn-secondary {
  background: transparent;
  color: var(--text-muted);
  border: 1px solid var(--border);
  font-weight: 500;
  padding: 3px 10px;
  font-size: 11px;
  border-radius: var(--radius-sm);
  cursor: pointer;
  line-height: 18px;
  transition: all 0.12s ease;
}

.header-actions-row .btn-secondary:hover {
  background: var(--bg-subtle);
  color: var(--text-main);
  border-color: var(--border-strong);
}

.header-actions-row .btn-primary {
  background: var(--accent-orange);
  color: #FFFFFF;
  border: 1px solid var(--accent-orange-hover);
  font-weight: 600;
  padding: 3px 14px;
  font-size: 11px;
  border-radius: var(--radius-pill);
  cursor: pointer;
  line-height: 18px;
  box-shadow: 0 1px 3px rgba(255, 90, 31, 0.2);
  transition: all 0.12s ease;
}

.header-actions-row .btn-primary:hover {
  background: var(--accent-orange-hover);
  border-color: var(--accent-orange-hover);
  box-shadow: 0 2px 6px rgba(255, 90, 31, 0.35);
}

/* Row 2: Analysis Navigation (Text-and-Icon underline tabs, NO boxed cards) */
.app-subnav {
  height: 34px;
  background: var(--bg-panel);
  padding: 0 16px;
  display: flex;
  align-items: center;
  gap: 26px;
  user-select: none;
  box-sizing: border-box;
}

.nav-tab {
  height: 34px;
  padding: 0 2px;
  background: transparent;
  border: none;
  border-bottom: 2px solid transparent;
  color: var(--text-muted);
  cursor: pointer;
  font-size: 12px;
  font-weight: 500;
  display: inline-flex;
  align-items: center;
  gap: 6px;
  transition: color 0.12s ease, border-color 0.12s ease;
  outline: none;
  box-sizing: border-box;
}

.nav-tab:hover {
  color: var(--text-main);
}

.nav-tab:focus-visible {
  outline: 2px solid var(--accent-orange);
  outline-offset: -2px;
}

.nav-tab.active {
  color: var(--text-main);
  font-weight: 600;
  border-bottom: 2px solid var(--accent-blue);
  background: transparent;
}

.nav-tab-icon {
  width: 13px;
  height: 13px;
  stroke: currentColor;
  flex-shrink: 0;
  opacity: 0.6;
  transition: opacity 0.12s ease, stroke 0.12s ease;
}

.nav-tab:hover .nav-tab-icon {
  opacity: 0.9;
  stroke: var(--text-main);
}

.nav-tab.active .nav-tab-icon {
  opacity: 1;
  stroke: var(--accent-orange);
}

.nav-count {
  font-size: 10px;
  font-family: var(--font-mono);
  color: var(--text-dim);
  font-weight: 400;
  background: transparent;
  border: none;
  padding: 0;
  margin-left: 2px;
  transition: color 0.12s ease;
}

.nav-tab:hover .nav-count {
  color: var(--text-main);
}

.nav-tab.active .nav-count {
  color: var(--accent-orange);
  font-weight: 600;
}

/* 30/70 Split Body */
.workbench-body {
  flex: 1;
  display: flex;
  flex-direction: row;
  overflow: hidden;
  position: relative;
  height: calc(100vh - 76px);
}

/* Left: Source Panel (Default 30%) */
.pane-source {
  width: 30%;
  min-width: 22%;
  max-width: 45%;
  background: #FFFFFF;
  border-right: 1px solid var(--border);
  display: flex;
  flex-direction: column;
  position: relative;
  overflow: hidden;
  flex-shrink: 0;
}
.pane-header {
  height: 34px;
  padding: 0 14px;
  background: var(--bg-panel);
  border-bottom: 1px solid var(--border);
  display: flex;
  align-items: center;
  justify-content: space-between;
  font-size: 11px;
  font-weight: 600;
  color: var(--text-muted);
  flex-shrink: 0;
}
.pane-title {
  text-transform: uppercase;
  font-size: 10px;
  letter-spacing: 0.08em;
  color: var(--text-muted);
  font-weight: 700;
}
.pane-badge {
  font-family: var(--font-mono);
  font-size: 10px;
  color: var(--text-dim);
  background: transparent;
  padding: 0;
  border: none;
}
.pane-header .btn-ghost {
  background: transparent;
  border: 1px solid transparent;
  color: var(--text-muted);
  font-size: 11px;
  padding: 2px 7px;
  border-radius: var(--radius-xs);
  cursor: pointer;
  transition: all 0.12s ease;
}
.pane-header .btn-ghost:hover {
  color: var(--text-main);
  background: var(--bg-subtle);
  border-color: var(--border);
}
.pane-header .btn-ghost.active {
  color: var(--accent-orange);
  background: var(--accent-orange-subtle);
  border-color: var(--accent-orange-border);
  font-weight: 600;
}
.source-stale-banner {
  display: none;
  padding: 6px 12px;
  background: var(--accent-amber-subtle);
  border-bottom: 1px solid rgba(217, 119, 6, 0.25);
  font-size: 11px;
  color: #92400e;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
}
.source-status-banner {
  display: none;
  padding: 6px 12px;
  background: var(--accent-rose-subtle);
  border-bottom: 1px solid rgba(220, 38, 38, 0.25);
  font-size: 11px;
  color: #991b1b;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
}
.source-editor-textarea {
  display: none;
  flex: 1;
  width: 100%;
  padding: 8px 12px;
  background: var(--bg-input);
  color: var(--text-main);
  font-family: var(--font-mono);
  font-size: 12px;
  line-height: 20px;
  border: none;
  resize: none;
  outline: none;
  white-space: pre;
  tab-size: 2;
  box-sizing: border-box;
}

/* Source Viewer */
.source-viewer {
  font-family: var(--font-mono);
  font-size: 12px;
  line-height: 20px;
  padding: 6px 0 24px 0;
  user-select: text;
  flex: 1;
  overflow-y: auto;
  overflow-x: auto;
  background: #FFFFFF;
}
.code-line {
  display: flex;
  align-items: stretch;
  min-height: 20px;
  line-height: 20px;
  padding: 0 8px;
  cursor: pointer;
  border-left: 3px solid transparent;
  transition: background 0.08s ease, border-left-color 0.08s ease;
  box-sizing: border-box;
}
.code-line:hover {
  background: var(--bg-subtle);
}
.code-line.active-line {
  background: var(--accent-orange-subtle);
  border-left-color: var(--accent-orange);
}
.code-line.highlighted-span {
  background: var(--accent-amber-subtle);
  border-left-color: var(--accent-amber);
}
.line-num {
  width: 36px;
  min-height: 20px;
  line-height: 20px;
  text-align: right;
  color: var(--text-dim);
  padding-right: 12px;
  user-select: none;
  flex-shrink: 0;
  border-right: 1px solid var(--border);
  margin-right: 8px;
  box-sizing: border-box;
}
.line-text {
  min-height: 20px;
  line-height: 20px;
  white-space: pre;
  flex: 1;
  color: var(--text-main);
  box-sizing: border-box;
}

/* Source Word Wrap Behavior */
#source-pane.word-wrap-enabled .source-viewer {
  overflow-x: hidden;
}
#source-pane.word-wrap-enabled .code-line {
  align-items: stretch;
  min-width: 0;
  width: 100%;
}
#source-pane.word-wrap-enabled .line-text {
  white-space: pre-wrap;
  word-break: break-word;
  overflow-wrap: anywhere;
  min-width: 0;
}
#source-pane.word-wrap-enabled .source-editor-textarea {
  white-space: pre-wrap;
  word-break: break-word;
  overflow-wrap: anywhere;
  overflow-x: hidden;
}

#source-pane:not(.word-wrap-enabled) .source-viewer {
  overflow-x: auto;
}
#source-pane:not(.word-wrap-enabled) .code-line {
  align-items: stretch;
  min-width: max-content;
}
#source-pane:not(.word-wrap-enabled) .line-text {
  white-space: pre;
  word-break: normal;
  overflow-wrap: normal;
}
#source-pane:not(.word-wrap-enabled) .source-editor-textarea {
  white-space: pre;
  word-break: normal;
  overflow-wrap: normal;
  overflow-x: auto;
}
.kw { color: #991B1B; font-weight: 600; }
.fn { color: #FF5A1F; font-weight: 500; }
.str { color: #065F46; }
.num { color: #B45309; }
.cmt { color: #77736B; font-style: italic; }

/* Draggable Splitter */
.splitter {
  width: 4px;
  cursor: col-resize;
  background: var(--border);
  flex-shrink: 0;
  position: relative;
  z-index: 10;
  transition: background 0.12s ease;
}
.splitter:hover, .splitter.active {
  background: var(--accent-orange);
}

/* Right: Dominant Analysis Canvas (~70%) */
.main-view-pane {
  flex: 1;
  min-width: 0;
  background: var(--bg-page);
  display: flex;
  flex-direction: column;
  position: relative;
  overflow: hidden;
}
.view-toolbar {
  height: 34px;
  background: #FFFFFF;
  border-bottom: 1px solid var(--border);
  padding: 0 14px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  flex-shrink: 0;
  gap: 12px;
}
.toolbar-group {
  display: flex;
  align-items: center;
  gap: 8px;
}
.toolbar-label {
  font-size: 11px;
  color: var(--text-dim);
  font-weight: 500;
}
.filter-box {
  position: relative;
  display: flex;
  align-items: center;
}
.filter-input {
  padding: 4px 8px;
  font-size: 11px;
  background: var(--bg-input);
  border: 1px solid var(--border);
  color: var(--text-main);
  border-radius: var(--radius-xs);
  width: 180px;
  outline: none;
  font-family: inherit;
  transition: border-color 0.12s, width 0.15s, box-shadow 0.12s;
}
.filter-input:focus {
  border-color: var(--accent-orange);
  box-shadow: var(--shadow-focus);
  width: 220px;
}
.filter-clear {
  position: absolute;
  right: 6px;
  background: transparent;
  border: none;
  color: var(--text-dim);
  cursor: pointer;
  font-size: 11px;
}
.edge-filter-group {
  display: flex;
  align-items: center;
  gap: 8px;
}
.toggle-chip {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  font-size: 11px;
  padding: 2px 5px;
  background: transparent;
  border: none;
  color: var(--text-muted);
  cursor: pointer;
  user-select: none;
  font-family: var(--font-sans);
  transition: color 0.12s ease;
}
.toggle-chip:hover {
  color: var(--text-main);
}
.toggle-chip input {
  cursor: pointer;
  accent-color: var(--accent-orange);
  margin: 0;
  width: 12px;
  height: 12px;
}
.toggle-chip.chip-ctrl { color: #D97706; }
.toggle-chip.chip-rowsel { color: var(--accent-purple); }
.toggle-chip.chip-unk { color: var(--accent-rose); }

/* SVG Graph Viewport */
.canvas-viewport {
  flex: 1;
  position: relative;
  overflow: hidden;
  background: var(--bg-canvas);
}
.graph-canvas-container {
  width: 100%;
  height: 100%;
  position: relative;
  overflow: hidden;
  background: radial-gradient(circle, #D8D2C4 1px, transparent 1px) #F7F5F0;
  background-size: 22px 22px;
  cursor: grab;
  user-select: none;
}
.graph-canvas-container:active {
  cursor: grabbing;
}
svg.graph-svg {
  width: 100%;
  height: 100%;
  display: block;
}

/* SVG Node Kinds (Precision Editorial & Ink System) */
.node-stmt { fill: #FFFFFF; stroke: #3A3936; stroke-width: 1.3; }
.node-vardef { fill: #FAF5FF; stroke: #7C3AED; stroke-width: 1.3; }
.node-varuse { fill: #F0FDFA; stroke: #0D9488; stroke-width: 1.3; }
.node-col { fill: #FFFFFF; stroke: var(--border-strong); stroke-width: 1.4; }
.node-expr { fill: #EFF6FF; stroke: #3B82F6; stroke-width: 1.3; }
.node-rowsel { fill: #FAF5FF; stroke: #7C3AED; stroke-width: 1.4; stroke-dasharray: 4,3; }
.node-unk { fill: #FFFBEB; stroke: #D97706; stroke-width: 1.4; stroke-dasharray: 4,3; }
.node-deriv { fill: #FFFFFF; stroke: #059669; stroke-width: 1.4; }

.svg-node {
  cursor: pointer;
  transition: filter 0.16s cubic-bezier(0.16, 1, 0.3, 1), opacity 0.16s cubic-bezier(0.16, 1, 0.3, 1);
  animation: nodeEntrance 0.2s cubic-bezier(0.16, 1, 0.3, 1) both;
}
.svg-node:hover {
  filter: drop-shadow(0 3px 10px rgba(18, 18, 18, 0.10));
}
.svg-node.selected rect {
  stroke: var(--accent-orange) !important;
  stroke-width: 2.2px !important;
  filter: drop-shadow(0 0 8px rgba(255, 90, 31, 0.35));
}
.svg-node.connected-upstream rect {
  stroke: var(--accent-orange) !important;
  stroke-width: 1.8px !important;
  filter: drop-shadow(0 0 6px rgba(255, 90, 31, 0.25));
}
.svg-node.connected-downstream rect {
  stroke: var(--accent-green) !important;
  stroke-width: 1.8px !important;
  filter: drop-shadow(0 0 6px rgba(5, 150, 105, 0.25));
}
.svg-node.dimmed { opacity: 0.12 !important; filter: grayscale(50%); }

/* SVG Edges */
.svg-edge { fill: none; transition: stroke-width 0.16s, opacity 0.16s, stroke 0.16s; animation: edgeEntrance 0.2s ease both; }
.edge-data { stroke: var(--accent-orange); stroke-width: 1.4; }
.edge-ctrl { stroke: #D97706; stroke-width: 1.4; }
.edge-rowsel { stroke: #7C3AED; stroke-width: 1.4; stroke-dasharray: 4,3; }
.edge-colver { stroke: #059669; stroke-width: 1.4; }
.edge-unk { stroke: #DC2626; stroke-width: 1.4; stroke-dasharray: 3,3; }
.svg-edge.highlighted { stroke-width: 2.4px !important; opacity: 1 !important; }
.svg-edge.highlighted.inbound { stroke: var(--accent-orange) !important; }
.svg-edge.highlighted.outbound { stroke: var(--accent-green) !important; }
.svg-edge.dimmed { opacity: 0.08 !important; }

/* Backward Slice View — Single Editorial Metric Strip */
.slice-container {
  padding: 18px;
  display: flex;
  flex-direction: column;
  gap: 14px;
  overflow-y: auto;
  height: 100%;
  background: var(--bg-canvas);
}
.slice-metrics-ribbon {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  background: #FFFFFF;
  border: 1px solid var(--border);
  border-radius: var(--radius-sm);
  box-shadow: var(--shadow-xs);
  overflow: hidden;
}
.metric-cell {
  background: #FFFFFF;
  border: none;
  border-right: 1px solid var(--border);
  padding: 12px 16px;
  box-shadow: none;
  display: flex;
  flex-direction: column;
  justify-content: center;
  transition: background 0.12s ease;
}
.metric-cell:last-child {
  border-right: none;
}
.metric-cell:hover {
  background: var(--bg-canvas);
}
.metric-num {
  font-size: 22px;
  font-weight: 700;
  font-family: var(--font-mono);
  color: var(--text-main);
  letter-spacing: -0.02em;
  line-height: 1.1;
}
.metric-desc {
  font-size: 9px;
  color: var(--text-muted);
  text-transform: uppercase;
  margin-top: 4px;
  font-weight: 600;
  letter-spacing: 0.08em;
}
.stmt-retention-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}
.stmt-card {
  background: #FFFFFF;
  border: 1px solid var(--border);
  border-radius: var(--radius-sm);
  padding: 10px 14px;
  cursor: pointer;
  transition: all 0.12s cubic-bezier(0.16, 1, 0.3, 1);
  box-shadow: var(--shadow-xs);
  animation: cardEntrance 0.2s cubic-bezier(0.16, 1, 0.3, 1) both;
}
.stmt-card:hover {
  border-color: var(--border-strong);
  background: #FFFFFF;
  box-shadow: var(--shadow-sm);
}
.stmt-card.full { border-left: 3px solid var(--accent-orange); }
.stmt-card.partial { border-left: 3px solid var(--accent-purple); }
.stmt-card.none { border-left: 3px solid var(--border-strong); opacity: 0.65; }
.stmt-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 5px;
}
.pill {
  font-size: 9px;
  font-weight: 700;
  padding: 2px 6px;
  border-radius: 2px;
  text-transform: uppercase;
  font-family: var(--font-mono);
  letter-spacing: 0.04em;
}
.pill-full { background: var(--accent-orange-subtle); color: var(--accent-orange); border: 1px solid var(--accent-orange-border); }
.pill-partial { background: var(--accent-purple-subtle); color: var(--accent-purple); border: 1px solid rgba(124, 58, 237, 0.25); }
.pill-none { background: var(--bg-subtle); color: var(--text-dim); border: 1px solid var(--border); }
.pill-keep { background: var(--accent-green-subtle); color: var(--accent-green); border: 1px solid rgba(5, 150, 105, 0.25); }
.pill-drop { background: var(--accent-rose-subtle); color: var(--accent-rose); border: 1px solid rgba(220, 38, 38, 0.25); }
.stmt-code {
  font-family: var(--font-mono);
  font-size: 11px;
  color: var(--text-main);
  background: var(--bg-subtle);
  padding: 5px 8px;
  border-radius: var(--radius-xs);
  border: 1px solid var(--border);
  white-space: pre-wrap;
}
.sub-elements-list {
  margin-top: 8px;
  padding-left: 10px;
  border-left: 1px dashed var(--border);
  display: flex;
  flex-direction: column;
  gap: 4px;
}
.sub-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  font-size: 11px;
  padding: 3px 6px;
  border-radius: 2px;
  background: var(--bg-subtle);
  border: 1px solid var(--border);
}
.sub-item:hover { background: #FFFFFF; border-color: var(--border-strong); }

/* Schema Evolution View */
.schema-timeline {
  padding: 18px;
  display: flex;
  flex-direction: column;
  gap: 12px;
  overflow-y: auto;
  height: 100%;
  background: var(--bg-canvas);
}
.schema-card {
  background: #FFFFFF;
  border: 1px solid var(--border);
  border-radius: var(--radius-sm);
  padding: 12px 14px;
  cursor: pointer;
  transition: all 0.12s cubic-bezier(0.16, 1, 0.3, 1);
  box-shadow: var(--shadow-xs);
  animation: cardEntrance 0.2s cubic-bezier(0.16, 1, 0.3, 1) both;
}
.schema-card:hover {
  border-color: var(--border-strong);
  background: #FFFFFF;
  box-shadow: var(--shadow-sm);
}
.schema-card-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 6px;
}
.schema-symbol {
  font-family: var(--font-mono);
  font-weight: 700;
  font-size: 12px;
  color: var(--accent-orange);
}
.schema-diff-summary {
  display: flex;
  gap: 5px;
  margin-bottom: 6px;
}
.diff-badge {
  font-size: 9px;
  font-weight: 700;
  padding: 1px 6px;
  border-radius: 2px;
  font-family: var(--font-mono);
}
.diff-added { background: var(--accent-green-subtle); color: var(--accent-green); border: 1px solid rgba(5, 150, 105, 0.25); }
.diff-removed { background: var(--accent-rose-subtle); color: var(--accent-rose); border: 1px solid rgba(220, 38, 38, 0.25); }
.diff-mod { background: var(--accent-orange-subtle); color: var(--accent-orange); border: 1px solid var(--accent-orange-border); }
.diff-ret { background: var(--bg-subtle); color: var(--text-muted); border: 1px solid var(--border); }
.schema-cols-grid {
  display: flex;
  flex-wrap: wrap;
  gap: 5px;
  margin-top: 8px;
}
.col-version-badge {
  font-family: var(--font-mono);
  font-size: 10px;
  padding: 2px 6px;
  border-radius: var(--radius-xs);
  background: var(--bg-subtle);
  border: 1px solid var(--border);
  color: var(--text-main);
}
.col-version-badge.highlighted {
  border-color: var(--accent-orange);
  background: var(--accent-orange-subtle);
  color: var(--accent-orange);
}
.schema-predicate-note {
  font-size: 10px;
  color: var(--accent-amber);
  margin-top: 6px;
  font-style: italic;
  display: flex;
  align-items: center;
  gap: 4px;
}

/* Contextual Inspector Panel */
.contextual-inspector {
  width: 310px;
  background: #FFFFFF;
  border-left: 1px solid var(--border);
  display: flex;
  flex-direction: column;
  position: absolute;
  right: -310px;
  top: 0;
  bottom: 0;
  z-index: 40;
  transition: right 0.22s cubic-bezier(0.16, 1, 0.3, 1);
  box-shadow: -4px 0 16px rgba(18, 18, 18, 0.05);
}
.contextual-inspector.open {
  right: 0;
}
.inspector-header {
  height: 36px;
  padding: 0 14px;
  background: var(--bg-panel);
  border-bottom: 1px solid var(--border);
  display: flex;
  align-items: center;
  justify-content: space-between;
  flex-shrink: 0;
}
.inspector-title-group {
  display: flex;
  align-items: center;
  gap: 8px;
}
.inspector-content {
  padding: 14px;
  display: flex;
  flex-direction: column;
  gap: 12px;
  overflow-y: auto;
  flex: 1;
  background: #FFFFFF;
}
.card {
  background: #FFFFFF;
  border: 1px solid var(--border);
  border-radius: var(--radius-sm);
  padding: 12px;
  box-shadow: none;
}
.prop-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 11px;
}
.prop-table td {
  padding: 4px 6px;
  border-bottom: 1px solid var(--border);
  vertical-align: top;
}
.prop-table td.key {
  color: var(--text-dim);
  width: 95px;
  font-weight: 500;
}
.prop-table td.val {
  color: var(--text-main);
  font-family: var(--font-mono);
  word-break: break-all;
}
.raw-json {
  background: var(--bg-subtle);
  border: 1px solid var(--border);
  padding: 8px 10px;
  font-family: var(--font-mono);
  font-size: 10px;
  color: var(--text-muted);
  border-radius: var(--radius-xs);
  max-height: 180px;
  overflow-y: auto;
  white-space: pre-wrap;
  word-break: break-all;
}
.inspector-section {
  margin-top: 10px;
}
.inspector-section-title {
  font-size: 10px;
  font-weight: 700;
  color: var(--text-dim);
  text-transform: uppercase;
  letter-spacing: 0.06em;
  margin-bottom: 5px;
  display: flex;
  align-items: center;
  justify-content: space-between;
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
  background: var(--bg-canvas);
  border: 1px solid var(--border);
  border-radius: 2px;
  font-size: 11px;
  cursor: pointer;
  transition: all 0.12s ease;
}
.conn-item:hover {
  background: #FFFFFF;
  border-color: var(--accent-orange);
}
.conn-label {
  font-family: var(--font-mono);
  color: var(--text-main);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 190px;
}
.conn-pill {
  font-size: 9px;
  padding: 1px 5px;
  border-radius: 2px;
  font-weight: 600;
  text-transform: uppercase;
  font-family: var(--font-mono);
}

/* Evidence Drawer */
.evidence-drawer {
  position: absolute;
  bottom: -220px;
  left: 0;
  right: 0;
  height: 220px;
  background: #FFFFFF;
  border-top: 1px solid var(--border);
  display: flex;
  flex-direction: column;
  z-index: 60;
  transition: bottom 0.22s cubic-bezier(0.16, 1, 0.3, 1);
  box-shadow: 0 -4px 16px rgba(18, 18, 18, 0.05);
}
.evidence-drawer.open {
  bottom: 0;
}
.drawer-header {
  height: 34px;
  padding: 0 16px;
  background: var(--bg-panel);
  border-bottom: 1px solid var(--border);
  display: flex;
  align-items: center;
  justify-content: space-between;
  flex-shrink: 0;
}
.drawer-tabs {
  display: flex;
  gap: 16px;
}
.drawer-tab {
  font-size: 11px;
  font-weight: 600;
  color: var(--text-muted);
  cursor: pointer;
  padding: 2px 0;
  border-bottom: 2px solid transparent;
}
.drawer-tab.active {
  color: var(--accent-orange);
  border-bottom-color: var(--accent-orange);
}
.drawer-content {
  flex: 1;
  padding: 12px 16px;
  overflow-y: auto;
  font-size: 11px;
  background: #FFFFFF;
}
.diag-item {
  padding: 6px 10px;
  border-radius: var(--radius-xs);
  margin-bottom: 6px;
  border: 1px solid var(--border);
  background: var(--bg-subtle);
  display: flex;
  align-items: center;
  justify-content: space-between;
}
.diag-item.error { border-color: rgba(220, 38, 38, 0.3); background: var(--accent-rose-subtle); color: #991b1b; }
.diag-item.warning { border-color: rgba(217, 119, 6, 0.3); background: var(--accent-amber-subtle); color: #92400e; }
.diag-item.info { border-color: rgba(5, 150, 105, 0.25); background: var(--accent-green-subtle); color: #065F46; }

/* Responsive Adjustments */
@media (max-width: 1024px) {
  .pane-source { width: 35%; }
  .contextual-inspector { width: 290px; }
}
@media (max-width: 768px) {
  .workbench-body { flex-direction: column; }
  .pane-source { width: 100%; height: 40%; max-width: none; }
  .main-view-pane { width: 100%; height: 60%; }
  .splitter { display: none; }
  .contextual-inspector { width: 100%; max-width: none; }
}
@media (prefers-reduced-motion: reduce) {
  *, *::before, *::after {
    animation-duration: 0.01ms !important;
    animation-iteration-count: 1 !important;
    transition-duration: 0.01ms !important;
  }
  .entry-screen, .workspace-screen, .contextual-inspector, .evidence-drawer {
    transition: none !important;
  }
}

/* AST Explorer */
.ast-explorer-container {
  display: flex;
  flex-direction: column;
  width: 100%;
  height: 100%;
  background: var(--bg-canvas);
  overflow: hidden;
}

.ast-toolbar {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 6px 14px;
  border-bottom: 1px solid var(--border);
  background: #FFFFFF;
  flex-wrap: wrap;
}

.ast-rep-selector {
  display: inline-flex;
  border-radius: var(--radius-sm);
  background: var(--bg-subtle);
  padding: 2px;
  border: 1px solid var(--border);
}

.ast-rep-btn {
  padding: 3px 10px;
  border: none;
  background: transparent;
  font-size: 11px;
  font-weight: 500;
  color: var(--text-muted);
  border-radius: 2px;
  cursor: pointer;
  transition: all 0.12s ease;
  font-family: inherit;
}

.ast-rep-btn:hover {
  color: var(--text-main);
}

.ast-rep-btn.active {
  background: #FFFFFF;
  color: var(--accent-orange);
  box-shadow: var(--shadow-xs);
  font-weight: 600;
}

.ast-search-box {
  display: flex;
  align-items: center;
  position: relative;
  background: var(--bg-input);
  border: 1px solid var(--border);
  border-radius: var(--radius-xs);
  padding: 3px 8px;
  gap: 6px;
  transition: border-color 0.12s ease;
}

.ast-search-box:focus-within {
  border-color: var(--accent-orange);
  box-shadow: var(--shadow-focus);
}

.ast-search-input {
  border: none;
  background: transparent;
  font-size: 11px;
  color: var(--text-main);
  outline: none;
  width: 180px;
  font-family: inherit;
}

.ast-search-clear {
  border: none;
  background: transparent;
  cursor: pointer;
  color: var(--text-dim);
  font-size: 11px;
  display: none;
  padding: 0 2px;
}

.ast-search-badge {
  font-size: 10px;
  font-family: var(--font-mono);
  color: var(--accent-orange);
  background: var(--accent-orange-subtle);
  padding: 1px 5px;
  border-radius: 3px;
  border: 1px solid var(--accent-orange-border);
  display: none;
}

.ast-tree-viewport {
  flex: 1;
  overflow-y: auto;
  overflow-x: auto;
  padding: 14px 18px 40px;
  background: #FFFFFF;
  font-family: var(--font-mono);
  font-size: 11px;
  line-height: 22px;
}

.ast-node-row {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 1px 4px;
  border-radius: 2px;
  cursor: pointer;
  transition: background 0.08s ease;
  white-space: nowrap;
}

.ast-node-row:hover {
  background: var(--bg-subtle);
}

.ast-node-row.selected {
  background: var(--accent-orange-subtle);
}

.ast-caret {
  width: 12px;
  font-size: 9px;
  color: var(--text-dim);
  display: inline-block;
  text-align: center;
  user-select: none;
}

.ast-tag {
  font-size: 9px;
  font-weight: 700;
  padding: 1px 5px;
  border-radius: 2px;
  text-transform: uppercase;
  font-family: var(--font-mono);
}

.tag-stmt { background: #EEF2FF; color: #4338CA; border: 1px solid rgba(67, 56, 202, 0.2); }
.tag-call { background: var(--accent-orange-subtle); color: var(--accent-orange); border: 1px solid var(--accent-orange-border); }
.tag-id { background: #F0FDF4; color: #15803D; border: 1px solid rgba(21, 128, 61, 0.2); }
.tag-lit { background: #FFFBEB; color: #B45309; border: 1px solid rgba(180, 83, 9, 0.2); }
.tag-op { background: #F5F3FF; color: #7C3AED; border: 1px solid rgba(124, 58, 237, 0.2); }
.tag-err { background: var(--accent-rose-subtle); color: var(--accent-rose); border: 1px solid rgba(220, 38, 38, 0.2); }

.ast-node-kind {
  color: var(--text-main);
  font-weight: 600;
}

.ast-node-text {
  color: var(--text-muted);
}

.ast-node-span {
  font-size: 10px;
  color: var(--text-dim);
  background: var(--bg-subtle);
  padding: 1px 5px;
  border-radius: 2px;
  border: 1px solid var(--border);
  margin-left: 4px;
}

.ast-node-children {
  margin-left: 18px;
  padding-left: 8px;
  border-left: 1px dashed var(--border);
}

.ast-node-children.collapsed {
  display: none;
}

.ast-match-highlight {
  background: #fef08a !important;
  color: #854d0e !important;
  border-radius: 2px;
  padding: 0 2px;
}
'''

with open(target_path, 'r', encoding='utf-8') as f:
    orig = f.read()

# Replace CSS_STYLES
pattern_css = r'pub const CSS_STYLES: &str = r###".*?"###;'
assert re.search(pattern_css, orig, flags=re.DOTALL) is not None, "Could not find CSS_STYLES block!"
updated = re.sub(pattern_css, f'pub const CSS_STYLES: &str = r###"{css_content}"###;', orig, flags=re.DOTALL)

# Now check HTML_BODY_MARKUP header lines
old_header_markup = '''        <!-- Middle / Right: Status indicator, status meta, metrics & actions -->
        <div class="header-actions-row">
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
            <div class="stat-item" title="Analyzed R Statements"><span class="stat-lbl">Stmts:</span> <span class="stat-val" id="stat-stmts">0</span></div>
            <div class="stat-item" title="Tracked Column Versions"><span class="stat-lbl">Cols:</span> <span class="stat-val" id="stat-cols">0</span></div>
            <div class="stat-item" title="Unified PDG Nodes"><span class="stat-lbl">PDG:</span> <span class="stat-val" id="stat-nodes">0</span></div>
          </div>

          <a href="/landing" class="btn btn-secondary" title="View The Investigation landing narrative" style="text-decoration:none; display:inline-flex; align-items:center; gap:4px; font-weight:600; color:#FF5A1F; font-size:11px;"><span>Investigation ↗</span></a>
          <button class="btn btn-secondary" id="btn-toggle-evidence" title="Toggle Evidence & Diagnostics Drawer">Evidence</button>
          <button class="btn btn-primary" id="btn-new-analysis" title="Return to code entry screen">New Analysis</button>
        </div>'''

new_header_markup = '''        <!-- Middle: Quiet Status & Plain Metrics (NO Individual Boxed Badges) -->
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

        <!-- Right: Primary Actions (Investigation, Evidence, New Analysis) -->
        <div class="header-actions-row">
          <a href="/landing" class="btn-editorial-link" title="View The Investigation landing narrative"><span>Investigation ↗</span></a>
          <button class="btn btn-secondary" id="btn-toggle-evidence" title="Toggle Evidence & Diagnostics Drawer">Evidence</button>
          <button class="btn btn-primary" id="btn-new-analysis" title="Return to code entry screen">New Analysis</button>
        </div>'''

if old_header_markup in updated:
    updated = updated.replace(old_header_markup, new_header_markup)

# Write back
with open(target_path, 'w', encoding='utf-8') as f:
    f.write(updated)

print("Updated format_html_assets.rs with production-grade editorial redesign!")
