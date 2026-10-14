# Visualization Ecosystem & Grammar of Graphics Semantics Study

This document investigates how existing R static analysis tools interact with data visualization code, contrasting base R procedural plotting with the declarative Grammar of Graphics (`ggplot2`).

---

## 1. Paradigms of Visualization in R

R contains two fundamentally conflicting visualization architectures:

1. **Procedural Side-Effect Graphics (Base R `graphics` / `grDevices`):**
   - Execution model: Commands write directly to an active graphics device canvas (`plot()`, `lines()`, `points()`, `abline()`, `legend()`).
   - Return value: Typically `NULL` or invisible; state lives entirely inside the C graphics engine.
2. **Declarative Grammar of Graphics (`ggplot2`):**
   - Execution model: A plot is a first-class R object of S3 class `c("gg", "ggplot")`. Layers, aesthetics, scales, coordinate systems, and facets are accumulated via the overloaded `+` operator (`+.gg`).
   - Evaluation model: The plot canvas is only rendered when `print()` or `plot()` is explicitly or implicitly called on the `ggplot` object.

---

## 2. Evidence of Visualization Handling in Existing Tools

### 2.1 CodeDepends: Plot Detection & `aes()` Handlers
- **Plot Detection (`R/isPlot.R`):**
  - Contains a hardcoded heuristic list of base plotting functions (`plot`, `hist`, `barplot`, `boxplot`, `pie`, `pairs`, `contour`, `image`, `persp`).
  - Classifies expressions containing these calls as "plot-producing" side-effect nodes in the dependency graph.
- **Aesthetic Mapping Handler (`R/functionHandlers.R` line 709):**
  - Registers `aes = fullnsehandler`.
  - Recognizes that inside `aes(x = wt, y = mpg, color = cyl)`, the argument expressions are non-standard evaluation (NSE) column names rather than variables in the caller environment.
- **Verdict:** CodeDepends correctly tags `aes()` arguments as NSE, but cannot verify whether those columns exist in the dataset or what visual channels they bind to.

### 2.2 rdtLite: Dynamic Device Hooking
- **Implementation:** `R/IOTrace.R`.
- **Mechanism:** Intercepts runtime calls to graphics devices (`grDevices::pdf`, `png`, `jpeg`, `dev.off()`). When a graphics device closes, rdtLite hashes the resulting image file and creates an output artifact node in the PROV-JSON graph.
- **Verdict:** Dynamic execution capture works reliably, but requires executing the entire script and produces zero static insights before run-time.

### 2.3 flowR, lintr, tree-sitter-r: The `+.gg` Semantic Blindspot
- **Implementation:** Binary operator parsing.
- **Failure Mode:** In `p <- ggplot(df, aes(x, y)) + geom_point() + geom_smooth()`, every static tool models `+` as standard **numeric addition** (`BinaryOperator.Add`).
- **Consequences:**
  - In `flowR`: Models dataflow through `+` as an arithmetic operation. Cannot infer that `geom_point()` mutates or extends the layer stack of `ggplot()`.
  - In `lintr`: Arithmetic linters (e.g. spaces around operators, line length) apply generically, but cannot validate ggplot2 syntax (e.g. misuse of `%>%` instead of `+`, mapping constants inside `aes()` instead of outside, or invalid aesthetic parameters like `size = "blue"`).
  - In `languageserver`: Code completion inside `geom_*` cannot autocomplete column names from the upstream `ggplot(data = ...)` call because it does not model the accumulated plot object pipeline.

---

## 3. Evidence-Backed Gaps in Visualization Understanding

| Visualization Capability | Current Status Across Ecosystem | Technical Barrier |
| :--- | :--- | :--- |
| **Declarative Layer Stack Modeling (`+.gg`)** | **Completely Missing.** Treated as arithmetic `+`. | Requires dedicated `PlotPipelineNode` in AST recognizing `+.gg` layer composition. |
| **Aesthetic Channel Validation** | **Completely Missing.** No static tool checks if mandatory aesthetics (e.g. `ymin`, `ymax` for `geom_ribbon`) are supplied. | Requires layer geometry schema specifications (catalog of `geom_*` requirements). |
| **Piping Operator Confusion (`+` vs `\|>`)** | **Syntax Error at Runtime Only.** Common user trap: using `%>%` or `\|>` instead of `+` on ggplot objects. | Linters catch this only via brittle regex/XPath matching. |
| **Continuous vs. Discrete Scale Type Checking** | **Completely Missing.** Mapping a continuous variable to a discrete-only scale (or vice-versa) crashes at runtime. | Requires column type propagation from dataframe into aesthetics. |
| **Plot Return Value vs. Render Side-Effect** | **Confused in Dependency Analyzers.** Code that builds a ggplot without printing it performs no output, yet tools classify it as plotting. | Requires tracking object consumption up to `print()` or saving calls (`ggsave()`). |

---

## 4. Architectural Opportunity for R-PARSE

1. **Semantic Plot Pipeline IR:** Lower `ggplot2` chains not into arithmetic binary trees, but into a canonical `VisualizationGraph` representing:
   - Data Source (`df`)
   - Aesthetic Bindings (`x -> colA`, `y -> colB`, `color -> colC`)
   - Layer Sequence (`PointLayer`, `SmoothLayer`)
   - Coordinate & Facet Specs.
2. **Static Visualization Linters:** First-in-class static validation of ggplot2 code:
   - Detecting constants mistakenly placed inside `aes(color = "red")` (which creates a categorical variable named `"red"`).
   - Validating required aesthetics per geom before script execution.
   - Recommending appropriate geoms based on inferred column types.
