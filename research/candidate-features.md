# Candidate Opportunities & Differentiators for R-PARSE

Based on the evidence-backed gap analysis, this document identifies and evaluates candidate technical capabilities and architectural opportunities for R-PARSE.

> **CRITICAL REMINDER:** These are candidate opportunities for architectural evaluation. The final scope and architecture of R-PARSE are **NOT** frozen yet.

---

## 1. Candidate Capability Portfolio

```
┌────────────────────────────────────────────────────────────────────────────┐
│                    R-PARSE CANDIDATE CAPABILITY MATRIX                     │
├────┬─────────────────────────────┬────────────────────┬──────────────────┤
│ ID │ Candidate Feature           │ Technical Feasib.  │ Ecosystem Impact │
├────┼─────────────────────────────┼────────────────────┼──────────────────┤
│ C1 │ Standalone Native Core      │ High (Rust / C++)  │ Transformative   │
│ C2 │ Extended Tree-sitter Parser │ Very High          │ Essential        │
│ C3 │ Column-Level Schema Lineage │ Moderate           │ High (Novel)     │
│ C4 │ Statistical Formula IR      │ High               │ High (Novel)     │
│ C5 │ ggplot2 Layer Composition IR│ High               │ Moderate (Novel) │
│ C6 │ IDE-Integrated Slicing      │ Moderate           │ High             │
│ C7 │ 100x Native Linter Engine   │ High               │ Transformative   │
└────┴─────────────────────────────┴────────────────────┴──────────────────┘
```

---

## 2. Detailed Candidate Profiles

### Candidate C1: Standalone Native Core Engine (Rust / C++)
- **Description:** A self-contained, single-binary engine providing parsing, normalized AST, control flow graph construction, and dataflow analysis.
- **Why It Matters:** Eliminates the hard GNU R runtime dependency that handicaps `lintr` and `languageserver`. Enables instant CLI startup (<10ms vs 500ms+ for Rscript) and seamless cross-platform execution (Linux, macOS, Windows, WASM in browser).
- **Evidence of Demand:** Every modern developer tooling ecosystem has transitioned to native rust/C++ engines (Ruff in Python, Biome/Oxc in JavaScript). R currently has no equivalent.

### Candidate C2: Extended Tree-sitter Parser (100% R 4.x Coverage)
- **Description:** Fork or extend `tree-sitter-r` to fix identified grammar gaps, notably the R 4.2+ native pipe placeholder (`_`) and raw string edge cases.
- **Why It Matters:** Gives R-PARSE a bulletproof, fault-tolerant, sub-millisecond incremental parsing foundation with complete modern language coverage.

### Candidate C3: Schema-Aware Tidyverse & Column-Level Lineage
- **Description:** Static modeling of standard tidyverse verbs (`select`, `mutate`, `filter`, `transmute`, `rename`) to maintain a virtual schema table across pipeline steps.
- **Why It Matters:**
  - Solves the longstanding issue of false positive global dependencies in workflow tools (`targets`, `codetools`).
  - Enables fine-grained column dependency tracking: determining which raw data columns actually flow into a given chart or model.

### Candidate C4: Statistical Formula Semantic IR
- **Description:** A dedicated intermediate representation for Wilkinson-Rogers formulas (`y ~ x1 + x2 + (1|group)`), distinguishing model response, fixed effects, interactions (`:`), crossings (`*`), and group intercepts (`|`).
- **Why It Matters:** Bridges the historical gap where static analyzers treated statistical models as generic arithmetic operations. Allows static validation of model specifications against supplied dataset columns.

### Candidate C5: Visualization Pipeline (`ggplot2`) Semantic Modeling
- **Description:** Explicitly modeling `+.gg` as layer stack accumulation rather than numeric addition.
- **Why It Matters:** Enables first-in-class static validation of visualization code: catching misplaced constants in `aes()`, validating required aesthetics per geometry before rendering, and autocompleting columns inside layers.

### Candidate C6: IDE-Integrated Static Program Slicing (LSP)
- **Description:** Exposing backward and forward program slicing as first-class LSP code actions (e.g. "Generate minimal reproducible script for variable `p`").
- **Why It Matters:** Transforms academic research (flowR) into practical, everyday developer value in VS Code, Positron, and Neovim.

### Candidate C7: Native Execution of the Linter Catalog
- **Description:** Implementing a native rule execution engine capable of evaluating the 118 lintr rules at 100x speed without R runtime overhead.
- **Why It Matters:** Enables instant pre-commit hooks and real-time keystroke diagnostics on multi-thousand line R files.
