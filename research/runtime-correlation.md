# Runtime Correlation & Dynamic Verification Study

This document analyzes the divergence between static analysis models and real runtime execution behavior in R, and outlines how dynamic execution tools (`rdtLite`, `rpy2`) provide empirical ground truth for validating static tools.

---

## 1. Why Static Analysis of R Diverges from Runtime Reality

R was designed from its inception (inheriting from Scheme and S) as an interactive, highly dynamic programming environment. Several fundamental language features make purely static analysis sound only under conservative approximations:

### 1.1 Non-Standard Evaluation (NSE) & Data Masking
- **The Mechanic:** Functions in R can capture unevaluated argument expressions via `substitute()` and evaluate them in custom environment frames constructed on the fly (such as the rows or columns of a data frame).
- **Static Ambiguity:**
  ```r
  filter(data, status == "active" & score > threshold)
  ```
  Statically, it is impossible without runtime knowledge or schema annotations to know whether `status`, `score`, and `threshold` are:
  - Columns within the dataframe `data`, OR
  - Variables in the caller's lexical enclosing environment.
- **Consequence:** Static tools that do not model data-masking either generate false positive warnings ("undefined variable `status`") or miss true bugs.

### 1.2 Lazy Evaluation & Promise Semantics
- **The Mechanic:** Function arguments are not evaluated when passed. Instead, they are wrapped in **promises** (capturing the expression and the calling environment). A promise is only evaluated when its value is explicitly accessed (forced).
- **Default Argument Execution:** Default arguments are evaluated inside the **function body's environment**, not the caller's environment:
  ```r
  compute <- function(x, n = length(x)) { ... }
  ```
- **Consequence:** Static analyzers must model two distinct evaluation contexts for function parameters depending on whether the argument was supplied by the caller or defaulted by the callee.

### 1.3 Metaprogramming & Dynamic Scope Manipulation
- Functions like `assign()`, `get()`, `rm()`, `attach()`, and `eval(parse(text = ...))` modify environment symbol tables at runtime without explicit syntactic assignment tokens.
- **Empirical Prevalence:** While discouraged in modern tidyverse guidelines, dynamic assignment remains common in legacy CRAN packages and statistical simulation scripts.

### 1.4 Dynamic S3/S4 Method Dispatch
- Method resolution occurs at runtime by inspecting the `class` attribute of the dispatch argument.
- Because an object's class can be arbitrarily mutated in R via `class(x) <- c("custom", "base")`, static analyzers cannot guarantee which method implementation will execute without robust class inference.

---

## 2. Dynamic Ground Truth & Empirical Verification

To ensure that static analysis claims do not remain purely theoretical, the R ecosystem provides two dynamic engines capable of generating empirical ground truth:

### 2.1 rdtLite (Runtime Provenance Graph Oracle)
- **Role:** Empirical ground truth for **dataflow and dependency graphs**.
- **Operation:**
  - `rdtLite::prov.run("script.R")` instruments execution statement by statement.
  - Generates a W3C PROV-JSON Data Derivation Graph (DDG) recording:
    - Exactly which variables were created at each step.
    - Exactly which memory addresses and variable versions flowed into which statements.
    - The cryptographic hash and snapshot of intermediate data frames.
- **Verification Protocol for R-PARSE:**
  1. Take an R script.
  2. Run R-PARSE static analysis to generate predicted Def-Use edges and program slices.
  3. Run `rdtLite` on the same script to generate the dynamic DDG.
  4. Compare the static graph against the dynamic DDG to compute **Precision** (avoidance of false dependencies) and **Recall** (capture of true runtime flows).

### 2.2 rpy2 (In-Process Execution & State Oracle)
- **Role:** High-speed automated test execution oracle for Python/standalone test harnesses.
- **Operation:**
  - Embeds GNU R in-process without shell spawning overhead.
  - Allows unit test runners to inspect memory frames, symbol tables, and evaluated expressions directly after code modification.

---

## 3. Recommended Soundness Strategy for R-PARSE

To remain both useful and technically honest, R-PARSE should adopt a **tiered certainty model**:

1. **Provably Static Constructs (Certainty Level: Tier 1 - Sound):**
   - Pure lexical bindings (`<-`, `->`, `=`, `function`, `local()`).
   - Standard control flow (`if/else`, loops, returns).
   - Base primitives with fixed argument evaluation.
2. **Heuristically Bound Constructs (Certainty Level: Tier 2 - High Confidence):**
   - Well-modeled domain packages (`dplyr`, `tidyr`, `ggplot2`, `purrr`) using explicit function signatures.
   - Formula arguments (`~`) paired with identifiable `data=` arguments.
3. **Dynamic / Unconstrained Constructs (Certainty Level: Tier 3 - Unknown / Dynamic):**
   - Calls to `eval()`, `assign()`, `do.call()`, `source()`.
   - Dynamic formula construction (`as.formula(paste(...))`).
   - Unrecognized NSE functions without registered handlers.

When R-PARSE encounters Tier 3 constructs, it should explicitly mark affected AST nodes and dataflow edges as **UNKNOWN / DYNAMIC** rather than making unverified assumptions.
