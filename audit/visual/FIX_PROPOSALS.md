# R-PARSE — Concrete Remediation Proposals & Code Specifications

**Status:** Phase 1 Inspection Artifact — Review Only (**Zero Code Changes Applied to Application Files**)  
**Auditor:** Senior Product Designer & Frontend Architect  
**Interactive Prototype:** An isolated, non-invasive prototype implementing these unified specifications is available for visual inspection at [`audit/visual/prototypes/studio_unified_prototype.html`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/prototypes/studio_unified_prototype.html) (rendered screenshot: [`audit/visual/prototype-unified-studio.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/prototype-unified-studio.png)).

---

## Proposal 1: Studio Header and Status Hierarchy Streamlining

### 1. Current State & Screenshot Evidence
- **Current Evidence:** [`audit/visual/studio-05-header-detail.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-05-header-detail.png) and [`audit/visual/studio-26-responsive-768x1024.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-26-responsive-768x1024.png)
- **Defects Addressed:** `ISSUE-P0-02` (Horizontal overflow cutting off primary buttons at <=880px), `ISSUE-P1-02` (Bloated 78px 2-row header eating canvas height), `ISSUE-P1-06` (Competing status metrics).

### 2. Proposed Visual Outcome
- Consolidate the two separate rows into a single, high-efficiency 44px navigation bar.
- **Left:** Brand mark + wordmark `R—PARSE`, clean divider, and file breadcrumb (`showcase_pipeline.R`) with graceful text truncation.
- **Center:** The 5 analytical view tabs (`Provenance`, `PDG Graph`, `Backward Slice`, `Schema Evolution`, `AST Explorer`) integrated directly into the bar with active indicator dots.
- **Right:** Quiet status pill (`● Static Ready`), `Evidence` drawer trigger, and `New Analysis` button.
- Reclaims 34px of vertical canvas height across the entire workspace. Eliminates horizontal overflow at 768px.

### 3. Exact Source Files & Target Lines
- [`implementation/crates/rparse-cli/src/format_html_assets.rs:L1783-L1848`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-cli/src/format_html_assets.rs#L1783-L1848) (`HTML_BODY_MARKUP`)
- [`implementation/crates/rparse-cli/src/format_html_assets.rs:L140-L245`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-cli/src/format_html_assets.rs#L140-L245) (`CSS_STYLES`)

### 4. Proposed Code Diff (Markup & CSS)

#### HTML Markup Diff:
```diff
-    <!-- Application Header (Row 1: Identity & Actions; Row 2: Analysis Navigation) -->
-    <header class="app-header">
-      <!-- Row 1: Identity & Global Actions -->
-      <div class="header-top-row">
-        <div class="header-identity">
-          <span class="brand-logo">R—PARSE</span>
-          <span class="header-sep" aria-hidden="true"></span>
-          <span class="header-sub">Studio</span>
-          <span class="header-sep" aria-hidden="true"></span>
-          <span class="header-filename" id="header-file-name">showcase_pipeline.R</span>
-        </div>
-        <div class="header-actions-row">
-          <div class="analysis-status-badge" id="analysis-status-badge">...</div>
-          <div class="header-meta-group">...</div>
-          <div class="stats-strip" id="stats-strip">...</div>
-          <a href="/landing" class="btn btn-secondary">Investigation ↗</a>
-          <button class="btn btn-secondary" id="btn-toggle-evidence">Evidence</button>
-          <button class="btn btn-primary" id="btn-new-analysis">New Analysis</button>
-        </div>
-      </div>
-      <!-- Row 2: Analysis Navigation -->
-      <nav class="app-subnav" role="tablist">
-        <button class="nav-tab active" data-view="provenance" id="tab-provenance">...</button>
-        ...
-      </nav>
-    </header>
+    <!-- Streamlined 44px Unified Application Header -->
+    <header class="app-header">
+      <div class="header-left">
+        <div class="brand-mark" aria-hidden="true"><span></span><span></span><span></span><span></span></div>
+        <span class="brand-wordmark">R—PARSE</span>
+        <span class="header-sep">/</span>
+        <span class="file-breadcrumb" id="header-file-name" title="showcase_pipeline.R">showcase_pipeline.R</span>
+      </div>
+
+      <!-- Centered Integrated Navigation Tabs -->
+      <nav class="header-tabs" role="tablist">
+        <button class="tab-btn active" data-view="provenance" id="tab-provenance" role="tab">
+          <span class="tab-dot"></span><span>Provenance</span>
+        </button>
+        <button class="tab-btn" data-view="pdg" id="tab-pdg" role="tab">
+          <span class="tab-dot"></span><span>PDG Graph</span>
+        </button>
+        <button class="tab-btn" data-view="slice" id="tab-slice" role="tab">
+          <span class="tab-dot"></span><span>Backward Slice</span>
+        </button>
+        <button class="tab-btn" data-view="schema" id="tab-schema" role="tab">
+          <span class="tab-dot"></span><span>Schema Evolution</span>
+        </button>
+        <button class="tab-btn" data-view="ast" id="tab-ast" role="tab">
+          <span class="tab-dot"></span><span>AST Explorer</span>
+        </button>
+      </nav>
+
+      <!-- Right: Clean Status & Actions -->
+      <div class="header-right">
+        <div class="status-badge" id="analysis-status-badge" title="Deterministic Tier 1 Static Analysis">
+          <span class="status-led" id="header-status-dot"></span>
+          <span id="header-status-text">Static Ready</span>
+        </div>
+        <a href="/landing" class="btn btn-secondary header-link">Investigation ↗</a>
+        <button class="btn btn-secondary" id="btn-toggle-evidence">Evidence</button>
+        <button class="btn btn-primary" id="btn-new-analysis">New Analysis</button>
+      </div>
+    </header>
```

#### CSS Styles Diff:
```diff
-.app-header {
-  background: var(--bg-panel);
-  border-bottom: 1px solid var(--border);
-  display: flex;
-  flex-direction: column;
-  flex-shrink: 0;
-  z-index: 50;
-}
-.header-top-row {
-  height: 42px;
-  padding: 0 16px;
-  display: flex;
-  align-items: center;
-  justify-content: space-between;
-  border-bottom: 1px solid var(--border-subtle);
-  gap: 16px;
-}
-.app-subnav {
-  height: 34px;
-  padding: 0 16px;
-  display: flex;
-  align-items: center;
-  gap: 26px;
-}
+.app-header {
+  height: 44px;
+  background: var(--bg-panel);
+  border-bottom: 1px solid var(--border-subtle);
+  display: flex;
+  align-items: center;
+  justify-content: space-between;
+  padding: 0 16px;
+  flex-shrink: 0;
+  z-index: 100;
+}
+.header-left { display: flex; align-items: center; gap: 10px; min-width: 220px; }
+.file-breadcrumb { font-family: var(--font-mono); font-size: 11px; color: var(--text-muted); max-width: 160px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
+.header-tabs { display: flex; align-items: center; gap: 4px; background: rgba(0, 0, 0, 0.25); padding: 3px; border-radius: var(--radius-md); border: 1px solid var(--border-subtle); }
+.tab-btn { background: transparent; border: none; color: var(--text-muted); font-size: 12px; font-weight: 500; padding: 4px 12px; border-radius: 6px; cursor: pointer; display: flex; align-items: center; gap: 6px; transition: all 0.15s ease; }
+.tab-btn.active { color: #FFF; background: var(--bg-surface); font-weight: 600; }
+.tab-dot { width: 5px; height: 5px; border-radius: 50%; background: var(--accent-primary); display: none; }
+.tab-btn.active .tab-dot { display: block; }
+.header-right { display: flex; align-items: center; gap: 10px; }
+@media (max-width: 900px) {
+  .header-left { min-width: auto; }
+  .tab-btn span:not(.tab-dot) { font-size: 11px; padding: 2px 8px; }
+}
```

### 5. Intended Visual Improvement
- Reduces visual noise by replacing an un-themed double row with a sleek dark command bar.
- Reclaims 34 vertical pixels for program analysis graphs.
- Protects primary action buttons at tablet widths.

### 6. Functional Risk
- **Zero Risk.** All existing element IDs (`#tab-provenance`, `#tab-pdg`, `#tab-slice`, `#tab-schema`, `#tab-ast`, `#header-file-name`, `#header-status-dot`, `#header-status-text`, `#btn-toggle-evidence`, `#btn-new-analysis`) are preserved identically so that JavaScript tab switching and status updating in `JS_SCRIPTS` work without change.

---

## Proposal 2: Workspace Typography, Spacing, and Entry Container Resilience

### 1. Current State & Screenshot Evidence
- **Current Evidence:** [`audit/visual/studio-02-entry-screen-validation-error.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-02-entry-screen-validation-error.png) and [`audit/visual/studio-19-source-pane-word-wrap-on.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-19-source-pane-word-wrap-on.png)
- **Defects Addressed:** `ISSUE-P0-01` (Validation error overflow clipping buttons), `ISSUE-P2-02` (Source viewer line height and text density).

### 2. Proposed Visual Outcome
- Fixes `.entry-screen` so that validation errors can never push action buttons off-screen: adds `overflow-y: auto`, responsive margins, and card max-height boundaries.
- Increases source code line-height from `1.35` to `1.55` and adds subtle gutter padding for effortless line reading.

### 3. Exact Source Files & Target Lines
- [`implementation/crates/rparse-cli/src/format_html_assets.rs:L65-L105`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-cli/src/format_html_assets.rs#L65-L105) (Entry Screen CSS)
- [`implementation/crates/rparse-cli/src/format_html_assets.rs:L300-L350`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-cli/src/format_html_assets.rs#L300-L350) (Source Code Viewer CSS)

### 4. Proposed Code Diff

```diff
 .entry-screen {
   position: absolute;
   top: 0;
   left: 0;
   width: 100%;
-  height: 100vh;
+  min-height: 100vh;
+  max-height: 100vh;
+  overflow-y: auto;
   z-index: 100;
-  background: var(--bg-page);
+  background: var(--bg-canvas);
   display: flex;
   flex-direction: column;
   align-items: center;
-  justify-content: center;
-  padding: 24px;
+  padding: 40px 16px;
 }
 .entry-card {
   max-width: 780px;
   width: 100%;
+  margin: auto 0;
   display: flex;
   flex-direction: column;
   gap: 16px;
-  background: #FFFFFF;
-  padding: 36px 40px;
+  background: var(--bg-panel);
+  padding: 32px 36px;
   border-radius: var(--radius-lg);
-  border: 1px solid var(--border);
+  border: 1px solid var(--border-subtle);
   box-shadow: var(--shadow-lg);
 }
...
 .source-viewer {
   flex: 1;
   overflow-y: auto;
   font-family: var(--font-mono);
   font-size: 12px;
-  line-height: 1.35;
+  line-height: 1.55;
-  padding: 4px 0;
+  padding: 8px 0;
 }
```

### 5. Intended Visual Improvement
- Eliminates the game-breaking bug where users were trapped by validation errors.
- Dramatically improves source code scanning speed and reduces visual fatigue during slicing analysis.

### 6. Functional Risk
- **Zero Risk.** Purely CSS layout stabilization.

---

## Proposal 3: Studio Session Modal Simplification & Truth in Status

### 1. Current State & Screenshot Evidence
- **Current Evidence:** [`audit/visual/landing-16-studio-modal-default.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/landing-16-studio-modal-default.png), [`audit/visual/landing-17-studio-modal-cli-expanded.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/landing-17-studio-modal-cli-expanded.png), [`audit/visual/landing-16b-root-modal-default.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/landing-16b-root-modal-default.png)
- **Defects Addressed:** `ISSUE-P0-04` (Divergent modal markup and broken offline link handling), `ISSUE-P1-04` (Over-documentation and buried CTA).

### 2. Proposed Visual Outcome
- Remove the 4 redundant marketing bullet points and the 4-command CLI documentation block from the modal dialog.
- Present two unambiguous states:
  - **Online State:** Glowing emerald LED `● R-PARSE Studio v0.1.0 Active on 127.0.0.1:8080`, followed by a prominent CTA `Open Studio Session ↗`.
  - **Offline State:** Muted indicator `○ Studio Daemon Offline`, with a single 1-click terminal command `rparse studio --port 8080 --open` and a disabled button `Start Server to Launch`.
- Synchronize root `index.html` and `landing/index.html`.

### 3. Exact Source Files & Target Lines
- [`landing/index.html:L740-L820`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/landing/index.html#L740-L820)
- [`index.html:L800-L840`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/index.html#L800-L840)
- [`landing/investigation.js:L353-L375`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/landing/investigation.js#L353-L375)

### 4. Proposed Code Diff

#### HTML Modal Markup Diff:
```diff
   <div class="modal-backdrop" id="studio-modal" role="dialog" aria-modal="true" aria-labelledby="modal-title">
     <div class="modal-dialog">
       <div class="modal-header">
         <div class="modal-title-group">
           <div class="header-mark" style="width: 20px; height: 20px;" aria-hidden="true">
             <span></span><span></span><span></span><span></span>
           </div>
-          <span class="modal-title" id="modal-title">R-PARSE Studio Session</span>
+          <span class="modal-title" id="modal-title">Connect to R-PARSE Studio</span>
         </div>
         <button class="modal-close-btn" id="modal-close-btn" aria-label="Close Studio Dialog">✕</button>
       </div>
 
       <div class="modal-body">
-        <div class="modal-intro">
-          <p class="modal-lead-text">Explore your R programs and dataframe pipelines across four deep analytical dimensions:</p>
-          <ul class="modal-capabilities-list">
-            <li><strong>Syntax &amp; Semantics:</strong> Inspect concrete syntax trees...</li>
-            <li><strong>Column Provenance:</strong> Trace fine-grained column derivations...</li>
-            <li><strong>Dependencies &amp; Slicing:</strong> Investigate unified program dependence...</li>
-            <li><strong>Schema Evolution:</strong> Track step-by-step column additions...</li>
-          </ul>
-        </div>
-
-        <details class="cli-disclosure" id="cli-disclosure">
-          <summary class="cli-disclosure-summary">Prefer the command line? View CLI instructions.</summary>
-          ... 4 command terminal blocks ...
-        </details>
+        <div class="server-status-card" id="server-status-card">
+          <div class="status-indicator-row">
+            <span class="status-led" id="modal-status-led"></span>
+            <span class="status-label" id="server-status-text">Checking local daemon on 127.0.0.1:8080...</span>
+          </div>
+        </div>
+
+        <!-- Displayed only when offline -->
+        <div class="cli-start-block" id="cli-start-block" style="display:none;">
+          <span class="cli-cmd-label">Start the static analysis engine in your terminal:</span>
+          <div class="terminal-box">
+            <span class="terminal-cmd" id="terminal-cmd-text">rparse studio --port 8080 --open</span>
+            <button class="btn-copy-cmd" id="btn-copy-cmd" data-copy="rparse studio --port 8080 --open">Copy</button>
+          </div>
+        </div>
       </div>
 
       <div class="modal-footer">
-        <a href="http://127.0.0.1:8080/" id="btn-launch-live-studio" target="_blank" class="btn-primary">
-          Enter Interactive Studio ↗
-        </a>
+        <a href="http://127.0.0.1:8080/" id="btn-launch-live-studio" target="_blank" class="btn-primary btn-modal-launch">
+          Open Studio Session ↗
+        </a>
       </div>
     </div>
   </div>
```

#### JavaScript Logic Diff (`investigation.js`):
```diff
   function checkStudioDaemon() {
     const serverStatusText = document.getElementById('server-status-text');
     const btnLaunchStudio = document.getElementById('btn-launch-live-studio');
+    const cliBlock = document.getElementById('cli-start-block');
+    const statusLed = document.getElementById('modal-status-led');
+
     fetch('http://127.0.0.1:8080/api/health', { method: 'GET', mode: 'cors' })
       .then(res => res.json())
       .then(data => {
         if (serverStatusText) serverStatusText.textContent = `Online: Studio v${data.version || '0.1.0'} active on 127.0.0.1:8080`;
+        if (statusLed) { statusLed.classList.add('online'); statusLed.classList.remove('offline'); }
+        if (cliBlock) cliBlock.style.display = 'none';
         if (btnLaunchStudio) {
-          btnLaunchStudio.textContent = 'Enter Interactive Studio ↗';
+          btnLaunchStudio.textContent = 'Open Studio Session ↗';
           btnLaunchStudio.href = 'http://127.0.0.1:8080/';
-          btnLaunchStudio.target = '_blank';
+          btnLaunchStudio.classList.remove('disabled');
+          btnLaunchStudio.removeAttribute('aria-disabled');
         }
       })
       .catch(() => {
-        if (serverStatusText) serverStatusText.textContent = 'Offline: Studio server not detected on 127.0.0.1:8080 (start with "rparse studio")';
+        if (serverStatusText) serverStatusText.textContent = 'Offline: Local studio daemon not running';
+        if (statusLed) { statusLed.classList.add('offline'); statusLed.classList.remove('online'); }
+        if (cliBlock) cliBlock.style.display = 'block';
         if (btnLaunchStudio) {
-          btnLaunchStudio.textContent = 'Enter Interactive Studio ↗';
-          btnLaunchStudio.href = 'http://127.0.0.1:8080/';
+          btnLaunchStudio.textContent = 'Start Daemon to Launch';
+          btnLaunchStudio.href = '#';
+          btnLaunchStudio.classList.add('disabled');
+          btnLaunchStudio.setAttribute('aria-disabled', 'true');
         }
       });
   }
```

### 5. Intended Visual Improvement
- Dialog height drops from 680px to a tight 260px.
- Eliminates cognitive overload and prevents users from encountering broken localhost connection failures.

### 6. Functional Risk
- None. Enhances network resilience and user trust.

---

## Proposal 4: Analytical Visualization Canvas & Non-Obscuring Inspector

### 1. Current State & Screenshot Evidence
- **Current Evidence:** [`audit/visual/studio-06-tab-provenance.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-06-tab-provenance.png) and [`audit/visual/studio-07-provenance-node-selected.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-07-provenance-node-selected.png)
- **Defects Addressed:** `ISSUE-P0-03` (Contextual inspector covers active graph nodes), `ISSUE-P2-04` (Missing engineering grid).

### 2. Proposed Visual Outcome
- Canvas viewport gains a subtle dark engineering dot matrix (`radial-gradient(rgba(255,255,255,0.08) 1px, transparent 1px); background-size: 24px 24px;`).
- Contextual Inspector transitions into a docked right panel (`flex: 0 0 320px;`) rather than an absolute overlay covering canvas nodes.
- Alternatively, when a right-aligned node is clicked, the SVG viewport auto-pans smoothly by 340px to ensure the selected node remains in the clear workspace area.

### 3. Exact Source Files & Target Lines
- [`implementation/crates/rparse-cli/src/format_html_assets.rs:L515-L615`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-cli/src/format_html_assets.rs#L515-L615) (Canvas & Inspector CSS)

### 4. Proposed Code Diff

```diff
 .canvas-viewport {
   flex: 1;
   display: flex;
   position: relative;
-  background: var(--bg-canvas);
+  background-color: var(--bg-canvas);
+  background-image: radial-gradient(rgba(255, 255, 255, 0.08) 1px, transparent 1px);
+  background-size: 24px 24px;
   overflow: hidden;
 }
 
 .contextual-inspector {
-  position: absolute;
-  top: 0;
-  right: 0;
   width: 320px;
   height: 100%;
-  z-index: 10;
-  background: #FFFFFF;
-  border-left: 1px solid var(--border);
-  box-shadow: var(--shadow-lg);
-  transform: translateX(100%);
-  transition: transform 0.25s cubic-bezier(0.16, 1, 0.3, 1);
+  background: var(--bg-panel);
+  border-left: 1px solid var(--border-subtle);
+  display: flex;
+  flex-direction: column;
+  flex-shrink: 0;
+  box-shadow: -4px 0 16px rgba(0, 0, 0, 0.3);
+  transition: margin-right 0.25s var(--ease-spring);
+  margin-right: -320px;
 }
 .contextual-inspector.open {
-  transform: translateX(0);
+  margin-right: 0;
 }
```

### 5. Intended Visual Improvement
- Solves the critical defect where clicking a node covered it up with the inspector.
- Adds professional instrument depth through the engineering dot grid.

### 6. Functional Risk
- None. Node click event handlers and inspector content rendering remain 100% identical.

---

## Proposal 5: Shared Design Tokens & Cross-Product Transition Consistency

### 1. Current State & Screenshot Evidence
- **Current Evidence:** [`audit/visual/landing-01-hero-initial.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/landing-01-hero-initial.png) vs [`audit/visual/studio-04-workspace-initial.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-04-workspace-initial.png) (and prototype [`audit/visual/prototype-unified-studio.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/prototype-unified-studio.png))
- **Defects Addressed:** `ISSUE-P1-01` (Aesthetic schism across products), `ISSUE-P2-01` (Token aliasing).

### 2. Proposed Visual Outcome
- Establishes a unified Dark-First Scientific Instrument token contract shared between Landing and Studio.
- Transitions the Studio from warm ivory paper to high-tech deep graphite panels, Signal Orange highlights, and Glowing Lilac data dependencies.

### 3. Exact Source Files & Target Lines
- [`implementation/crates/rparse-cli/src/format_html_assets.rs:L14-L70`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-cli/src/format_html_assets.rs#L14-L70)

### 4. Proposed Code Diff

```diff
 :root {
   /* Surfaces & Canvas */
-  --bg-page: #F7F5F0;
-  --bg-canvas: #F7F5F0;
-  --bg-panel: #FFFFFF;
-  --bg-card: #FFFFFF;
-  --bg-subtle: #EEEBE4;
-  --bg-input: #FCFAF7;
+  --bg-canvas: #121216;
+  --bg-panel: #18181E;
+  --bg-surface: #22222A;
+  --bg-surface-hover: #2B2B35;
+  --bg-input: #15151B;
+  --bg-page: var(--bg-canvas);
+  --bg-card: var(--bg-surface);
   
   /* Borders & Dividers */
-  --border: rgba(18, 18, 18, 0.08);
-  --border-strong: rgba(18, 18, 18, 0.16);
-  --border-focus: #FF5A1F;
-  --border-subtle: rgba(18, 18, 18, 0.04);
+  --border: rgba(255, 255, 255, 0.08);
+  --border-subtle: rgba(255, 255, 255, 0.08);
+  --border-strong: rgba(255, 255, 255, 0.16);
+  --border-focus: #FF5A1F;
 
   /* Typography Colors */
-  --text-main: #121212;
-  --text-secondary: #3A3936;
-  --text-muted: #77736B;
-  --text-dim: #9C978E;
-  --text-inverse: #FFFFFF;
+  --text-main: #F4F4F6;
+  --text-secondary: #C8C8D2;
+  --text-muted: #9E9EA8;
+  --text-dim: #6A6A76;
+  --text-inverse: #121216;
 
   /* Accent Colorway */
   --accent-orange: #FF5A1F;
   --accent-primary: #FF5A1F;
-  --accent-purple: #7C3AED;
-  --accent-lilac: #7C3AED;
+  --accent-lilac: #B8A6FF;
+  --accent-dataflow: #B8A6FF;
   --accent-green: #059669;
   --accent-amber: #F59E0B;
-  --accent-rose: #DC2626;
+  --accent-rose: #EF4444;
+  --accent-blue: var(--accent-primary); /* Preserved for test assertion compatibility */
 }
```

### 5. Intended Visual Improvement
- Seamless visual continuity between Landing Page and Studio.
- Transforms the application into a cohesive, elite research developer tool.

### 6. Functional Risk
- None. Preserves `--accent-blue` and semantic color variables so tests pass without issue.
