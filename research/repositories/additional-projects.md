# xmlparsedata — Repository Analysis

## 1. Project Identity
- **Name:** xmlparsedata
- **Repository:** https://github.com/r-lib/xmlparsedata
- **Organization:** r-lib
- **Language:** R
- **License:** MIT
- **Status:** Stable, maintained
- **Size:** Very small — 4 R source files, ~231 lines of core code

## 2. Purpose
Converts R's native `utils::getParseData()` output into an XML tree. This XML tree can then be queried using XPath expressions. Used extensively by `lintr` and `languageserver` for static analysis.

## 3. Architecture
```text
R Source Code → parse(keep.source=TRUE) → getParseData() → xml_parse_data() → XML string → xml2::read_xml() → XPath queries
```

## 4. Parsing
**Does not parse R code itself.** Delegates entirely to R's native `parse()` function. Its sole job is reformatting the parse data into XML.

**Evidence:** `package.R` line 71: `pd <- getParseData(x, includeText = includeText)`

## 5. AST / IR
The XML output preserves all of R's parse data tokens. The top-level tag is `<exprlist>`. Each expression is an `<expr>` tag. Terminal tokens become self-closing tags with their token type as the tag name.

**Attributes preserved:** `line1`, `col1`, `line2`, `col2`, `start`, `end`

**Token name mapping:** Some R token names are invalid XML tag names and are renamed:
- `'?'` → `OP-QUESTION`
- `'~'` → `OP-TILDE`  
- `'+'` → `OP-PLUS` (etc.)
- `'\\'` → `OP-LAMBDA`

**Evidence:** `package.R` lines 200-221 (`xml_parse_token_map`)

## 6. Static Analysis
**None.** xmlparsedata is a data format converter, not an analyzer.

## 7-9. Statistical/Visualization/Runtime
**None.**

## 10. Testing
- `tests/` directory exists
- Small test suite appropriate for the package's limited scope

## 11-13. Extensibility / Strengths / Limitations
**Strengths:** Simple, reliable, enables XPath-based code analysis. Foundation of lintr and languageserver analysis.
**Limitations:** Requires R runtime for parsing. XML format is verbose. No incremental parsing.
**Verified limitation:** Comments require fixup (`fix_comments` function)

## 14. Reusable Ideas for R-PARSE
- Token name mapping table for XML compatibility
- XPath as a query language for AST traversal (proven pattern by lintr/languageserver)

## 15. Overlap with R-PARSE
Low. xmlparsedata is a thin utility layer. R-PARSE would either use it as a data source or replace it entirely.

---

# lintr — Repository Analysis

## 1. Project Identity
- **Name:** lintr
- **Repository:** https://github.com/r-lib/lintr
- **Organization:** r-lib
- **Language:** R
- **License:** MIT
- **Status:** Active, well-maintained
- **Size:** 152 R source files, **118 linter files**

## 2. Purpose
Static code analysis (linting) for R. Checks style, potential bugs, and best practices.

## 3. Architecture
```text
R Source Code → parse(keep.source=TRUE) → getParseData() → xmlparsedata::xml_parse_data() → XML → XPath queries per linter rule → Lint objects
```

**Critical insight:** lintr's analysis is entirely based on **XPath queries** against the XML parse tree produced by xmlparsedata. Each linter is essentially one or more XPath expressions.

## 4. Parsing
Delegates to R's native parser via `xmlparsedata`. No custom parser.

## 5. AST / IR
Uses xmlparsedata's XML representation. Each linter navigates this XML tree using XPath.

## 6. Static Analysis — 118 Linters
Verified from source: 118 `*_linter.R` files covering:

**Style linters:** assignment_linter, brace_linter, commas_linter, commented_code_linter, line_length_linter, spaces, indentation, etc.

**Best practice linters:** absolute_path_linter, backport_linter, boolean_arithmetic_linter, class_equals_linter, comparison_negation_linter, duplicate_argument_linter, equals_na_linter, etc.

**Testing linters:** expect_comparison_linter, expect_identical_linter, expect_length_linter, expect_named_linter, expect_not_linter, expect_null_linter, expect_s3_class_linter, expect_s4_class_linter, etc.

**Complexity linters:** cyclocomp_linter

**What lintr does NOT do:**
- No dataflow analysis
- No program slicing
- No dependency tracking
- No call graph
- No type inference
- No cross-file analysis
- Pattern matching only (XPath on parse tree)

## 7. Statistical Understanding
**None.** Linters are syntactic pattern matchers. No understanding of statistical functions, formulas, or models.

## 8. Visualization Understanding
**None.**

## 9. Runtime Integration
Requires R runtime for parsing. Linting is purely static.

## 12. Strengths
1. **Comprehensive linter collection** (118 rules)
2. **Well-tested, production-quality**
3. **GitHub Actions integration**
4. **Extensible** — custom linters can be written as XPath queries
5. **tidyverse ecosystem standard**

## 13. Limitations
- **Verified:** XPath-only analysis — no semantic understanding
- **Verified:** No cross-file analysis
- **Verified:** No dataflow or control flow analysis
- **Verified:** Requires R runtime for parsing

## 14. Reusable Ideas
- XPath query patterns for common R code patterns
- Linter rule catalog as a checklist for R code quality
- CI/CD integration patterns

## 15. Overlap with R-PARSE
Moderate. R-PARSE could subsume lintr's functionality by providing deeper analysis. But lintr is already mature and well-adopted.

---

# languageserver — Repository Analysis

## 1. Project Identity
- **Name:** languageserver
- **Repository:** https://github.com/REditorSupport/languageserver
- **Language:** R
- **License:** MIT
- **Status:** Active, maintained
- **Size:** 47 R source files, substantial codebase

## 2. Purpose
Implements the Language Server Protocol (LSP) for R, providing IDE features: completion, diagnostics, hover, definition, references, formatting, code actions, etc.

## 3. Architecture
```text
R Source Code → R parser (parse/getParseData) → xmlparsedata → XPath analysis → LSP responses
                                                    ↕
                                             R runtime (for completion, evaluation)
```

## 6. Static Analysis
Uses xmlparsedata + XPath for:
- **Definition lookup** (`definition.R`) — finds variable/function definitions using XPath on parse data
- **References** (`references.R`) — finds all references to a symbol
- **Diagnostics** (`diagnostics.R`) — delegates to lintr
- **Hover** (`hover.R`) — provides documentation on hover
- **Completion** (`completion.R`, 39KB) — comprehensive code completion
- **Rename** (`rename.R`) — symbol renaming
- **Call hierarchy** (`call_hierarchy.R`) — call hierarchy navigation
- **Type hierarchy** (`type_hierarchy.R`, 44KB!) — S3/S4/R5/R6 type hierarchy
- **Semantic tokens** (`semantic.R`) — semantic highlighting
- **Code actions** (`code_action.R`) — automated fixes
- **Refactoring** (`refactor.R`, 32KB) — code refactoring

**Evidence:** `definition.R` lines 1-7 show complex XPath queries against xmlparsedata output for finding definitions: LEFT_ASSIGN, RIGHT_ASSIGN, EQ_ASSIGN, SYMBOL_FORMALS, forcond.

**Critical insight:** The languageserver has significant analysis capability, but it's all based on XPath pattern matching against the parse tree — not true dataflow analysis.

## 7. Statistical Understanding
**None.**

## 8. Visualization Understanding  
**None.**

## 12. Strengths
1. **Production-quality LSP implementation** for R
2. **Comprehensive IDE features** (47 source files)
3. **S3/S4/R5/R6 type hierarchy support** (44KB type_hierarchy.R)
4. **Active community** (REditorSupport)

## 13. Limitations
- **Verified:** Pattern-matching analysis (XPath), not semantic
- **Verified:** No dataflow analysis
- **Verified:** Requires R runtime
- **Verified:** Single-file analysis primarily

## 15. Overlap with R-PARSE
Moderate. R-PARSE could provide a superior analysis backend for language server features.

---

# CodeDepends — Repository Analysis

## 1. Project Identity
- **Name:** CodeDepends
- **Repository:** https://github.com/duncantl/CodeDepends
- **Language:** R
- **License:** GPL
- **Status:** Legacy, intermittent updates
- **Author:** Duncan Temple Lang (academic)

## 2. Purpose
Analyzes R scripts to determine variable dependencies, function inputs/outputs, and code structure. Builds dependency graphs between code blocks.

## 3. Architecture
```text
R Source Code → parse() → Expression-level analysis → Input/Output tracking → Dependency graph
```

## 6. Static Analysis
**Evidence from `R/codeDepends.R` and `R/functionHandlers.R` (34KB):**
- **Variable tracking:** Identifies inputs (variables read) and outputs (variables written) per expression
- **Function handlers:** Extensive custom handlers for specific R functions (34KB of handlers!)
- **Library tracking:** Identifies library() and require() calls
- **File tracking:** Identifies file reads/writes
- **Call graph:** `R/callGraph.R` — function call graph construction
- **Code types:** `R/codeTypes.R` — categorizes code expressions
- **Side effects:** `R/sideEffects.R`
- **NSE variables:** Tracks non-standard evaluation variables
- **Section dependencies:** `R/sectionDepends.R` (11KB) — dependency analysis within document sections

**Evidence:** `codeDepends.R` line 31 — `inputCollector` function tracks: libraries, files, strings, vars (inputs), set (outputs), functions, removes, updates, sideEffects, nsevalVars

**Critical detail:** `functionHandlers.R` (34KB) contains **custom handlers for specific R functions**. This is the closest thing to "statistical understanding" in any project — it knows how specific functions use their arguments.

## 7. Statistical Understanding
**Partial — through function handlers.** CodeDepends has custom handlers for specific functions that understand their argument semantics (e.g., which arguments are data inputs, which are outputs). However, this is not statistical domain knowledge per se — it's argument-level input/output tracking.

## 12. Strengths
1. **Pioneer in R code dependency analysis**
2. **Function-specific handlers** (34KB of domain knowledge)
3. **Plot detection:** `R/isPlot.R` — can detect plotting calls
4. **Output detection:** `R/isOutput.R` — can detect output operations

## 13. Limitations
- **Verified:** Legacy codebase, limited modern R support
- **Verified:** No tree-sitter or modern parser integration
- **Likely:** Limited handling of modern tidyverse patterns
- **Verified:** No dataflow graph (tracks inputs/outputs per expression block, not full dataflow)

## 15. Overlap with R-PARSE
Moderate. CodeDepends' function handler pattern is a reusable idea. flowR supersedes most of CodeDepends' capabilities.

---

# rpy2 — Repository Analysis

## 1. Project Identity
- **Name:** rpy2
- **Repository:** https://github.com/rpy2/rpy2
- **Language:** Python, C
- **License:** GPL-2.0
- **Status:** Active, mature

## 2. Purpose
Python interface to R. Embeds the R runtime within Python, allowing Python code to call R functions, pass data, and retrieve results.

## 3. Architecture
```text
Python Code → rpy2.robjects (high-level) → rpy2.rinterface (C extension) → Embedded R Runtime → Results
```

**Two layers:**
- `rpy2-rinterface`: Low-level C extensions wrapping R's C API
- `rpy2-robjects`: High-level Pythonic interface

## 6. Static Analysis
**None.** rpy2 is a runtime bridge, not an analyzer.

## 9. Runtime Integration
**This is rpy2's sole purpose.** It embeds the full R runtime within Python.
- Embedded R (not subprocess)
- Full R object access from Python
- DataFrame interoperability
- Package management from Python

## 12. Strengths
1. **Mature Python↔R bridge**
2. **Embedded R** (more efficient than subprocess)
3. **Rich object mapping** (R vectors ↔ Python/numpy arrays)

## 13. Limitations
- **Verified:** No parsing or analysis capability
- **Verified:** No AST access
- **Verified:** Complex installation (requires R + C compilation)
- **Verified:** GPL license

## 15. Overlap with R-PARSE
Low for analysis. **Useful for runtime correlation** — R-PARSE could use rpy2 (or subprocess) to execute R code and compare runtime results with static analysis predictions.

---

# targets — Repository Analysis

## 1. Project Identity
- **Name:** targets
- **Repository:** https://github.com/ropensci/targets
- **Language:** R
- **License:** MIT
- **Status:** Active, gold standard for R pipelines

## 2. Purpose
Pipeline management for reproducible R workflows. Uses static code analysis to build dependency DAGs and skip up-to-date targets.

## 3. Architecture
```text
_targets.R (pipeline definition) → Static code analysis → Dependency DAG → Execution plan → Parallel execution → Cached results
```

## 6. Static Analysis
targets uses static code analysis to:
- Parse R function bodies
- Extract variable/function dependencies
- Build directed acyclic graphs (DAGs)
- Determine which targets need re-execution

**Important:** targets' analysis is limited to its pipeline DSL. It doesn't analyze arbitrary R code.

## 12. Strengths
1. **Production-quality pipeline management**
2. **Static dependency detection for pipelines**
3. **Reproducibility guarantees**

## 15. Overlap with R-PARSE
Low. targets solves a different problem (pipeline management). However, its static dependency detection is relevant context for R-PARSE's workflow analysis goals.
