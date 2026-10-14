# R Ecosystem Feature & Capability Matrix

This matrix provides a comprehensive, evidence-backed comparison of the major projects in the R programming-language analysis ecosystem. Every entry is derived directly from cloned repository source code, test suites, and grammar definitions.

---

## 1. High-Level Architecture & Parsing Matrix

| Project | Implementation Language | License | Standalone (No R Runtime Required) | Parser Architecture | Fault Tolerant Error Recovery | Incremental Reparsing | Preserves Comments & Trivia |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **tree-sitter-r** | C / JS (Grammar DSL) | MIT | **Yes** | GLR / LR(1) Parser | **Yes** (Tree-sitter error recovery nodes) | **Yes** (Sub-millisecond subtree reuse) | **Yes** (Explicit comment tokens) |
| **antlr-r** | ANTLR4 | BSD-3 | **Yes** | LL(*) Adaptive Predictor | Partial (ANTLR standard resync) | No | No (Trivia routed to hidden channel) |
| **xmlparsedata** | R | MIT | **No** | Base R C Bison parser (`getParseData()`) | **No** (Fails on syntax error) | No | Yes (Fixes negative comment parents) |
| **lintr** | R | MIT | **No** | Base R C parser via `xmlparsedata` | **No** (Reports parse errors as lints) | No | Yes (Via parse table text) |
| **flowR** | TypeScript | LGPL-3.0 | **Yes** (via tree-sitter backend) / Optional R | Dual: `tree-sitter-r` or R shell `parse()` | **Yes** (when using tree-sitter backend) | Partial (via tree-sitter backend) | Yes |
| **languageserver** | R | MIT | **No** | Base R C parser via `xmlparsedata` | **No** (Invalid syntax breaks LSP tree) | No | Yes |
| **CodeDepends** | R | GPL-2+ | **No** | Base R C parser (`parse()`) | **No** (Throws R parse exception) | No | No (Drops comments during S-exp parse) |
| **targets** | R | MIT | **No** | Base R C parser (`embody_expr`) | **No** (Throws R parse exception) | No | No (Deparse-based normalization) |
| **rdtLite** | R | GPL-3.0 | **No** | Base R C parser (`parse()`) | **No** (Halts on unparseable script) | No | No |
| **rpy2** | Python / C | GPL-2+ | **No** (Embeds `libR`) | Base R C parser (`ri.parse()`) | **No** (Throws R interface error) | No | No |

---

## 2. Program Representation & Intermediate Representation (IR) Matrix

| Project | Primary Intermediate Representation (IR) | Normalized AST | Control Flow Graph (CFG) | SSA / Def-Use Chains | Call Graph Construction | Symbol Scope Table |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **tree-sitter-r** | Concrete Syntax Tree (CST) | No | No | No | No | No |
| **antlr-r** | Parse Tree (CST) | No | No | No | No | No |
| **xmlparsedata** | XML DOM Document (`<exprlist>`) | No | No | No | No | No |
| **lintr** | XML DOM (`xml2` document) | No | No | No | No | Lexical token search only |
| **flowR** | Normalized TypeScript AST + Dataflow Graph | **Yes** (`ast/model/model.ts`) | **Yes** (Intra-procedural Basic Blocks) | **Yes** (Statement-level SSA versioning) | **Yes** (`call-graph/call-graph.ts`) | **Yes** (Lexical environment hierarchy) |
| **languageserver** | Cached XML DOM + R session symbols | No | No | No | **Yes** (`call_hierarchy.R`) | **Yes** (Scoped XPath variable resolution) |
| **CodeDepends** | R Language S-expressions | No | No | Block Def-Use (`inputCollector`) | **Yes** (`R/callGraph.R`) | Partial (Dynamic env scoping) |
| **targets** | Function S-expressions + `igraph` DAG | No | No | Coarse target DAG dependencies | No | Global scoping via `codetools` |
| **rdtLite** | S-expressions + PROV-JSON DDG | No | No | Runtime Def-Use (`DDGStatement.R`) | Runtime Call Stack | Runtime R Environment Frames |
| **rpy2** | R S-expressions (`LangVector`) | No | No | No | No | Full R Runtime Environments |

---

## 3. Advanced Static Analysis & Slicing Matrix

| Project | Intra-procedural Dataflow | Inter-procedural Dataflow | Static Program Slicing | Dead Code Elimination | Abstract Interpretation / Type Inference | Cross-File Package Analysis |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **tree-sitter-r** | None | None | None | None | None | None |
| **antlr-r** | None | None | None | None | None | None |
| **xmlparsedata** | None | None | None | None | None | None |
| **lintr** | None | None | None | Syntactic warning (`unused_import`) | None | No (File-by-file batch) |
| **flowR** | **Yes** (Reaching definitions) | **Partial** (Built-in signature DB) | **Yes** (Backward static slices) | **Yes** (CFG dead code detection) | Basic value/type propagation | Partial (File loading support) |
| **languageserver** | None | None | None | Syntactic unused symbol warning | None | **Yes** (Workspace-wide symbol index) |
| **CodeDepends** | Coarse (Expression I/O) | Partial (Function dependency graph) | No | No | None | Partial (Source file tracking) |
| **targets** | None | Target DAG level | Target-level pruning | Outdated target pruning | None | **Yes** (Multi-file pipeline scripts) |
| **rdtLite** | Dynamic trace only | Dynamic trace only | Dynamic provenance slicing | No | Observed runtime types only | Dynamic `source()` trace |
| **rpy2** | None | None | None | None | Runtime introspection | Runtime package loading |

---

## 4. Domain-Specific R Semantic Understanding Matrix

| Project | Tidyverse / Pipe / NSE Awareness | Statistical Formula Semantics (`~`) | Visualization Introspection (`ggplot2`/`plot`) | OOP / Class Hierarchy (S3, S4, R6) | Runtime Execution Grounding |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **tree-sitter-r** | Syntactic operator only | Syntactic operator only | None | None | None |
| **antlr-r** | Missing modern pipes (`\|>`) | Missing formula operator rules | None | None | None |
| **xmlparsedata** | Token tag (`OP-LAMBDA`, `PIPE`) | Token tag (`OP-TILDE`) | None | None | None |
| **lintr** | Ad-hoc XPath rules | Ignored / treated as expression | None | Limited linting (`class_equals_linter`) | None |
| **flowR** | **Partial** (Dataflow NSE heuristic) | Generic binary operator | None | Basic class declaration AST nodes | Optional R shell bridge |
| **languageserver** | Pipeline completion | Ignored in definition search | None | **Yes** (`type_hierarchy.R`, S3/S4/RC/R6) | **Yes** (Interactive background session) |
| **CodeDepends** | **Yes** (70+ specialized handlers in `functionHandlers.R`) | **Yes** (`formulahandler`, isolates model vars) | **Partial** (`isPlot.R` plot detection) | None | Heuristic evaluation fallback |
| **targets** | Masking prone (false globals in dplyr) | `substitute(~, identity)` to catch vars | None | None | Caches executed targets |
| **rdtLite** | Masking prone (false uses in dplyr) | Evaluated dynamically | **Partial** (`IOTrace.R` device hooking) | Dynamic S4 dispatch | **Full** (PROV-JSON execution trace) |
| **rpy2** | Evaluated via R runtime | Evaluated via `stats::model.frame` | Saves plot graphics via R devices | Full runtime R object dispatch | **Full** (In-process R execution) |

---

## 5. Summary of Ecosystem Fragmentation

The matrix reveals four fragmented "islands" of tooling that have never been unified:
1. **The High-Performance Native Parsing Island (`tree-sitter-r`):** Fast, fault-tolerant, standalone, and incremental—but completely devoid of semantic analysis, dataflow, or R domain understanding.
2. **The R-Dependent Syntax Tree Island (`xmlparsedata`, `lintr`, `languageserver`):** Leverages base R's parser via XML translation and XPath. Rich IDE and style checking, but crippled by XML serialization latency, single-threaded execution, and lack of true dataflow analysis.
3. **The Academic Program Analysis Island (`flowR`, `CodeDepends`):** Pioneered dataflow graphs, program slicing, and NSE function handlers—but isolated from mainstream IDE workflows, lacking standalone packaging (CodeDepends requires legacy R libraries; flowR requires Node.js/TypeScript runtime and has limited CRAN deployment).
4. **The Execution & Workflow Island (`targets`, `rdtLite`, `rpy2`):** World-class pipeline management, execution caching, and runtime provenance—but relying on primitive static approximations (`codetools::findGlobals`), resulting in persistent false positives with modern tidyverse code.
