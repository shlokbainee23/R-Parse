# lintr — Repository Analysis

## 1. Project Identity

- **Name:** lintr
- **Repository:** https://github.com/r-lib/lintr
- **Organization:** r-lib (RStudio / Posit)
- **Language:** R (100%)
- **License:** MIT
- **Status:** Active, industry standard for R code style and static analysis
- **Size:** 152 R source files, including **118 dedicated `*_linter.R` implementations**
- **Dependencies:** `xmlparsedata`, `xml2`, `rex`, `knitr`, `digest`, `cli`

---

## 2. Purpose & Problem Solved

Provides static code analysis and linting for R scripts, packages, and literate programming documents (`.Rmd`, `.qmd`, `.Rnw`). It detects syntax anomalies, stylistic deviations from the tidyverse style guide, potential runtime traps (e.g., `is.null(x) == TRUE`, class comparison via `==`), testing pitfalls in `testthat`, and maintainability issues (cyclomatic complexity).

---

## 3. Architecture & Execution Flow

```text
Target File (.R, .Rmd, .qmd)
      │
      ▼
lint() / lint_dir() / lint_package() [R/lint.R line 45]
      │
      ▼
get_source_expressions() [R/get_source_expressions.R line 1]
      ├─► readLines()
      ├─► parse(keep.source=TRUE)
      ├─► utils::getParseData()
      └─► xmlparsedata::xml_parse_data()
            │
            ├─► List of per-expression nodes: xml_parsed_content
            └─► Full file node: full_xml_parsed_content
      │
      ▼
Linter Loop (Iterates over configured linters in linters.R)
      │
      ├─► Evaluates XPath expressions via xml2::xml_find_all()
      │     (e.g., //SYMBOL_FUNCTION_CALL[text() = 'class'] / following-sibling::EQ_ASSIGN)
      │
      └─► Maps matched XML node attributes (line1, col1, line2, col2) -> Lint objects
      │
      ▼
Exclusion Filter & Reporting (CLI / GitHub Actions / SARIF)
```

---

## 4. Parsing Mechanism

- Delegates entirely to base R's native C parser via `parse(keep.source=TRUE)`.
- Handles multi-language files (`.Rmd`, `.qmd`) by extracting R chunks via `knitr::purl()` or custom chunk splitters before passing R code to `parse()`.
- Captures parse errors as `Lint` objects rather than crashing (`R/get_source_expressions.R` line 53).

---

## 5. AST & IR Representation

- Uses the XML DOM tree produced by `xmlparsedata`.
- Linters navigate this tree using XPath 1.0 queries.
- Each `source_expression` object contains:
  - `parsed_content`: base R's `getParseData()` data frame.
  - `xml_parsed_content`: `xml2` document node for the specific top-level expression.
  - `full_xml_parsed_content`: `xml2` document node representing the entire file (all expressions combined).
  - Helper functions like `xml_find_function_calls(function_names)`.

---

## 6. Static Analysis Engine & Rule Taxonomy (118 Linters)

The rule inventory covers five primary categories:

1. **Style & Formatting:**
   - `assignment_linter`: Disallows `=` for variable assignment (enforces `<-`).
   - `brace_linter`, `commas_linter`, `indentation_linter`, `line_length_linter`, `quotes_linter`, `trailing_whitespace_linter`.
2. **Bug Prevention & Defensive Programming:**
   - `class_equals_linter`: Flags `class(x) == "foo"` (vulnerable to inheritance / multi-classes; enforces `inherits(x, "foo")`).
   - `equals_na_linter`: Flags `x == NA` (enforces `is.na(x)`).
   - `matrix_apply_linter`: Flags misuse of `apply()` on matrices where `rowSums`/`colSums` is required.
   - `duplicate_argument_linter`: Flags accidental repeated keyword parameters in calls.
   - `condition_message_linter`: Flags invalid message generation in `stop()` or `warning()`.
3. **Efficiency & Idiom:**
   - `any_duplicated_linter`: Enforces `anyDuplicated(x) > 0` over `any(duplicated(x))`.
   - `seq_linter`: Flags `1:length(x)` or `1:nrow(x)` (enforces `seq_along(x)` or `seq_len(x)` to avoid `1:0` bug).
   - `vector_logic_linter`: Flags `&` or `|` inside `if (...)` conditions (enforces `&&`, `||`).
4. **Testing (`testthat` rules):**
   - `expect_comparison_linter`, `expect_identical_linter`, `expect_length_linter`, `expect_named_linter`, `expect_null_linter`, `expect_true_false_linter`.
5. **Structural Complexity:**
   - `cyclocomp_linter`: Calculates cyclomatic complexity per function by counting branching tokens (`IF`, `FOR`, `WHILE`, `AND`, `OR`).

---

## 7. What lintr Does NOT Do (Verified Gaps)

1. **No Dataflow Analysis:** Cannot determine whether variable `x` defined on line 5 reaches line 50, or if a variable is used before initialization across branches.
2. **No Call Graph or Cross-File Resolution:** Each file is linted in total isolation. If a function is defined in `R/utils.R` and called in `R/main.R`, `lintr` cannot trace the call or check parameter types.
3. **No Type Inference:** Cannot infer the return type of an expression.
4. **No Semantic Tidyverse Pipeline Tracking:** Linters matching tidyverse pipelines rely on fragile XPath sibling/child patterns rather than tracking transformed column names or dataframe schemas.

---

## 8. Strengths

- **Vast rule library (118 linters):** Represents years of distilled best practices in the R community.
- **Battle-tested production stability:** Integral part of CRAN package development and CI/CD pipelines.
- **Literate programming support:** Smooth extraction of R code from Quarto and RMarkdown.

---

## 9. Limitations & Failure Modes

- **XPath 1.0 Performance & Expressiveness:** Complex syntactic checks require long, convoluted XPath strings that are difficult to debug and slow to execute across millions of nodes.
- **Single-expression vs. Whole-file Tension:** Many linters run per-expression, missing relationships between preceding and succeeding statements.
- **Zero Inter-procedural Reasoning:** Cannot perform escape analysis, purity checking, or inter-procedural taint analysis.

---

## 10. Relevance to R-PARSE

- **Linter Rule Catalog as Requirements:** The 118 linters in `lintr` serve as an explicit capability checklist for static rules that R-PARSE must be able to express cleanly.
- **Replacement Target:** R-PARSE could offer a high-performance Rust/C++ engine capable of executing lintr's rule catalog at 100x speed without R runtime dependencies.
