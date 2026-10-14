# R-PARSE — Visual Forensic Audit Index

**Project:** R-PARSE (Robust R Language Parsing and Program Analysis Engine)  
**Phase:** Phase 1 — Visual Forensic Audit (Inspection & Architecture Review Only)  
**Audit Environment:** Desktop (1440 × 900), Tablet (1024 × 768), Tablet Portrait (768 × 1024), Mobile (390 × 844)  
**Active Services Audited:**
- Landing Page: `http://127.0.0.1:3000/landing/index.html` (Python Loopback HTTP Server)
- Root Landing: `http://127.0.0.1:3000/index.html`
- Interactive Studio: `http://127.0.0.1:8080/` (Native Rust `rparse studio --port 8080`)
**Total Screenshots Captured:** 53 verified browser screenshots  
**Interactive Visual Index:** Open [`contact_sheet.html`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/contact_sheet.html) in your browser for a categorized photographic contact sheet.

---

## 1. Screenshot Index & State Catalog

### Category A: Landing Page Visuals (`landing/`)

| Filename | Viewport | State Represented | Audit Significance / Forensic Value |
| :--- | :--- | :--- | :--- |
| [`landing-01-hero-initial.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/landing-01-hero-initial.png) | 1440×900 | Hero above-the-fold viewport | Baseline brand visual identity: dark canvas (`#16161A`), editorial typography (General Sans/Plus Jakarta Sans), Signal Orange CTA (`#FF5A1F`), Three.js ribbon canvas. Demonstrates high-craft cinematic standard. |
| [`landing-02-header-scrolled.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/landing-02-header-scrolled.png) | 1440×900 | Sticky header after 750px scroll | Shows reading progress indicator line, blur backdrop, active chapter ticker (`CASE 000 · ORIGIN OF RESULT`), and "Enter the Studio" CTA in scrolled mode. |
| [`landing-03-nav-drawer-open.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/landing-03-nav-drawer-open.png) | 1440×900 | Slide-out table of contents menu | Full drawer navigation showing narrative chapters 00–07, secondary links, and launch studio button. |
| [`landing-04-section-question.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/landing-04-section-question.png) | 1440×900 | Section 01: The Core Question | Large display typography ("Where did this number come from?"), dual narrative tracks, high contrast editorial layout. |
| [`landing-05-section-case-opens.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/landing-05-section-case-opens.png) | 1440×900 | Section 02: The Case Opens | Comparison between black-box statistical evaluation vs deterministic fine-grained static analysis. |
| [`landing-06-pinned-evidence-stage-1.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/landing-06-pinned-evidence-stage-1.png) | 1440×900 | Section 03: Pinned Stage (Step 1) | Pinned scroll sequence showing raw messy R pipeline script before analysis. |
| [`landing-07-pinned-evidence-stage-2.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/landing-07-pinned-evidence-stage-2.png) | 1440×900 | Section 03: Pinned Stage (Step 2) | Scroll transition revealing highlighted column backward slice and derivation paths. |
| [`landing-08-section-central-reveal.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/landing-08-section-central-reveal.png) | 1440×900 | Section 04: Central Reveal | Mathematical premise ($C_{in} \to C_{out}$), large central statement, typographic hierarchy. |
| [`landing-09-evidence-room-ast.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/landing-09-evidence-room-ast.png) | 1440×900 | Section 05: Evidence Room (AST Tab) | First interactive exhibit: Tree-sitter CST and IR breakdown in an editorial tab container. |
| [`landing-10-evidence-room-provenance.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/landing-10-evidence-room-provenance.png) | 1440×900 | Section 05: Provenance Tab | Fine-grained column lineage exhibit showing input-to-output derivation flows. |
| [`landing-11-evidence-room-pdg.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/landing-11-evidence-room-pdg.png) | 1440×900 | Section 05: PDG Exhibit Tab | Unified Program Dependence Graph with row selection and control distinctions. |
| [`landing-12-evidence-room-slicing.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/landing-12-evidence-room-slicing.png) | 1440×900 | Section 05: Slicing Exhibit Tab | Column-aware backward slicing exhibit showing sub-statement retention (`[KEEP]` vs `[DROP]`). |
| [`landing-13-section-limits.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/landing-13-section-limits.png) | 1440×900 | Section 06: Explicit Boundaries | Architectural honesty section detailing `UNKNOWN_STATIC`, static boundaries, and zero-runtime execution. |
| [`landing-14-pinned-assembly-stage.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/landing-14-pinned-assembly-stage.png) | 1440×900 | Section 07: Studio Architecture | Exploded diagram assembly transition showing 30/70 workspace composition. |
| [`landing-15-footer.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/landing-15-footer.png) | 1440×900 | Site Footer | Final CTA, offline showcase report link, technical ADR links, copyright, and back-to-top button. |
| [`landing-16-studio-modal-default.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/landing-16-studio-modal-default.png) | 1440×900 | Studio Modal (Default Opened) | The Studio Session launch dialog opened via hero CTA. Displays daemon probing status, capabilities list, CLI details toggle, and launch CTA. Key audit focus. |
| [`landing-16b-root-modal-default.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/landing-16b-root-modal-default.png) | 1440×900 | Root `index.html` Studio Modal | Alternate modal markup in root file featuring uncollapsed raw CLI command terminals and hardcoded slice demonstration. Demonstrates divergence between root and landing files. |
| [`landing-17-studio-modal-cli-expanded.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/landing-17-studio-modal-cli-expanded.png) | 1440×900 | Studio Modal with CLI Details Open | Expanded state revealing 4 separate CLI command blocks with copy buttons, hints, and explanatory text. Highlights modal verbosity and documentation clutter. |
| [`landing-18-mobile-hero.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/landing-18-mobile-hero.png) | 390×844 | Mobile viewport hero section | Audit of headline wrapping, ribbon canvas scaling, and touch target sizing. |
| [`landing-19-mobile-nav-drawer.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/landing-19-mobile-nav-drawer.png) | 390×844 | Mobile navigation drawer | Audit of drawer full-screen coverage, link spacing, and exit button accessibility. |
| [`landing-20-tablet-evidence-room.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/landing-20-tablet-evidence-room.png) | 1024×768 | Tablet exhibit tabs | Audit of tab bar wrapping, exhibit panel responsiveness, and code snippet scrolling. |

---

### Category B: Studio Workspace Visuals (`studio-`)

| Filename | Viewport | State Represented | Audit Significance / Forensic Value |
| :--- | :--- | :--- | :--- |
| [`studio-01-entry-screen.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-01-entry-screen.png) | 1440×900 | Code Entry Screen (Initial) | Minimalist centered card ("Understand your R code."), textarea editor, actions bar, sample dropdown. |
| [`studio-02-entry-screen-validation-error.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-02-entry-screen-validation-error.png) | 1440×900 | Entry Screen with Validation Error | Critical forensic evidence: showing red error banner ("Please enter valid R code to analyze") pushing the card down and clipping action buttons off-screen. |
| [`studio-03-entry-example-dropdown.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-03-entry-example-dropdown.png) | 1440×900 | Entry Screen Example Loaded | State with `showcase_pipeline` selected from dropdown and code populated into textarea. |
| [`studio-04-workspace-initial.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-04-workspace-initial.png) | 1440×900 | Workspace Screen (Default) | Full 30/70 layout with default `showcase_pipeline.R` loaded. Demonstrates immediate visual disconnect from landing page (warm ivory canvas vs dark cinematic landing). |
| [`studio-05-header-detail.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-05-header-detail.png) | 1440×120 (Crop) | Studio Header Close-up | Forensic analysis of 2-row header: brand mark, filename, status dot ("Analysis ready"), meta items ("Static analysis • Known static"), stats strip ("Stmts: 15 Cols: 7 PDG: 22"), action buttons, and 5 subnav tabs. |
| [`studio-06-tab-provenance.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-06-tab-provenance.png) | 1440×900 | Tab 1: Column Provenance | Bipartite graph canvas showing input columns on left, transformations in center, output columns on right. Toolbar with search input, zoom, and fit-view buttons. |
| [`studio-07-provenance-node-selected.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-07-provenance-node-selected.png) | 1440×900 | Provenance Node Inspected | Selected column derivation node with right-side contextual Inspector panel expanded. Shows how inspector overlay obscures graph nodes underneath. |
| [`studio-08-provenance-filtered.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-08-provenance-filtered.png) | 1440×900 | Provenance with Filter Active | Search filter query ("total") applied, dimming non-matching nodes and highlighting matched column paths. |
| [`studio-09-tab-pdg.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-09-tab-pdg.png) | 1440×900 | Tab 2: Unified PDG Graph | Full program dependence graph with edge type filter chips: Data, Control, RowSelection, Unknown. |
| [`studio-10-pdg-edge-filters-toggled.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-10-pdg-edge-filters-toggled.png) | 1440×900 | PDG with Edge Filters Toggled | Control and Unknown edges unchecked, isolating pure data flow and row selection dependencies. |
| [`studio-11-pdg-node-selected.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-11-pdg-node-selected.png) | 1440×900 | PDG Node Selected & Inspected | Statement node selected in PDG; inspector displays incoming/outgoing dependencies, expression type, and AST link. |
| [`studio-12-tab-backward-slice.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-12-tab-backward-slice.png) | 1440×900 | Tab 3: Backward Slice | Backward slicing view with criterion selector dropdown (`df2$total`), confidence badges (`COMPLETE`, `KNOWN`), and line-by-line attribution cards. |
| [`studio-13-slice-criterion-changed.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-13-slice-criterion-changed.png) | 1440×900 | Slice with Secondary Criterion | Alternative criterion selected (`df$subtotal`), updating retention states and source highlights. |
| [`studio-14-tab-schema-evolution.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-14-tab-schema-evolution.png) | 1440×900 | Tab 4: Schema Evolution | Step-by-step dataframe transformation timeline. Diff legend badges (+added, -removed, ~modified, =retained). |
| [`studio-15-schema-step-selected.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-15-schema-step-selected.png) | 1440×900 | Schema Step Inspected | Selected step card displaying column-level diffs, mutation operations, and schema shape changes in the inspector. |
| [`studio-16-tab-ast-explorer-cst.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-16-tab-ast-explorer-cst.png) | 1440×900 | Tab 5: AST Explorer (CST Mode) | Tree-sitter Concrete Syntax Tree representation with expandable node hierarchy, token spans, and node count stats. |
| [`studio-17-tab-ast-explorer-ir.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-17-tab-ast-explorer-ir.png) | 1440×900 | AST Explorer (Semantic IR Mode) | Normalized semantic intermediate representation tree showing statements and expressions. |
| [`studio-18-ast-search-filter.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-18-ast-search-filter.png) | 1440×900 | AST Explorer Search Active | Search filter ("call") applied to AST tree, highlighting matching node types. |
| [`studio-19-source-pane-word-wrap-on.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-19-source-pane-word-wrap-on.png) | 1440×900 | Source Pane (Word Wrap ON) | Default state of left 30% source pane with wrap toggle active. Lines wrap cleanly at pane boundary. |
| [`studio-20-source-pane-word-wrap-off.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-20-source-pane-word-wrap-off.png) | 1440×900 | Source Pane (Word Wrap OFF) | Word wrap disabled, revealing horizontal scrolling requirement for long pipeline statements. |
| [`studio-21-source-pane-edit-mode.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-21-source-pane-edit-mode.png) | 1440×900 | Source Pane in Edit Mode | Interactive source textarea exposed for inline editing, replacing the read-only syntax viewer. |
| [`studio-22-source-stale-banner.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-22-source-stale-banner.png) | 1440×900 | Stale Source Warning Banner | Warning banner ("⚠️ Source modified since analysis") with `Reanalyze` and `Revert` action buttons. |
| [`studio-23-evidence-drawer-open.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-23-evidence-drawer-open.png) | 1440×900 | Evidence Drawer Open | Bottom slide-up drawer displaying compiler diagnostics, AST node count, PDG edge totals, and deterministic isolation card. |
| [`studio-24-inspector-collapsed-vs-expanded.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-24-inspector-collapsed-vs-expanded.png) | 1440×900 | Inspector Layout Comparison | Evaluates the inspector panel width (320px) against remaining canvas space. |
| [`studio-25-responsive-1024x768.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-25-responsive-1024x768.png) | 1024×768 | Studio at 1024px Width | Header items compress; source pane and canvas share narrower horizontal space. |
| [`studio-26-responsive-768x1024.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-26-responsive-768x1024.png) | 768×1024 | Studio at 768px Width (Tablet Portrait) | Critical evidence: header top row overflows horizontally by 110px (`topRowScrollWidth: 878px`), pushing action buttons off-screen. |
| [`studio-27-responsive-390x844.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/studio-27-responsive-390x844.png) | 390×844 | Studio at 390px Width (Mobile) | Severe layout breakdown: source pane and canvas squashed, subnav tabs truncated, buttons overlapping. |

---

### Category C: Cross-Product Transitions (`journey-`)

| Filename | Viewport | State Represented | Audit Significance / Forensic Value |
| :--- | :--- | :--- | :--- |
| [`journey-01-landing-to-modal.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/journey-01-landing-to-modal.png) | 1440×900 | Landing Page to Studio Modal | User clicks "Enter the Studio" in landing header; modal opens with backdrop blur. |
| [`journey-02-modal-to-studio.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/journey-02-modal-to-studio.png) | 1440×900 | Modal Launch CTA | Modal footer action ("Enter Interactive Studio ↗") linking to `http://127.0.0.1:8080/`. |
| [`journey-03-studio-to-landing.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/journey-03-studio-to-landing.png) | 1440×900 | Studio to Landing Navigation | Studio header link ("Investigation ↗") linking back to `/landing`. Audit of cross-navigation affordance. |
| [`journey-04-new-analysis-flow.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/journey-04-new-analysis-flow.png) | 1440×900 | New Analysis Reset Flow | Clicking "New Analysis" returns user from workspace to code entry screen. |
| [`journey-05-node-to-source-jump.png`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/journey-05-node-to-source-jump.png) | 1440×900 | Node-to-Source Jump Sync | Clicking "Jump to Source" in the inspector highlights the exact character-accurate line and column span in the left source pane. |

---

## 2. Master Audit Findings Summary

A high-level synthesis of our forensic investigation across the 53 captures reveals **five foundational architectural fractures**:

1. **Aesthetic Schism (Two Different Products):**
   The Landing Page is an award-grade, cinematic, dark-themed experience (`#16161A` canvas, subtle glassmorphic borders, Signal Orange `#FF5A1F` accents, JetBrains Mono + Plus Jakarta Sans). The Studio Workspace is a generic light ivory/beige web application (`#F7F5F0` background, flat white boxes, weak contrast, default buttons). Entering the Studio from the landing page feels like dropping into an unrelated legacy dashboard.
2. **Container Gridlock & Border Bloat:**
   The workspace is partitioned by dozens of nested rectangular containers with redundant borders. Header top-row, header subnav, source header, source body, canvas toolbar, canvas viewport, inspector header, inspector properties, status strips, and banners all sport separate borders and cards, eating screen real estate and creating visual fatigue.
3. **Severe Header Clutter & Responsive Overflow Bug:**
   The 2-row header (`76px` high) permanently occupies excessive vertical screen space while simultaneously packing 9 competing items into its top right row. At viewports under 880px, the top row overflows horizontally by 110px without a scrollbar, silently pushing the primary action buttons (`New Analysis`, `Evidence`) completely off-screen.
4. **Modal Over-Documentation & False Health Claims:**
   The Studio Session modal attempts to serve as a complete reference manual rather than a focused launch dialog. It features 4 verbose capability bullets, 4 separate terminal command blocks with copy buttons, and hardcodes loopback IP addresses (`127.0.0.1:8080`). Furthermore, in `index.html`, the modal claims "Probing local Studio daemon..." while providing no graceful handling when offline.
5. **Inspector Viewport Occlusion & Canvas Squeeze:**
   The Contextual Inspector is implemented as an absolute slide-out overlay (`320px` wide) inside the canvas. When a user clicks a node on the right side of the graph, the inspector slides out and covers the exact node that was just selected, forcing manual panning.

See [`AUDIT_REPORT.md`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/AUDIT_REPORT.md) for the full evidence-based finding logs, [`COMPONENT_INVENTORY.md`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/COMPONENT_INVENTORY.md) for the component breakdown, [`FIX_PLAN.md`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/FIX_PLAN.md) for the remediation schedule, and [`FIX_PROPOSALS.md`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/audit/visual/FIX_PROPOSALS.md) for the concrete code proposals.
