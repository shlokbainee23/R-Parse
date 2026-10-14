# Workflow & Dataflow Analysis Study

This document analyzes how existing R systems represent, analyze, and optimize data transformation workflows and computational pipelines.

---

## 1. The Anatomy of Modern R Workflows

Modern R code is dominated by linear data-transformation pipelines composed of:
1. **Pipe Operators:** The Magrittr pipe (`%>%`) and base R pipe (`|>`).
2. **Table Transformation Verbs (`dplyr` / `tidyr`):** `select`, `filter`, `mutate`, `summarise`, `group_by`, `arrange`, `pivot_longer`, `pivot_wider`.
3. **Higher-Order Functional Iteration (`purrr` / `lapply`):** `map`, `map_dfr`, `apply`, `lapply`.
4. **Coarse-Grained Pipeline Orchestration (`targets`):** Declarative DAGs of long-running data ingestion, modeling, and rendering tasks.

---

## 2. Comparison of Workflow Modeling Across Surveyed Projects

### 2.1 flowR: Statement-Level Fine-Grained Dataflow Graph
- **Implementation:** `src/dataflow/` and `src/slicing/`.
- **Approach:** Constructs a fine-grained, statement-level reaching-definitions dataflow graph with explicit SSA-like versioning.
- **Pipe Handling:** Unrolls pipelines into chained expressions where intermediate values flow along direct edges.
- **Strengths:** 
  - Allows backward and forward program slicing: given a target variable or table, flowR extracts only the upstream statements required to compute it.
  - Detects dead code branches and unread variables with high precision.
- **Weaknesses:**
  - **Column-Agnostic:** flowR models dataframes as monolithic scalar values. If statement 1 adds column `A` and statement 2 reads column `B`, flowR sees a complete data dependency between statement 1 and 2, unable to trace individual column lifecycles.

### 2.2 CodeDepends: Expression Block-Level Input/Output Analysis
- **Implementation:** `R/codeDepends.R` (`getInputs`, `inputCollector`) and `R/functionHandlers.R`.
- **Approach:** Analyzes each top-level expression block to extract inputs (read), outputs (assigned), updates (in-place modified), and library dependencies.
- **Pipe & Tidyverse Handling:** Uses specialized function handlers (`pipehandler`, `filterhandler`, `nseafterfirst`) to differentiate dataframe inputs from column arguments.
- **Strengths:**
  - Captures script-level dependencies without requiring execution.
  - Can reorder independent code chunks or build visual script dependency graphs (`makeCodeGraph`).
- **Weaknesses:**
  - **Coarse Granularity:** Cannot perform statement-level slicing inside complex functions or multi-line blocks.
  - **No Value Propagation:** Does not propagate constant values or type information forward through the pipeline.

### 2.3 targets: Declarative Pipeline Make-DAG
- **Implementation:** `R/class_pipeline.R`, `R/tar_deps.R`, `R/utils_language.R`.
- **Approach:** Operates strictly at the inter-task level. Targets defined in `tar_target(name, command)` form vertices in an `igraph` DAG.
- **Dependency Detection:** Uses `codetools::findGlobals()` to discover target dependencies.
- **Strengths:**
  - Production-grade execution caching, hash invalidation, and multi-core / cloud execution.
- **Weaknesses:**
  - **Black-Box Commands:** Intra-command statements are completely opaque to targets.
  - **NSE Column Leakage:** Column names used inside dplyr verbs are frequently misidentified as missing targets, forcing awkward workarounds.

---

## 3. The Unsolved Column-Level Dataflow Gap

Across all surveyed projects, the greatest unaddressed challenge in R workflow analysis is **fine-grained column-level lineage (schema provenance)**.

### Illustrative Scenario:
```r
df_raw <- read_csv("data.csv")            # Columns: id, age, income, zipcode
df_clean <- df_raw %>%
  filter(age > 18) %>%                     # Reads: age
  mutate(tax = income * 0.2) %>%           # Reads: income; Creates: tax
  select(id, tax)                          # Keeps: id, tax; Drops: age, income
final_sum <- sum(df_clean$tax)             # Reads: tax
```

### Analysis Capability Breakdown:
1. **Current Tools (flowR, CodeDepends, targets):**
   - Trace `df_raw -> df_clean -> final_sum` as whole-object dependencies.
   - If `age` is modified in the input, all downstream steps are deemed affected, even though `final_sum` depends mathematically only on `income`.
2. **Column-Level Lineage Requirement (Unimplemented in R):**
   - Ability to trace that `final_sum` depends on `df_raw$income`, but is **completely independent** of `df_raw$age` and `df_raw$zipcode`.
   - Enables fine-grained change impact analysis, automated feature store generation, and targeted data caching.

---

## 4. Key Architectural Takeaways for R-PARSE

1. **Object-Level Dataflow is Solved (flowR / CodeDepends):** Building basic block CFGs and whole-variable def-use chains is established practice.
2. **Column-Level Lineage is the Open Frontier:** Statically inferring schema transformations through `select`, `mutate`, `rename`, and `pivot` is the single most valuable workflow analysis capability currently missing in the R language ecosystem.
