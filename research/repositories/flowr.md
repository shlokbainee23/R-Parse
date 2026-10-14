# flowR — Repository Analysis

## 1. Project Identity

- **Name:** flowR
- **Repository:** https://github.com/flowr-analysis/flowr
- **Organization:** flowr-analysis (academic project, University of Ulm)
- **Language:** TypeScript
- **License:** GPLv3 (with LGPL and other components)
- **Status:** Active, under heavy development
- **Last meaningful activity:** Ongoing (2024-2025+)
- **Stars/forks:** ~200+
- **Version:** v2.15.9+ (rapid release cycle)
- **Related repos:** vscode-flowr, rstudio-addin-flowr, flowr-r-adapter

## 2. Purpose

A sophisticated static dataflow analyzer for R. Provides program slicing, dataflow analysis, dependency analysis, call graph generation, linting, and a query API. The most advanced R static analysis tool currently available.

## 3. Architecture

```text
R Source Code
     ↓
Parser (R-shell via utils::getParseData OR tree-sitter-r)
     ↓
Raw AST (R parse data or tree-sitter CST)
     ↓
Normalized AST (flowR's own RNode types)
     ↓
Dataflow Graph Construction
     ↓
Program Analysis (slicing, queries, linting)
     ↓
Output (REPL, VS Code extension, API)
```

**Critical finding:** flowR supports TWO parser backends:
1. **R-shell:** Uses the real R parser via `utils::getParseData()` — requires R installation
2. **Tree-sitter:** Uses `web-tree-sitter` with tree-sitter-r grammar — no R required

**Evidence:** `src/r-bridge/parser.ts` lines 1-63 define parser interfaces. `KnownParser = RShell | RShellExecutor | TreeSitterExecutor`. The parser name can be `'r-shell'` or `'tree-sitter'`.

## 4. Parsing

### Parser Architecture
- **Not its own parser** — delegates to R-shell or tree-sitter
- `src/r-bridge/shell.ts` (20KB) manages the R subprocess
- `src/r-bridge/lang-4.x/tree-sitter/` contains tree-sitter integration
- Parsed output is normalized into flowR's internal AST

### Normalization Layer
- `src/r-bridge/lang-4.x/ast/` contains the normalization pipeline
- `parser/` — parsing adapters for both backends
- `model/` — normalized AST node definitions
- `processing/` — AST processing utilities (decoration, visiting, node IDs)

### Grammar Version
- References "R grammar v14" for tree-sitter engine

## 5. AST / IR

### Normalized AST (RNode types)
flowR defines its own normalized AST with 20+ node types. **Evidence:** `src/r-bridge/lang-4.x/ast/model/type.ts`:

**RType enum:**
- `RAccess` — array/list access
- `RArgument` — function call argument
- `RBinaryOp` — binary operator
- `RExpressionList` — list of expressions
- `RForLoop` — for loop
- `RFunctionCall` — function call
- `RFunctionDefinition` — function definition
- `RIfThenElse` — if/else
- `RParameter` — function parameter
- `RPipe` — pipe operator
- `RRepeatLoop` — repeat loop
- `RUnaryOp` — unary operator
- `RWhileLoop` — while loop
- `RBreak` — break statement
- `RComment` — comment
- `RLineDirective` — line directive
- `RLogical` — TRUE/FALSE
- `RNext` — next statement
- `RNumber` — numeric literal
- `RString` — string literal
- `RSymbol` — identifier/symbol
- `RProject` — project container
- `RDelimiter` — delimiter (punctuation)

### Metadata (Source interface)
**Evidence:** `model.ts` lines 44-67:
- `fullRange: SourceRange` — complete source range
- `fullLexeme: string` — complete source text
- `adToks: OtherInfoNode[]` — additional tokens (inline comments!)
- `file: string` — file location

### RAstNodeBase
**Evidence:** `model.ts` lines 74-80:
- `type: RType` — node type
- `lexeme: string` — original string from R
- `info: Info & Source` — combined metadata

### Key Features
- **Source positions:** ✓ via SourceRange
- **Comments:** ✓ preserved as RComment nodes AND inline via adToks
- **Parent relationships:** ✓ via ParentInformation decoration
- **Node IDs:** ✓ unique NodeId system for cross-referencing
- **AST ID Map:** ✓ AstIdMap for O(1) node lookup

## 6. Static Analysis

**This is flowR's core strength.** Analysis capabilities verified from source:

### Dataflow Analysis
- `src/dataflow/` — comprehensive dataflow graph construction
- `src/dataflow/graph/` — graph data structure
- `src/dataflow/environments/` — environment/scope tracking
- `src/dataflow/eval/` — abstract evaluation
- `src/dataflow/fn/` — function-specific handling
- Tracks: definitions, uses, reads, writes, side effects, control flow

**Evidence:** `src/dataflow/info.ts` — defines `ControlDependency` with id, when (boolean branch), byIteration (loop), file assumptions

### Control Flow Analysis
- `src/control-flow/` — control flow graph construction

### Program Slicing
- `src/slicing/` — backward program slicer
- `src/slicing/criterion/` — slicing criterion specification
- `src/slicing/static/` — static slicing implementation

### Query System (31 query types!)
**Evidence:** `src/queries/catalog/` contains 31 query directories:
1. `absint-query` — abstract interpretation
2. `call-context-query` — call context analysis
3. `call-graph-query` — call graph generation
4. `cluster-query` — code clustering
5. `config-query` — configuration queries
6. `control-flow-query` — control flow queries
7. `dataflow-lens-query` — dataflow lens
8. `dataflow-query` — dataflow queries
9. `dependencies-query` — dependency analysis (libraries, files, scripts)
10. `dice-query` — dynamic slicing
11. `does-call-query` — function call detection
12. `files-query` — file queries
13. `function-info-query` — function information
14. `guess-dep-versions-query` — dependency version guessing
15. `happens-before-query` — ordering analysis
16. `id-map-query` — AST ID map
17. `input-sources-query` — input source identification
18. `inspect-exceptions-query` — exception analysis
19. `inspect-fn-props-query` — function property inspection
20. `inspect-higher-order-query` — higher-order function detection
21. `inspect-recursion-query` — recursion detection
22. `linter-query` — code linting
23. `location-map-query` — location mapping
24. `normalized-ast-query` — AST queries
25. `origin-query` — origin tracking
26. `project-query` — project-level queries
27. `provenance-query` — provenance tracking
28. `resolve-value-query` — value resolution
29. `search-query` — code search
30. `signature-query` — function signatures
31. `static-slice-query` — static program slicing

### Linting
**Evidence:** README lines 31-104 show 18 linter rules:
- deprecated-functions
- file-path-validity
- seeded-randomness
- absolute-file-paths
- unused-definitions
- network-functions
- dataframe-access-validation
- dead-code
- useless-loop
- problematic-inputs
- stop-call
- roxygen-arguments
- no-leaked-credentials
- undefined-symbol
- unused-import
- unclosed-connection
- unescaped-arguments
- namespace-access

### Reconstruction
- `src/reconstruct/` — code reconstruction from sliced AST

## 7. Statistical Understanding

**Partial — function-name level only.** flowR can identify function calls (e.g., `lm()`, `aov()`), but does NOT semantically understand:
- What statistical method a function implements
- Formula interpretation (y ~ x treated as binary operator)
- Model diagnostics
- Statistical assumptions
- Experimental design
- Statistical workflow sequences

**Evidence:** The query catalog has no statistical-specific queries. Function calls are tracked generically through the dataflow graph. The `call-context-query` can identify calls to specific functions but doesn't interpret their statistical semantics.

**Verified limitation:** No statistical domain knowledge.

## 8. Visualization Understanding

**None verified.** flowR does not have ggplot2-specific analysis, aesthetic mapping extraction, or visualization intent recognition.

**Evidence:** No visualization-related queries in the catalog. ggplot2 calls are treated as ordinary function calls.

**Verified limitation:** No visualization domain knowledge.

## 9. Runtime Integration

**Yes — via R subprocess.** flowR uses R's real parser:
- `src/r-bridge/shell.ts` — manages R subprocess
- `src/r-bridge/shell-executor.ts` — executes R commands
- `RShellInformation.sendCommandWithOutput()` — sends commands to R

However, this is primarily for **parsing**, not for correlating runtime results with static analysis.

**Evidence:** `parser.ts` line 40: `sendCommandWithOutput(command: string)` — for sending R commands.

## 10. Testing

- **Framework:** Vitest (`vitest.config.js`)
- **Test directory:** `test/` (large, comprehensive)
- **Coverage:** codecov integration with 80%+ coverage
- **Types:** Parser tests, normalization tests, dataflow tests, slicing tests, query tests
- **CI:** GitHub Actions QA pipeline

## 11. Extensibility

- **Adding analyses:** New queries can be added to `src/queries/catalog/` following the query pattern
- **Adding linters:** New linter rules can be added to the linter-query
- **Adding parsers:** Parser interface is modular (already supports R-shell and tree-sitter)
- **API:** REPL interface, VS Code extension API, R adapter
- **Well-structured:** TypeScript with clear module boundaries

## 12. Strengths (Evidence-Based)

1. **Most comprehensive R static analyzer** — 31 query types, dataflow graph, program slicing
2. **Dual parser support** — R-shell and tree-sitter backends
3. **Rich normalized AST** — preserves comments, source ranges, file info
4. **Program slicing** — backward slicer with criterion specification
5. **Dependency analysis** — libraries, files, scripts, version guessing
6. **Active development** — rapid release cycle, academic backing
7. **Production-ready integrations** — VS Code, Positron, RStudio, Docker
8. **Provenance tracking** — origin and provenance queries

## 13. Limitations (Evidence-Based)

- **Verified:** No statistical domain knowledge (formulas, models, diagnostics)
- **Verified:** No visualization understanding (ggplot2, plotly)
- **Verified:** No visualization intent extraction
- **Verified:** GPLv3 license (restrictive for some use cases)
- **Verified:** TypeScript implementation (may limit embedding in non-JS systems)
- **Likely:** Performance may be an issue for very large codebases (TypeScript + R subprocess)
- **Likely:** Abstract interpretation is partial (R's dynamic nature limits precision)

## 14. Reusable Ideas for R-PARSE

1. **Normalized AST design** — flowR's RType/RNode hierarchy is well-designed
2. **Dual parser backend pattern** — supporting multiple parsers
3. **Query catalog architecture** — modular, extensible query system
4. **Dataflow graph construction** — sophisticated environment and scope tracking
5. **ControlDependency model** — elegant representation of control flow dependencies
6. **Source metadata preservation** — fullRange, fullLexeme, adToks pattern
7. **Program slicing algorithm** — backward slicing with criterion specification

## 15. Potential Overlap with R-PARSE

**CRITICAL: flowR is the most directly overlapping project.**

flowR already provides:
- ✓ R parsing (via two backends)
- ✓ Normalized AST
- ✓ Dataflow analysis
- ✓ Control flow analysis
- ✓ Program slicing
- ✓ Dependency analysis
- ✓ Linting
- ✓ Call graph generation
- ✓ Provenance tracking

R-PARSE must differentiate by providing what flowR does NOT:
- Statistical domain understanding
- Visualization intent extraction
- Statistical workflow reconstruction
- Interactive program-understanding graphs
- Runtime correlation (static ↔ runtime comparison)
- Broad, domain-aware program comprehension

**Honest assessment:** If R-PARSE cannot provide substantial capabilities beyond flowR, there is no justification for building it. flowR already solves the "static analysis for R" problem to a significant degree.
