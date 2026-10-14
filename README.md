# R-PARSE

**R-PARSE** is a broad-compatibility R program analysis system that extends semantic program representation with fine-grained column provenance and uses that information to enable column-aware static program slicing.

---

## Research Contribution

Standard program slicing and dependency analysis systems treat dataframes as atomic, monolithic variables. In data-centric R programming, operations frequently transform individual columns while preserving or ignoring others. Treating dataframes as coarse objects leads to severe over-approximation, retaining entire data-wrangling pipelines even when querying a single isolated column.

R-PARSE solves this problem through two core technical contributions:
1. **Fine-Grained Column Provenance ($C_{in} \to C_{out}$):** Modeling column-level derivations across tidyverse (`mutate`, `transmute`, `select`, `rename`, `filter`) and base R operations, tracking explicit schema versions and column transformation chains.
2. **Column-Aware Static Program Slicing:** A unified Program Dependence Graph (PDG) featuring fine-grained column ports, variable data dependencies, control dependencies, row-selection dependencies, and explicit uncertainty edges (`UNKNOWN_STATIC`), enabling backward static slicing that eliminates operations on unrelated sibling columns.

---

## Two-Tier Hybrid Architecture

Per [ADR-001](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/ADR-001-architecture.md), R-PARSE implements a two-tier hybrid architecture:

```text
                             R-PARSE
                                │
        ┌───────────────────────┴───────────────────────┐
        ▼                                               ▼
   Analysis Core (Tier 1)                      Runtime Bridge (Tier 2)
   [Deterministic Rust]                         [Optional GNU R]
        │                                               │
   Tree-sitter Parser                               Subprocess
        ↓                                           Isolation
   Semantic IR & Scope                                  │
        ↓                                           JSON-RPC 2.0
   CFG & Dataflow Analysis                              │
        ↓                                       Schema Observation
   DataFrame Semantics & Lineage                        │
        ↓                                               │
   Column Provenance Engine                             │
        ↓                                               │
   Unified PDG & Column Slicing                         │
        │                                               │
        └───────────────────────┬───────────────────────┘
                                │
                                ▼
                       CLI Interface Layer
                          (rparse-cli)
                                │
                     ┌──────────┴──────────┐
                     ▼                     ▼
                Text Output           JSON Output
              (Human-readable)      (Machine-readable)
```

- **Tier 1 (Rust Analysis Engine):** 100% autonomous, deterministic, and offline. Never spawns child processes or invokes GNU R.
- **Tier 2 (GNU R Runtime Bridge):** Strictly optional, isolated subprocess bridge (`Rscript --vanilla`) using framed JSON-RPC 2.0 over stdio for empirical schema observation.
- **CLI Layer (`rparse-cli`):** Thin orchestrator and output formatter over the analysis core. Contains zero analysis logic.

---

## Installation & Build

Build R-PARSE using Cargo from the `implementation/` workspace:

```powershell
# Build release executable
cargo build --workspace --release

# The compiled binary is located at:
# target/release/rparse (or target/release/rparse.exe on Windows)
```

To run all tests across the 11 workspace crates:

```powershell
cargo test --workspace
```

---

## Unified Web Service & Scientific Analysis Studio

R-PARSE serves a unified single-origin web application containing its editorial landing page, interactive Scientific Analysis Studio workspace, documentation guide, and live Rust analysis engine.

### One-Command Local Startup

Launch the integrated service with automatic browser launch:

```powershell
# Windows PowerShell
.\start.ps1

# Or run directly with Cargo:
cd implementation
cargo run -p rparse-cli -- studio --open
```

```bash
# Linux / macOS
./start.sh

# Or run directly with Cargo:
cd implementation
cargo run -p rparse-cli -- studio --open
```

The application starts on **`http://localhost:8080`** under a single origin:

| Route | Description |
| :--- | :--- |
| **`GET /`** | Editorial Investigation landing page with animated launch transition. |
| **`GET /studio`** | Interactive Scientific Analysis Studio workspace with five analytical views. |
| **`GET /how-to-use`** | Beginner-friendly tutorial and reference guide. |
| **`GET /api/health`** | Service health status and version verification. |
| **`POST /api/analyze`** | Authoritative static analysis engine running 100% deterministic Rust analysis. |
| **`GET /favicon.svg`** | Canonical R-PARSE 4-quad brand SVG asset. |
| **Static Assets** | Integrated CSS, JavaScript, and typography assets. |

### Docker Production Deployment

Build and run the reproducible multi-stage Docker container:

```bash
# 1. Build production container
docker build -t rparse .

# 2. Run container (serves landing page, studio, and API automatically)
docker run -p 8080:8080 -e PORT=8080 rparse
```

The service reads the listening port from the `PORT` environment variable (compatible with Railway, Render, Fly.io, Google Cloud Run, and AWS ECS).

---

## CLI Usage

### Commands

| Command | Description |
| :--- | :--- |
| `rparse analyze <FILE>` | Runs the complete static analysis pipeline and summarizes program structure, dataframe schemas, and PDG statistics. |
| `rparse provenance <FILE> --column <COL>` | Traces fine-grained input-to-output derivation chains for a specific column (`df$col`). |
| `rparse slice <FILE> --criterion <CRIT>` | Computes a column-aware backward static slice from a targeted criterion (`df$col`). |
| `rparse --version` | Displays the stable project version. |
| `rparse --help` | Displays help information and CLI documentation. |

### Global Options

- `--format <text|json|dot|html>`: Output format (default: `text`).
  - `text`: Structured, human-readable console output.
  - `json`: Machine-readable JSON topology and results.
  - `dot`: Deterministic Graphviz DOT digraph.
  - `html`: Standalone offline interactive HTML visualizer workbench (supports `-o / --output <FILE>`).
- `--graph <pdg|provenance|cfg>`: Selects graph topology for export (used with `analyze`).
- `-o, --output <PATH>`: Writes command output directly to a target file.
- `--mode <static|assisted|execute>`: Runtime execution mode (default: `static`).
  - `static`: Strictly offline analysis; never invokes GNU R.
  - `assisted`: Bounded on-demand runtime schema observation.
  - `execute`: Explicit full GNU R execution permission.
- `--seed <SYMBOL>`: Pre-seeded dataframe symbols for schema discovery (repeatable).

---

## Canonical Demonstrations

### 1. Fine-Grained Column Provenance

Given `example.R`:
```r
df <- data.frame(price = 10, quantity = 5)
df$total <- df$price * df$quantity
df2 <- mutate(df, tax = total * 0.1)
```

Trace derivation of `df$total`:
```bash
rparse provenance example.R --column 'df$total'
```
**Output:**
```text
R-PARSE COLUMN PROVENANCE
─────────────────────────
Target Column:     df$total (v1)
Certainty State:   KNOWN_STATIC

Direct Input Columns:
  ← df$price (v0)
  ← df$quantity (v0)

Transitive Source Columns:
  ← df$price (v0)
  ← df$quantity (v0)

Derivation Chain:
  [BaseRDirectAssignment] df$price, df$quantity → df$total
```

Trace derivation of multi-step column `df2$tax`:
```bash
rparse provenance example.R --column 'df2$tax'
```
**Output:**
```text
R-PARSE COLUMN PROVENANCE
─────────────────────────
Target Column:     df2$tax (v2)
Certainty State:   KNOWN_STATIC

Direct Input Columns:
  ← df$total (v1)

Transitive Source Columns:
  ← df$price (v0)
  ← df$quantity (v0)

Derivation Chain:
  [Mutate] df$total → df2$tax
  [BaseRDirectAssignment] df$price, df$quantity → df$total
```

### 2. Column-Aware Static Program Slicing

Slice relative to `df$total`:
```bash
rparse slice example.R --criterion 'df$total'
```
**Output:**
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

The downstream calculation `df2 <- mutate(...)` is recognized as irrelevant to `df$total` and excluded (`[NONE]`), demonstrating column-level isolation.

---

## Machine-Readable JSON Output

Every command supports `--format json` with stable, documented JSON schemas:

```bash
rparse slice example.R --criterion 'df$total' --format json
```
```json
{
  "command": "slice",
  "confidence": "COMPLETE",
  "contains_unknowns": false,
  "criterion": "df$total",
  "file": "example.R",
  "fully_retained_statement_count": 2,
  "partially_retained_statement_count": 0,
  "retained_element_count": 14,
  "statements": [
    { "start_line": 1, "end_line": 1, "state": "fully_retained" },
    { "start_line": 2, "end_line": 2, "state": "fully_retained" },
    { "start_line": 3, "end_line": 3, "state": "not_retained" }
  ],
  "status": "success",
  "tool": "rparse",
  "version": "0.1.0"
}
```

---

## Interactive HTML Visualizer Workbench

R-PARSE includes a zero-dependency, self-contained offline HTML visualizer that renders interactive SVG graphs, synchronized source code spans, static backward slices with sub-statement attributions, and dataframe schema evolution:

```bash
# Generate standalone HTML technical report
rparse analyze fixtures/showcase_pipeline.R --format html -o showcase_report.html
```

The resulting `showcase_report.html` is completely offline, loads instantly in any modern browser without web servers, and embeds the complete ground-truth dataset in a secure JSON payload.

---

## Canonical Showcase Corpus

The repository includes canonical showcase fixtures in `fixtures/`:

| Fixture | Description | Command |
| :--- | :--- | :--- |
| [`fixtures/showcase_pipeline.R`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/fixtures/showcase_pipeline.R) | Multi-step tidyverse pipeline combining filtering, column derivation, and sub-statement pruning. | `rparse slice fixtures/showcase_pipeline.R --criterion 'df2$total'` |
| [`fixtures/partial_retention.R`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/fixtures/partial_retention.R) | Sub-statement mutate pruning: demonstrates `[KEEP]` vs `[DROP]` regions within a single statement. | `rparse slice fixtures/partial_retention.R --criterion 'df2$total'` |
| [`fixtures/filter_provenance.R`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/fixtures/filter_provenance.R) | Row-selection dependency preservation across sequential filter stages. | `rparse slice fixtures/filter_provenance.R --criterion 'df3$total'` |
| [`fixtures/uncertainty.R`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/fixtures/uncertainty.R) | Explicit uncertainty tracking: dynamic column indexing handled honestly as `UNKNOWN_STATIC`. | `rparse slice fixtures/uncertainty.R --criterion 'df2$dynamic_col'` |
| [`fixtures/adversarial/adv04_alias_rename.R`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/implementation/fixtures/adversarial/adv04_alias_rename.R) | Simple dataframe variable aliasing (`df_alias <- df`) propagating column versions and provenance. | `rparse slice fixtures/adversarial/adv04_alias_rename.R --criterion 'df_final$final_score'` |

---

## Exit Codes

| Code | Name | Description |
| :---: | :--- | :--- |
| **0** | `SUCCESS` | Command completed successfully. |
| **1** | `ANALYSIS_ERROR` | Analysis, resolution, or runtime error occurred (e.g., column not found). |
| **2** | `USAGE_ERROR` | Invalid CLI arguments, missing required options, or unrecognized flags. |
| **3** | `INPUT_ERROR` | Input file error (file missing, unreadable, or invalid UTF-8). |
| **4** | `INTERNAL_ERROR` | Unexpected internal panic or catastrophic error. |

---

## Explicit Boundaries & Limitations

R-PARSE is engineered strictly within its frozen research scope:
- **No Universal R Semantic Understanding:** Dynamic metaprogramming (`eval`, `substitute`), non-standard evaluation beyond supported tidyverse verbs, and arbitrary C/Fortran calls are not modeled as concrete semantics.
- **Explicit Uncertainty (`UNKNOWN_STATIC`):** Where dynamic behaviors preclude exact static resolution, R-PARSE marks columns and dependencies as `UNKNOWN_STATIC` rather than guessing or fabricating edges.
- **No Standalone Executable Reprex Generation:** Slicing operates at the semantic IR level; R-PARSE does not rewrite or resynthesize executable R source text.
- **No Interpretation or Replacement of GNU R:** R-PARSE is an analysis tool, not an R execution environment.
- **Out of Scope:** Web interfaces, GUI dashboards, Language Server Protocol (LSP) servers, and AI/LLM natural language wrappers are intentionally excluded from this engine.
