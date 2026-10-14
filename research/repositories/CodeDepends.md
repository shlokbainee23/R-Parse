# CodeDepends — Repository Analysis

## 1. Project Identity

- **Name:** CodeDepends
- **Repository:** https://github.com/duncantl/CodeDepends
- **Organization:** Duncan Temple Lang (UC Davis) & Gabriel Becker (Genentech)
- **Language:** R (100%)
- **License:** GPL (>= 2)
- **Status:** Academic / foundational research prototype (last active updates around tidyverse support)
- **Size:** 25 R source files in `R/`, ~150 KB code
- **Dependencies:** `codetools`, `methods`, `graph` (optional), `Rgraphviz` (optional)

---

## 2. Purpose & Problem Solved

Analyzes R scripts to extract dependencies between expressions, functions, and variables. Its goal is to allow automated workflow recreation, script reorganization, caching, and execution DAG generation by statically inferring which variables an expression reads (inputs) and which it modifies or creates (outputs).

---

## 3. Architecture & Execution Flow

```text
R Script / Expressions
      │
      ▼
readScript() / parse()
      │
      ▼
getInputs() [R/codeDepends.R line 185]
      │
      ├─► Dispatches to Function Handler Registry [R/functionHandlers.R line 696]
      │     ├─► Assignment handler (`<-`, `=`, `<<-`)
      │     ├─► Pipe handler (`%>%`)
      │     ├─► Non-Standard Evaluation (NSE) handlers (`subset`, `filter`, `mutate`, `aes`)
      │     ├─► Higher-Order Function handlers (`lapply`, `apply`, `purrr::map`)
      │     ├─► Modeling Formula handler (`~`, `lm`)
      │     └─► Default fallback handler
      │
      ▼
inputCollector() state [R/codeDepends.R line 31]
      ├─► inputs (variables read)
      ├─► outputs/set (variables created)
      ├─► updates (in-place variable mutations)
      ├─► functions/calls (functions invoked)
      ├─► libraries (packages loaded)
      ├─► files (file paths accessed)
      └─► nsevalVars (NSE / column symbols)
      │
      ▼
makeCodeGraph() / makeCallGraph() [R/graph.R, R/callGraph.R]
      └─► Dependency Graph between Script Expressions
```

---

## 4. Parsing Mechanism

- Delegates entirely to base R's native `parse()`.
- Walks the resulting R S-expression AST recursively in R using `is.call()`, `is.symbol()`, `is.name()`, `as.character(e[[1]])`.

---

## 5. Domain-Specific Semantic Understanding (Function Handlers)

CodeDepends contains the most comprehensive **domain-specific semantic catalog** in the R ecosystem (`R/functionHandlers.R`, 789 lines, 34 KB):

1. **Explicit Tidyverse Comprehension (`functionHandlers.R` lines 709-775):**
   - **`%>%` (Magrittr Pipe):** Threads LHS into the first parameter (or `.` placeholder) of RHS.
   - **Data-Masking / NSE:** Differentiates regular arguments from data-mask arguments using `nseafterfirst` and `nsehandlerfactory`.
     - In `filter(df, a > 5)`, `df` is classified as a standard variable input, while `a` is tagged as `nsevalVars`.
     - Handlers registered for `filter`, `mutate`, `transmute`, `rename`, `slice`, `summarise`, `arrange`, `select`, `group_by`, `distinct`.
   - **Reshaping:** Custom handlers for `gather`, `spread`, `separate`, `unnest`.
   - **Purrr Functional Mapping:** Custom handlers for `map`, `map_dbl`, `map_chr`, `map_int`, `map2_dbl`, `pmap`, extracting the callback function argument (`.f`).
2. **Statistical Formulas (`formulahandler`, lines 150-184):**
   - Specifically analyzes `~` formulas (e.g. `y ~ x + z`).
   - Distinguishes formula variables intended to come from the global scope versus variables intended to be looked up inside a data frame environment (controlled via `formulaInputs` parameter).
3. **Higher-Order Functions:**
   - `apply`, `lapply`, `sapply`, `mapply`, `tapply` handlers extract the function argument position and resolve called symbols recursively.
4. **Side Effect Tracking:**
   - Detects plotting calls (`R/isPlot.R`), file reads/writes (`libreqhandler`, `datahandler`), and symbol removal (`rmhandler`).

---

## 6. What CodeDepends Does NOT Do (Verified Gaps)

1. **No Granular SSA / Value-Flow:** Dependency tracking is at the **coarse expression/block level** (`ScriptNodeInfo`), not at the fine-grained statement or SSA-variable level.
2. **Fragile Handlers for Modern Tidyverse:** Designed around older tidyverse verbs (`gather`, `spread`, `mutate_each`); does not model modern `across()`, `pick()`, or tidy-select operators (`where()`, `starts_with()`).
3. **No Program Slicing:** Cannot compute executable backward or forward slices.
4. **No Type Inference:** Does not know vector types, column types, or dataframe shapes.
5. **No Tree-sitter / Incremental Parsing:** Relies on R's native AST traversal.

---

## 7. Strengths

- **Rich Semantic Knowledge Base:** The function handler architecture provides the earliest and most detailed model in the literature for solving R's NSE/data-masking ambiguity.
- **Script-Level Dependency Graphs:** Directly constructs execution DAGs from scripts.

---

## 8. Limitations & Failure Modes

- **Maintenance Decay:** Infrequent releases; relies on legacy Bioconductor/graph packages.
- **Evaluation Leaks:** Some handlers invoke `eval()` or `codetools::findGlobals()` during static analysis (`funchandler`, line 143), which can trigger unexpected side effects.

---

## 9. Relevance to R-PARSE

- **Blueprint for Semantic Dispatch:** The function-handler registry pattern is essential for any R static analyzer that hopes to understand tidyverse data manipulation and statistical modeling.
