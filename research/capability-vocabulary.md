# Normalized Capability Vocabulary for R Static & Dynamic Analysis

This document defines the canonical vocabulary used across all R-PARSE comparison matrices, architectural studies, and gap analyses. It resolves taxonomic ambiguities across academic literature, CRAN packages, LSP implementations, and compiler toolchains.

---

## 1. Parsing & Grammar Capabilities

### 1.1 Lexical & Concrete Syntax Recognition (CST)
- **Definition:** The ability to convert a stream of UTF-8 characters into a concrete syntax tree or token stream, preserving all source trivia (whitespace, comments, parentheses, semicolons).
- **Maturity Levels:**
  - *None:* Cannot parse text.
  - *Lossy:* Drops comments, formatting, or exact byte offsets (e.g., base R `parse()` without `keep.source=TRUE`).
  - *Full CST:* Complete lossless concrete tree with exact byte offsets for every token and comment (e.g., `tree-sitter-r`).

### 1.2 Error Recovery & Fault-Tolerant Parsing
- **Definition:** The ability to produce a useful partial syntax tree when source code contains syntax errors (unmatched brackets, dangling operators, incomplete statements).
- **Maturity Levels:**
  - *Fail-Fast:* Throws a fatal syntax error and halts parsing (base GNU R `parse()`, ANTLR default).
  - *Synchronizing Error Recovery:* Discards tokens until a synchronization token (semicolon, newline) is encountered.
  - *Incremental Tree Repair:* Inserts virtual error nodes and preserves valid sibling subtrees without stopping (e.g., Tree-sitter error recovery).

### 1.3 Incremental Reparsing
- **Definition:** The ability to update an existing parse tree in sub-millisecond time based only on a diff edit range (start byte, old end byte, new end byte), without re-scanning the entire document.
- **Maturity Levels:**
  - *None:* Requires full re-parse of the entire file.
  - *Full:* Sub-millisecond incremental splice of modified AST subtrees (Tree-sitter).

---

## 2. Abstract Syntax & Intermediate Representation (AST/IR)

### 2.1 Normalized Abstract Syntax Tree (Normalized AST)
- **Definition:** An abstraction layer above concrete syntax that standardizes equivalent language constructs into uniform node types (e.g., unifying `<-`, `=`, and `->` into a single `Assignment` node; unifying named function definitions and anonymous function assignments).
- **Maturity Levels:**
  - *None / Raw CST:* Retains parser-specific concrete tokens.
  - *XML Wrapper:* Wraps raw parse tables into XML elements without semantic canonicalization (`xmlparsedata`).
  - *Semantic AST:* Strongly-typed AST where expressions are normalized into canonical syntactic nodes with resolved source spans (`flowR` AST model).

### 2.2 Control Flow Graph (CFG)
- **Definition:** Directed graph representation of all paths that might be traversed through a program during execution, partitioned into Basic Blocks with explicit branch conditions and jump edges.
- **Maturity Levels:**
  - *None:* Linear statement lists only.
  - *Intra-procedural CFG:* Constructs basic blocks for `if/else`, `for`, `while`, `repeat`, `break`, `next`, and `return` within a single function (`flowR`).
  - *Inter-procedural CFG:* Links function call sites to target function entry points across procedural boundaries.

### 2.3 Single Static Assignment (SSA) & Def-Use Chains
- **Definition:** Program representation where every variable is assigned exactly once, and every use is explicitly linked to its reaching definition (with phi nodes at merge points).
- **Maturity Levels:**
  - *None:* Free variable string collection.
  - *Block Def-Use:* Identifies inputs/outputs per expression block (`CodeDepends`).
  - *Fine-Grained SSA:* Tracks variable versioning at statement level across branching control flow (`flowR`).

---

## 3. Dataflow & Program Analysis Capabilities

### 3.1 Intra-procedural Dataflow Analysis
- **Definition:** Computes value propagation, reaching definitions, and live variables across statements inside a single function or script scope.
- **Maturity Levels:**
  - *Heuristic / Coarse:* Symbol name frequency or block-level input/output lists (`CodeDepends`, `targets`).
  - *Reaching Definitions Graph:* Exact edges connecting value definitions to value consumption sites (`flowR`).

### 3.2 Inter-procedural Dataflow Analysis
- **Definition:** Propagates dataflow facts across function call boundaries, tracking parameters into function bodies and return values back to call sites.
- **Maturity Levels:**
  - *None:* Functions analyzed as isolated black boxes.
  - *Summary-Based:* Uses predefined function signatures or side-effect summaries (`flowR` built-in sigdb).
  - *Whole-Program Propagation:* Recursively solves dataflow equations across user-defined function boundaries.

### 3.3 Program Slicing
- **Definition:** Extracting the exact subset of program statements that affect (backward slice) or are affected by (forward slice) a specific slicing criterion (variable at a specific program point).
- **Maturity Levels:**
  - *None:* No slicing capability.
  - *Static Backward Slice:* Extracts all statements influencing a variable without executing code (`flowR`).
  - *Executable Slice:* Produces a syntactically valid, runnable R script containing only the necessary statements.

---

## 4. R-Specific Semantic Understanding

### 4.1 Non-Standard Evaluation (NSE) & Data-Masking Comprehension
- **Definition:** The ability to distinguish whether an identifier refers to a variable in the lexical environment or a column/field inside an evaluated data frame (e.g., `dplyr::filter(df, age > 30)`).
- **Maturity Levels:**
  - *Blind:* Treats all identifiers as global variables (causes false dependencies in `targets` / `codetools`).
  - *Syntactic Whitelist / Heuristic:* Flags arguments to known NSE functions as "quoted/data" based on hardcoded argument positions (`CodeDepends` `nseafterfirst`).
  - *Schema-Aware Data-Mask Tracking:* Traces dataframe column names and resolves column symbols against inferred data schemas.

### 4.2 Statistical Formula Semantics
- **Definition:** Interpreting the Wilkinson-Rogers formula grammar (`y ~ x1 + x2 + x1:x2 + (1|group)`) to extract response variables, fixed effects, interaction terms, and random effects.
- **Maturity Levels:**
  - *Syntax only:* Treats `~` as a generic binary/unary operator (`tree-sitter-r`, `lintr`).
  - *Scope Isolation:* Replaces `~` or marks formula variables to avoid polluting lexical dependencies (`CodeDepends`, `targets`).
  - *Statistical Model Graph:* Extracts explicit model specification matrices, predictor hierarchies, and response relationships.

### 4.3 Visualization Pipeline Introspection
- **Definition:** Semantic understanding of graphics construction (base R `plot()`, `ggplot2` grammar of graphics), including aesthetic mappings (`aes(x, y, color)`), geometric layers (`geom_*`), facets, and statistical transformations (`stat_*`).
- **Maturity Levels:**
  - *Device Hooking:* Detects graphics device open/close calls dynamically (`rdtLite`, `CodeDepends` `isPlot`).
  - *Syntactic Tagging:* Flags `aes()` arguments as NSE symbols (`CodeDepends`).
  - *Visualization Semantic Graph:* Statically models layer stacks, aesthetic bindings, coordinate mappings, and backing data sources.

### 4.4 Object-Oriented Dispatch & Type Hierarchy
- **Definition:** Detecting and resolving class definitions, inheritance chains, and method dispatch across R's multiple object systems (S3, S4, RC, R6).
- **Maturity Levels:**
  - *None:* Untyped symbol handling.
  - *Syntactic Class Detection:* Detects `setClass()`, `R6Class()`, and S3 `structure(..., class=...)` calls (`languageserver`).
  - *Method Dispatch Resolution:* Resolves generic function calls (e.g. `predict(model)`) to the concrete method implementation based on static class inference.

---

## 5. Runtime & Ecosystem Integration

### 5.1 Standalone Portability
- **Definition:** Ability to parse and analyze R source code without requiring a local installation of GNU R, Rscript, or R shared libraries.
- **Maturity Levels:**
  - *Runtime-Dependent:* Requires active R process to parse or inspect code (`xmlparsedata`, `lintr`, `languageserver`, `targets`, `CodeDepends`, `rdtLite`).
  - *Standalone:* Compiles and executes anywhere in self-contained native code or WASM (`tree-sitter-r`, `flowR` via tree-sitter backend).

### 5.2 Dynamic Execution Grounding & Provenance
- **Definition:** Correlating static analysis abstractions with real runtime execution traces, variable states, and evaluation hashes.
- **Maturity Levels:**
  - *Static Only:* No execution tracking (`tree-sitter-r`, `lintr`).
  - *Target DAG Execution:* Task-level re-computation checking (`targets`).
  - *Fine-Grained Provenance:* Full PROV-JSON Data Derivation Graphs capturing intermediate values and execution traces (`rdtLite`).
