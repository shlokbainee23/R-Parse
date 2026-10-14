# Empirical Benchmark & Validation Plan for R-PARSE

This document establishes the empirical benchmark suite designed to evaluate R-PARSE against existing parsers, linters, and analysis engines.

---

## 1. Benchmark Dimensions & Target Metrics

```
┌────────────────────────────────────────────────────────────────────────┐
│                        BENCHMARK SPECIFICATIONS                        │
├───────────────────┬────────────────────────────┬───────────────────────┤
│ Dimension         │ Competitors                │ Target Metric         │
├───────────────────┼────────────────────────────┼───────────────────────┤
│ 1. Cold Throughput│ base R, xmlparsedata, TS   │ > 200,000 LOC / sec   │
│ 2. Memory Footpr. │ xml2 DOM, flowR V8 heap    │ < 100 bytes / AST node│
│ 3. Incremental Lat│ base R (full reparse), TS  │ < 1.0 ms per keystroke│
│ 4. Error Recovery │ base R, ANTLR, TS-r        │ 100% partial CST yield│
│ 5. Dataflow Accur.│ targets, CodeDepends, rdtL │ Precision & Recall    │
└───────────────────┴────────────────────────────┴───────────────────────┘
```

---

## 2. Benchmark Corpus

The benchmark suite utilizes a curated corpus of top CRAN / Bioconductor packages representing distinct coding idioms:

| Package | Lines of Code (LOC) | Characteristics & Idioms Represented |
| :--- | :--- | :--- |
| **`ggplot2`** | ~45,000 | Deep S3 class usage, `+.gg` chaining, non-standard evaluation (`aes()`). |
| **`dplyr`** | ~35,000 | Tidyverse data-masking, `tidyselect` expressions, C++ (`cpp11`) bindings. |
| **`data.table`** | ~50,000 | Complex bracket sub-assignment (`[i, j, by]`), in-place mutation (`:=`). |
| **`base R (src/library/base)`** | ~80,000 | Idiomatic base R, primitive calls, S3 generics, vectorized algorithms. |
| **`targets`** | ~30,000 | Metaprogramming, pipeline DSL expressions, language object manipulation. |

---

## 3. Detailed Benchmark Protocols

### Protocol 1: Parsing Throughput (Cold & Warm)
- **Objective:** Measure cold process startup time and continuous parsing throughput.
- **Methodology:**
  1. Parse each corpus package from disk.
  2. Measure elapsed wall-clock time and peak resident set size (RSS).
  3. Compare:
     - Base R `parse(keep.source = TRUE)` via `Rscript`.
     - Base R + `xmlparsedata` + `xml2::read_xml()`.
     - `tree-sitter-r` C standalone CLI.
     - `flowR` (Node.js engine).
     - R-PARSE candidate parser.

### Protocol 2: Incremental Keystroke Reparsing Latency
- **Objective:** Measure latency when updating a document during active editing.
- **Methodology:**
  1. Load a 5,000 LOC R file into memory.
  2. Simulate 1,000 single-character edits at random positions (insertions, deletions, replacements).
  3. Measure the time required to restore a valid syntax tree and update diagnostics.
  4. Pass Criteria: 99th percentile latency must remain below **5 milliseconds**.

### Protocol 3: Fault Tolerance & Error Recovery Stress Test
- **Objective:** Verify parser resilience against malformed syntax.
- **Test Corpus:** 500 synthetic code snippets with intentional errors:
  - Unclosed parentheses, braces, and brackets (`(`, `[`, `{`).
  - Dangling binary operators (`x +`, `df %>%`).
  - Misplaced keywords (`if else x`).
  - Unterminated string literals.
- **Evaluation Criteria:**
  - Parser must not crash or enter infinite loops.
  - Sibling statements surrounding the error must parse into valid AST nodes.

### Protocol 4: Dataflow Soundness & Ground-Truth Verification
- **Objective:** Validate that static def-use and dependency graphs reflect runtime reality.
- **Methodology (using `rdtLite` as Oracle):**
  1. Select 50 standalone R data science analysis scripts.
  2. Run `rdtLite::prov.run()` on each script to collect the ground-truth PROV-JSON Data Derivation Graph (DDG).
  3. Run R-PARSE static dataflow analysis on the same scripts.
  4. Compute edge correlation:
     - **False Positives:** Static edges predicted that never occur dynamically.
     - **False Negatives:** Dynamic data derivations that the static analyzer missed.
  5. Compare results directly against `targets` (`tar_deps`) and `CodeDepends` (`getInputs`).
