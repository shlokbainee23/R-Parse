# R-PARSE — Architectural Overview

## Decision: Option D (Hybrid Rust Core + GNU R Runtime Bridge)
Per [`research/ADR-001-architecture.md`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/ADR-001-architecture.md), R-PARSE implements a two-tier hybrid model:

```text
                      R-PARSE
                         │
             ┌───────────┴───────────┐
             ▼                       ▼
        RUST CORE               GNU R RUNTIME
        (Tier 1)                  (Tier 2)
       [Autonomous]              [Optional]
             │                       │
       Parsing / CST             Execution
       Semantic IR               Inspection
       Scope Analysis            Runtime Evidence
       CFG / Dataflow
       Provenance Engine
       Slicing Engine
             │                       │
             └────── Runtime Bridge ─┘
```

## Tier 1 — Rust Core
- **Responsibility:** Ingests R source code, builds concrete syntax trees (CST), normalizes to semantic AST, constructs CFG and def-use chains, computes column provenance, and executes backward slicing.
- **Independence:** 100% standalone, deterministic, and offline. Never requires a working R runtime or external network access.
- **Safety:** Implemented in safe, memory-managed Rust with zero native segfault risks.

## Tier 2 — GNU R Runtime Bridge (Deferred Beyond Phase 1)
- **Responsibility:** When explicitly invoked, delegates expression execution or dataframe schema inspection to a real GNU R subprocess to resolve dynamic ambiguity.
- **Evidence Tagging:** Annotates states with empirical evidence (`RuntimeEvidence::Observed`) without corrupting static analysis models.
- **Non-Goal:** Does not replace GNU R.

## Phase 1 Implementation Architecture
In Phase 1, only the foundational parsing tier is constructed in `implementation/`:

### 1. Workspace Organization
- **Location:** `implementation/`
- **Purpose:** Production Rust Cargo workspace isolating production code from exploratory research records in `research/`.

### 2. Core Abstractions (`rparse-core`)
- **Location:** `implementation/crates/rparse-core`
- **Responsibility:** Foundational, dependency-light domain primitives shared by all compiler passes:
  - `NodeId`: Compact 32-bit internal node identity.
  - `Span` & `Position`: Source coordinates.
  - `SourceFile` & `SourceId`: Document registry and byte-to-line index.
  - `Diagnostic` & `DiagnosticSeverity`: Structured compiler error/warning reporting.
  - `RParseError`: Canonical error enumerations implementing standard Rust error traits.
- **Isolation Constraint:** Must never depend on Tree-sitter, dataframes, CFG, provenance, slicing, or GNU R.

### 3. Parser & CST Layer (`rparse-parser`)
- **Location:** `implementation/crates/rparse-parser`
- **Parser Responsibility:** Ingest UTF-8 R source code into a robust Tree-sitter parse tree. Guarantee zero panics on valid or malformed input.
- **CST Responsibility:** Safe wrapper abstraction (`CstTree`, `CstNode`) around raw Tree-sitter C pointers. Preserves 1:1 token trivia and whitespace, exposes named/unnamed child navigation, field lookup, and collects syntax recovery diagnostics without premature semantic lowering.

### 4. Semantic IR & Scope Layer (`rparse-ir`, `rparse-scope`) [Phase 2]
- `rparse-ir`: Normalized semantic IR representations (`Program`, `Stmt`, `Expr`, `LValue`).
- `rparse-scope`: Lexical and data-masking scope analysis (`ScopeModel`, `Symbol`, `Definition`, `Reference`).

### 5. Control-Flow Graph & Dataflow Layer (`rparse-cfg`, `rparse-dataflow`) [Phase 3]
- `rparse-cfg`: Basic blocks, edges (`Normal`, `TrueBranch`, `FalseBranch`, `LoopBack`, `Return`), graph invariant validation, and DOT export.
- `rparse-dataflow`: Variable def/use collection, forward reaching definitions (fixed-point worklist), graph-based post-dominance, and control dependency computation.

### 6. DataFrame Semantics & Column Provenance Layer (`rparse-dataframe`) [Phase 4]
- `rparse-dataframe`: Immutable `DataFrameState`, `ColumnVersion` identity tracking, grouped derivation provenance hyperedges (`ProvenanceEdge`), row-selection dependencies (`RowSelectionDependency`), explicit `StaticKnowledge` (`KnownStatic`, `UnknownStatic`), recursive expression dependency extraction, left-to-right `mutate`/`transmute` evaluation, `select`/`rename`/`filter` semantics, base R extraction/subsetting, and CFG reaching-definition integration.

### 7. Unified Program Dependence Graph Layer (`rparse-pdg`) [Phase 5]
- `rparse-pdg`: Constructs and validates the unified `ProgramDependenceGraph` integrating:
  - Scalar variable data dependencies (`VariableDataDependency`)
  - Column derivation events (`ColumnDerivation`)
  - Column-to-statement/expression bindings (`ColumnStatementBinding`)
  - Control dependencies (`ControlDependency`)
  - Row-selection dependencies distinct from control flow (`RowSelectionDependency`)
  - Explicit uncertainty edges (`UnknownDependency`)
  - Exposes fine-grained column ports (`ColumnVersion` nodes) and validates graph invariants.

### 8. Column-Aware Slicing Layer (`rparse-slicing`) [Phase 5]
- `rparse-slicing`: Executes graph-based static backward slicing over the Unified PDG:
  - Slicing criteria: scalar variables, whole dataframe objects, individual column ports (`df$col`), and named columns.
  - Granular retention: retains only relevant column derivations while eliminating unrelated sibling columns (`df$unused`).
  - Partial semantic retention: distinguishes `FullyRetained`, `PartiallyRetained`, and `NotRetained` statements.
  - Slice confidence reporting: `Complete` vs `PartialContainsUnknowns`.
  - Deterministic traversal and ordering.

### 9. GNU R Runtime Bridge Layer (`rparse-runtime`) [Phase 6]
- `rparse-runtime`: Implements the Tier 2 GNU R runtime bridge:
  - Process management: Spawns and manages GNU R child processes (`Rscript --vanilla`) communicating over stdio.
  - Wire protocol: Robust JSON-RPC 2.0 with `Content-Length` framing and newline-delimited fallback.
  - Controlled evaluation environment: Evaluates code in a temporary `new.env(parent = globalenv())` environment without polluting `.GlobalEnv`.
  - Operations: Code evaluation, object inspection, dataframe schema discovery, and graceful shutdown.
  - Empirical evidence model: Preserves orthogonal `StaticKnowledge` (`KnownStatic`, `UnknownStatic`) and `RuntimeEvidence` (`None`, `Observed`).
  - Analysis modes: `Static` (strictly forbidden from launching R), `Assisted` (bounded on-demand queries), `Execute` (explicit full execution).
  - Configurable timeouts and guaranteed child process cleanup.
  - Correlation models: Correlates static dataframe states with observed columns (`Confirmed`, `EmpiricalOnly`, `StaticOnly`, `Unobserved`).

### 10. CLI & Production Orchestration Layer (`rparse-cli`) [Phase 7]
- `rparse-cli`: Implements the user-facing command-line interface:
  - Commands: `analyze` (static pipeline summary), `provenance` (fine-grained column derivations), `slice` (column-aware static backward slicing).
  - Output formats: `--format text` (human-readable summary) and `--format json` (stable machine-readable JSON schema).
  - Runtime mode flags: `--mode static` (default, zero GNU R execution), `--mode assisted` (bounded runtime observation), `--mode execute` (full execution).
  - Standard exit codes: 0 (Success), 1 (AnalysisError), 2 (UsageError), 3 (InputError), 4 (InternalError).
  - Orchestration pipeline: Reusable `AnalysisContext` orchestrating parser → IR → scope → CFG → dataflow → dataframe → PDG → slicing without duplicating compiler graph construction.
  - Pure interface: Contains zero analysis or transformation logic.

### 11. Source Span Conventions
- **Byte Offsets (`start_byte`, `end_byte`):** 0-based byte offsets into UTF-8 source buffer, half-open `[start_byte, end_byte)`.
- **Line Numbers (`start_line`, `end_line`):** 1-based line coordinates (line 1 is the first line).
- **Column Offsets (`start_col`, `end_col`):** 0-based byte/character column offsets.

### 12. Running the Test Suite
From the `implementation/` directory:
```powershell
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets --all-features
```
