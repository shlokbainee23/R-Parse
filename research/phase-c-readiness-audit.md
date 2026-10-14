# R-PARSE — Phase C: Project Readiness & Frontend Audit

> **Status:** Completed  
> **Date:** October 2026  
> **Auditor:** Antigravity AI  
> **Scope Anchor:** [`research/scope-freeze.md`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/scope-freeze.md), [`research/ADR-001-architecture.md`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/ADR-001-architecture.md), [`research/technical-specification.md`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/technical-specification.md)

---

## Executive Summary

This audit assesses the implementation maturity, graph capabilities, frontend readiness, and demonstration quality of the R-PARSE workspace (`implementation/`). 

The R-PARSE core engine is **exceptionally mature and fully functional**: all 11 crates build cleanly on Windows native MSVC, pass 213 of 213 unit and integration tests, pass Clippy with zero warnings, and complete an end-to-end 8-dimension static pipeline in under 5 milliseconds. The core research contribution—**fine-grained dataframe column provenance integrated with program dependence graphs and column-aware static backward slicing**—is fully implemented and empirically verified.

However, **visualization and frontend capabilities are currently NOT PRESENT**: there is no web frontend, no graph visualization interface, and the Unified PDG lacks DOT or JSON graph export functionality. Demonstrations are currently restricted to terminal CLI text and tabular JSON outputs.

---

## Audit 1 — Repository & Implementation Readiness

### 1.1 Crate Implementation & Integration Status

All 11 crates reside in [`implementation/crates`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates) and are fully integrated into the unified workspace:

| Crate | Path | Status | Verification Evidence & Exact File Locations |
|---|---|---|---|
| [`rparse-core`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-core) | `crates/rparse-core` | **Implemented & Integrated** | Defines foundational domain primitives: [`NodeId`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-core/src/node_id.rs), [`Span`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-core/src/span.rs), [`SourceFile`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-core/src/source.rs), [`Diagnostic`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-core/src/diagnostic.rs), and [`RParseError`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-core/src/error.rs). Zero external analysis dependencies. 10 unit tests pass. |
| [`rparse-parser`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-parser) | `crates/rparse-parser` | **Implemented & Integrated** | Safe Rust wrapper over Tree-sitter R grammar. [`RParser`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-parser/src/parser.rs), [`CstTree`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-parser/src/cst.rs), [`span_ext`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-parser/src/span_ext.rs). C parser compiled via native MSVC (`cl.exe`). Addresses Tree-sitter pipe placeholder `_` bug. 50 tests pass. |
| [`rparse-ir`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-ir) | `crates/rparse-ir` | **Implemented & Integrated** | Normalized Semantic AST: [`Program`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-ir/src/ast.rs#L8), [`Stmt`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-ir/src/ast.rs#L16), [`Expr`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-ir/src/ast.rs#L77), [`LValue`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-ir/src/ast.rs#L42). Context-sensitive assignment lowering, desugaring, golden serializer [`to_golden_string`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-ir/src/golden.rs#L10). 6 tests pass. |
| [`rparse-scope`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-scope) | `crates/rparse-scope` | **Implemented & Integrated** | Lexical scopes: [`ScopeModel`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-scope/src/scope.rs), [`ScopeKind`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-scope/src/scope.rs#L10) (`Global`, `Function`, `DataMask`), [`Symbol`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-scope/src/symbol.rs#L34), [`Definition`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-scope/src/symbol.rs#L10), [`Reference`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-scope/src/symbol.rs#L22). Pronouns `.data` and `.env` handled. 13 tests pass. |
| [`rparse-cfg`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-cfg) | `crates/rparse-cfg` | **Implemented & Integrated** | Basic block CFG: [`Cfg`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-cfg/src/graph.rs#L14), [`BasicBlock`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-cfg/src/block.rs#L8), [`CfgEdge`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-cfg/src/block.rs#L64) with typed edges (`Normal`, `TrueBranch`, `FalseBranch`, `LoopBack`, `Return`). Section 29 invariant verification. DOT export [`Cfg::to_dot`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-cfg/src/graph.rs#L243). 12 tests pass. |
| [`rparse-dataflow`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-dataflow) | `crates/rparse-dataflow` | **Implemented & Integrated** | [`DefUseInfo`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-dataflow/src/def_use.rs), forward reaching definitions [`compute_reaching_definitions`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-dataflow/src/reaching_defs.rs), post-dominator tree [`compute_post_dominators`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-dataflow/src/post_dominance.rs), control dependencies [`compute_control_dependencies`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-dataflow/src/control_dependence.rs). 14 tests pass. |
| [`rparse-dataframe`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-dataframe) | `crates/rparse-dataframe` | **Implemented & Integrated** | Column versioning: [`DataFrameState`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-dataframe/src/state.rs), [`ColumnVersion`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-dataframe/src/column.rs), [`ProvenanceGraph`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-dataframe/src/provenance.rs#L84), [`ProvenanceEdge`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-dataframe/src/provenance.rs#L29), [`RowSelectionDependency`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-dataframe/src/dependency.rs#L21). Operations: `mutate`, `transmute`, `select`, `rename`, `filter`, `df$col <-`, `transform`. 39 tests pass. |
| [`rparse-pdg`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-pdg) | `crates/rparse-pdg` | **Implemented & Integrated** | Unified graph: [`ProgramDependenceGraph`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-pdg/src/graph.rs#L105). 7 node kinds ([`PdgNodeKind`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-pdg/src/node.rs#L12)), 6 edge kinds ([`PdgEdgeKind`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-pdg/src/edge.rs#L12)), column ports, Section 33 invariant validator ([`validate_pdg`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-pdg/src/builder.rs#L20)). 7 tests pass. |
| [`rparse-slicing`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-slicing) | `crates/rparse-slicing` | **Implemented & Integrated** | Column-aware static backward slicing: [`compute_backward_slice`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-slicing/src/slicer.rs#L21), [`SliceCriterion`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-slicing/src/criterion.rs#L10), [`RetentionState`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-slicing/src/element.rs#L10) (`FullyRetained`, `PartiallyRetained`, `NotRetained`), [`SliceConfidence`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-slicing/src/result.rs#L11). 16 tests pass. |
| [`rparse-runtime`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-runtime) | `crates/rparse-runtime` | **Implemented & Integrated** | Tier 2 GNU R runtime bridge: [`RuntimeBridge`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-runtime/src/bridge.rs), [`RProcess`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-runtime/src/process.rs), JSON-RPC 2.0 protocol ([`protocol.rs`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-runtime/src/protocol.rs)), empirical evidence ([`RuntimeEvidence`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-runtime/src/evidence.rs#L10)), worker script ([`worker.R`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-runtime/src/worker.R)). 27 tests pass. |
| [`rparse-cli`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-cli) | `crates/rparse-cli` | **Implemented & Integrated** | Top-level CLI & pipeline orchestrator: [`AnalysisContext`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-cli/src/pipeline.rs#L27), subcommands `analyze`, `provenance`, `slice`, formatters ([`format_text.rs`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-cli/src/format_text.rs), [`format_json.rs`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-cli/src/format_json.rs)), exit codes ([`exit_code.rs`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-cli/src/exit_code.rs)). 19 tests pass. |

### 1.2 Partially Implemented or Unexposed Features

While the analysis algorithms are complete, several analytical outputs are **only partially exposed** to consumers:

1. **PDG Serialization Missing:** While [`rparse-cfg`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-cfg) implements `Cfg::to_dot()`, [`rparse-pdg`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-pdg) provides **no serialization method** (neither Graphviz DOT nor JSON graph export). Consumers cannot inspect the unified graph topology directly.
2. **Provenance Graph Serialization Missing:** The [`ProvenanceGraph`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-dataframe/src/provenance.rs#L84) stores all grouped derivation hyperedges, but only queries for a single target column version are serialized via CLI `provenance`. There is no command or format to dump the full provenance DAG.
3. **Partial Statement Sub-AST Span Unexposed:** The slicer computes partial statement retention (`PartiallyRetained`) and records exact `SliceElement` IDs for retained mutate arguments. However, CLI `slice` output only returns statement-level line numbers (`start_line`, `end_line`, `state: partially_retained`). It does not emit the byte/character coordinates of which sub-arguments were kept or dropped.
4. **Graph Topology Omitted from JSON:** `rparse analyze --format json` emits scalar summary counts (`pdg_node_count`, `pdg_edge_count`, `column_port_count`) and schema versions, but omits the graph adjacency matrix, node labels, and edge types.

### 1.3 Fixtures and Demonstration Assets

- **Existing Fixtures:** Only one fixture exists on disk: [`implementation/fixtures/canonical.R`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/fixtures/canonical.R):
  ```r
  df <- data.frame(price = 10, quantity = 5)
  df$total <- df$price * df$quantity
  df2 <- mutate(df, tax = total * 0.1)
  ```
- **Corpus Gap:** While tests in `tests.rs` embed rich test scripts in-memory (e.g. Benchmark 1–7, sequential filter chains, multi-step dplyr pipelines, unknown access patterns), there are no corresponding fixture files on disk for easy CLI demonstration or external tooling evaluation.

---

## Audit 2 — Analysis & Graph Capabilities

### 2.1 Graph Representations in Code

| Graph Layer | In-Memory Data Structure | Node Types | Edge Types | Export Capability |
|---|---|---|---|---|
| **Control Flow Graph (CFG)** | [`Cfg`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-cfg/src/graph.rs#L14) | [`BasicBlock`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-cfg/src/block.rs#L8) (`BlockId`) | `Normal`, `TrueBranch`, `FalseBranch`, `LoopBack`, `Return` | **DOT Exporter Implemented** (`Cfg::to_dot()`) |
| **Column Provenance Graph** | [`ProvenanceGraph`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-dataframe/src/provenance.rs#L84) | [`ColumnVersion`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-dataframe/src/column.rs#L11), `SymbolId` | [`ProvenanceEdge`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-dataframe/src/provenance.rs#L29) (Grouped inputs $\to$ output) | **None** (Only single-column trace via CLI `provenance`) |
| **Unified PDG** | [`ProgramDependenceGraph`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-pdg/src/graph.rs#L105) | 7 Kinds: `Statement`, `Expression`, `VariableDefinition`, `VariableUse`, `ColumnVersion`, `RowSelectionPredicate`, `Unknown` | 6 Kinds: `VariableData`, `ColumnDerivation`, `ColumnStatementBinding`, `Control`, `RowSelection`, `Unknown` | **None** (No DOT, no JSON graph format) |

### 2.2 Fidelity to Research Thesis

The code strictly respects the frozen research boundary:
- **Separation of Schema vs. Derivation:** `DataFrameState.schema` records known column names; `ProvenanceGraph` records input-to-output causal lineage edges.
- **Grouped Multi-Column Derivations:** `ProvenanceEdge` groups input columns in a `BTreeSet<ColumnVersionId>`, preserving multi-input derivation semantics (`{price, quantity} -> total`).
- **Separation of Control vs. Row-Selection:** In `pdg_control.rs`, conditional branches produce `ControlDependency` edges; `filter()` operations produce `RowSelectionDependency` edges without generating spurious CFG control edges.
- **Explicit Uncertainty:** Unresolvable column access produces explicit `PdgNodeKind::Unknown { reason }` and `StaticKnowledge::UnknownStatic`. Slices hitting unknowns degrade to `SliceConfidence::PartialContainsUnknowns`.

---

## Audit 3 — Frontend & Visualization Readiness

### 3.1 Existing Frontend Status

> [!IMPORTANT]
> **No frontend, web server, GUI, or visualization dashboard exists in the repository.**
> An exhaustive search for web assets (`.html`, `.css`, `.js`, `.ts`, `.jsx`, `.tsx`, `.svg`, `.vue`, `.svelte`) across `implementation/` identified zero visualization files.

### 3.2 Evaluation of Demonstration Requirements

| Requirement | Current Capability | Gap / Status |
|---|---|---|
| **1. File Selection & Ingestion** | CLI arguments (`<FILE>`) and stdin (`-`) | No GUI file picker or interactive drop zone |
| **2. Analysis Status & Diagnostics** | Text & JSON CLI output with severity, line coordinates, and message | No visual code gutter or diagnostic callout |
| **3. DataFrame Schema Evolution** | CLI `analyze` lists states ($v_0, v_1, v_2$) and column arrays | No visual schema diff table or version slider |
| **4. Column Provenance DAG** | CLI `provenance` lists direct/transitive inputs and text chain | No interactive graph rendering of column lineage DAG |
| **5. PDG Exploration** | CLI reports total node/edge counts only | No visual graph rendering of the Unified PDG |
| **6. Backward Slice Display** | CLI `slice` lists statements with `[FULL]`, `[PARTIAL]`, `[NONE]` | No syntax-highlighted code overlay showing retained vs. grayed-out code |
| **7. Uncertainty Explanation** | Emits `Confidence: COMPLETE` vs `PARTIAL` and `is_unknown: bool` | No visual badge highlighting unknown nodes or dynamic column risks |
| **8. Structured Data Inspection** | `--format json` for all 3 CLI commands | No interactive JSON viewer or structured tree view |

---

## Audit 4 — Demonstration Quality Assessment

### 4.1 CLI Output Clarity for New Users

Using the existing CLI on [`fixtures/canonical.R`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/fixtures/canonical.R):
```r
df <- data.frame(price = 10, quantity = 5)
df$total <- df$price * df$quantity
df2 <- mutate(df, tax = total * 0.1)
```

Running `rparse slice fixtures/canonical.R --criterion 'df$total'`:
```text
R-PARSE BACKWARD STATIC SLICE
─────────────────────────────
Criterion:         df$total
Confidence:        COMPLETE
Contains Unknowns: NO

Slice Metrics:
  Retained Semantic Elements:  14
  Fully Retained Statements:    2
  Partially Retained Statements:0

Retained Program Statements:
  [FULL] Line 1:   df <- data.frame(price = 10, quantity = 5)
  [FULL] Line 2:   df$total <- df$price * df$quantity
  [NONE] Line 3:   df2 <- mutate(df, tax = total * 0.1)
```

**Evaluation:**
- **Clarity of Retained Statements:** High. It is immediately clear that lines 1 and 2 are retained and line 3 is eliminated.
- **Clarity of Exclusion Rationale:** Low. The terminal output does not explain *why* line 3 was eliminated (namely, that `tax` in `df2` is a downstream consumer of `total`, but does not contribute to `df$total`).
- **Clarity of Partial Retentions:** In scripts where `mutate(total = price * quantity, unused = age * 2)` occurs, terminal output marks the line as `[PARTIAL]`, but does not show that `total` was kept while `unused` was dropped.

### 4.2 Proposed Showcase Candidate

To demonstrate the full scientific contribution, R-PARSE requires a multi-step showcase script incorporating:
1. Multi-column calculation (`subtotal = price * quantity`)
2. Predicate filtering (`filter(price > 0)`)
3. Sibling column exclusion (`unused = age * 2`)
4. Downstream consumer exclusion (`df2 <- mutate(tax = total * 0.18)`)
5. Provenance lineage query tracing back to raw input columns

The test suite's 8-dimension benchmark script ([`rparse-cli/src/tests.rs:L700-719`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/crates/rparse-cli/src/tests.rs#L700-L719)) serves as the ideal canonical showcase script.

---

## Audit 5 — Readiness Scorecard

| Assessment Dimension | Rating | Evidence & Rationale |
|---|:---:|---|
| **Overall Project Readiness** | **PARTIAL** | Core engine is production-ready; presentation and visualization layers are missing. |
| **Core Engine Readiness** | **PASS** | 11/11 crates build cleanly on native MSVC; 213/213 unit/integration tests pass; sub-5ms pipeline execution. |
| **Provenance & Slicing Readiness** | **PASS** | Frozen thesis completely implemented; column port granularity, grouped derivation hyperedges, and partial statement slicing fully validated. |
| **Graph Export Readiness** | **PARTIAL** | CFG has DOT export (`to_dot`). Unified PDG and ProvenanceGraph have zero graph export methods. CLI JSON omits graph topology. |
| **Frontend Readiness** | **NOT PRESENT** | Zero frontend assets exist. No HTML, CSS, JavaScript, WebAssembly, or UI crate exists in the workspace. |
| **Documentation & Demonstration** | **PARTIAL** | Architectural documents, technical specs, and `BUILDING.md` are comprehensive. Fixture corpus is sparse (1 file), and demonstration is limited to CLI text. |
| **Build & Toolchain Health** | **PASS** | Verified clean rebuild on Windows native MSVC; zero compiler warnings; Smart App Control documented and resolved. |

---

## Audit 6 — Prioritized Plan of Action

### Classification Criteria
- **P0 (Blocker):** Correctness, build, or integration issues preventing core execution.
- **P1 (Core Demonstration Gap):** Missing capabilities that prevent demonstrating the core research contribution.
- **P2 (Frontend / Polish):** User interface, interactivity, and presentation enhancements.

### Gap Analysis Matrix

| Priority | ID | Gap Description | Technical Evidence | Proposed Solution | Dependencies | Acceptance Criteria |
|:---:|:---:|---|---|---|---|---|
| **P1** | **G1** | **PDG & Provenance Graph Export Missing** | `rparse-pdg/src/graph.rs` has no serialization method; CLI `analyze` prints only node/edge count. | Add `to_dot()` and `to_json()` methods to `ProgramDependenceGraph` and `ProvenanceGraph`. Add `--graph <pdg\|provenance\|cfg>` flag to CLI. | `rparse-pdg`, `rparse-dataframe`, `rparse-cli` | `rparse analyze <file> --graph pdg --format dot` outputs valid Graphviz DOT; `--format json` outputs node/edge graph topology. |
| **P1** | **G2** | **Sparse Demonstration Fixtures** | Only `fixtures/canonical.R` exists (3 lines). Rich benchmarks exist only in test code. | Create a dedicated `fixtures/` showcase directory with documented canonical scripts: `showcase_pipeline.R`, `partial_retention.R`, `filter_provenance.R`, and `uncertainty.R`. | `implementation/fixtures` | Each fixture has a documented expected provenance derivation and slice result runnable via CLI. |
| **P1** | **G3** | **Unexposed Partial Statement Slices** | Slicer identifies `PartiallyRetained` elements, but CLI output only reports statement line bounds. | Enhance `SliceResult` and CLI formatters to report exact retained vs. excluded expressions and argument names for partially retained statements. | `rparse-slicing`, `rparse-cli` | CLI `slice --format json` includes `partially_retained_elements` with exact sub-statement coordinates. |
| **P2** | **G4** | **Self-Contained Interactive HTML Visualizer** | Zero frontend exists; graph outputs cannot be viewed interactively without third-party Graphviz tools. | Implement a zero-dependency, standalone HTML/SVG report generator (`rparse report <FILE> --output report.html`). Embeds an interactive SVG graph viewer (D3 or vis-network standalone bundle), source code viewer with slice highlighting, and schema tables. | G1, G2, G3 | Running `rparse report fixtures/canonical.R` produces a single standalone `.html` file opening directly in any browser with interactive DAG and slice overlays. |

---

## Recommended Architecture for Visualization Layer

To strictly uphold **Rule 8 of `AGENTS.md`** (*"Future layers must NOT be implemented without authorization"*), the visualization layer should **NOT** introduce a heavyweight web framework, Node.js daemon, or background web server into the Rust workspace.

Instead, the cleanest, most portable architecture is a **Static Standalone HTML Report Generator**:
1. **Core Independence:** `rparse-cli` emits a self-contained, offline HTML file (similar to `cargo-llvm-cov` or `pytest-html`).
2. **Zero Runtime Dependencies:** Requires zero web servers, npm installs, or background processes.
3. **Components:**
   - **Interactive DAG Viewer:** Renders the Column Provenance DAG and Unified PDG using client-side SVG.
   - **Code Slice Overlay:** Displays the original R script with green highlights for retained code, yellow for partial retentions, and strike-through/gray for excluded statements.
   - **Schema Inspector:** Shows dataframe version transitions ($v_0 \to v_1 \to v_2$) with column addition/removal tags.
