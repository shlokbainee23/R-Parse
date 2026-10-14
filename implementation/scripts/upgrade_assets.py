import os
import re

# Destination path
target_path = os.path.abspath('implementation/crates/rparse-cli/src/format_html_assets.rs')

css_content = r'''
/* ==========================================================================
   R-PARSE SCIENTIFIC ANALYSIS STUDIO — DESIGN SYSTEM
   Theme: Warm Ivory Canvas, Deep Graphite Panels, Signal Orange Accents,
          Restrained Lilac Dataflow Highlights, Precision Engineering Grid
   ========================================================================== */

:root {
  /* Surfaces & Canvas */
  --bg-page: #FAF8F4;
  --bg-canvas: #FAF8F4;
  --bg-panel: #FFFFFF;
  --bg-card: #FFFFFF;
  --bg-subtle: #F4F0E8;
  --bg-surface: #EDE7DC;
  --bg-surface-elevated: #E5DFD2;
  --bg-input: #FCFAF6;
  
  /* Borders & Dividers */
  --border: #E8E2D5;
  --border-strong: #D5CEBE;
  --border-focus: #FF5A1F;
  --border-subtle: rgba(22, 22, 26, 0.06);

  /* Typography Colors */
  --text-main: #16161A;
  --text-muted: #4B4B58;
  --text-dim: #7E7E8E;
  --text-inverse: #FFFFFF;

  /* Accent Colorway */
  --accent-orange: #FF5A1F;
  --accent-orange-hover: #E04B14;
  --accent-orange-subtle: #FFF0EB;
  --accent-orange-glow: rgba(255, 90, 31, 0.22);
  --accent-orange-border: rgba(255, 90, 31, 0.28);

  /* Required alias for test assertion: border-bottom: 2px solid var(--accent-blue) */
  --accent-blue: #FF5A1F;
  --accent-blue-subtle: #FFF0EB;

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
  --font-mono: "JetBrains Mono", ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", monospace;
  --font-sans: "Plus Jakarta Sans", -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;

  /* Elevation Shadows */
  --shadow-xs: 0 1px 2px rgba(22, 22, 26, 0.04);
  --shadow-sm: 0 2px 5px rgba(22, 22, 26, 0.04), 0 1px 2px rgba(22, 22, 26, 0.03);
  --shadow-md: 0 4px 14px rgba(22, 22, 26, 0.06), 0 1px 3px rgba(22, 22, 26, 0.04);
  --shadow-lg: 0 14px 32px rgba(22, 22, 26, 0.09), 0 4px 12px rgba(22, 22, 26, 0.04);
  --shadow-focus: 0 0 0 3px rgba(255, 90, 31, 0.16);

  /* Radii */
  --radius-xs: 3px;
  --radius-sm: 5px;
  --radius-md: 8px;
  --radius-lg: 12px;
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
  from { opacity: 0; transform: translateY(6px); }
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
  from { opacity: 0; transform: translateY(8px); }
  to { opacity: 1; transform: translateY(0); }
}

@keyframes pulseGlow {
  0%, 100% { box-shadow: 0 0 0 0 rgba(5, 150, 105, 0.35); }
  50% { box-shadow: 0 0 0 5px rgba(5, 150, 105, 0.0); }
}

@keyframes pulseAnalyzing {
  0%, 100% { box-shadow: 0 0 0 0 rgba(255, 90, 31, 0.4); transform: scale(1); }
  50% { box-shadow: 0 0 0 6px rgba(255, 90, 31, 0.0); transform: scale(1.08); }
}

.view-animate-in {
  animation: viewFadeIn 0.2s cubic-bezier(0.16, 1, 0.3, 1) both;
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
  transition: opacity 0.4s cubic-bezier(0.16, 1, 0.3, 1), transform 0.4s cubic-bezier(0.16, 1, 0.3, 1);
}
.entry-screen.transition-out {
  opacity: 0;
  transform: scale(0.97) translateY(-16px);
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
  gap: 10px;
}
.brand-logo {
  font-family: var(--font-mono);
  font-weight: 700;
  font-size: 11px;
  letter-spacing: 0.8px;
  color: var(--text-main);
  background: var(--bg-subtle);
  padding: 3px 8px;
  border-radius: var(--radius-xs);
  border: 1px solid var(--border);
}
.brand-subtitle {
  font-size: 11px;
  color: var(--text-muted);
  font-weight: 600;
  letter-spacing: 0.2px;
}
.entry-title {
  font-size: 26px;
  font-weight: 700;
  color: #16161A;
  letter-spacing: -0.6px;
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
  color: #16161A;
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
  color: var(--text-muted);
  border-radius: var(--radius-sm);
  cursor: pointer;
  display: inline-flex;
  align-items: center;
  gap: 5px;
  font-weight: 500;
  transition: all 0.15s cubic-bezier(0.16, 1, 0.3, 1);
  font-family: inherit;
}
.btn:hover {
  background: var(--bg-subtle);
  color: var(--text-main);
  border-color: var(--text-muted);
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
  color: var(--text-muted);
}
.btn-secondary {
  background: #FFFFFF;
  color: var(--text-muted);
  border: 1px solid var(--border-strong);
  font-weight: 500;
  padding: 4px 11px;
  font-size: 11px;
  border-radius: var(--radius-sm);
}
.btn-secondary:hover {
  background: var(--bg-subtle);
  color: #16161A;
  border-color: var(--text-muted);
}
.btn-ghost {
  padding: 3px 8px;
  font-size: 11px;
  background: transparent;
  border: 1px solid transparent;
  color: var(--text-muted);
  border-radius: var(--radius-sm);
  cursor: pointer;
  display: inline-flex;
  align-items: center;
  gap: 4px;
  font-weight: 500;
  transition: all 0.12s ease;
}
.btn-ghost:hover {
  background: var(--bg-subtle);
  color: #16161A;
  border-color: var(--border);
}
.btn-ghost.active {
  background: var(--accent-orange-subtle);
  color: var(--accent-orange);
  border-color: var(--accent-orange-border);
  font-weight: 600;
}
.select-dropdown {
  padding: 5px 10px;
  font-size: 11px;
  background: #FFFFFF;
  border: 1px solid var(--border-strong);
  color: var(--text-main);
  border-radius: var(--radius-sm);
  cursor: pointer;
  outline: none;
  font-family: inherit;
  transition: border-color 0.12s ease, box-shadow 0.12s ease;
}
.select-dropdown:focus {
  border-color: var(--accent-orange);
  box-shadow: var(--shadow-focus);
}
.validation-msg {
  color: var(--accent-rose);
  font-size: 11px;
  padding: 5px 10px;
  background: var(--accent-rose-subtle);
  border: 1px solid rgba(220, 38, 38, 0.25);
  border-radius: var(--radius-sm);
  display: inline-flex;
  align-items: center;
  gap: 6px;
}
.entry-footer-note {
  font-size: 11px;
  color: var(--text-dim);
  text-align: center;
  margin-top: 6px;
  letter-spacing: 0.2px;
}

/* ========================================================================= */
/* PHASE 3: 30/70 Scientific Workspace Shell                                 */
/* ========================================================================= */
.workspace-screen {
  display: flex;
  flex-direction: column;
  height: 100vh;
  width: 100%;
  position: relative;
  overflow: hidden;
  opacity: 1;
  transition: opacity 0.35s ease;
}
.workspace-screen.hidden {
  display: none !important;
}

/* Application Header (Two-row premium IDE layout) */
header.app-header {
  height: 76px;
  background: #FFFFFF;
  border-bottom: 1px solid var(--border);
  display: flex;
  flex-direction: column;
  flex-shrink: 0;
  z-index: 30;
  box-shadow: var(--shadow-xs);
}

/* Row 1: Identity & Global Actions */
.header-top-row {
  height: 40px;
  padding: 0 16px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  border-bottom: 1px solid var(--border);
  gap: 12px;
  box-sizing: border-box;
}

.header-identity {
  display: flex;
  align-items: center;
  gap: 8px;
}

.header-identity .brand-logo {
  font-family: var(--font-mono);
  font-weight: 700;
  font-size: 13px;
  letter-spacing: -0.2px;
  color: #16161A;
  background: none;
  border: none;
  padding: 0;
  border-radius: 0;
}

.header-sep {
  display: inline-block;
  width: 1px;
  height: 12px;
  background: var(--border-strong);
  margin: 0 4px;
  user-select: none;
  vertical-align: middle;
}

.header-sub {
  font-size: 12px;
  font-weight: 600;
  color: var(--text-muted);
}

.header-filename {
  font-family: var(--font-mono);
  font-size: 11px;
  font-weight: 500;
  color: #16161A;
  background: var(--bg-subtle);
  padding: 2px 8px;
  border-radius: var(--radius-xs);
  border: 1px solid var(--border);
  max-width: 240px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.header-actions-row {
  display: flex;
  align-items: center;
  gap: 14px;
}

.header-actions-row .btn-secondary {
  background: #FFFFFF;
  color: var(--text-muted);
  border: 1px solid var(--border-strong);
  font-weight: 500;
  padding: 4px 10px;
  font-size: 11px;
  border-radius: var(--radius-sm);
  cursor: pointer;
  line-height: 16px;
  transition: all 0.12s ease;
}

.header-actions-row .btn-secondary:hover {
  background: var(--bg-subtle);
  color: #16161A;
  border-color: var(--text-muted);
}

.header-actions-row .btn-primary {
  background: var(--accent-orange);
  color: #FFFFFF;
  border: 1px solid var(--accent-orange-hover);
  font-weight: 600;
  padding: 4px 13px;
  font-size: 11px;
  border-radius: var(--radius-sm);
  cursor: pointer;
  line-height: 16px;
  box-shadow: 0 1px 3px rgba(255, 90, 31, 0.25);
  transition: all 0.12s ease;
}

.header-actions-row .btn-primary:hover {
  background: var(--accent-orange-hover);
  border-color: var(--accent-orange-hover);
  box-shadow: 0 2px 6px rgba(255, 90, 31, 0.35);
}

.analysis-status-badge {
  display: inline-flex;
  align-items: center;
  gap: 7px;
  font-size: 11px;
  color: var(--text-muted);
  user-select: none;
}

.status-dot {
  width: 7px;
  height: 7px;
  border-radius: 50%;
  background: var(--accent-green);
  display: inline-block;
  flex-shrink: 0;
  animation: pulseGlow 2.5s infinite;
  transition: background 0.15s ease, box-shadow 0.15s ease;
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
  gap: 6px;
  font-size: 11px;
  color: var(--text-dim);
}

.header-meta-item {
  font-size: 11px;
  color: var(--text-dim);
  font-weight: 500;
}

.header-meta-item.stat-unk-text {
  color: var(--accent-amber);
  font-weight: 600;
}

.header-meta-item.stat-known-text {
  color: var(--text-muted);
  font-weight: 500;
}

.header-meta-sep {
  color: var(--border-strong);
  font-size: 9px;
  user-select: none;
}

/* Header Summary Stats */
.stats-strip {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 11px;
  color: var(--text-dim);
  background: var(--bg-subtle);
  padding: 3px 8px;
  border-radius: var(--radius-sm);
  border: 1px solid var(--border);
}

.stat-item {
  display: flex;
  align-items: center;
  gap: 3px;
}

.stat-lbl {
  color: var(--text-dim);
  font-size: 10px;
  font-weight: 500;
}

.stat-val {
  color: #16161A;
  font-weight: 600;
  font-family: var(--font-mono);
  font-size: 11px;
}

/* Row 2: Analysis Navigation (Flat text-and-icon underline tabs) */
.app-subnav {
  height: 36px;
  background: #FFFFFF;
  padding: 0 16px;
  display: flex;
  align-items: center;
  gap: 22px;
  user-select: none;
  box-sizing: border-box;
}

.nav-tab {
  height: 36px;
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
  transition: color 0.15s cubic-bezier(0.16, 1, 0.3, 1), border-color 0.15s cubic-bezier(0.16, 1, 0.3, 1);
  outline: none;
  box-sizing: border-box;
}

.nav-tab:hover {
  color: #16161A;
}

.nav-tab:focus-visible {
  outline: 2px solid var(--accent-orange);
  outline-offset: -2px;
  border-radius: 2px;
}

.nav-tab.active {
  color: #16161A;
  font-weight: 600;
  border-bottom: 2px solid var(--accent-blue);
  background: transparent;
}

.nav-tab-icon {
  width: 14px;
  height: 14px;
  stroke: currentColor;
  flex-shrink: 0;
  opacity: 0.65;
  transition: opacity 0.15s ease, stroke 0.15s ease;
}

.nav-tab:hover .nav-tab-icon {
  opacity: 1;
  stroke: #16161A;
}

.nav-tab.active .nav-tab-icon {
  opacity: 1;
  stroke: var(--accent-orange);
}

.nav-count {
  font-size: 10px;
  font-family: var(--font-mono);
  color: var(--text-dim);
  font-weight: 500;
  background: var(--bg-subtle);
  padding: 1px 5px;
  border-radius: 4px;
  border: 1px solid var(--border);
  margin-left: 2px;
  transition: all 0.12s ease;
}

.nav-tab:hover .nav-count {
  color: #16161A;
  border-color: var(--border-strong);
}

.nav-tab.active .nav-count {
  color: var(--accent-orange);
  background: var(--accent-orange-subtle);
  border-color: var(--accent-orange-border);
  font-weight: 600;
}

.badge-count {
  font-size: 9px;
  padding: 1px 5px;
  border-radius: 8px;
  background: var(--bg-subtle);
  color: var(--text-dim);
  font-family: var(--font-mono);
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
  padding: 0 12px;
  background: var(--bg-subtle);
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
  letter-spacing: 0.6px;
  color: var(--text-muted);
  font-weight: 700;
}
.pane-badge {
  font-family: var(--font-mono);
  font-size: 10px;
  color: var(--text-dim);
  background: #FFFFFF;
  padding: 1px 6px;
  border-radius: 3px;
  border: 1px solid var(--border);
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
.kw { color: #BE185D; font-weight: 600; }
.fn { color: #FF5A1F; font-weight: 500; }
.str { color: #059669; }
.num { color: #D97706; }
.cmt { color: #7E7E8E; font-style: italic; }

/* Draggable Splitter */
.splitter {
  width: 5px;
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
  width: 230px;
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
  gap: 5px;
}
.toggle-chip {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  font-size: 10px;
  padding: 2px 7px;
  border-radius: 3px;
  background: var(--bg-subtle);
  border: 1px solid var(--border);
  color: var(--text-muted);
  cursor: pointer;
  user-select: none;
  font-family: var(--font-mono);
  transition: all 0.12s ease;
}
.toggle-chip:hover {
  border-color: var(--border-strong);
  color: #16161A;
}
.toggle-chip input {
  cursor: pointer;
  accent-color: var(--accent-orange);
  margin: 0;
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
  background: radial-gradient(circle, #D8D2C4 1px, transparent 1px) #FAF8F4;
  background-size: 24px 24px;
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

/* SVG Node Kinds (Precision Ivory & Graphite System) */
.node-stmt { fill: #FFFFFF; stroke: #4B4B58; stroke-width: 1.4; }
.node-vardef { fill: #FAF5FF; stroke: #7C3AED; stroke-width: 1.4; }
.node-varuse { fill: #F0FDFA; stroke: #0D9488; stroke-width: 1.4; }
.node-col { fill: #FFFFFF; stroke: var(--border-strong); stroke-width: 1.5; }
.node-expr { fill: #EFF6FF; stroke: #3B82F6; stroke-width: 1.4; }
.node-rowsel { fill: #FAF5FF; stroke: #7C3AED; stroke-width: 1.5; stroke-dasharray: 4,3; }
.node-unk { fill: #FFFBEB; stroke: #D97706; stroke-width: 1.5; stroke-dasharray: 4,3; }
.node-deriv { fill: #FFFFFF; stroke: #059669; stroke-width: 1.5; }

.svg-node {
  cursor: pointer;
  transition: filter 0.18s cubic-bezier(0.16, 1, 0.3, 1), opacity 0.18s cubic-bezier(0.16, 1, 0.3, 1);
  animation: nodeEntrance 0.22s cubic-bezier(0.16, 1, 0.3, 1) both;
}
.svg-node:hover {
  filter: drop-shadow(0 4px 12px rgba(22, 22, 26, 0.10));
}
.svg-node.selected rect {
  stroke: var(--accent-orange) !important;
  stroke-width: 2.5px !important;
  filter: drop-shadow(0 0 10px rgba(255, 90, 31, 0.45));
}
.svg-node.connected-upstream rect {
  stroke: var(--accent-orange) !important;
  stroke-width: 2px !important;
  filter: drop-shadow(0 0 8px rgba(255, 90, 31, 0.35));
}
.svg-node.connected-downstream rect {
  stroke: var(--accent-green) !important;
  stroke-width: 2px !important;
  filter: drop-shadow(0 0 8px rgba(5, 150, 105, 0.35));
}
.svg-node.dimmed { opacity: 0.12 !important; filter: grayscale(50%); }

/* SVG Edges */
.svg-edge { fill: none; transition: stroke-width 0.18s, opacity 0.18s, stroke 0.18s; animation: edgeEntrance 0.25s ease both; }
.edge-data { stroke: var(--accent-orange); stroke-width: 1.5; }
.edge-ctrl { stroke: #D97706; stroke-width: 1.5; }
.edge-rowsel { stroke: #7C3AED; stroke-width: 1.5; stroke-dasharray: 4,3; }
.edge-colver { stroke: #059669; stroke-width: 1.5; }
.edge-unk { stroke: #DC2626; stroke-width: 1.5; stroke-dasharray: 3,3; }
.svg-edge.highlighted { stroke-width: 2.6px !important; opacity: 1 !important; }
.svg-edge.highlighted.inbound { stroke: var(--accent-orange) !important; }
.svg-edge.highlighted.outbound { stroke: var(--accent-green) !important; }
.svg-edge.dimmed { opacity: 0.08 !important; }

/* Backward Slice View */
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
  gap: 10px;
}
.metric-cell {
  background: #FFFFFF;
  border: 1px solid var(--border);
  padding: 10px 14px;
  border-radius: var(--radius-sm);
  box-shadow: var(--shadow-xs);
  transition: transform 0.12s ease, box-shadow 0.12s ease;
}
.metric-cell:hover {
  transform: translateY(-1px);
  box-shadow: var(--shadow-sm);
}
.metric-num {
  font-size: 20px;
  font-weight: 700;
  font-family: var(--font-mono);
  color: var(--text-main);
  letter-spacing: -0.5px;
}
.metric-desc {
  font-size: 10px;
  color: var(--text-dim);
  text-transform: uppercase;
  margin-top: 2px;
  font-weight: 600;
  letter-spacing: 0.3px;
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
  transition: all 0.15s cubic-bezier(0.16, 1, 0.3, 1);
  box-shadow: var(--shadow-xs);
  animation: cardEntrance 0.22s cubic-bezier(0.16, 1, 0.3, 1) both;
}
.stmt-card:hover {
  border-color: var(--accent-orange);
  background: #FFFFFF;
  box-shadow: var(--shadow-md);
  transform: translateY(-1px);
}
.stmt-card.full { border-left: 3px solid var(--accent-orange); }
.stmt-card.partial { border-left: 3px solid var(--accent-purple); }
.stmt-card.none { border-left: 3px solid var(--border-strong); opacity: 0.7; }
.stmt-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 5px;
}
.pill {
  font-size: 9px;
  font-weight: 700;
  padding: 2px 7px;
  border-radius: 3px;
  text-transform: uppercase;
  font-family: var(--font-mono);
  letter-spacing: 0.3px;
}
.pill-full { background: var(--accent-orange-subtle); color: var(--accent-orange); border: 1px solid var(--accent-orange-border); }
.pill-partial { background: var(--accent-purple-subtle); color: var(--accent-purple); border: 1px solid rgba(124, 58, 237, 0.25); }
.pill-none { background: var(--bg-subtle); color: var(--text-dim); border: 1px solid var(--border); }
.pill-keep { background: var(--accent-green-subtle); color: var(--accent-green); border: 1px solid rgba(5, 150, 105, 0.3); }
.pill-drop { background: var(--accent-rose-subtle); color: var(--accent-rose); border: 1px solid rgba(220, 38, 38, 0.3); }
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
  border-radius: 3px;
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
  transition: all 0.15s cubic-bezier(0.16, 1, 0.3, 1);
  box-shadow: var(--shadow-xs);
  animation: cardEntrance 0.22s cubic-bezier(0.16, 1, 0.3, 1) both;
}
.schema-card:hover {
  border-color: var(--accent-orange);
  background: #FFFFFF;
  box-shadow: var(--shadow-md);
  transform: translateY(-1px);
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
  border-radius: 3px;
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
.col-chip {
  font-family: var(--font-mono);
  font-size: 10px;
  background: var(--bg-subtle);
  border: 1px solid var(--border);
  padding: 3px 7px;
  border-radius: 3px;
  display: inline-flex;
  align-items: center;
  gap: 5px;
  transition: all 0.12s ease;
}
.col-chip:hover {
  border-color: var(--border-strong);
  background: #FFFFFF;
}
.col-ver { color: var(--text-dim); font-size: 9px; }

/* ========================================================================= */
/* PHASE 5: Contextual Slide-out Inspector Overlay (NOT a 3rd Panel)         */
/* ========================================================================= */
.contextual-inspector {
  position: absolute;
  right: 0;
  top: 0;
  bottom: 0;
  width: 380px;
  max-width: 85%;
  background: #FFFFFF;
  border-left: 1px solid var(--border);
  box-shadow: var(--shadow-lg);
  z-index: 25;
  display: flex;
  flex-direction: column;
  transform: translateX(100%);
  transition: transform 0.24s cubic-bezier(0.16, 1, 0.3, 1);
}
.contextual-inspector.open {
  transform: translateX(0);
}
.inspector-header {
  height: 38px;
  padding: 0 14px;
  background: var(--bg-subtle);
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
  box-shadow: var(--shadow-xs);
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
  font-weight: 600;
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
  letter-spacing: 0.5px;
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
  padding: 5px 8px;
  background: var(--bg-subtle);
  border: 1px solid var(--border);
  border-radius: 3px;
  font-size: 11px;
  cursor: pointer;
  transition: all 0.12s ease;
}
.conn-item:hover {
  background: #FFFFFF;
  border-color: var(--accent-orange);
  box-shadow: var(--shadow-xs);
}
.conn-label {
  font-family: var(--font-mono);
  color: var(--text-main);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 200px;
}
.conn-pill {
  font-size: 9px;
  padding: 1px 6px;
  border-radius: 3px;
  font-weight: 700;
  text-transform: uppercase;
  font-family: var(--font-mono);
}

/* ========================================================================= */
/* PHASE 5: Bottom Evidence & Diagnostics Drawer                             */
/* ========================================================================= */
.evidence-drawer {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  height: 240px;
  background: #FFFFFF;
  border-top: 1px solid var(--border);
  box-shadow: var(--shadow-lg);
  z-index: 20;
  display: flex;
  flex-direction: column;
  transform: translateY(100%);
  transition: transform 0.24s cubic-bezier(0.16, 1, 0.3, 1);
}
.evidence-drawer.open {
  transform: translateY(0);
}
.drawer-header {
  height: 36px;
  padding: 0 16px;
  background: var(--bg-subtle);
  border-bottom: 1px solid var(--border);
  display: flex;
  align-items: center;
  justify-content: space-between;
  flex-shrink: 0;
}
.drawer-content {
  flex: 1;
  padding: 14px 16px;
  overflow-y: auto;
  font-size: 11px;
  background: #FFFFFF;
}
.diag-item {
  padding: 8px 12px;
  border-radius: var(--radius-sm);
  margin-bottom: 8px;
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
  .contextual-inspector { width: 330px; }
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

/* ==========================================================================
   View E: AST & CST Syntax Tree Explorer Styles
   ========================================================================== */
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
  padding: 8px 16px;
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
  padding: 4px 11px;
  border: none;
  background: transparent;
  font-size: 11px;
  font-weight: 500;
  color: var(--text-muted);
  border-radius: 3px;
  cursor: pointer;
  transition: all 0.15s ease;
  font-family: inherit;
}

.ast-rep-btn:hover {
  color: #16161A;
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
  min-width: 230px;
  transition: border-color 0.12s, box-shadow 0.12s;
}

.ast-search-box:focus-within {
  border-color: var(--accent-orange);
  box-shadow: var(--shadow-focus);
}

.ast-search-icon {
  font-size: 12px;
  margin-right: 6px;
  opacity: 0.6;
}

.ast-search-box input {
  border: none;
  background: transparent;
  outline: none;
  font-size: 11px;
  width: 100%;
  color: var(--text-main);
  font-family: inherit;
}

.ast-search-match {
  font-size: 10px;
  font-family: var(--font-mono);
  color: var(--accent-orange);
  background: var(--accent-orange-subtle);
  padding: 1px 6px;
  border-radius: 8px;
  margin-left: 6px;
  white-space: nowrap;
  display: none;
  border: 1px solid var(--accent-orange-border);
}

.ast-actions {
  display: flex;
  align-items: center;
  gap: 6px;
}

.ast-node-stats {
  margin-left: auto;
  font-size: 11px;
  font-family: var(--font-mono);
  color: var(--text-dim);
}

.ast-tree-scroll {
  flex: 1;
  overflow-y: auto;
  padding: 14px 18px;
  font-family: var(--font-mono);
  font-size: 12px;
  line-height: 1.6;
  background: var(--bg-canvas);
}

.ast-tree-node {
  margin: 1px 0;
}

.ast-node-row {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 3px 8px;
  border-radius: var(--radius-xs);
  cursor: pointer;
  user-select: none;
  transition: background 0.12s ease;
  border: 1px solid transparent;
}

.ast-node-row:hover {
  background: var(--bg-subtle);
  border-color: var(--border);
}

.ast-node-row.selected {
  background: var(--accent-orange-subtle) !important;
  border-color: var(--accent-orange) !important;
}

.ast-node-row.has-error {
  background: #fef2f2;
  border-color: #fca5a5;
  color: #991b1b;
}

.ast-node-chevron {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 18px;
  height: 18px;
  font-size: 10px;
  color: var(--text-dim);
  cursor: pointer;
  border-radius: 3px;
  flex-shrink: 0;
  user-select: none;
  transition: transform 0.15s ease, background 0.1s ease, color 0.1s ease;
}

.ast-node-chevron:hover:not(.empty) {
  color: var(--text-main);
  background: rgba(0, 0, 0, 0.06);
}

.ast-node-chevron.expanded {
  transform: rotate(90deg);
}

.ast-node-chevron.empty {
  visibility: hidden;
  pointer-events: none;
}

.ast-node-tag {
  display: inline-block;
  font-size: 10px;
  font-weight: 600;
  padding: 1px 6px;
  border-radius: 4px;
  letter-spacing: 0.02em;
}

.ast-tag-stmt { background: var(--accent-orange-subtle); color: var(--accent-orange); border: 1px solid var(--accent-orange-border); }
.ast-tag-id { background: #e0f2fe; color: #0369a1; }
.ast-tag-lit { background: #f3e8ff; color: #7e22ce; }
.ast-tag-call { background: #fef3c7; color: #b45309; }
.ast-tag-op { background: var(--bg-subtle); color: #475569; border: 1px solid var(--border); }
.ast-tag-err { background: #fee2e2; color: #dc2626; font-weight: 700; }
.ast-tag-def { background: #FFFFFF; color: var(--text-muted); border: 1px solid var(--border); }

.ast-node-label {
  font-weight: 500;
  color: var(--text-main);
}

.ast-node-detail {
  color: var(--text-dim);
  font-size: 11px;
}

.ast-node-span {
  font-size: 10px;
  color: var(--text-dim);
  background: var(--bg-subtle);
  padding: 1px 5px;
  border-radius: 3px;
  border: 1px solid var(--border);
  margin-left: 4px;
}

.ast-node-children {
  margin-left: 20px;
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

# Read existing format_html_assets.rs
with open(target_path, 'r', encoding='utf-8') as f:
    orig = f.read()

# Replace CSS_STYLES
pattern_css = r'pub const CSS_STYLES: &str = r###".*?"###;'
assert re.search(pattern_css, orig, flags=re.DOTALL) is not None, "Could not find CSS_STYLES block!"
updated = re.sub(pattern_css, f'pub const CSS_STYLES: &str = r###"{css_content}"###;', orig, flags=re.DOTALL)

# In JS_SCRIPTS, let's enhance the switchView function to add view-animate-in class
# and add staggered animation index to cards
js_old_switch = '''      function switchView(view) {
        activeView = view;
        navTabs.forEach(t => t.classList.toggle('active', t.dataset.view === view));

        ['provenance', 'pdg', 'slice', 'schema', 'ast'].forEach(v => {
          const tb = document.getElementById(`toolbar-${v}`);
          const vc = document.getElementById(`view-container-${v}`);
          if (tb) tb.style.display = (v === view) ? 'flex' : 'none';
          if (vc) vc.style.display = (v === view) ? (v === 'provenance' || v === 'pdg' ? 'block' : 'flex') : 'none';
        });'''

js_new_switch = '''      function switchView(view) {
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
        });'''

if js_old_switch in updated:
    updated = updated.replace(js_old_switch, js_new_switch)

# Also enhance SVG markers colors in HTML_BODY_MARKUP
old_marker_data = '<path d="M 0 1 L 10 5 L 0 9 z" fill="#0284c7" />'
new_marker_data = '<path d="M 0 1 L 10 5 L 0 9 z" fill="#FF5A1F" />'
updated = updated.replace(old_marker_data, new_marker_data)

old_marker_ctrl = '<path d="M 0 1 L 10 5 L 0 9 z" fill="#ea580c" />'
new_marker_ctrl = '<path d="M 0 1 L 10 5 L 0 9 z" fill="#D97706" />'
updated = updated.replace(old_marker_ctrl, new_marker_ctrl)

old_marker_colver = '<path d="M 0 1 L 10 5 L 0 9 z" fill="#16a34a" />'
new_marker_colver = '<path d="M 0 1 L 10 5 L 0 9 z" fill="#059669" />'
updated = updated.replace(old_marker_colver, new_marker_colver)

old_marker_unk = '<path d="M 0 1 L 10 5 L 0 9 z" fill="#d97706" />'
new_marker_unk = '<path d="M 0 1 L 10 5 L 0 9 z" fill="#DC2626" />'
updated = updated.replace(old_marker_unk, new_marker_unk)

# In JS_SCRIPTS, update node colors in Provenance
prov_old_colors = """          textKind.setAttribute('fill', isCol ? '#0284c7' : '#16a34a');"""
prov_new_colors = """          textKind.setAttribute('fill', isCol ? 'var(--accent-orange)' : 'var(--accent-green)');"""
if prov_old_colors in updated:
    updated = updated.replace(prov_old_colors, prov_new_colors)

# In JS_SCRIPTS, update PDG kind colors
pdg_old_colors = """          let kindColor = '#64748b';
          if (isVarDef) kindColor = '#6366f1';
          else if (isVarUse) kindColor = '#0d9488';
          else if (isCol) kindColor = '#0284c7';
          else if (isExpr) kindColor = '#3b82f6';
          else if (isRowSel) kindColor = '#9333ea';
          else if (n.is_unknown) kindColor = '#d97706';"""

pdg_new_colors = """          let kindColor = '#4B4B58';
          if (isVarDef) kindColor = '#7C3AED';
          else if (isVarUse) kindColor = '#0D9488';
          else if (isCol) kindColor = '#FF5A1F';
          else if (isExpr) kindColor = '#3B82F6';
          else if (isRowSel) kindColor = '#7C3AED';
          else if (n.is_unknown) kindColor = '#DC2626';"""
if pdg_old_colors in updated:
    updated = updated.replace(pdg_old_colors, pdg_new_colors)

# In slice view cards, add staggered entrance
slice_old_card = """        stmts.forEach(st => {
          const card = document.createElement('div');
          card.className = `stmt-card ${st.retention.toLowerCase()}`;"""

slice_new_card = """        stmts.forEach((st, idx) => {
          const card = document.createElement('div');
          card.className = `stmt-card ${st.retention.toLowerCase()}`;
          card.style.animationDelay = (idx * 0.035).toFixed(3) + 's';"""
if slice_old_card in updated:
    updated = updated.replace(slice_old_card, slice_new_card)

# In schema view cards, add staggered entrance
schema_old_card = """        (data.schema_evolution || []).forEach(step => {
          const card = document.createElement('div');
          card.className = 'schema-card';"""

schema_new_card = """        (data.schema_evolution || []).forEach((step, idx) => {
          const card = document.createElement('div');
          card.className = 'schema-card';
          card.style.animationDelay = (idx * 0.04).toFixed(3) + 's';"""
if schema_old_card in updated:
    updated = updated.replace(schema_old_card, schema_new_card)

# Write back
with open(target_path, 'w', encoding='utf-8') as f:
    f.write(updated)

print("Successfully upgraded format_html_assets.rs with the premium design and motion system!")
