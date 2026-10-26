# R-PARSE — FORMAL SCOPE FREEZE SPECIFICATION

> **STATUS: SCOPE FROZEN**  
> **Date:** October 2026  
> **Document Authority:** Authoritative Scope Boundary for R-PARSE Core Engine  
> **Supersedes:** All preliminary research statements stating "scope is not frozen yet" (including statements in [`FINAL-RESEARCH-REPORT.md`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/FINAL-RESEARCH-REPORT.md#L5), [`candidate-features.md`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/candidate-features.md#L5), and [`architecture-options.md`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/architecture-options.md#L5)).

---

## Executive Decision Table

| Decision Area | Status | Boundary & Rationale Summary |
|---|---|---|
| **Core thesis** | **FROZEN** | Fine-grained column provenance + column-aware static program slicing |
| **Column provenance** | **FROZEN** | Input-to-output column derivation edges; distinct from schema shape tracking |
| **Column-aware slicing** | **FROZEN** | Backward traversal targeting specific dataframe columns/expressions |
| **Broad parsing** | **FROZEN** | Robust, fault-tolerant parser covering base R, pipes, formulas, tidyverse syntax |
| **Semantic representation** | **FROZEN** | Two-tier model: Concrete Syntax Representation $\rightarrow$ Normalized Semantic Representation |
| **Symbol/scope analysis** | **FROZEN** | Lexical scopes, definitions, references, def-use relationships |
| **CFG/dataflow foundation** | **FROZEN** | Statement-level control flow graph, reaching definitions, dependency propagation |
| **UNKNOWN semantics** | **FROZEN** | Orthogonal StaticKnowledge (KNOWN_STATIC, UNKNOWN_STATIC) and RuntimeEvidence (NONE, OBSERVED); no speculative dependencies |
| **Formula semantics** | **DEFERRED** | Statistical formula parsing/IR is a future domain extension, not an MVP core product |
| **ggplot2 semantics** | **DEFERRED** | Aesthetic layer modeling is a future domain extension, not an MVP core product |
| **Runtime bridge & correlation** | **APPROVED SUPPORTING CAPABILITY (TIER 2)** | Option D selected ([ADR-001](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/ADR-001-architecture.md)): Bounded, optional GNU R runtime assistance for execution/observation; Rust static analysis core remains independent |
| **LSP / IDE integration** | **DEFERRED** | Editor servers and interactive code actions deferred until engine validation |
| **Full interpreter / compiler** | **EXCLUDED** | Non-goal; will not build GNU R runtime replacement or native code compiler |
| **Guaranteed executable reprex** | **EXCLUDED** | Non-goal; R-PARSE does not guarantee that static slicing can always produce a short, standalone, executable reprex |

---

## 1. Status

The R-PARSE project has concluded its exploratory research phase. **The project scope is formally FROZEN.**

- **Exploratory Phase Closed:** All comparative audits, repository forensic investigations, and adversarial stress tests are complete.
- **Evidence Base:** Grounded in the validated findings of:
  - [`FINAL-RESEARCH-REPORT.md`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/FINAL-RESEARCH-REPORT.md)
  - [`gap-analysis.md`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/gap-analysis.md)
  - [`opportunity-matrix.md`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/opportunity-matrix.md)
  - [`adversarial-validation-v2.md`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/adversarial-validation-v2.md)
  - [`flowr-deep-gap-audit.md`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/flowr-deep-gap-audit.md)
  - [`candidate-features.md`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/candidate-features.md)
  - [`architecture-options.md`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/architecture-options.md)
- **Supersession Notice:** Prior research documents containing caveats such as *"scope is not frozen yet"* are now superseded by this document. Those historical statements must not be cited or interpreted as representing current project status.
- **Implementation Gate:** Architecture decisions and production code implementation remain blocked until this frozen scope document is audited and approved.

---

## 2. Authoritative Project Definition

Freeze the following as the **sole, authoritative project definition**:

> **R-PARSE is a broad-compatibility R program analysis system that extends semantic program representation with fine-grained column provenance and uses that information to enable column-aware static program slicing.**

### Positioning Constraints
To preserve scientific precision and avoid marketing inflation, R-PARSE must **never** be defined as:
- *"Ruff for R"* (R-PARSE is an analysis and provenance engine, not primarily a linter CLI)
- *"A better flowR"* (R-PARSE addresses a specific, decoupled gap in column derivation and slicing, acknowledging flowR's extensive foundations)
- *"An R parser"* (Parsing is an infrastructure dependency, not the research thesis)
- *"An R interpreter / compiler"* (R-PARSE performs static semantic analysis, not code execution or code generation)

---

## 3. Core Scope (MVP — Must Be Implemented)

The core scope comprises eight foundational components required to deliver the primary contribution.

```text
R Source Code
      ↓
[A. Broad R Parsing]
      ↓
[B. Semantic Program Representation] (Normalized AST)
      ↓
[C. Symbol & Scope Analysis] + [D. CFG / Dataflow Infrastructure]
      ↓
[E. Column-Level Provenance] ← [F. Supported Dataframe Semantics] + [G. UNKNOWN Semantics]
      ↓
[H. Column-Aware Static Backward Slicing]
```

### A. Broad R Parsing
- **Coverage Requirement:** Must parse ordinary, modern R programs reliably and with high fault tolerance.
- **Language Constructs:** Must parse Base R syntax, native pipe (`|>`), magrittr pipe (`%>%`), formulas (`~`), function definitions, lambdas (`\(x)`), control structures, and indexing syntax (`$`, `[[`, `[`).
- **Conformance & Acceptance Target:** 100% pass rate on the defined parser conformance corpus, with no panics and graceful recovery for unsupported semantic analysis (does not claim universal parsing coverage from a finite benchmark).
- **Role Boundary:** Parser robustness (including addressing grammar defects such as Tree-sitter's pipe placeholder `_`) is strictly **supporting engineering infrastructure**, not the primary research contribution.

### B. Semantic Program Representation
- **Two-Tier Architecture:**
  ```text
  R Source
     ↓
  Concrete Syntax Representation (CST)
     ↓
  Normalized Semantic Representation (AST)
  ```
- **Representation Targets:** Must normalize and represent:
  - Expressions and binary/unary operators
  - Context-sensitive assignment: Desugars statement/block assignment (`<-`, `->`, and top-level `=`) into `AssignmentStmt`, while strictly distinguishing named function arguments (`foo(x = b)`, lowered as `NamedArgument`) and dataframe column definitions (`mutate(col = expr)`)
  - Conservative pipe lowering: Explicit `PipeLowering` rules for recognized patterns; unsupported or ambiguous pipe semantics remain un-lowered (`PipeExpr`) and degrade to `UNKNOWN_STATIC` rather than being incorrectly rewritten
  - Super-assignment (`<<-`) with non-local scope target flag
  - Variables, symbols, and literals
  - Function calls and arguments (named, positional, and NSE)
  - Lexical scopes and blocks
  - Data dependencies between program variables
  - Dataframe manipulation operations
- **Complexity Constraint:** Do not introduce premature AST node complexity before specific analysis passes require it.

### C. Symbol and Scope Analysis
- **Core Semantic Foundations:**
  - Identification of variable definitions (writes) and references (reads)
  - Lexical environment hierarchy (global, function local, nested closures)
  - Disambiguation of local variable bindings from dataframe column names in data-masking contexts
  - Symbol resolution across sequential statements

### D. CFG / Dataflow Infrastructure
- **Dependency Foundations:**
  - Intra-procedural Control Flow Graph (CFG) construction
  - Definition-Use (def-use) and Use-Definition (use-def) chains
  - Reaching definitions analysis across branching and loops
  - Dependency graph structure capable of efficient backward traversal

### E. Column-Level Provenance (Primary Research Contribution)
- **Conceptual Model:** Transitions analysis from schema presence to derivation lineage:
  ```text
  Traditional Schema Tracking (flowR):
  df contains columns {price, quantity, total}

  R-PARSE Column Provenance:
  price ───────┐
               ├──→ total
  quantity ────┘
  ```
- **Representation Distinction:** Preserves a strict distinction between:
  1. *Schema Presence:* Asserting that a column name exists in an object's schema.
  2. *Column Derivation / Provenance:* Asserting that output column $C_{out}$ was computed from input column(s) $\{C_{in1}, C_{in2}, \dots\}$ via expression $E$.
- **Preservation of Derivation Grouping:** Multi-column provenance events (e.g., $\{ \text{price}, \text{quantity} \} \to \text{total}$) remain representable as a single, atomic derivation event (`ColumnDerivationNode`), retaining grouping even if internal graph traversal uses pairwise edges.
- **Ecosystem Boundary:** R-PARSE does not claim that column lineage is nonexistent in computer science. The contribution is the **formal integration of fine-grained column derivation edges into general R program dependency graphs and static slicing**.

### F. Supported Dataframe Semantics
- **Bounded Transformation Scope:** The MVP implements a deliberately restricted, well-defined set of core dataframe transformations:
  - `select`: Column retention, projection, and removal
  - `rename`: Column aliasing and identity forwarding
  - `mutate`: Column creation and transformation with multi-column input dependencies, evaluated with strict left-to-right sequential semantics (columns defined earlier are visible to later assignments in the same call, permitting chains like `mutate(total = price * quantity, tax = total * 0.18)`)
  - `transmute`: Combined selection and transformation with identical left-to-right sequential evaluation
  - `filter`: Row subsetting modeled as a semantic `RowSelectionDependency` (NOT a CFG control dependency); surviving columns inherit dependency on predicate columns during backward slicing
  - Base R Equivalents: Column assignment (`df$col <- ...`), basic subsetting (`df[, c(...)]`), and `transform()`
- **Boundary Rule:** Do not attempt to model the entire tidyverse, `data.table` in-place semantics, or complex reshape verbs (`pivot_longer`, `pivot_wider`) in the MVP. Additional verbs will be added modularly only when the core engine is validated.

### G. Explicit UNKNOWN Semantics (Orthogonal Uncertainty Model)
- **Uncertainty Principle:** Dynamic, ambiguous, or unresolvable R language constructs must **never be guessed or speculatively inferred**.
- **Orthogonal Information Model:**
  - **StaticKnowledge:**
    - `KNOWN_STATIC`: Dependency is established by the implemented static semantic model under its documented assumptions (no claim of universal whole-program soundness).
    - `UNKNOWN_STATIC`: Dependency cannot be proven statically due to dynamic constructs, unmodeled calls, or dynamic data masking.
  - **RuntimeEvidence:**
    - `NONE`: No runtime observation has occurred (default in STATIC mode).
    - `OBSERVED`: Observed during a specific runtime execution context in GNU R.
- **No Lattice Promotion:** Runtime observation annotates `UNKNOWN_STATIC` with empirical evidence (`RuntimeEvidence::Observed`), but does NOT promote it to `KNOWN_STATIC`. It provides context-specific evidence, not universal proof.
- **Soundness Rule:** When facing metaprogramming, dynamic strings, external data sources, or unmodeled functions, the system must emit an explicit `UNKNOWN_STATIC` dependency rather than fabricating false certainty or dropping dependencies silently.

### H. Column-Aware Static Backward Slicing
- **Analytical Objective:** Answers the targeted query:
  > *"Which specific dataframe columns, expressions, and program statements contributed to this selected column/result?"*
- **Semantic-Node Slicing Granularity:** Slicing operates at semantic-node granularity (`SliceElement` / `SliceNode`), not merely coarse `Set<StmtId>`. The abstraction represents:
  - Statements (`StmtId`)
  - Expressions (`ExprId`)
  - Definitions (`DefinitionId`)
  - Column versions (`ColumnVersionId`)
  - Derivations (`DerivationId`)
  - Control predicates (`ExprId`)
  - Row-selection predicates (`ExprId`)
- **Sub-Statement Region Attribution:** A single source statement is explicitly permitted to contain both retained and excluded semantic regions (e.g., in `mutate(target = price * count, unused = age + 10)`, the derivation for `target` is retained while `unused` is excluded).
- **Explicit Non-Promise:** R-PARSE does **NOT** promise automated executable source code reconstruction or standalone reprex generation from slices; it provides fine-grained semantic-node attribution and source span mapping.
- **Progression from Object to Column Slicing:**
  ```text
  Object-Level Slice (Prior Art / flowR):
  Slicing criterion: df2
  Result: Every statement touching df or df2 is retained.

  Column-Aware Slice (R-PARSE):
  Slicing criterion: df2$total
  Result: Statements defining price and quantity are retained;
          statements modifying unrelated columns (e.g., df$notes, df$customer_id)
          are eliminated.
  ```
- **Unified Engine:** Lineage and slicing must operate as an integrated capability across the program dependency graph, not as disparate tools.

---

## 4. Primary Research Contribution

Freeze the primary scientific and technical contribution as:

> **Fine-grained column provenance integrated with program-level dependency analysis and column-aware static program slicing for R.**

### What the Contribution IS NOT
To prevent scope drift and maintain research rigor, the primary contribution is explicitly **NOT**:
1. Creating a new R parser (Tree-sitter and Bison grammars already exist).
2. Creating a new linter or rewriting `lintr` rules (rule catalogs are mature).
3. Creating a new R interpreter or execution runtime.
4. Merely tracking dataframe schemas (flowR already tracks `must`/`may` column sets).
5. Merely implementing isolated column lineage for SQL or pipeline DAGs.
6. Merely wrapping or rebranding flowR.

### Concrete Forensic Evidence Supporting the Gap
As established in the forensic source audit ([`flowr-deep-gap-audit.md`](flowr-deep-gap-audit.md)), against flowR commit `3ea266c712b34a585402f1a3c66811850340313b`:
- **flowR Dataframe Analysis is Exclusively Geometric/Schema Tracking:** FlowR's `DataFrameDomain` ([`dataframe-domain.ts:L6-L15`](https://github.com/flowr-analysis/flowr/blob/3ea266c712b34a585402f1a3c66811850340313b/src/abstract-interpretation/data-frame/dataframe-domain.ts#L6-L15)) tracks only `colnames` (`SetRangeDomain`), `cols` (`PosIntervalDomain`), and `rows` (`PosIntervalDomain`). It records no column-to-column derivation edges.
- **flowR Slicing is Decoupled from Dataframe Analysis:** FlowR's static slicer ([`static-slicer.ts:L66-L203`](https://github.com/flowr-analysis/flowr/blob/3ea266c712b34a585402f1a3c66811850340313b/src/slicing/static/static-slicer.ts#L66-L203)) traverses `DataflowGraph` treating dataframes as monolithic scalar variables (`VariableDefinitionVertex`). There is zero connection between `DataFrameShapeAnalysis` and `staticSlice`.
- **Termination of Information:** FlowR's inferred dataframe shapes terminate strictly in a linter check and a CLI debug printer. They never flow into dataflow, reaching definitions, or program slices.

R-PARSE fills this verified architectural void.

---

## 5. Supporting Capabilities (Infrastructure Substrates)

The following capabilities are required to build and support the core system. They are **engineering prerequisites**, not independent research novelties:
1. **Source-to-AST Ingestion:** Robust translation from concrete syntax tree tokens into normalized semantic nodes.
2. **Def-Use Graph Construction:** Graph representation capturing standard variable dataflow.
3. **Control-Flow Graph Builder:** Basic block decomposition, edge branching (`if`/`else`), loop cycles (`for`/`while`), and early exits (`return`).
4. **Environment / Scope Stack:** Lexical environment frames resolving identifiers to lexical declarations.
5. **Evaluation Benchmark Suite:** Synthetic and empirical R scripts providing ground-truth test cases for provenance propagation and slicing accuracy.
6. **GNU R Runtime-Assisted Execution & Observation (Tier 2 Bridge):** Bounded, optional GNU R process integration for runtime evidence collection and schema discovery. Explicit invocation only; STATIC mode remains runtime-free.

---

## 6. Scope Amendments & Deferred Extensions

### Scope Amendment 1: Runtime-Assisted Analysis (Approved Supporting Capability)
- **Status:** **APPROVED AS SUPPORTING CAPABILITY (TIER 2)** via [`ADR-001-architecture.md`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/ADR-001-architecture.md) (Option D: Hybrid Rust Core + GNU R Runtime Bridge).
- **Scope Alignment:** Runtime correlation and runtime-assisted analysis are **no longer marked deferred**. They represent an approved supporting capability.
- **Explicit Scope Amendment:**
  - Rust static analysis remains the authoritative core engine, functioning 100% autonomously and offline.
  - GNU R runtime-assisted execution/observation is now an approved supporting capability, invoked strictly on-demand via explicit user request.
  - Formula semantics, ggplot2 semantics, and LSP remain deferred.
  - Runtime execution is explicit, bounded, and isolated via evaluation environment / process isolation (`new.env(parent = globalenv())`), not an OS/container security boundary (no "sandbox" guarantees).
  - Configurable execution timeout defaults provisionally to 500ms subject to benchmark validation, not an immutable architectural guarantee.
  - STATIC mode remains 100% runtime-free.
  - Runtime observations describe specific execution contexts and do not constitute universal proof of static behavior across unobserved inputs.
- **Thesis Invariance:** The core research thesis remains strictly intact.

### Deferred Extensions (Preserved Beyond MVP)
The following candidate capabilities were evaluated during research and remain **strictly deferred beyond the initial development phase**:

#### A. Statistical Formula Semantics (`~`)
- **Description:** Modeling Wilkinson-Rogers formulas (`y ~ x1 + x2 + (1|group)`) with dedicated AST nodes separating response, fixed effects, interactions, and random group intercepts.
- **Status:** **DEFERRED.** Formula analysis is a domain-specific semantic consumer of the provenance engine, not an MVP core product.

#### B. ggplot2 Visualization Semantics (`+.gg`)
- **Description:** Modeling Grammar of Graphics layer accumulation, aesthetic mapping channels (`aes(x, y)`), and geometry requirements.
- **Status:** **DEFERRED.** Visualization semantics will consume the provenance/slicing model once the core data engine is stable.

#### C. IDE & Language Server Protocol (LSP)
- **Description:** Exposing slices as interactive code actions, hover hints, and refactoring utilities in VS Code, Positron, or Neovim.
- **Status:** **DEFERRED.** Editor UX must not dictate or distract from early engine architecture.

---

## 7. Explicit Non-Goals

R-PARSE explicitly will **NOT** attempt to become:

1. **A Full R Interpreter:** R-PARSE will not execute R code or implement an alternative runtime to GNU R.
2. **An R Compiler:** R-PARSE will not compile R into LLVM, WebAssembly, machine code, or C.
3. **A Generic Linter Replacement:** R-PARSE will not re-implement the 118 style rules of `lintr`.
4. **A Direct Replacement for flowR:** R-PARSE acknowledges flowR's research and solves an orthogonal, integrated provenance/slicing problem.
5. **A Workflow Execution Scheduler:** R-PARSE will not replace `targets` or manage pipeline execution caches.
6. **A Standalone Formula Engine:** Formulas are not a separate product.
7. **A Standalone Visualization Engine:** ggplot2 layer checking is not a separate product.
8. **A Universal Sound Static Type System:** R's dynamic dispatch and metaprogramming make universal static typing impossible.
9. **Guaranteed Whole-Program Soundness:** Metaprogramming, runtime `eval()`, reflection, and dynamic imports will be handled conservatively via `UNKNOWN_STATIC`, not guaranteed sound analysis. `KNOWN_STATIC` means the dependency is established by the implemented static semantic model under its documented assumptions, without claiming universal whole-program soundness.
10. **Guaranteed Automatic Reprex Generation:** R-PARSE does not guarantee that static slicing can always produce a short, standalone, executable reprex. Adversarial validation showed that dynamic runtime dependencies, environment states, and package-level side effects make guaranteed executable reprex synthesis unsound without execution. R-PARSE provides conservative, intra-procedural program slicing with semantic-node attribution, not guaranteed executable reprex generation.
11. **Universal Proof via Runtime Observation:** Runtime observation captures a specific executed context. It must **not** be treated as universal proof of static behavior across all inputs or alternative control-flow branches. Dynamic/unresolvable static behavior may still legitimately remain `UNKNOWN_STATIC`.
12. **OS/Container Security Sandbox:** `new.env(parent = globalenv())` is an evaluation environment and process isolation mechanism, not an OS or container security boundary. R-PARSE does not provide arbitrary untrusted code execution sandboxing.

---

## 8. Compatibility Philosophy

Freeze the guiding development principle:

> **"Broad compatibility, conservative semantics."**

```text
Parse broadly
     ↓
Represent broadly
     ↓
Analyze precisely where supported
     ↓
Represent uncertainty explicitly (UNKNOWN)
```

### Operational Rules
1. **Valid R syntax must be parsed and represented without analysis failure; inability to determine semantics must degrade to UNKNOWN.** The parser and AST normalizer must not crash on valid R programs, even if subsequent semantic analysis passes cannot resolve their behavior.
2. **Graceful Degradation:** When encountering unmodeled packages, complex closures, or dynamic evaluation (`eval(parse(...))`), the system must degrade gracefully to `UNKNOWN` rather than failing the entire file.
3. **No False Precision:** It is far better to output an `UNKNOWN` dependency edge than to fabricate a clean but incorrect column relationship.
4. **No Narrow Subsetting:** R-PARSE must not restrict its parser to a fragile, artificial "clean subset of R" just to make analysis appear sound.

---

## 9. Scope Change Rule

From this point forward, every proposed feature, AST node, analysis pass, or configuration option must pass this single gating test:

> **"Does this directly strengthen the core objective of fine-grained column provenance integrated with column-aware program slicing?"**

- **If NO:** The proposal is immediately rejected or cataloged as Deferred / Future.
- **If YES:** The proposal may be evaluated for core implementation.

No feature may enter the core engine solely because it is technically intriguing, academically elegant, or requested by adjacent tooling paradigms.

---

## 10. Success Criteria

The frozen core scope is considered successful if and only if the implementation demonstrates, through reproducible, automated test suites, the complete analysis pipeline:

```text
R Source Code
     ↓
Semantic AST Representation
     ↓
Dataframe Transformation Understanding
     ↓
Column Provenance Graph
     ↓
Dependency Propagation
     ↓
Column-Aware Backward Slice
```

### Mandatory Verification Benchmarks

#### Benchmark Input Schema Discovery Rule
For benchmarks whose input dataframe is external or undefined (e.g., `df` in Benchmarks 1–6):
The analyzer does **NOT** assume magical static knowledge of external schemas. The input schema becomes known via either:
- **Option A (Declared Benchmark Fixture Schema):** A static fixture declaration in the test suite specifying column names and types (e.g., `fixture: df = {price: numeric, quantity: numeric, ...}`).
- **Option B (Runtime-Assisted Schema Discovery):** In `ASSISTED` mode upon explicit user invocation, the Tier 2 GNU R runtime queries the active session to observe `colnames(df)`.
- *Fallback:* If neither is provided, static analysis treats `df`'s schema as `UNKNOWN_STATIC`, propagating uncertainty gracefully without failing.

The initial implementation must demonstrate correct analytical behavior across seven benchmark test scenarios:

1. **Direct Column Derivation:**
   ```r
   # Input schema for df declared via fixture: {price: numeric, quantity: numeric}
   df2 <- df %>% mutate(total = price * quantity)
   ```
   *Expected:* Derivation edges `price → total` and `quantity → total` recorded as an atomic `ColumnDerivationNode`.
2. **Multi-Step Propagation (with Sequential Mutate Semantics):**
   ```r
   # Input schema for df declared via fixture: {price: numeric, quantity: numeric}
   df2 <- df %>% mutate(total = price * quantity)
   df3 <- df2 %>% mutate(tax = total * 0.18)
   ```
   *Expected:* Provenance path `price → total → tax` and `quantity → total → tax` preserved across sequential transformations. In single-call `mutate(total = price * quantity, tax = total * 0.18)`, left-to-right sequential visibility also ensures `tax` derives from `total`.
3. **Renaming & Projection:**
   ```r
   # Input schema for df declared via fixture: {price: numeric, customer_id: character, notes: character}
   df2 <- df %>% rename(cost = price) %>% select(cost, customer_id)
   ```
   *Expected:* Identity alias `cost ← price`, retention of `customer_id`, elimination of unselected columns (`notes`).
4. **Multi-Column Input Combination:**
   ```r
   # Input schema for df declared via fixture: {first_name: character, last_name: character}
   df2 <- df %>% mutate(full_name = paste(first_name, last_name))
   ```
   *Expected:* Output column `full_name` correctly links to both `first_name` and `last_name` in a single derivation event.
5. **Object vs. Column Separation:**
   ```r
   # Input schema for df declared via fixture: {x: numeric, y: numeric}
   df2 <- df %>% mutate(a = x + 1, b = y + 1)
   ```
   *Expected:* Slicing on `df2$a` retains `a = x + 1` (partially retained statement) and does NOT depend on `y`.
6. **Explicit UNKNOWN Handling:**
   ```r
   # Input schema for df declared via fixture: {col_a: numeric}
   df2 <- df %>% mutate(dynamic_col = .data[[get_col_name()]])
   ```
   *Expected:* `df2$dynamic_col` depends on `UNKNOWN_STATIC`, flagging uncertainty without aborting analysis.
7. **Column-Aware Backward Slicing Extraction (Row Selection & Partial Statement Retention):**
   ```r
   raw <- read.csv("data.csv") # schema discovered via runtime assist or fixture
   clean <- raw %>% filter(valid == TRUE)
   df2 <- clean %>% mutate(target = price * count, unused = age + 10)
   res <- df2$target
   ```
   *Expected:* Slicing criterion `res` or `df2$target` retains semantic nodes:
   - Statement 1: `raw <- read.csv("data.csv")` (Fully Retained)
   - Statement 2: `clean <- raw %>% filter(valid == TRUE)` (Fully Retained; traverses `RowSelectionDependency` to retain `valid`)
   - Statement 3: `df2 <- clean %>% mutate(...)` (**Partially Retained**: retains derivation for `target = price * count`; strictly excludes `unused = age + 10`)
   - Statement 4: `res <- df2$target` (Fully Retained)
   while strictly excluding the unrelated `unused` / `age` branch.

---

## 11. Frozen Decisions

The following decisions are permanently locked for the initial development phase:

1. **Research Thesis Locked:** R-PARSE is defined exclusively by column provenance + column-aware program slicing.
2. **Parser Role Locked:** Parsing is infrastructure; Tree-sitter extension is the baseline.
3. **Two-Tier AST Locked:** Concrete syntax tree separated from normalized semantic AST.
4. **Dataframe Abstraction Locked:** Must represent column derivation edges, not merely schema sets.
5. **Uncertainty Principle Locked:** Unsupported dynamic behavior maps explicitly to `UNKNOWN`.
6. **Target Transformations Locked:** MVP bounded strictly to `select`, `rename`, `mutate`, `transmute`, `filter`, and basic Base R equivalents (`$`, `[`, `transform`).
7. **Architecture Locked:** Option D (Hybrid Rust Core + GNU R Runtime Bridge) selected in [`ADR-001-architecture.md`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/ADR-001-architecture.md), formalizing the two-tier separation between deterministic Rust static analysis and bounded GNU R runtime observation.

---

## 12. Deferred Decisions

The following architectural and feature decisions remain intentionally open and deferred:

1. **Formula AST Design:** Design of `FormulaNode` deferred until core provenance engine is validated.
2. **ggplot2 Layer Pipeline Modeling:** Design of aesthetic channel validation deferred.
3. **LSP Server Transport & Packaging:** Stdio vs socket daemon packaging deferred until engine passes benchmark suite.

---

> **FORMAL FREEZE COMPLETE.**  
> *This document establishes the authoritative scope boundary for the initial development phase. Any modification requires an explicit scope-change review under Section 9.*
