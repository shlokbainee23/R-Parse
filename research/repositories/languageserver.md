# languageserver — Repository Analysis

## 1. Project Identity

- **Name:** languageserver
- **Repository:** https://github.com/REditorSupport/languageserver
- **Organization:** REditorSupport (community standard for VS Code, Sublime Text, Vim/Neovim, Emacs)
- **Language:** R (100%)
- **License:** MIT
- **Status:** Active, widely used language server for R
- **Size:** 47 R source files, ~1.2 MB codebase
- **Dependencies:** `xmlparsedata`, `lintr`, `xml2`, `collections`, `callr`, `fs`, `jsonlite`

---

## 2. Purpose & Problem Solved

Implements Microsoft's Language Server Protocol (LSP) for R. Powers editor intelligence including:
- Auto-completion (functions, packages, arguments, data frame columns).
- Go to Definition / Find References.
- Hover tooltips and documentation rendering.
- Document and Workspace Symbol search.
- Document formatting (via `styler`).
- Inline diagnostics (via `lintr`).
- Type hierarchy exploration (S3, S4, Reference Classes, R6).
- Code actions and AST-based refactoring (extract variable/function).

---

## 3. Architecture & Execution Flow

```text
Editor (VS Code, Neovim, etc.)
      │ (JSON-RPC over stdin/stdout)
      ▼
Server Loop [R/languageserver.R]
      │
      ├─► Document Store [R/document.R, R/workspace.R]
      │     └─► On textDocument/didChange:
      │           parse(text, keep.source=TRUE)
      │           └─► xmlparsedata::xml_parse_data() -> Cached XML DOM
      │
      ├─► Static Analysis Handlers (XPath on XML DOM)
      │     ├─► Definition [R/definition.R]
      │     ├─► References [R/references.R]
      │     ├─► Type Hierarchy [R/type_hierarchy.R] (44 KB)
      │     ├─► Refactoring [R/refactor.R] (32 KB)
      │     └─► Diagnostics [R/diagnostics.R] (delegates to lintr)
      │
      └─► Dynamic Runtime Execution (for Completion & Package Docs)
            └─► Evaluates in running R session or background worker via `callr`
                  to inspect loaded environments, namespaces, and datasets.
```

---

## 4. Parsing Mechanism

- Depends on base R's native `parse(text, keep.source=TRUE)`.
- Re-parses full document on text document synchronization events.
- Parse trees are converted to XML DOMs via `xmlparsedata` and cached per-document in `workspace.R`.

---

## 5. AST & Semantic Capabilities

`languageserver` provides the most sophisticated static analysis in the native R ecosystem:

1. **Definition Resolution (`R/definition.R`):**
   - Matches identifier tokens at cursor to assignment expressions:
     - `LEFT_ASSIGN` (`<-`), `RIGHT_ASSIGN` (`->`), `EQ_ASSIGN` (`=`)
     - Function arguments in formals (`SYMBOL_FORMALS`)
     - Loop iteration variables in `for (var in seq)` (`forcond`)
   - Resolves package-qualified symbols (`pkg::func` via namespace queries).
2. **OOP & Type Hierarchy (`R/type_hierarchy.R`, 1281 lines):**
   - **R6 Classes:** Detects `R6Class("ClassName", inherit = SuperClass, ...)` calls.
   - **S4 Classes:** Detects `setClass("ClassName", contains = c("Super1", ...))` calls.
   - **RefClasses:** Detects `setRefClass("ClassName", contains = ...)` calls.
   - **S3 Classes:** Identifies constructor patterns, `structure(..., class = "...")`, and explicit `class(x) <- "..."` assignments.
   - Computes bidirectional supertype and subtype graphs across workspace files.
3. **Refactoring Engine (`R/refactor.R`, 897 lines):**
   - Extracts expressions to variables.
   - Extracts blocks of statements to functions, determining required parameter inputs by analyzing free symbols in the selected range.
4. **Completion Engine (`R/completion.R`, 1081 lines):**
   - Combines static token matching with runtime inspection of loaded packages, namespace exports, function argument lists, and dataframe columns (`df$...`).

---

## 6. What languageserver Does NOT Do (Verified Gaps)

1. **No True SSA or Dataflow Graph:** Definition lookup is lexical/scope-based rather than reaching-definition analysis; it cannot distinguish multiple sequential mutations to the same variable across complex control flow.
2. **No Abstract Interpretation or Program Slicing:** Cannot compute backward or forward slices.
3. **High Latency from XML Overhead:** Re-serializing AST to XML and running XPath queries on every keystroke causes noticeable latency in large files (> 2,000 LOC).
4. **Fragile NSE and Pipeline Comprehension:** Column completions rely on simple runtime evaluation or literal regexes; cannot statically trace dataframe column transformations through multiple piped dplyr verbs (`select`, `mutate`, `filter`).

---

## 7. Strengths

- **Gold-standard IDE protocol compliance:** Covers almost the entire LSP specification.
- **Rich OOP Class Detection:** Best-in-class support for R's multiple object systems (S3, S4, R5, R6).
- **Graceful degradation:** Combines static AST queries with background runtime execution when static data is ambiguous.

---

## 8. Limitations & Failure Modes

- **Single-threaded bottlenecks:** Running LSP server in pure R can freeze editor responses during long-running evaluations.
- **Memory consumption:** Storing full XML parse trees and document lines for multi-file packages consumes significant memory.
- **No syntax fault-tolerance:** A single unclosed paren invalidates the entire parse tree until repaired, temporarily breaking all IDE features.

---

## 9. Relevance to R-PARSE

- **Feature Benchmark:** R-PARSE's LSP or IDE backend should aim to match `languageserver`'s semantic breadth (definition, hover, type hierarchy, completion) while eliminating its reliance on the R runtime, XML string conversions, and syntax-break fragility.
