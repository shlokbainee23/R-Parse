# R-PARSE — Phase C: Prioritized Plan of Action

> **Document Status:** Complete & Pending Approval  
> **Date:** October 2026  
> **Authority Anchor:** [`research/phase-c-readiness-audit.md`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/phase-c-readiness-audit.md), [`research/scope-freeze.md`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/scope-freeze.md)

---

## 1. Implementation Philosophy & Constraints

To strictly preserve the frozen research scope and prevent architectural drift:
1. **Core Non-Interference:** No changes to existing core analysis algorithms in `rparse-core`, `rparse-parser`, `rparse-ir`, `rparse-scope`, `rparse-cfg`, `rparse-dataflow`, `rparse-dataframe`, `rparse-pdg`, `rparse-slicing`, or `rparse-runtime`.
2. **Pure Interface Extension:** All new capabilities are serialization and presentation extensions in `rparse-pdg`, `rparse-dataframe`, and `rparse-cli`.
3. **Zero Heavyweight Web Stacks:** No Node.js dependencies, npm toolchains, or long-running web servers will be introduced into the repository. All visualization deliverables will be delivered as pure, self-contained Rust generators producing standalone HTML/SVG artifacts.

---

## 2. Milestone Roadmap

```text
┌────────────────────────────────────────────────────────┐
│ MILESTONE 1: Graph Export & Topology Serialization    │
│ - PDG Graphviz DOT & JSON Export                       │
│ - Column Provenance DAG DOT & JSON Export              │
│ - CLI `--graph <pdg|prov|cfg>` Integration             │
└───────────────────────────┬────────────────────────────┘
                            │
                            ▼
┌────────────────────────────────────────────────────────┐
│ MILESTONE 2: Fixture Corpus & Sub-Statement Slicing    │
│ - Curated Canonical Showcase Fixtures                  │
│ - Sub-statement mutate argument retention reporting    │
│ - CLI JSON schema enhancement                          │
└───────────────────────────┬────────────────────────────┘
                            │
                            ▼
┌────────────────────────────────────────────────────────┐
│ MILESTONE 3: Self-Contained Interactive Visualizer     │
│ - `rparse report` command producing standalone HTML    │
│ - Client-side interactive DAG rendering (SVG/Canvas)   │
│ - Syntax-highlighted code with visual slice overlays   │
│ - Schema version evolution inspector                   │
└────────────────────────────────────────────────────────┘
```

---

## 3. Detailed Milestone Specifications

### Milestone 1 — Graph Export & Topology Serialization (Priority: P1)

#### Objective
Enable users, researchers, and automated tools to directly extract, inspect, and render the two core graph structures that embody R-PARSE's research contribution: the **Unified Program Dependence Graph (PDG)** and the **Column Provenance DAG**.

#### Detailed Tasks
1. **PDG Graphviz DOT Exporter (`rparse-pdg`):**
   - Implement `ProgramDependenceGraph::to_dot(&self) -> String`.
   - Distinguish node shapes and colors by `PdgNodeKind`:
     - `ColumnVersion`: Ellipse / Port style with schema identity (`df@v1.total`).
     - `Statement`: Rectangles with source coordinates.
     - `Expression`: Rounded rectangles.
     - `VariableDefinition` / `VariableUse`: Octagons / Hexagons.
     - `Unknown`: Dashed, alert-colored boxes.
   - Distinguish edge styles by `PdgEdgeKind`:
     - `ColumnDerivation`: Solid bold blue edges ($C_{in} \to C_{out}$).
     - `VariableData`: Solid black dataflow edges.
     - `Control`: Dashed red edges.
     - `RowSelection`: Dotted purple edges (explicitly distinct from control flow).
     - `Unknown`: Dashed orange edges.
2. **PDG JSON Topology Exporter (`rparse-pdg`):**
   - Implement `ProgramDependenceGraph::to_json(&self) -> JsonValue`.
   - Emits arrays of `{ id, kind, label, span }` nodes and `{ id, from, to, kind }` edges.
3. **Column Provenance DAG Exporter (`rparse-dataframe`):**
   - Implement `ProvenanceGraph::to_dot(&self, df_analysis: &DataFrameAnalysis) -> String`.
   - Implement `ProvenanceGraph::to_json(&self, df_analysis: &DataFrameAnalysis) -> JsonValue`.
   - Visualizes atomic multi-column derivation clusters (`{price, quantity} -> total`).
4. **CLI Integration (`rparse-cli`):**
   - Add `--graph <pdg|provenance|cfg>` flag to `rparse analyze`.
   - Support `--format dot` alongside existing `--format text` and `--format json`.

#### Definition of Done (DoD)
- `rparse analyze fixtures/canonical.R --graph pdg --format dot` outputs valid Graphviz DOT rendering successfully with `dot -Tsvg`.
- `rparse analyze fixtures/canonical.R --graph provenance --format dot` outputs clean derivation clusters.
- All new exporters covered by unit tests verifying deterministic output.

---

### Milestone 2 — Fixture Showcase Suite & Sub-Statement Attribution (Priority: P1)

#### Objective
Provide comprehensive, self-documenting demonstration scripts that exercise all facets of the column provenance and slicing engine, and expose fine-grained sub-statement retention coordinates.

#### Detailed Tasks
1. **Showcase Fixture Corpus (`implementation/fixtures/`):**
   - [`showcase_pipeline.R`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/fixtures/showcase_pipeline.R): Multi-step dplyr pipeline (`raw_data %>% filter(...) %>% mutate(...) %>% select(...)`) demonstrating multi-input derivation, row selection, and column independence.
   - [`partial_retention.R`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/fixtures/partial_retention.R): Demonstrates single-statement partial slicing where `mutate(total = price * quantity, unused = age * 2)` keeps `total` and eliminates `unused`.
   - [`filter_provenance.R`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/fixtures/filter_provenance.R): Demonstrates sequential filters creating `RowSelectionDependency` without polluting CFG control edges.
   - [`uncertainty.R`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/fixtures/uncertainty.R): Demonstrates dynamic column access (`df[[var]]`) generating explicit `UNKNOWN_STATIC` and `PartialContainsUnknowns` confidence.
2. **Sub-Statement Region Attribution (`rparse-slicing` & `rparse-cli`):**
   - Enrich `StatementSlice` in `rparse-slicing` to include `retained_sub_regions: Vec<Span>` and `excluded_sub_regions: Vec<Span>`.
   - In `format_slice_json`, emit `sub_elements` for partially retained statements:
     ```json
     {
       "line": 3,
       "state": "partially_retained",
       "retained_arguments": ["total = price * quantity"],
       "excluded_arguments": ["unused = age * 2"]
     }
     ```
   - In `format_slice_text`, display exact argument-level annotations for partially retained lines.

#### Definition of Done (DoD)
- Each fixture can be executed via `rparse analyze`, `rparse provenance`, and `rparse slice`.
- Terminal output clearly indicates which parts of a multi-argument statement were retained or dropped.
- Integration tests in `rparse-cli` validate all fixtures.

---

### Milestone 3 — Self-Contained Interactive HTML Visualizer (Priority: P2)

#### Objective
Provide a zero-dependency, single-file interactive HTML demonstration report that allows anyone to inspect R-PARSE analysis results, explore graph topologies, and review slices in their web browser without installing additional tools.

#### Detailed Tasks
1. **CLI `report` Subcommand (`rparse-cli`):**
   - Add command: `rparse report <FILE> [--output <PATH>] [--criterion <CRIT>]`.
   - Defaults output to `<FILE>.rparse.html`.
2. **Standalone HTML Template Generator:**
   - Single, self-contained HTML document with zero external network requests.
   - Embeds minimal CSS for clean dark/light mode presentation.
   - Embeds client-side vector renderer (SVG-based DAG layout) to display:
     - **Tab 1: Column Provenance DAG** (Interactive nodes showing input $\to$ output column flow).
     - **Tab 2: Unified PDG** (Full program dependence graph with filterable edge categories).
     - **Tab 3: Program Slicing Overlay** (Original R source code with line-by-line syntax styling: green for fully retained, amber for partially retained with sub-expression highlights, and strike-through/gray for excluded code).
     - **Tab 4: Schema Evolution Table** (Table of dataframe states showing column additions, renames, and deletions across versions).
3. **Self-Contained Inlining:**
   - All data payload (AST, PDG, Slices, Source Code) inlined as structured JSON in `<script id="rparse-data" type="application/json">`.

#### Definition of Done (DoD)
- Running `rparse report fixtures/canonical.R` produces a valid `canonical.R.rparse.html`.
- Opening the file in any modern web browser immediately displays the interactive report offline.
- No npm dependencies, no Node.js build step, and no active server processes required.

---

## 4. Dependencies, Risks, and Mitigations

| Risk | Impact | Mitigation |
|---|---|---|
| **Scope Creep into Web App** | Introducing web servers, frameworks, or npm packages violates Rule 8 of `AGENTS.md`. | Strict constraint: Generate only **static, standalone, offline HTML files** via pure Rust string templating. Zero node/npm dependencies. |
| **Performance Overhead of Large Graphs** | Generating massive DOT/JSON strings for large R files could degrade memory. | Exporters will use buffered streaming writers with sequential node generation. |
| **Windows Security / SAC Interference** | Freshly compiled tools being blocked by Windows security. | The native MSVC toolchain and developer security configuration have been fully verified and documented in `BUILDING.md`. |

---

## 5. Next Immediate Action

Upon user authorization, proceed immediately with **Milestone 1**:
1. Implement `ProgramDependenceGraph::to_dot()` and `ProgramDependenceGraph::to_json()` in `rparse-pdg`.
2. Implement `ProvenanceGraph::to_dot()` in `rparse-dataframe`.
3. Add `--graph` and `--format dot` options to `rparse-cli`.
4. Validate with `cargo test --workspace` and confirm Graphviz DOT generation on `fixtures/canonical.R`.
