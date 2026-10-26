# R-PARSE — Comprehensive Ecosystem Research Report

**Lead Research Engineer & Software Archaeology Agent Report**  
**Date:** October 2026  
**Status:** Historical snapshot (pre-architecture and pre-scope freeze)

The canonical external-project record is now [`reference-landscape.md`](reference-landscape.md). Two inventory rows below are superseded in specificity and were left in place so this snapshot stays intact:

- tree-sitter-r and the pipe placeholder `_`: at local commit `58a2279`, `grammar.js` accepts a lone `_` as an identifier and does not give it a separate token. See the landscape.
- flowR and dataframes: shape analysis exists and is not what the slicer walks. See [`flowr-deep-gap-audit.md`](flowr-deep-gap-audit.md).

The paths `research/repos/...` in the inventory named local checkouts. Those checkouts are not part of the public tree.

---

## Executive Summary

This research report documents a deep, repository-level investigation of the existing R programming language tooling ecosystem. Across ten cloned and inspected repositories, dozens of academic papers, and official language specifications, this study investigates what already exists, how existing systems function internally, where capabilities overlap, and which technical gaps genuinely remain.

### Key Strategic Findings:
1. **The Ecosystem is Fragmented into Four Isolated Islands:**
   - *High-Performance Parsing:* Standalone, fault-tolerant C parsers (`tree-sitter-r`) with zero semantic understanding.
   - *R-Dependent Syntax Tools:* Rich IDE and style checking (`lintr`, `languageserver`) handicapped by base R runtime dependencies, single-threaded locks, and slow XML DOM conversions (`xmlparsedata`).
   - *Academic Program Analysis:* Pioneering dataflow, CFG, and program slicing engines (`flowR`, `CodeDepends`) isolated from mainstream developer workflows and production IDEs.
   - *Workflow Orchestration & Provenance:* State-of-the-art execution DAGs and runtime tracking (`targets`, `rdtLite`) hindered by coarse static approximations that generate false positives on modern tidyverse code.
2. **Critical Evidence-Backed Opportunities:**
   - **Standalone Native Performance:** Building a native (Rust/C++) engine that executes lints, formatting, and analysis in milliseconds with zero GNU R runtime dependencies.
   - **Tidyverse Schema Awareness:** Gradual, best-effort column lineage tracking through dplyr verbs (`select`, `mutate`, `filter`), eliminating the false-global dependency problem.
   - **Domain-Specific R Semantics:** First-in-class intermediate representations for statistical formulas (`~`) and Grammar of Graphics layer accumulation (`ggplot2` / `+.gg`).
   - **Production IDE Program Slicing:** Exposing flowR-style backward slicing as an interactive Language Server Protocol (LSP) code action for automated minimal reproducible example (reprex) generation.
3. **Explicit Non-Goals (What Not to Reinvent):**
   - Do NOT rewrite a new linter rule set from scratch: execute the 118 established rules of `lintr`.
   - Do NOT replace `targets` as an execution scheduler: provide better static dependency graphs *for* targets.
   - Do NOT invent a non-standard AST from scratch: extend the battle-tested two-tier pattern (Tree-sitter CST -> Normalized Semantic AST).

---

## 1. Inventory of Inspected Repositories

| Repository | Source Location | Focus / Role in Ecosystem | Key Finding |
| :--- | :--- | :--- | :--- |
| **tree-sitter-r** | `research/repos/tree-sitter-r` | Standalone C parser & CST | Fast, fault-tolerant, but missing R 4.2+ native pipe placeholder `_`. |
| **antlr-r** | `research/repos/grammars-v4/r` | ANTLR4 grammar specification | Severely obsolete; missing pipes, lambdas, raw strings. Unusable. |
| **xmlparsedata** | `research/repos/xmlparsedata` | Native parse table -> XML DOM | Thin data converter; creates severe performance bottlenecks in downstream tools. |
| **lintr** | `research/repos/lintr` | Static code linting (118 linters) | Gold standard rule catalog; entirely reliant on XPath 1.0 against XML DOM. |
| **flowR** | `research/repos/flowr` | Dataflow, CFG & program slicing | Most capable academic analyzer; models dataframes as monolithic scalars. |
| **languageserver** | `research/repos/languageserver` | Language Server Protocol (LSP) | Excellent OOP/type hierarchy detection; single-threaded R runtime bound. |
| **CodeDepends** | `research/repos/CodeDepends` | Dependency analysis & handlers | Pioneered domain-specific function handlers for tidyverse & formulas. |
| **targets** | `research/repos/targets` | Pipeline DAG orchestration | Uses `codetools::findGlobals()` on dummy functions; prone to NSE false globals. |
| **rdtLite** | `research/repos/rdtLite` | Dynamic provenance collection | Generates empirical PROV-JSON ground truth; provides benchmark oracle. |
| **rpy2** | `research/repos/rpy2` | Python-R in-process FFI bridge | Demonstrates runtime embedding vs static analysis trade-offs. |

---

## 2. Synthesized Architectural Comparisons

### 2.1 Grammar & Parser Architecture
- **GNU R Bison Reference:** 100% grammar compliance, but zero error tolerance; any syntax error immediately aborts parsing.
- **Tree-sitter-r:** The premier foundation for modern tooling. Provides lossless syntax preservation, sub-millisecond incremental reparsing, and robust error recovery.
- **Grammar Defect Identified:** Tree-sitter-r lacks grammar support for R 4.2+ pipe argument placeholder `_`. R-PARSE must patch or extend this grammar.

### 2.2 AST & Intermediate Representations
- **S-Expressions (Base R):** Lossy; comments and redundant parentheses are discarded. Unsuitable for source-to-source transformations or linting.
- **XML DOM (`xmlparsedata`):** Allows XPath queries, but suffers from severe memory allocation and serialization overhead.
- **Two-Tier Architecture (Tree-sitter CST -> Normalized Semantic AST):** Identified as the optimal design. The CST preserves all trivia for formatting and syntax highlighting; the normalized AST simplifies semantic passes, CFG construction, and dataflow tracking.

### 2.3 Domain Semantics: Statistical Formulas & Visualization
- **Wilkinson-Rogers Formulas (`~`):** Existing analyzers either treat `~` as an arithmetic operator (`flowR`) or substitute it with `identity` (`targets`). R-PARSE has an opportunity to introduce a dedicated `FormulaNode` in its AST that models response, fixed effects, interactions, and random group intercepts.
- **Grammar of Graphics (`ggplot2`):** Every existing parser treats `+.gg` layer composition as arithmetic addition. R-PARSE can model the accumulated layer stack, catching misplaced constants in `aes()` and validating aesthetic channels statically.

---

## 3. Adversarially Validated Gaps

Through adversarial stress testing, the candidate gaps were filtered to five high-confidence pillars:

1. **Standalone Native Speed & Portability:** Data science CI/CD and pre-commit hooks demand sub-100ms execution without installing or compiling GNU R and heavy package dependencies.
2. **Defect-Free Modern Parsing:** Extended Tree-sitter grammar covering 100% of R 4.0–4.4 syntax.
3. **Domain-Specific Diagnostics:** Catching notorious silent bugs in ggplot2 and formula specifications before runtime.
4. **Gradual Schema Tracking:** Eliminating false positive dependencies in tidyverse data pipelines through best-effort column awareness.
5. **Practical IDE Slicing:** Exposing flowR-style backward slicing directly into editors for automated minimal reprex generation.

---

## 4. Architectural Alternatives for Future Consideration

Four architectural options have been documented in [`architecture-options.md`](architecture-options.md):
- **Option A (Pure Rust Standalone):** Maximum speed, zero dependencies, single-binary distribution. Highest initial build effort.
- **Option B (TypeScript / Node.js Engine):** Fast prototype extending `flowR`. Heavy runtime footprint; unpopular for CLI tooling.
- **Option C (Pure R + C FFI):** Native CRAN package. Reintroduces R interpreter bottlenecks and runtime dependencies.
- **Option D (Hybrid Rust Core + Optional R IPC Bridge):** Standalone Rust engine for 95% of tasks, with an optional background daemon for dynamic environment inspection when GNU R is installed.

---

## 5. Scope & Decision Gates (Next Steps)

In strict accordance with the research protocol:
- **Scope is NOT frozen yet.**
- **Architecture is NOT selected yet.**
- **Implementation has NOT started.**

### Recommended Next Steps for the Engineering Team:
1. **Review and Align on Findings:** Validate the synthesized feature matrix and gap analysis with project stakeholders.
2. **Execute Benchmark Baseline:** Implement the test suite defined in [`benchmark-plan.md`](benchmark-plan.md) to measure baseline throughput on `ggplot2`, `dplyr`, and `data.table` using existing tools.
3. **Formal Architecture Decision Record (ADR):** Evaluate Option A vs Option D to determine whether standalone Rust or a hybrid engine best meets project requirements.
4. **Technical Specification:** Author the formal AST schema and dataflow IR specifications before writing production code.
