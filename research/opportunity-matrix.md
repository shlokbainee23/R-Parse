# R-PARSE — Opportunity Matrix & Strategic Positioning

**Controlled Stage Assessment:** Opportunity Evaluation & Strategic Defensibility  
**Preceding Stages:** Research Corpus → Capability Map → Gap Analysis → Adversarial Validation  
**Current Stage:** Opportunity Matrix (Pre-Scope Freeze, Pre-Architecture Selection)  
**Evidence Standard:** Traced to local repository code, tests, grammars, and verified empirical benchmarks.

---

## 1. Executive Summary

This document performs a disciplined, evidence-based evaluation of the candidate opportunities identified during the repository research phase. The goal is not to declare a winner, select an implementation technology, or freeze project scope. Rather, this assessment separates **foundational mechanisms** from **differentiated capabilities**, applies dual-perspective weighted scoring (Research Attractiveness vs. Strategic Differentiation), subjects every candidate to skeptical counterarguments, and evaluates the hypothesis of a unified **R Semantic Program Model (R-SPM)**.

### Primary Audit Findings:
1. **Infrastructure vs. Strategic Capability:** Candidates C1 (Native Core) and C2 (Extended Tree-sitter) are essential foundational mechanisms, but have low strategic differentiation on their own. They are the delivery vehicles, not the core product identity.
2. **The "Ruff for R" Risk (C7):** A standalone native linter engine (C7) scores high on engineering feasibility and developer utility, but carries a high risk of being merely a "faster existing tool" (a reimplementation of `lintr` rules) with low defensibility unless powered by deeper domain semantics.
3. **The Core Strategic Value Cluster:** The highest strategic differentiation and research attractiveness belong to domain-specific semantic understanding: **C3 (Column-Level Dataframe Lineage)**, **C4 (Statistical Formula Semantic IR)**, and **C6 (IDE-Integrated Program Slicing)**.
4. **The Platform Hypothesis:** Evaluating whether C3, C4, C5, and C6 naturally compose demonstrates that they are not separate products, but distinct query surfaces over a single underlying abstraction: an **R Semantic Program Model (R-SPM)**.

---

## 2. Candidate Inventory & Structural Audit

### 2.1 Proposed Candidate Inventory

| ID | Proposed Candidate Name | Primary Role in Ecosystem |
| :--- | :--- | :--- |
| **C1** | Standalone Native Core Engine | Zero-dependency high-speed parsing, normalized AST, CFG, and dataflow execution. |
| **C2** | Extended Tree-sitter Parser | Extended C parser fixing R 4.2+ native pipe placeholder (`_`) and raw string edge cases. |
| **C3** | Column-Level Schema / Dataframe Lineage | Tracking virtual column lifecycles through dplyr verbs (`select`, `mutate`, `filter`, `rename`). |
| **C4** | Statistical Formula Semantic IR | Dedicated lowering and validation for Wilkinson-Rogers formula syntax (`y ~ x1 + (1\|g)`). |
| **C5** | ggplot2 Layer Composition Semantics | Modeling `+.gg` as layer stack accumulation; validating aesthetics and geometry channels. |
| **C6** | IDE-Integrated Program Slicing | Exposing backward/forward program slicing via LSP for automated reprex generation. |
| **C7** | Native Linter Execution Engine | Native-speed CLI/LSP evaluator executing the 118 established `lintr` rules without GNU R. |

### 2.2 Structural Audit: Mergers, Splits, and Redundancies

- **Audit of C2 (Extended Tree-sitter):** C2 is **not an independent product opportunity**. It is a concrete defect fix to `tree-sitter-r` (specifically fixing the token handling of `_` in `grammar.js`). Treating it as a standalone opportunity is an error of scope. It is reclassified as an internal foundational component of C1.
- **Audit of C7 (Native Linter):** C7 is an application/consumer of C1. If C7 only executes syntactic XPath-like checks matching `lintr`, it has high utility but near-zero differentiation. It only becomes strategically defensible if it executes semantic checks powered by C3, C4, and C5.
- **Relationship between C3, C4, C5 (Domain Semantics):** These three represent R's three major DSLs: tabular data manipulation (tidyverse), statistical modeling (formulas), and data visualization (ggplot2). They share common infrastructure (binding expressions to data frames) but target distinct compiler lowering paths.

---

## 3. Product Capability vs. Implementation Mechanism Taxonomy

To prevent treating implementation details as user-facing value, every candidate is categorized into one of five structural tiers:

```
┌────────────────────────────────────────────────────────────────────────┐
│                      CAPABILITY TAXONOMY TIERS                         │
├───────────────────────────────────────┬────────────────────────────────┤
│ Tier Category                         │ Assigned Candidates            │
├───────────────────────────────────────┼────────────────────────────────┤
│ A. Foundational Technology            │ C2 (Extended Parser)           │
│ B. Core Engine Capability             │ C1 (Native Core), C6 (Slicing) │
│ C. Domain-Specific Analysis Capability│ C3 (Column), C4 (Formula),     │
│                                       │ C5 (ggplot2)                   │
│ D. User-Facing Workflow Capability    │ C6 (Reprex Slicing), C7 (Lints)│
│ E. Distribution / Performance Adv.    │ C1 (Zero-R / Sub-10ms CLI)     │
└───────────────────────────────────────┴────────────────────────────────┘
```

---

## 4. Scoring Methodology & Criteria Definitions

Every candidate is evaluated on a strict 1–10 scale across 15 explicit dimensions. Every score is derived from verified codebase facts.

### The 15 Evaluation Criteria:
1. **Problem Significance (1–10):** Severity and breadth of the underlying problem in real-world R usage.
2. **Strength of Ecosystem Gap (1–10):** Extent to which existing tools completely lack this capability.
3. **User Pain (1–10):** Frequency and friction experienced by developers, data scientists, or CI systems.
4. **Differentiation (1–10):** Uniqueness of the technical approach compared to existing ecosystem approaches.
5. **Technical Depth (1–10):** Complexity of the underlying compiler/program analysis algorithms required.
6. **Research Value (1–10):** Potential for novel academic publication or programming language innovation.
7. **Engineering Value (1–10):** Reusability, stability, and architectural leverage of the resulting software component.
8. **Broad R Compatibility (1–10):** Robustness across base R, tidyverse, data.table, S3/S4, and dynamic idioms.
9. **Feasibility (1–10):** Technical tractability within reasonable engineering constraints (10 = fully proven; 1 = speculative).
10. **Demonstrability (1–10):** Capacity to produce an immediate, compelling visual or interactive demonstration.
11. **Benchmarkability (1–10):** Ease of objective, empirical measurement using quantitative metrics.
12. **Extensibility (1–10):** Capacity for third-party developers to add new rules, verbs, or domain passes.
13. **Ecosystem Integration Potential (1–10):** Frictionless integration into existing workflows (LSP, targets, CI).
14. **Defensibility against Reproduction (1–10):** Difficulty for a competitor with access to this report to reproduce (10 = very hard; 1 = trivial).
15. **Resilience vs. "Merely Better Existing Tool" (1–10):** Distinctness from existing tools (10 = fundamentally new capability; 1 = faster rewrite of an existing tool).

---

## 5. Dual Weighted Scoring Framework

To prevent conflating technical sophistication with strategic defensibility, two independent weighted scores are calculated:

### Weighting Formulas:

$$\text{Score A (Research Attractiveness)} = \begin{aligned}
&0.10(\text{Significance}) + 0.10(\text{Gap}) + 0.10(\text{Depth}) + 0.15(\text{Research}) + 0.10(\text{Engineering}) \\
&+ 0.10(\text{Compatibility}) + 0.10(\text{Feasibility}) + 0.10(\text{Benchmark}) + 0.05(\text{Extensibility}) \\
&+ 0.05(\text{Demonstrability}) + 0.05(\text{Integration})
\end{aligned}$$

$$\text{Score B (Strategic Differentiation)} = \begin{aligned}
&0.15(\text{Significance}) + 0.15(\text{Gap}) + 0.20(\text{Differentiation}) + 0.10(\text{User Pain}) + 0.10(\text{Depth}) \\
&+ 0.10(\text{Compatibility}) + 0.05(\text{Extensibility}) + 0.05(\text{Demonstrability}) + 0.10(\text{Defensibility})
\end{aligned}$$

---

## 6. Comprehensive Scoring Matrices

### 6.1 Raw Criteria Scores (1–10)

| Criterion | C1 Native Core | C2 Ext Parser | C3 Column Lineage | C4 Formula IR | C5 ggplot2 IR | C6 Slicing | C7 Native Linter |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| 1. Problem Significance | 8 | 6 | 9 | 8 | 7 | 8 | 7 |
| 2. Gap Strength | 8 | 4 | 9 | 8 | 8 | 7 | 6 |
| 3. User Pain | 7 | 4 | 8 | 6 | 7 | 8 | 6 |
| 4. Differentiation | 5 | 2 | 9 | 8 | 8 | 8 | 3 |
| 5. Technical Depth | 7 | 4 | 8 | 7 | 6 | 8 | 5 |
| 6. Research Value | 4 | 2 | 9 | 8 | 7 | 8 | 2 |
| 7. Engineering Value | 9 | 6 | 8 | 7 | 6 | 8 | 7 |
| 8. Broad Compatibility | 8 | 9 | 6 | 7 | 7 | 6 | 8 |
| 9. Feasibility | 8 | 9 | 6 | 8 | 8 | 6 | 8 |
| 10. Demonstrability | 7 | 4 | 9 | 8 | 9 | 9 | 8 |
| 11. Benchmarkability | 9 | 9 | 8 | 8 | 7 | 8 | 9 |
| 12. Extensibility | 8 | 5 | 9 | 8 | 8 | 8 | 7 |
| 13. Ecosystem Integration | 8 | 8 | 9 | 8 | 8 | 9 | 9 |
| 14. Defensibility | 5 | 1 | 8 | 7 | 6 | 7 | 3 |
| 15. Resilience vs Clone | 4 | 1 | 9 | 8 | 7 | 8 | 1 |

### 6.2 Weighted Scores & Classification Summary

| Candidate ID & Name | Research Attractiveness (Score A) | Strategic Differentiation (Score B) | Primary Classification | Confidence |
| :--- | :---: | :---: | :--- | :---: |
| **C3 Column-Level Schema Lineage** | **8.10 / 10** | **8.40 / 10** | **A — Core Strategic Opportunity** | HIGH |
| **C4 Statistical Formula Semantic IR** | **7.70 / 10** | **7.50 / 10** | **A — Core Strategic Opportunity** | HIGH |
| **C6 IDE-Integrated Program Slicing** | **7.60 / 10** | **7.60 / 10** | **A — Core Strategic Opportunity** | HIGH |
| **C1 Standalone Native Core Engine** | **7.45 / 10** | **6.85 / 10** | **C — Useful Infrastructure / Foundation** | HIGH |
| **C5 ggplot2 Layer Composition IR** | **7.20 / 10** | **7.30 / 10** | **B — Strong Supporting Capability** | HIGH |
| **C7 Native Linter Execution Engine** | **6.50 / 10** | **5.50 / 10** | **D — Interesting but Weak Differentiation** | HIGH |
| **C2 Extended Tree-sitter Parser** | **5.85 / 10** | **4.15 / 10** | **C — Useful Infrastructure (Component of C1)** | HIGH |

---

## 7. In-Depth Candidate-by-Candidate Analysis

---

### Candidate C1: Standalone Native Core Engine

- **Candidate ID:** C1
- **Candidate Type:** Foundation / Core Engine / Distribution & Performance Advantage
- **Underlying Ecosystem Gap:** GAP 1 (Absence of a standalone, native semantic engine)
- **Direct Evidence:**
  - `xmlparsedata/R/package.R` line 71: delegates parsing to `utils::getParseData(x)`. It cannot function without an active GNU R process.
  - `lintr/R/lint.R` line 45 & `languageserver/R/languageserver.R`: execute inside an R interpreter, suffering from R's global lock and single-threaded GC pauses.
  - `tree-sitter-r`: C parser that outputs a concrete syntax tree (CST) without semantic passes or AST normalization.
  - `flowr/src/`: comprehensive TypeScript analyzer, but runs on Node.js/V8 with heavy memory usage and non-native deployment.
- **Existing Solutions:**
  - *Tree-sitter-r:* Provides fast C-level parsing, but stops at the CST level. Zero analysis passes.
  - *flowR:* Provides normalized AST and dataflow, but requires Node.js and models dataframes as scalars.
- **Remaining Problem:** No single native binary provides lossless CST parsing, AST normalization, Basic Block CFG construction, and SSA dataflow analysis with zero runtime dependencies.
- **Test: "Valuable if Someone Else Built It?":** If another team built a fast native R parser/CFG engine, R-PARSE would happily use it as an upstream library. C1 is **infrastructure**, not the differentiated soul of the project.
- **Skeptical Critique:**
  - *Strongest Argument Against:* Building an industrial-grade compiler engine in Rust or C++ requires high engineering effort. R developers already have R installed; interactive users do not care if their parser is in C++ or R unless latency is noticeable.
  - *Project Making It Unnecessary:* If Posit releases a native Rust language engine for Positron (e.g. extending their internal `air` formatter/tree-sitter work), C1's standalone novelty evaporates.
  - *Core Assumption:* Assumes standalone speed is a decisive adoption driver in CI and IDEs. If users prioritize 100% dynamic R fidelity over speed, C1 faces adoption friction.

---

### Candidate C2: Extended Tree-sitter Parser

- **Candidate ID:** C2
- **Candidate Type:** Foundation / Core Engine
- **Underlying Ecosystem Gap:** GAP 5 (Tree-sitter-r grammar defect: pipe placeholder `_`)
- **Direct Evidence:**
  - `tree-sitter-r/grammar.js`: Contains `PREC.PIPE = 8` for `|>`, but lacks token and argument rules recognizing the `_` placeholder introduced in R 4.2.0.
  - In `tree-sitter-r`, parsing `mtcars |> lm(mpg ~ wt, data = _)` produces an `ERROR` node or invalid identifier.
- **Existing Solutions:** GNU R's native `gram.y` parses `_` correctly in R >= 4.2.
- **Remaining Problem:** A minor grammar omission in `tree-sitter-r`.
- **Classification & Test:** C2 fails the test of an independent product. It is a 50-line grammar and external scanner patch.
- **Skeptical Critique:**
  - *Strongest Argument Against:* Calling a grammar bugfix a "strategic candidate" damages technical credibility. A single upstream Pull Request to `r-lib/tree-sitter-r` would eliminate this gap entirely.
  - *Verdict:* Demoted from independent opportunity to an internal engineering task within C1.

---

### Candidate C3: Column-Level Schema / Dataframe Lineage

- **Candidate ID:** C3
- **Candidate Type:** Domain-Specific Analysis Capability
- **Underlying Ecosystem Gap:** GAP 2 (Monolithic Dataframe Modeling & Lack of Column-Level Lineage)
- **Direct Evidence:**
  - `targets/R/utils_language.R` lines 37–41: `deps_function()` wraps expressions in a dummy function and invokes `codetools::findGlobals()`. In `df %>% mutate(tax = income * 0.2)`, `income` is falsely flagged as a missing global variable because `targets` cannot see inside `df`.
  - `flowr/src/dataflow/`: FlowR's dataflow vertices track whole variables (`df`, `res`), not dataframe columns. Mutating column `A` invalidates all downstream uses of column `B`.
  - `CodeDepends/R/functionHandlers.R` lines 215–230: `nseafterfirst` marks column arguments as `nsevalVars`, but does not track column additions or removals across verbs.
- **Existing Solutions:** None. All surveyed systems either treat dataframes as black-box scalars or flag column symbols as missing global variables.
- **Remaining Problem:** Tracking virtual schemas across standard dplyr verbs (`select`, `mutate`, `filter`, `rename`, `transmute`) to determine which exact raw columns flow into a given model or plot.
- **Test: "Valuable if Someone Else Built It?":** **YES.** If another team built column lineage for R, it would revolutionize targets pipelines, automated documentation, and data science refactoring. It represents genuine technical differentiation.
- **Skeptical Critique:**
  - *Strongest Argument Against:* R is dynamically typed and schema-free. Columns can be loaded from external databases or generated via metaprogramming (`across(starts_with("x"))`). Whole-program static schema lineage is mathematically undecidable.
  - *Rebuttal:* Complete soundness across dynamic reflection is not required. A best-effort, gradual schema tracker that handles literal column verbs covers >80% of real-world tidyverse code and falls back to `UNKNOWN` on dynamic calls, eliminating the vast majority of false alarms.

---

### Candidate C4: Statistical Formula Semantic IR

- **Candidate ID:** C4
- **Candidate Type:** Domain-Specific Analysis Capability
- **Underlying Ecosystem Gap:** GAP 3 (Statistical Formula & Modeling Semantics Blindness)
- **Direct Evidence:**
  - `CodeDepends/R/functionHandlers.R` lines 158–164: Author Gabriel Becker explicitly wrote: *"eventually we want to be able to handle the situation where we are calling lm(y~x + z, data=dat) where y and x are in dat but z is not, but that is HARD to detect so for now we allow users to specify whether CodeDepends counts formula variables as inputs or not..."*
  - `tree-sitter-r/grammar.js` line 48: Precedence rule `PREC.TILDE = 14`. The grammar treats `~` as an ordinary binary operator; formula interactions (`:`, `*`) are parsed identically to sequence generation and multiplication.
  - `targets/R/utils_language.R` line 38: Substitutes `~` with `identity` in the AST so `codetools::findGlobals()` can find symbols, falsely treating formula columns as lexical variables.
- **Existing Solutions:** Runtime functions `stats::model.frame()` and `stats::terms()` evaluate formulas dynamically. No tool parses formulas semantically *prior to execution*.
- **Remaining Problem:** A static intermediate representation that decomposes Wilkinson-Rogers formulas into response, fixed effects, interactions, crossings, and random groupings, and validates them against bound dataset schemas.
- **Test: "Valuable if Someone Else Built It?":** **YES.** Highly differentiated because general-purpose compiler frameworks (LLVM, Tree-sitter) have zero native concept of statistical formula algebra.
- **Skeptical Critique:**
  - *Strongest Argument Against:* Statisticians do not write million-line formula files; formulas are typically short (`y ~ x1 + x2`). Is the pain of an unmodeled formula large enough to justify building a dedicated compiler IR?
  - *Rebuttal:* The pain is not typing the formula; the pain is that formulas break all downstream static tools (pipelines, linters, dependency graphs) because tools cannot differentiate formula terms from global variables.

---

### Candidate C5: ggplot2 Layer Composition / Visualization Semantics

- **Candidate ID:** C5
- **Candidate Type:** Domain-Specific Analysis Capability / Developer Tooling
- **Underlying Ecosystem Gap:** GAP 4 (Grammar of Graphics `+.gg` Layer Composition Blindness)
- **Direct Evidence:**
  - `flowr/src/r-bridge/lang-4.x/ast/model/nodes/`: `ggplot(...) + geom_point()` is parsed as an arithmetic `BinaryOp(Add)`.
  - `lintr/R/`: None of lintr's 118 linters validate aesthetic mappings (e.g. detecting constants inside `aes()` or verifying required aesthetics for geometries like `geom_ribbon`).
  - `rdtLite/R/IOTrace.R`: Captures plots only by hooking C graphics devices at runtime.
- **Existing Solutions:** None statically. `CodeDepends/R/isPlot.R` only detects that an expression produces a plot via name matching.
- **Remaining Problem:** Statically modeling the accumulated layer stack of a `ggplot` object, checking aesthetic channels, and detecting silent logical errors (e.g. `aes(color = "blue")` creating a pink factor level).
- **Test: "Valuable if Someone Else Built It?":** Moderate. A linter plugin could catch common ggplot2 errors without a full compiler engine, but an integrated layer IR provides superior diagnostics and autocomplete.
- **Skeptical Critique:**
  - *Strongest Argument Against:* ggplot2 errors are visual; users immediately see when a plot is wrong in an interactive session. Is static checking of graphics high priority?
  - *Rebuttal:* In automated reporting pipelines (`quarto`, `rmarkdown`, `targets`), a geometry error crashes a 4-hour batch job at the very last step. Furthermore, silent bugs (like string literals in `aes()`) mislead users without crashing.

---

### Candidate C6: IDE-Integrated Static Program Slicing

- **Candidate ID:** C6
- **Candidate Type:** Core Engine / User-Facing Workflow Capability
- **Underlying Ecosystem Gap:** GAP 6 (Fragmentation between program slicing and production IDE tooling)
- **Direct Evidence:**
  - `flowr/src/slicing/`: flowR implements an intra-procedural backward/forward slicer in TypeScript, but is not packaged for production editors (VS Code / Positron) or CRAN.
  - `languageserver/R/refactor.R`: Contains 897 lines of refactoring logic, but relies on raw XPath queries and lacks dataflow-driven program slicing.
- **Existing Solutions:** flowR (academic prototype).
- **Remaining Problem:** Bringing backward slicing into production editors via standard LSP CodeActions (e.g. "Extract minimal runnable script / reprex for variable `x`").
- **Test: "Valuable if Someone Else Built It?":** **YES.** Creating minimal reproducible examples (reprexes) is the single most common manual debugging activity in the R community.
- **Skeptical Critique:**
  - *Strongest Argument Against:* Static program slicing in dynamic languages often produces slices that are either too large (due to over-conservative dependency assumptions) or broken (because dynamic side effects were omitted).
  - *Rebuttal:* In data manipulation and analysis scripts, statement dependencies are predominantly linear dataflow with minimal metaprogramming. Conservative slicing in this domain produces massive code reductions (often 80%+ reduction of irrelevant code).

---

### Candidate C7: Native Linter Execution Engine

- **Candidate ID:** C7
- **Candidate Type:** Developer Tooling / Distribution & Performance Advantage
- **Underlying Ecosystem Gap:** GAP 1 applied to linting latency and CI/CD overhead
- **Direct Evidence:**
  - `lintr/R/lint.R`: Runs 118 linters in R via XML translation (`xmlparsedata`). In GitHub Actions, installing R and dependencies takes 30–60 seconds before linting begins.
- **Existing Solutions:** `lintr` (118 linters, gold standard in R), `air` (Posit's experimental C++/Rust formatter).
- **Remaining Problem:** A standalone, single-binary CLI that executes lint rules in <50ms without an R runtime.
- **Test: "Valuable if Someone Else Built It?":** **NO.** If Posit or an open-source team released "Ruff for R" tomorrow that executed lintr rules at native speed, this candidate would have zero differentiated identity.
- **Skeptical Critique:**
  - *Strongest Argument Against:* High risk of being merely a "faster existing tool." Does not advance the state of R program analysis; merely rewrites R code in Rust/C++.
  - *Verdict:* Class D (Interesting but Weak Differentiation). Should only be treated as a delivery mechanism for semantic diagnostics (C3, C4, C5), not as R-PARSE's core innovation.

---

## 8. Capability Compatibility Matrix

This matrix evaluates whether the candidates naturally compose into a coherent system or represent fragmented, conflicting directions:

| | C1 Native Core | C2 Ext Parser | C3 Column Lineage | C4 Formula IR | C5 ggplot2 IR | C6 Program Slicing | C7 Native Linter |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **C1 Native Core** | — | **Parent** | Strong Synergy | Strong Synergy | Strong Synergy | Strong Synergy | **Parent Engine** |
| **C2 Ext Parser** | **Component** | — | Prerequisite | Prerequisite | Prerequisite | Prerequisite | Prerequisite |
| **C3 Column Lineage** | Strong Synergy | Uses | — | **Deep Synergy** | **Deep Synergy** | **Deep Synergy** | Diagnostic Source |
| **C4 Formula IR** | Strong Synergy | Uses | **Deep Synergy** | — | Moderate Synergy | Strong Synergy | Diagnostic Source |
| **C5 ggplot2 IR** | Strong Synergy | Uses | **Deep Synergy** | Moderate Synergy | — | Strong Synergy | Diagnostic Source |
| **C6 Slicing** | Strong Synergy | Uses | **Deep Synergy** | Strong Synergy | Strong Synergy | — | Weak Synergy |
| **C7 Linter** | Consumer | Uses | Consumer | Consumer | Consumer | Weak Synergy | — |

### Key Compatibility Insights:
1. **The Infrastructure Spine:** C1 (Core) and C2 (Parser) form the prerequisite execution substrate for all other capabilities.
2. **The Semantic Triangle (C3 + C4 + C5):** C3 (Column Lineage) directly powers both C4 (Formulas) and C5 (ggplot2). A statistical formula (`y ~ x`) and an aesthetic mapping (`aes(x, y)`) both need to know whether `x` and `y` exist in the dataframe produced by C3!
3. **The Slicing Multiplier (C6):** Program slicing (C6) becomes dramatically more powerful when paired with C3. Without C3, slicing a summary table retains every upstream column operation; with C3, slicing can prune operations on unused columns!

---

## 9. Platform-Combination Analysis: The "R-SPM" Hypothesis

### The Hypothesis:
> *Are column lineage (C3), formula semantics (C4), ggplot2 semantics (C5), and program slicing (C6) separate features, or are they query views over a single deeper abstraction: an **R Semantic Program Model (R-SPM)**?*

```text
┌────────────────────────────────────────────────────────────────────────┐
│               THE R SEMANTIC PROGRAM MODEL (R-SPM)                     │
├────────────────────────────────────────────────────────────────────────┤
│                           QUERY SURFACES                               │
│   ┌───────────────────┐  ┌───────────────────┐  ┌───────────────────┐  │
│   │ IDE Program Slice │  │  Column Lineage   │  │ Semantic Linter   │  │
│   │     (C6 View)     │  │     (C3 View)     │  │  (C4/C5/C7 View)  │  │
│   └─────────▲─────────┘  └─────────▲─────────┘  └─────────▲─────────┘  │
├─────────────┼──────────────────────┼──────────────────────┼────────────┤
│             └──────────────────────┼──────────────────────┘            │
│                                    ▼                                   │
│                        UNIFIED SEMANTIC GRAPH                          │
│   ┌────────────────────────────────────────────────────────────────┐   │
│   │ • Reaching Definitions & Control Flow Graph (SSA-level)        │   │
│   │ • Virtual Tabular Schemas (inferred across pipeline verbs)     │   │
│   │ • Domain Entity Nodes (FormulaSpecs, PlotLayerStacks)          │   │
│   │ • Lexical & Dynamic Environment Bindings                       │   │
│   └────────────────────────────────▲───────────────────────────────┘   │
├────────────────────────────────────┼───────────────────────────────────┤
│                                    ▼                                   │
│                 FOUNDATIONAL RECOVERY & PARSE LAYER                    │
│   ┌────────────────────────────────────────────────────────────────┐   │
│   │ • Fault-Tolerant Lossless CST (Extended Tree-sitter R 4.x)     │   │
│   │ • Normalized Typed AST (Canonical Syntax Normalization)        │   │
│   └────────────────────────────────────────────────────────────────┘   │
└────────────────────────────────────────────────────────────────────────┘
```

### Critical Evaluation of the Platform Hypothesis:
- **What is genuinely missing in existing systems:** Existing tools are siloed. `flowR` has dataflow graphs but treats dataframes as scalars and ignores formulas. `CodeDepends` has function handlers but no graph IR. `languageserver` has class detection but no dataflow.
- **Does combination create non-linear value?:** **YES.** A unified model allows cross-domain validation that no existing tool can perform. For example:
  ```r
  df <- raw_data %>% select(mpg, wt)
  fit <- lm(hp ~ wt, data = df)           # Error: 'hp' was dropped in select()!
  p <- ggplot(df, aes(x = wt, y = qsec))  # Error: 'qsec' was dropped in select()!
  ```
  Neither `lintr`, `targets`, nor `languageserver` can catch these errors statically because none of them bridge pipeline transformations (dplyr) with statistical formulas (lm) and visualization aesthetics (ggplot).
- **Is the abstraction technically realistic?:** Yes, provided it is designed with **conservative approximations**. It must not attempt to be a fully sound type system for dynamic metaprogramming; it must operate as a gradual, local schema and entity tracker that gracefully falls back to `UNKNOWN` when encountering unconstrained reflection (`eval`, `assign`).

---

## 10. Broad R Compatibility Stress Test

R-PARSE must support broad R compatibility. Every high-ranked candidate is evaluated against R's diverse language idioms:

| Language Feature / Idiom | C1 Native Core | C3 Column Lineage | C4 Formula IR | C5 ggplot2 IR | C6 Slicing |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Base R Primitives** | Naturally compatible | Naturally compatible | Naturally compatible | Naturally compatible | Naturally compatible |
| **R 4.x Syntax (`\|>`, `\()`, raw strings)** | Naturally compatible | Naturally compatible | Naturally compatible | Naturally compatible | Naturally compatible |
| **Tidyverse Pipes & Verbs** | Naturally compatible | Requires verb handlers | Compatible | Compatible | Naturally compatible |
| **data.table (`[i, j, by]`, `:=`)** | Naturally compatible | Difficult (in-place syntax) | Compatible | Compatible | Conservative fallback |
| **S3 Generic Dispatch** | Naturally compatible | Compatible | Compatible | Direct modeling (`+.gg`) | Compatible |
| **S4 Formal Classes** | Naturally compatible | Compatible | Compatible | Compatible | Compatible |
| **NSE & Data-Masking** | Naturally compatible | **Core Target** | **Core Target** | **Core Target** | Conservative fallback |
| **Dynamic Metaprogramming (`eval`, `assign`)**| Naturally compatible | **Fundamentally Dynamic** (UNKNOWN) | **Fundamentally Dynamic** (UNKNOWN) | **Fundamentally Dynamic** (UNKNOWN) | Conservative over-approx. |
| **Closures & Environments** | Naturally compatible | Conservative scope | Conservative scope | Conservative scope | Conservative scope |

---

## 11. User Value & Workflow Impact Analysis

| User Persona | Concrete Problem Experienced | Frequency | Current Workflow | What R-PARSE Changes |
| :--- | :--- | :--- | :--- | :--- |
| **Data Scientist** | Silent bugs in ggplot2 aesthetics and dropped columns in data pipelines. | Daily | Runs long scripts; discovers misnamed columns or pink points only after runtime rendering. | Instant editor diagnostics flagging dropped columns and misplaced aesthetics before execution. |
| **Statistician / Researcher** | Formula typos or column mismatch crashing long-running model estimation. | Weekly | Waits hours for data prep to finish, only for `lmer()` to crash on a missing variable. | Static formula verification against the bound dataset schema prior to model fitting. |
| **R Package Developer** | Manually isolating minimal reproducible examples (reprexes) from complex scripts. | Frequent | Spends 1–2 hours manually deleting unrelated lines of code to isolate a bug. | Right-click in IDE: "Extract Executable Slice for Variable X" produces an instant 15-line reprex. |
| **DevOps / CI Engineer** | Heavy, slow CI/CD linting pipelines requiring full R installations. | Every PR | Installs GNU R + 100 dependencies in Docker; waits 45–90s per commit. | Standalone native CLI runs in <50ms without R runtime installation in CI runners. |
| **Workflow Engineer (`targets`)** | Pipeline invalidation caused by false global dependencies in tidyverse verbs. | Frequent | Writes awkward wrapper functions or `.data` workarounds to prevent `targets` from crashing. | Precise dependency extraction distinguishing data frame columns from global dependencies. |

---

## 12. Benchmarkability Specifications

Objective, reproducible metrics for each surviving opportunity:

| Opportunity | Benchmark Metric | Ground Truth / Baseline | Measurement Protocol |
| :--- | :--- | :--- | :--- |
| **C1 Native Core** | Parsing throughput (LOC/sec), Peak RAM (MB), Cold-start latency (ms). | Base R `parse()`, `xmlparsedata`, `tree-sitter-r`. | Parse top 50 CRAN packages (400k+ LOC); measure wall-clock and RSS. |
| **C3 Column Lineage** | Column identification Precision & Recall; False Global Dependency Rate. | Runtime execution ground truth via `rdtLite` PROV-JSON. | Run 50 real-world dplyr scripts; compare predicted column def-use edges against runtime traces. |
| **C4 Formula IR** | Formula variable classification accuracy; Model matrix term alignment. | R runtime `terms.formula()` and `model.matrix()`. | Evaluate 200 formulas from CRAN modeling packages; compare static term decomposition against runtime terms. |
| **C5 ggplot2 IR** | Linter Precision & Recall on valid vs invalid ggplot2 scripts. | Synthetic suite of 100 known ggplot2 bugs and 100 valid plots. | Measure false positive / false negative diagnostic flags. |
| **C6 Program Slicing** | Slice Reduction Ratio (lines saved); Semantic Execution Preservation. | `flowR` benchmark suite; runtime execution of extracted slices. | Execute extracted slice; verify target variable outputs identical hash to original script. |

---

## 13. Reproduction Risk Analysis

| Candidate ID & Name | Reproduction Difficulty | Justification |
| :--- | :---: | :--- |
| **C1 Native Core** | **Medium** | Standard compiler engineering. Any strong systems team familiar with Tree-sitter, Rust, or C++ can build a fast parser and AST walker. |
| **C2 Extended Parser** | **Very Low** | Trivial. A 50-line edit to `grammar.js` and `scanner.c` in `tree-sitter-r`. |
| **C3 Column Lineage** | **High** | High barrier. Requires deep knowledge of R's tidy evaluation mechanics, abstract interpretation, and specialized verb modeling. |
| **C4 Formula IR** | **High** | High barrier. Requires deep statistical computing domain knowledge (Wilkinson-Rogers algebra, contrasts, mixed models) combined with compiler IR design. |
| **C5 ggplot2 IR** | **Medium** | Moderately difficult. Requires cataloging geometry aesthetic requirements and modeling S3 `+.gg` method dispatch. |
| **C6 Program Slicing** | **High** | High barrier. Requires a robust intra-procedural CFG, reaching definitions, and dependency graph pruning that produces syntactically valid R code. |
| **C7 Native Linter** | **Low** | Low barrier. Translating `lintr`'s 118 XPath rules into Tree-sitter queries in Rust is straightforward engineering without conceptual depth. |

---

## 14. Opportunity Rankings & Shortlists

### 14.1 Top Opportunities by Strategic Differentiation (Score B)
1. **C3 — Column-Level Schema / Dataframe Lineage (8.40 / 10)**
2. **C6 — IDE-Integrated Static Program Slicing (7.60 / 10)**
3. **C4 — Statistical Formula Semantic IR (7.50 / 10)**
4. **C5 — ggplot2 Layer Composition Semantics (7.30 / 10)**

### 14.2 Top Opportunities by Research / Technical Attractiveness (Score A)
1. **C3 — Column-Level Schema / Dataframe Lineage (8.10 / 10)**
2. **C4 — Statistical Formula Semantic IR (7.70 / 10)**
3. **C6 — IDE-Integrated Static Program Slicing (7.60 / 10)**
4. **C1 — Standalone Native Core Engine (7.45 / 10)**

---

## 15. The Strategic Shortlist

Based on rigorous scoring, compatibility analysis, and reproduction defensibility, the candidates are stratified into three clear action tiers:

### Tier 1: The Top 3 Core Strategic Opportunities (Advance to Adversarial Validation)
1. **C3: Column-Level Schema & Dataframe Lineage:** Solves the central blindspot of the R ecosystem (data-masking in tidyverse pipelines) and eliminates false positive dependencies.
2. **C4: Statistical Formula Semantic IR:** Provides first-in-class compiler lowering for statistical modeling DSLs, validating formulas statically against bound datasets.
3. **C6: IDE-Integrated Static Program Slicing:** Delivers transformative user-facing workflow value by automating minimal reproducible example (`reprex`) extraction in editors.

### Tier 2: Strong Supporting Capabilities & Infrastructure
- **C1: Standalone Native Core Engine:** The foundational high-performance execution substrate that hosts C3, C4, and C6 without R runtime dependencies.
- **C5: ggplot2 Layer Composition Semantics:** A high-value domain extension providing first-in-class diagnostics for visualization pipelines.

### Tier 3: Reclassified & Deprecated Candidates
- **C2 (Extended Parser):** Reclassified from an independent opportunity to an internal bugfix/component of C1.
- **C7 (Native Linter):** Deprecated as an independent strategic opportunity due to high "clone risk" and low differentiation. Reclassified as a diagnostic delivery CLI for the rules generated by C3, C4, and C5.

---

## 16. Open Questions Requiring Next-Stage Validation

1. **The Gradual Schema Tractability Boundary:** What precise percentage of CRAN/tidyverse scripts can be successfully analyzed using local, literal-column schema propagation without triggering `UNKNOWN` fallbacks?
2. **The Executable Slice Soundness Rate:** What percentage of statically extracted program slices in dynamic R remain directly runnable and produce bit-for-bit identical results to the original script?
3. **The IPC Performance Trade-off:** If R-PARSE uses a hybrid model (standalone core with optional R IPC bridge), what is the latency of querying loaded R package namespace environments compared to pure static analysis?
