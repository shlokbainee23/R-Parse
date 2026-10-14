# R-PARSE — Testing Guidelines

## Testing Philosophy
1. **Deterministic & Offline:** Tests must never depend on network access, non-deterministic timers, or external system state.
2. **Panic Invariant:** Malformed or adversarial code snippets must never panic the parser or CST traversals.
3. **Comprehensive Coverage:** Every syntax construct and error recovery branch in Phase 1 must have regression coverage.

## Phase 1 Required Test Suites
All tests reside in `implementation/crates/rparse-core` and `implementation/crates/rparse-parser`:
- **Core Type Tests:** NodeId allocation, equality, hashing, display.
- **Span Tests:** Accurate byte range, line and column computation, multiline spans, UTF-8 character alignment.
- **Diagnostic Tests:** Severity levels, formatted messages, span attachments.
- **Parser Construction:** Tree-sitter R parser instantiation and re-use.
- **CST Traversal:** Node navigation (children, named children, kind checking, parent traversal).
- **Source Location Accuracy:** Verifying that extracted tokens match source slices.
- **Modern R Syntax Conformance:** Tests covering assignments (`<-`, `->`), function declarations, native pipe `|>`, magrittr `%>%`, formulas `~`, lambdas `\(x)`, control structures, complex subsetting (`$`, `[[`, `[`), nested calls, comments, and whitespace.
- **Native Pipe Placeholder `_` Regression:** Validating `x |> f(y = _)` parses without syntax error.
- **Error Recovery:** Tolerant parsing of unclosed delimiters (`foo(`, `if (`, `x <-`).

## Phase 4 Required Test Suites
All tests reside in `implementation/crates/rparse-dataframe/tests`:
- **DataFrame Semantics (`dataframe_semantics.rs`):** Direct column derivation, multi-step propagation, rename + select, multi-column combinations, column independence, left-to-right mutate evaluation, mutation overwrite, rebinding of same symbol, independent output symbols, and transmute semantics.
- **Expression Dependencies (`expression_dependencies.rs`):** Scalar variable + column separation, static `.data` indexing and dollar access, `.env` lexical scoping bypass, binary/unary operator dependency unions, and dataframe extraction.
- **Provenance & Operations (`provenance.rs`):** Filter row-selection preservation, sequential filter condition chaining, base R direct column assignment, base R projection, `transform()`, and pipeline forms.
- **Unknown & Determinism (`unknown.rs`):** Explicit dynamic indexing UNKNOWN_STATIC, unknown function handling, conservative UNKNOWN propagation with known dependencies, dynamic select helpers, branch-aware conservative merging, and analysis run determinism.

## Phase 5 Required Test Suites
- **PDG Construction & Validation (`rparse-pdg/tests/pdg_construction.rs`):** Verifies invariant validation, sequential and monotonic node/edge numbering, and deterministic graph construction across repeated runs.
- **PDG Column Ports & Lineage (`rparse-pdg/tests/pdg_columns.rs`):** Verifies column port granularity (`df@v1.target` vs `df@v1.unused`), direct column derivation edges, grouped provenance inputs, and multi-step column chaining.
- **PDG Control & Row Selection Separation (`rparse-pdg/tests/pdg_control.rs`):** Verifies classical CFG control dependencies on conditional branches and confirms that dataframe `filter()` operations produce `RowSelectionDependency` edges without generating spurious CFG control edges.
- **PDG Uncertainty Integration (`rparse-pdg/tests/pdg_unknown.rs`):** Verifies explicit `UnknownNode` and `UnknownDependency` edge creation for dynamic column access and unmodeled expressions.
- **Column-Aware Slicing Benchmarks (`rparse-slicing/tests/slicing_columns.rs`):** Covers canonical Benchmarks 1 (direct derivation), 2 (multi-step propagation), 3 (rename + select), 4 (multi-input paste expression), 5 (column independence: `a` keeps `x`, drops `y` and `b`), and 7 (frozen canonical slice: retains `raw`, `clean`, `valid`, `price`, `count`, `target`, `res`; excludes `unused`, `age`, and `unused = age + 10`).
- **Object vs Column Slicing Distinction (`rparse-slicing/tests/slicing_object.rs`):** Demonstrates that whole-object slice `df2` retains all schema columns while column-aware slice `df2$target` excludes unrelated sibling columns.
- **Control Dependency Slicing (`rparse-slicing/tests/slicing_control_dep.rs`):** Covers Benchmarks 27 and 28 (condition retention and nested control flow reachability).
- **Unknown Uncertainty Slicing (`rparse-slicing/tests/slicing_unknown_dep.rs`):** Covers Benchmark 6, ensuring conservative unknown retention and `PartialContainsUnknowns` confidence rating without guessing source columns.
- **Mutate Partial Retention (`rparse-slicing/tests/slicing_partial.rs`):** Covers Benchmark 29, verifying that individual mutate branches are sliced independently and statements receive `PartiallyRetained` status.
- **Slicing Determinism & Lineage (`rparse-slicing/tests/slicing_determinism.rs`):** Covers Benchmark 26 (scalar dependency crossing), 30 (sequential filters), 31 (rebinding same symbol), 32 (branch-aware conservative states), and deterministic run equality.

## Phase 6 Required Test Suites
All tests reside in `implementation/crates/rparse-runtime/tests`:
- **Protocol Tests (`protocol_tests.rs`):** Validates JSON-RPC 2.0 serialization/deserialization, Content-Length header framing, line-delimited fallback, malformed header rejection, unexpected EOF recovery, truncated payload detection, and typed schema payload parsing.
- **Bridge Mock Tests (`bridge_mock_tests.rs`):** Verifies runtime invariants without requiring host GNU R:
  - Requirement O: `STATIC` mode strictly forbids GNU R execution and yields `RuntimeEvidence::None`.
  - Requirements P & Q: `ASSISTED` and `EXECUTE` modes record empirical `RuntimeEvidence::Observed`.
  - Requirements R, S, T: Orthogonal separation of `StaticKnowledge` and `RuntimeEvidence`; dynamic/unknown columns remain statically `UnknownStatic` while recorded as `Observed`.
  - Requirement M: Timeout enforcement on delayed responses.
  - Requirement L: Clean error propagation on unexpected child process termination.
  - Requirement J: Typed error when GNU R executable cannot be found.
  - Integration: `AnnotatedDataFrameState` correlation (`Confirmed`, `EmpiricalOnly`, `StaticOnly`, `Unobserved`).
- **Real GNU R Integration Tests (`real_r_tests.rs`):** Executed when GNU R is discovered on host:
  - Process startup and ping handshake.
  - Successful expression evaluation and visible value output capture.
  - Object metadata inspection (`class`, `typeof`, `length`, `is_dataframe`).
  - Dataframe schema discovery with typed columns, classes, and row counts.
  - Multi-column dataframes and empty dataframes (`nrow = 0`).
  - Handling of non-existent symbols and dataframes.
  - Graceful capture of R syntax errors and runtime exceptions.
  - Hard timeout enforcement terminating runaway child processes.

## Phase 7 Required Test Suites
All tests reside in `implementation/crates/rparse-cli/src/tests.rs`:
- **CLI Invariant & Argument Tests:** Validates `--help`, `--version`, missing subcommand, unrecognized command, missing `--column`, missing `--criterion`, and unknown options with exact exit codes.
- **Input & File Handling:** Verifies missing file rejection (Exit Code 3), unreadable file handling, empty file handling, and syntax error resilience.
- **Analysis Command Tests (`analyze`):** Validates text and JSON output for standard programs, statement and diagnostic counting, schema reporting, and PDG graph statistics.
- **Column Provenance Tests (`provenance`):** Validates fine-grained derivation chains for direct assignments (`df$total <- df$price * df$quantity`), multi-step dplyr pipelines (`df2 <- mutate(...)`), and typed error on nonexistent columns (Exit Code 1).
- **Program Slicing Tests (`slice`):** Validates backward static slices for individual column criteria (`df$total`), verifies retention of necessary assignments and exclusion of unrelated sibling columns, and validates text and JSON schemas.
- **Runtime Mode Invariants:** Enforces that `STATIC` mode strictly executes offline with zero R subprocess invocations, and verifies `ASSISTED` mode flag handling.
- **Determinism Tests:** Verifies that repeated CLI invocations on identical input produce identical text, JSON, and graph metric outputs.
- **Benchmark Suite (8 Dimensions):** Built-in micro-benchmarks executing and verifying:
  1. Parser performance
  2. IR construction
  3. Scope analysis
  4. CFG & dataflow analysis
  5. DataFrame semantics & column provenance
  6. PDG construction
  7. Backward static slicing
  8. End-to-end static pipeline

## Running Test Suites
```powershell
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets --all-features
```
