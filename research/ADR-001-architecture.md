# ADR-001: Hybrid Rust Core + GNU R Runtime Bridge

## 1. ADR ID
`ADR-001`

---

## 2. Title
**Architecture Decision: Option D — Hybrid Rust Core + GNU R Runtime Bridge**

---

## 3. Status
**ACCEPTED**  
**Date:** October 2026  
**Decision Authority:** Lead Architect & Research Engineering Team  
**Scope Authority Reference:** [`research/scope-freeze.md`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/scope-freeze.md)

---

## 4. Context

During the research and forensic audit phase ([`FINAL-RESEARCH-REPORT.md`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/FINAL-RESEARCH-REPORT.md), [`gap-analysis.md`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/gap-analysis.md), [`flowr-deep-gap-audit.md`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/flowr-deep-gap-audit.md), [`adversarial-validation-v2.md`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/adversarial-validation-v2.md)), R-PARSE established its core thesis:
> **R-PARSE is a broad-compatibility R program analysis system that extends semantic program representation with fine-grained column provenance and uses that information to enable column-aware static program slicing.**

However, R is an intrinsically dynamic, non-standard evaluating (NSE), homoiconic language with run-time reflection, dynamic environment scoping, and late binding. Four architectural patterns were identified in [`architecture-options.md`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/architecture-options.md):
- **Option A:** Pure Rust Standalone Engine (zero R dependencies)
- **Option B:** TypeScript / Node.js Engine (extending flowR)
- **Option C:** Pure R + C Extension (CRAN-first package)
- **Option D:** Hybrid Rust Core + GNU R Runtime Bridge (two-tier architecture)

The team must formally select and specify the system architecture to guide development without compromising the frozen research scope or introducing premature implementation details.

---

## 5. Frozen Requirements

This architectural decision strictly adheres to the frozen boundaries locked in [`research/scope-freeze.md`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/scope-freeze.md):

1. **Thesis Integrity:** Must implement fine-grained column provenance ($C_{in} \to C_{out}$) and column-aware backward program slicing as the primary technical contribution.
2. **Infrastructure Foundations:** Must provide broad R parsing, normalized semantic IR, lexical scope/symbol analysis, and CFG/dataflow infrastructure.
3. **Dataframe Semantics:** Must support core bounded transformations (`select`, `rename`, `mutate`, `transmute`, `filter`, and basic Base R equivalents).
4. **Explicit Uncertainty:** Must represent `UNKNOWN` explicitly; dynamic ambiguities must never be guessed or fabricated.
5. **Deferred Boundaries Maintained:** Statistical formulas (`~`), ggplot2 layer composition (`+.gg`), and LSP/IDE interfaces remain deferred domain extensions and are **not** promoted into MVP requirements.
6. **Explicit Non-Goals Maintained:** R-PARSE is **not** an R interpreter, not an R compiler, does not replace GNU R, and does not claim guaranteed whole-program soundness or universal reprex generation.

---

## 6. Architectural Decision Drivers

The architecture must balance four conflicting forces:

1. **Deterministic Static Analysis Speed & Portability:** Slicing, CFG traversal, and AST normalization must execute in milliseconds in standalone, headless environments (e.g., CI/CD pipelines, pre-commit hooks) without requiring a functional GNU R installation.
2. **Real-World R Dynamic Fidelity:** Pure static analysis inevitably encounters opaque runtime boundaries (e.g., `.data[[col]]`, external database schemas, dynamic attributes). When an R runtime *is* available, the analyzer must be able to observe ground-truth facts to resolve uncertainty rather than prematurely abandoning analysis.
3. **Sound Separation of Concerns:** Runtime observation must never mutate, corrupt, or replace static semantic reasoning. Static analysis remains the authoritative model; runtime execution provides empirical evidence for a specific run.
4. **Zero Re-invention of the R Runtime:** Under no circumstances should the project attempt to write an alternative R runtime or interpreter. Actual R execution must always be delegated to the official GNU R binary.

---

## 7. Options Considered

### Option A: Pure Rust Standalone Engine
- Single native binary compiled from Rust.
- Tree-sitter C parser + Rust semantic IR + Rust dataflow + Rust static slicer.
- No interaction with GNU R or any external runtime.

### Option B: TypeScript / Node.js Engine
- Extension of the academic `flowR` codebase (upstream https://github.com/flowr-analysis/flowr, inspected at `3ea266c`; not vendored in this repository).
- Implements missing column provenance passes and connects shape analysis to flowR's existing slicer in TypeScript.
- Distributed as an npm package and VS Code extension.

### Option C: Pure R + C Extension
- Traditional CRAN package written in R with C extensions.
- Executes entirely inside the GNU R process.
- Direct memory access to R environments and symbols.

### Option D: Hybrid Rust Core + GNU R Runtime Bridge
- **Tier 1 (Rust):** High-performance, self-contained standalone engine responsible for all parsing, semantic representation, CFG, column provenance, and slicing. Functions 100% autonomously without R.
- **Tier 2 (GNU R):** Controlled, optional execution bridge to an external GNU R process. Invoked only on demand when the user or analysis pass requests runtime grounding for dynamic constructs.

---

## 8. Detailed Option A Analysis (Pure Rust Standalone)

- **Strengths:**
  - Maximum raw throughput and deterministic latency (<10ms execution).
  - Single zero-dependency binary; frictionless deployment in CI/CD and pre-commit hooks.
  - Memory safe, free of garbage collection pauses.
  - Completely isolated from R version incompatibilities or R package configuration drift.
- **Critical Weaknesses:**
  - **Permanently Blind at the Dynamic Boundary:** In R scripts reading external CSVs or using dynamic columns (`.data[[var]]`), a pure static analyzer has zero recourse: it must emit `UNKNOWN` for all downstream columns.
  - Incapable of verifying whether a statically inferred dependency matches the actual runtime object shape in active data science workflows.
  - Leaves a wide gap between static prediction and developer reality in exploratory data science.

---

## 9. Detailed Option D Analysis (Hybrid Rust Core + GNU R Runtime Bridge)

- **Strengths:**
  - **Best of Both Worlds:** Combines the blazing speed, safety, and standalone autonomy of Rust with the empirical ground truth of the real GNU R environment.
  - **Autonomous Core:** The Tier 1 Rust core remains 100% self-sufficient. If GNU R is absent, R-PARSE still parses, models schemas, tracks column provenance, and performs column-aware static slicing gracefully, marking dynamic edges as `UNKNOWN_STATIC`.
  - **Grounded Dynamic Refinement:** When GNU R is available and invoked, Tier 2 observes actual evaluated expressions and column schemas, upgrading `UNKNOWN_STATIC` to `RUNTIME_CONFIRMED` in the observed context.
  - **Zero Interpreter Re-invention:** All execution is handled by GNU R. R-PARSE never emulates R semantics or evaluates R AST nodes internally.
- **Trade-offs / Mitigations:**
  - Increased architectural complexity: requires a structured bridge boundary between Rust and R.
  - Mitigated by keeping the bridge boundary strictly decoupled: Rust drives requests; R replies with facts; no bidirectional tight memory coupling.

---

## 10. Comparison Matrix

| Evaluation Dimension | Option A (Pure Rust) | Option B (TypeScript) | Option C (Pure R + C) | Option D (Selected: Hybrid Rust + R) |
|---|:---:|:---:|:---:|:---:|
| **Frozen Thesis Alignment** | High | Moderate | Low | **Maximum** |
| **Column Provenance Modeling** | High (Static only) | Moderate (flowR gap) | Low (Interpreter overhead) | **Maximum (Static + Grounded)** |
| **Column-Aware Program Slicing** | High (Static only) | Moderate | Low | **Maximum** |
| **Static Analysis Autonomy** | Maximum (100% offline) | Moderate (Node.js bound) | Zero (Requires R) | **Maximum (Tier 1 is autonomous)** |
| **Runtime Grounding Capability** | Zero (Impossible) | Low | Maximum (In-process) | **High (Bounded external process)** |
| **Portability & Distribution** | Maximum (Single binary) | Low (Node/npm) | Low (CRAN/R runtime) | **High (Standalone binary + optional R)** |
| **Implementation Complexity** | Moderate | Low (Prototype) | Moderate | **Managed / High** |
| **Testability & Determinism** | High | Moderate | Low | **High** |
| **R Ecosystem Compatibility** | Moderate (Static only) | Low | Maximum | **High** |

*(Note: Matrix represents qualitative architectural trade-offs; no speculative benchmark numbers are asserted.)*

---

## 11. Decision

**Option D — Hybrid Rust Core + GNU R Runtime Bridge is formally ACCEPTED.**

### Core Rationale
R-PARSE is designed to solve program analysis for R as it actually exists in practice: a highly expressive language combining structured functional data pipelines with dynamic evaluation. 

Option D is selected because it enables:
1. A **native, robust Rust core** for deterministic parsing, semantic AST normalization, CFG, column provenance graphs, and static backward slicing.
2. An **external GNU R runtime bridge** for executing selected expressions and observing runtime object states when static reasoning alone encounters an unresolvable boundary.
3. Complete architectural separation: the Rust core never depends on R to perform its primary duties, and GNU R is never asked to perform static program analysis.

---

## 12. Selected Architecture

### Conceptual Architectural Diagram

```text
                      ┌──────────────────────────────────────────┐
                      │                 R-PARSE                  │
                      │               (Tier 1)                   │
                      │                                          │
R Source Code ──────► │              RUST CORE                   │
                      │  ──────────────────────────────────────  │
                      │  • Tree-sitter R Parser / CST Ingestion  │
                      │  • Normalized Semantic Representation    │
                      │  • Lexical Scope & Symbol Resolution     │
                      │  • Control Flow Graph (CFG) Construction │
                      │  • Intra-procedural Dataflow & Def-Use   │
                      │  • Dataframe Transformation Semantics    │
                      │  • Fine-Grained Column Provenance Graph  │
                      │  • Column-Aware Static Backward Slicer   │
                      └────────────────────┬─────────────────────┘
                                           │
                                    Runtime Bridge
                               (Bounded / Optional IPC)
                                           │
                                           ▼
                      ┌──────────────────────────────────────────┐
                      │              GNU R RUNTIME               │
                      │               (Tier 2)                   │
                      │  ──────────────────────────────────────  │
                      │  • Official GNU R Process (libR / R CLI) │
                      │  • Sandboxed Expression Evaluation       │
                      │  • Runtime Object / Environment Inspect  │
                      │  • Observed Column & Schema Reflection   │
                      │  • Runtime Evidence & Error Capture      │
                      └──────────────────────────────────────────┘
```

### Information Flow Across Tiers

#### Direction 1: Rust Core $\longrightarrow$ GNU R Runtime
- **Targeted Expression Payloads:** Rust transmits isolated source fragments, statements, or expressions requiring runtime evaluation.
- **Inspection Directives:** Rust requests specific metadata about an active symbol (e.g., column names of dataframe `df`, class hierarchy of object `obj`, current environment bindings).
- **Execution Context Parameters:** Rust supplies environmental context (working directory, loaded package namespaces, evaluation timeout limits).

#### Direction 2: GNU R Runtime $\longrightarrow$ Rust Core
- **Observed Schema Facts:** R returns observed column names, types, and dimensions of inspected dataframes.
- **Evaluation Status & Results:** R returns execution status (success, error, warning) along with structured JSON/primitive return values.
- **Runtime Evidence Tags:** R supplies contextual evidence confirming or refuting whether a specific column or variable was bound during the observed run.

---

## 13. Rust Core Responsibilities (Tier 1)

The Rust Core is the authoritative brain of R-PARSE. It is fully autonomous and carries exclusive responsibility for:
1. **Source Ingestion:** Ingesting R source code and constructing a lossless Concrete Syntax Tree (CST) using extended Tree-sitter.
2. **Semantic AST Normalization:** Transforming CST trivia into a strongly-typed, normalized intermediate representation.
3. **Symbol Table & Scoping:** Tracking lexical scope blocks, symbol definitions, reads, writes, and data-masking environments.
4. **CFG Construction:** Decomposing statements into basic blocks, control edges, branch conditions, and jump targets.
5. **Dataflow & Def-Use Chains:** Computing reaching definitions, variable lifecycles, and use-def links.
6. **Dataframe Semantics:** Modeling semantic transfer functions for `select`, `rename`, `mutate`, `transmute`, `filter`, and Base R dataframe operations.
7. **Column Provenance Engine:** Constructing and maintaining the fine-grained column derivation graph ($C_{in} \to C_{out}$).
8. **Static Backward Slicing:** Executing graph traversals from targeted column or statement criteria to produce minimized program slices.
9. **Bridge Orchestration:** Deciding when an unresolvable expression warrants a runtime query, formatting the request, and integrating returned runtime facts.

---

## 14. GNU R Runtime Responsibilities (Tier 2)

The GNU R Runtime is strictly an execution and observation subordinate. It carries exclusive responsibility for:
1. **Executing Real R Code:** Executing R expressions through the official GNU R interpreter.
2. **Dynamic Inspection:** Querying the live R session state (`colnames()`, `names()`, `dim()`, `class()`, `typeof()`).
3. **Environment Reflection:** Inspecting loaded packages, namespaces, and active search paths.
4. **Providing Ground-Truth Evidence:** Supplying observed execution facts to ground static assumptions.
5. **Failure & Error Reporting:** Capturing runtime evaluation errors and returning clean diagnostic structures without crashing the Rust engine.

### Strict Boundary Rule
**The GNU R Runtime is NEVER the analysis engine.** It does not construct ASTs, does not build CFGs, does not track def-use chains, and does not perform slicing. It is purely an oracle for dynamic execution and observation.

---

## 15. Runtime Bridge Boundary

### Conceptual Contract (No Premature IPC Protocol)
The communication boundary between Rust and R is governed by a strict architectural contract:

1. **Decoupled Lifecycle:** The Rust core must be able to boot, run all static analysis passes, and terminate without ever launching the GNU R runtime.
2. **On-Demand Invocation:** The R runtime is spawned or contacted only when explicitly enabled by the user or when static analysis encounters a configured dynamic boundary requiring runtime grounding.
3. **Rust $\to$ R Payload Schema:**
   - `action`: Evaluation, Inspection, or Environment Query.
   - `payload`: Source string or expression identifier.
   - `target_symbol`: Variable or dataframe to inspect.
   - `timeout_ms`: Strict execution timeout to prevent runaway scripts.
4. **R $\to$ Rust Response Schema:**
   - `status`: `Success`, `Error`, or `Timeout`.
   - `observed_schema`: List of confirmed column names and types.
   - `error_message`: Captured R condition message if evaluation failed.
   - `execution_context`: Metadata regarding the observed execution frame.
5. **Transport Agnosticism:** The architectural boundary does not dictate the wire transport. Transport options (e.g., JSON-RPC over standard I/O, domain sockets, or in-process `extendr`/C FFI) remain an implementation-level decision to be established during low-level design.

---

## 16. Static / Runtime Information Model

To ensure scientific validity and prevent false claims of soundness, R-PARSE explicitly models three distinct analytical knowledge states:

```text
                               ┌────────────────────────────────┐
                               │         KNOWN_STATIC           │
                               │  Statically proven dependency   │
                               │   or column derivation edge    │
                               └────────────────────────────────┘
                                                ▲
                                                │ (Static analysis alone
                                                │  resolves the construct)
                                                │
[ Program Expression ] ─────────────────────────┤
                                                │
                                                │ (Dynamic or unresolvable
                                                ▼  construct encountered)
                               ┌────────────────────────────────┐
                               │        UNKNOWN_STATIC          │
                               │ Static uncertainty explicitly   │
                               │ represented; no false guesses  │
                               └──────────────┬─────────────────┘
                                              │
                                              │ Runtime Bridge invoked
                                              │ (Observation in context E)
                                              ▼
                               ┌────────────────────────────────┐
                               │       RUNTIME_CONFIRMED        │
                               │ Column or dependency observed  │
                               │    in specific execution       │
                               └────────────────────────────────┘
```

### Semantics of Analytical States

1. **`KNOWN_STATIC`:**
   - The dependency or column derivation is statically established through AST normalization and transfer functions (e.g., `df %>% mutate(total = price * quantity)`).
   - Valid across all executions that conform to the modeled semantics.

2. **`UNKNOWN_STATIC`:**
   - The dependency cannot be resolved statically (e.g., `df %>% select(.data[[col_name]])` where `col_name` is an external runtime argument).
   - The analyzer explicitly emits an `UNKNOWN` edge. The analysis pipeline remains fully functional.

3. **`RUNTIME_CONFIRMED`:**
   - The dependency was observed in an actual execution of the program under GNU R with specific input data and environment state.
   - **Critical Principle:** `RUNTIME_CONFIRMED` describes an **observed empirical execution context**, NOT universal mathematical proof of whole-program static behavior. If the program is executed with different data or an alternate branch, the runtime facts may differ. R-PARSE documents this distinction explicitly.

---

## 17. Consequences

### Positive Consequences
- **Complete Autonomy:** Tooling (linters, pre-commit hooks, CI/CD) can run instantly as a standalone Rust binary without needing R installed.
- **Uncompromised Fidelity:** Data science workflows in interactive environments (Positron, VS Code, RStudio) can leverage active R sessions to resolve dynamic columns.
- **Clear Architectural Modularization:** The analysis passes in Rust cannot be polluted by base R quirks or single-threaded R interpreter locks.
- **Extensible Grounding:** Provides a structured mechanism for future dynamic verification tools (`rdtLite`) to correlate static provenance with runtime traces.

### Negative Consequences / Trade-offs
- **System Boundary Complexity:** Must manage cross-process lifecycle, serialization, and error recovery.
- **Potential State Divergence:** An observed runtime object state may reflect external side effects not visible in the static source script.

---

## 18. Rejected Alternatives

1. **Option A (Pure Rust Standalone):** Rejected because dynamic R pipelines (NSE masking, computed column indices, external data loading) would remain permanently ungroundable, severely limiting real-world adoption.
2. **Option B (TypeScript / Node.js Engine):** Rejected because Node.js has a heavy runtime footprint, lacks adoption in command-line data science workflows, and inherits `flowR`'s architectural limitations.
3. **Option C (Pure R + C Extension):** Rejected because executing static analysis inside the GNU R interpreter re-introduces single-threaded blocking, GC pauses, slow graph traversals, and a mandatory GNU R runtime dependency.

---

## 19. Risks

1. **Runaway Execution Risk:** Evaluating untrusted R expressions in the runtime bridge could hang (infinite loops) or cause destructive side effects (e.g., `rm()`, file deletion).
2. **Process Crash Propagation:** A crash or segmentation fault in an R C-extension could terminate the host process if tightly coupled.
3. **Latency Overhead:** Cross-process serialization could introduce latency if fine-grained queries are made repeatedly.
4. **Environment Discrepancy:** The R environment running the bridge may have different packages or options than the environment where the script was authored.

---

## 20. Mitigations

1. **Strict Execution Timeouts & Read-Only Directives:** All runtime requests must enforce strict wall-clock timeouts (e.g., 500ms default) and execute within isolated temporary environments.
2. **Process-Level Isolation:** Tier 2 operates as a separate process or decoupled worker; a crash in GNU R will never crash the Tier 1 Rust engine.
3. **Batching & Coarse-Grained Queries:** The Rust core collects all uncertain symbols within a basic block and dispatches a single batched inspection query rather than chatty micro-queries.
4. **Graceful Degradation to `UNKNOWN_STATIC`:** If GNU R is not installed, times out, crashes, or produces an error, the Rust engine automatically falls back to `UNKNOWN_STATIC` without aborting the slice.

---

## 21. Future Extensions (Preserved as Deferred)

The inclusion of the GNU R runtime bridge in Tier 2 does **not** change the status of deferred domain extensions:
- **Formula Semantics (`~`):** Still deferred. May eventually use runtime inspection to resolve model matrices (`model.matrix()`), but is not in the MVP.
- **ggplot2 Semantics (`+.gg`):** Still deferred. May eventually inspect runtime `ggplot_build()` layers, but is not in the MVP.
- **LSP Server / Interactive Code Actions:** Still deferred. Will expose static and runtime-assisted slices once the core engine is validated.

---

## 22. Implementation Constraints

1. **Rust Core Cleanliness:** Tier 1 crates must compile cleanly on standard Rust 2021 edition with zero dependency on GNU R headers or libraries.
2. **R Detection:** Tier 2 must detect GNU R via standard environment variables (`R_HOME`, `PATH`) and degrade silently to offline mode if R is unavailable.
3. **No Embedded Interpreter:** R-PARSE must not embed a custom or third-party R interpreter (e.g., no micro-R implementations). Execution is delegated strictly to GNU R.
4. **Provenance Graph Invariant:** Every node and edge in the provenance graph must track its origin (`STATIC_DERIVATION` vs `RUNTIME_OBSERVATION`).

---

## 23. Acceptance Criteria

The Option D architecture is correctly and successfully specified when:
- [x] The Rust core performs parsing, AST normalization, CFG, column provenance, and static slicing completely autonomously without R.
- [x] GNU R is utilized strictly for real execution and observation, never as a replacement analyzer.
- [x] The runtime bridge boundary cannot mutate or contaminate the Rust semantic program representation.
- [x] `UNKNOWN_STATIC` remains an authoritative, valid outcome when runtime assistance is disabled or unavailable.
- [x] `RUNTIME_CONFIRMED` observations are explicitly distinguished from universal static proofs.
- [x] Fine-grained column provenance ($C_{in} \to C_{out}$) remains the central representation.
- [x] Column-aware static program slicing remains the primary analytical output.
- [x] No deferred feature (Formulas, ggplot2, LSP) is prematurely promoted into MVP.
- [x] The core research thesis remains unchanged and authoritatively enforced.

---

```text
ARCHITECTURE DECISION ACCEPTED —
OPTION D: HYBRID RUST CORE + GNU R RUNTIME BRIDGE
```
