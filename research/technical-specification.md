# R-PARSE — MASTER TECHNICAL SPECIFICATION

> **DOCUMENT STATUS:** APPROVED TECHNICAL SPECIFICATION (PRECISION PATCHED)  
> **DATE:** October 2026  
> **PROJECT ARCHITECTURE:** Option D — Hybrid Rust Core + GNU R Runtime Bridge  
> **AUTHORITATIVE SCOPE REFERENCE:** [`research/scope-freeze.md`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/scope-freeze.md)  
> **ARCHITECTURE DECISION REFERENCE:** [`research/ADR-001-architecture.md`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/ADR-001-architecture.md)

---

## Executive Summary & Authoritative Definition

**R-PARSE is a broad-compatibility R program analysis system that extends semantic program representation with fine-grained column provenance and uses that information to enable column-aware static program slicing.**

### Primary Technical Contribution
> **Fine-grained column provenance integrated with program-level dependency analysis and column-aware static program slicing for R.**

### System Architecture Topology
```text
                     R-PARSE
                        │
            ┌───────────┴───────────┐
            │                       │
            ▼                       ▼
       RUST CORE               GNU R RUNTIME
       (Tier 1)                  (Tier 2)
            │                       │
      Parsing / CST             Execution
      Semantic IR               Inspection
      Scope Analysis            Runtime Evidence
      CFG / Dataflow
      Provenance Engine
      Slicing Engine
            │                       │
            └────── Runtime Bridge ─┘
```

The Rust Core is the **authoritative semantic and analysis engine**. The GNU R Runtime is the **official execution and observation environment**. R-PARSE **does not implement an alternative R interpreter or compiler**.

---

## SECTION 1 — SYSTEM BOUNDARY

```text
┌────────────────────────────────────────────────────────────────────────┐
│                        R-PARSE SYSTEM BOUNDARY                         │
├───────────────────────────────────┬────────────────────────────────────┤
│ IN-BOUNDS (Tier 1 Rust Core)      │ IN-BOUNDS (Tier 2 GNU R Runtime)   │
│ • R Source Code Ingestion         │ • Real GNU R Process Execution     │
│ • Lossless CST Construction       │ • Controlled Expression Evaluation │
│ • Normalized Semantic IR          │ • Dynamic Object / Class Inspect   │
│ • Lexical Scoping & Def-Use       │ • Live Dataframe Colnames/Dim      │
│ • CFG & Control Dependency Graph  │ • Runtime Error & Warning Capture  │
│ • Dataframe Versioned State Model │ • Empirical Observation Evidence   │
│ • Column Provenance Graph         │                                    │
│ • Column-Aware Backward Slicing   │                                    │
│ • Explicit UNKNOWN Propagation    │                                    │
│ • JSON & Text Serialization       │                                    │
├───────────────────────────────────┴────────────────────────────────────┤
│ RUNTIME BRIDGE BOUNDARY (Decoupled IPC / Process Interface)            │
│ • Rust -> R: Action, Source Expression, Target Symbol, Timeout MS      │
│ • R -> Rust: Status, Observed Schema, Value Summary, Error Diagnostics │
├────────────────────────────────────────────────────────────────────────┤
│ OUT-OF-BOUNDS (Explicit Non-Goals & System Exclusions)                 │
│ • Alternative R Interpreter or Native Code Compiler                    │
│ • Universal Whole-Program Soundness Across Arbitrary Metaprogramming   │
│ • General-purpose Linter Rule Replacement (e.g., 118 lintr rules)      │
│ • Workflow DAG Scheduler (e.g., targets replacement)                   │
│ • Guaranteed Executable Reprex Generation                              │
│ • LSP Server & Editor Extensions (Deferred Beyond MVP)                 │
└────────────────────────────────────────────────────────────────────────┘
```

### Separation of Concerns
1. **STATIC ANALYSIS:** Owned 100% by the Tier 1 Rust Core. Computes semantic models, provenance graphs, and program slices without executing arbitrary code. Operates deterministically in offline environments.
2. **RUNTIME EXECUTION:** Owned 100% by the Tier 2 GNU R Process. Executes user-provided or extracted expressions when runtime assistance is explicitly requested by the user.
3. **RUNTIME OBSERVATION:** The act of inspecting the live state of an R session resulting from execution. Yields observational evidence that annotates static states with empirical facts (`RuntimeEvidence::Observed`) without claiming universal proof for unobserved execution paths or alternate inputs.

---

## SECTION 2 — COMPONENT ARCHITECTURE

R-PARSE is decomposed into 14 distinct functional components:

```text
               ┌──────────────────────────────────────────────┐
               │              1. Parser (Tree-sitter)         │
               └──────────────────────┬───────────────────────┘
                                      │ Emits
                                      ▼
               ┌──────────────────────────────────────────────┐
               │              2. CST Representation           │
               └──────────────────────┬───────────────────────┘
                                      │ Normalizes
                                      ▼
               ┌──────────────────────────────────────────────┐
               │         3. Normalized Semantic IR            │
               └───────┬──────────────────────────────┬───────┘
                       │                              │
                       ▼                              ▼
      ┌─────────────────────────────────┐   ┌─────────────────────────────────┐
      │   4. Symbol & Scope Analysis    │   │      5. CFG Construction        │
      └────────────────┬────────────────┘   └────────────────┬────────────────┘
                       │                                     │
                       ▼                                     ▼
      ┌─────────────────────────────────┐   ┌─────────────────────────────────┐
      │   6. Def-Use / Use-Def Analysis │   │       7. Dataflow Engine        │
      └────────────────┬────────────────┘   └────────────────┬────────────────┘
                       │                                     │
                       └──────────────┬──────────────────────┘
                                      ▼
                      ┌─────────────────────────────────┐
                      │   8. Dataframe Semantic Layer   │
                      └───────────────┬─────────────────┘
                                      ▼
                      ┌─────────────────────────────────┐
                      │  9. Column Provenance Engine    │
                      └───────┬─────────────────┬───────┘
                              │                 │
                              ▼                 │
      ┌─────────────────────────────────┐       │
      │    12. UNKNOWN Handling Layer   │       │
      └───────┬─────────────────────────┘       │
              │                                 │
              ▼                                 ▼
      ┌──────────────────────────────────────────────────┐
      │     10. Dependency Graph / PDG Integration       │
      └───────────────────────┬──────────────────────────┘
                              │
                              ▼
      ┌──────────────────────────────────────────────────┐
      │        11. Column-Aware Slicing Engine           │
      └───────────────────────┬──────────────────────────┘
                              │
                              ▼
      ┌──────────────────────────────────────────────────┐
      │           14. CLI / Output Interface             │
      └───────────────────────┬──────────────────────────┘
                              │ (Optional On-Demand)
                              ▼
      ┌──────────────────────────────────────────────────┐
      │         13. GNU R Runtime Bridge Client          │
      └──────────────────────────────────────────────────┘
```

### Component Directory & Responsibilities
1. **Parser:** Wraps extended Tree-sitter C library. Ingests raw UTF-8 R code, produces concrete syntax trees with fault recovery.
2. **CST Representation:** Preserves exact token trivia, comments, and whitespace; maintains 1:1 fidelity with source bytes.
3. **Normalized Semantic IR:** Strongly typed, desugared AST stripped of lexical trivia; standardizes assignment variants (`<-`, `=`, `->`, `<<-`) and operator hierarchies.
4. **Symbol/Scope Analysis:** Resolves lexical environments, declarations, references, closures, and data-masking scopes.
5. **CFG Construction:** Decomposes normalized statements into basic blocks, control edges (`BranchTrue`, `BranchFalse`), and terminal exits.
6. **Def-Use/Use-Def Analysis:** Computes variable lifecycles, reaching definitions, and value flow across CFG basic blocks.
7. **Dataflow Engine:** Executes iterative forward/backward abstract interpretation over the CFG.
8. **Dataframe Semantic Layer:** Maps recognized dataframe operations to semantic transfer functions and versioned states.
9. **Column Provenance Engine:** Derives and records fine-grained input-to-output column transformation edges.
10. **Dependency Graph / PDG Layer:** Unifies scalar data dependencies, dataframe state transitions, column derivation edges, and control dependencies into a single directed query graph.
11. **Column-Aware Slicing Engine:** Performs backward graph reachability from targeted column criteria, identifying retained/excluded semantic nodes and sub-statement regions.
12. **UNKNOWN Handling:** Represents missing, ambiguous, or dynamic dependencies as first-class graph entities; prevents silent edge dropping.
13. **Runtime Bridge:** Manages IPC communication with the external GNU R process for bounded execution and schema reflection.
14. **CLI Interface:** Primary user and machine interface; exposes `analyze`, `provenance`, and `slice` commands emitting JSON or text.

*(Note: Language Server Protocol (LSP) is explicitly excluded from MVP).*

---

## SECTION 3 — SOURCE REPRESENTATION (TWO-TIER ARCHITECTURE)

```text
R Source Code (UTF-8 Bytes)
            ↓
Tier 1: Concrete Syntax Tree (Tree-sitter CST)
            ↓ [AST Normalizer / Lowering Pass]
Tier 2: Normalized Semantic IR (Typed AST)
```

### 3.1 Node Identity & Source Locations
Every node in both CST and Semantic IR carries a globally unique identifier and a physical source span:

```text
NodeId: u32 (Dense index in Arena allocator)
Span {
    start_byte: u32,
    end_byte:   u32,
    start_line: u32,
    start_col:  u32,
    end_line:   u32,
    end_col:    u32
}
```

### 3.2 Normalized Semantic Node Hierarchy
The Semantic IR is purposefully compact, containing only nodes required for static program analysis:

```text
SemanticNode
├── Statement
│   ├── AssignmentStmt { target: LValue, value: ExprId, span: Span }
│   ├── ExprStmt       { expr: ExprId, span: Span }
│   ├── IfStmt         { condition: ExprId, then_branch: BlockId, else_branch: Option<BlockId>, span: Span }
│   ├── ForStmt        { iterator: SymbolId, iterable: ExprId, body: BlockId, span: Span }
│   ├── WhileStmt      { condition: ExprId, body: BlockId, span: Span }
│   └── ReturnStmt     { value: Option<ExprId>, span: Span }
├── Expr
│   ├── LiteralExpr    { kind: LiteralKind (Numeric, String, Bool, Null), span: Span }
│   ├── IdentifierExpr { symbol: SymbolId, span: Span }
│   ├── BinaryOpExpr   { op: BinaryOp, left: ExprId, right: ExprId, span: Span }
│   ├── UnaryOpExpr    { op: UnaryOp, target: ExprId, span: Span }
│   ├── CallExpr       { callee: ExprId, args: Vec<Argument>, span: Span }
│   ├── ExtractExpr    { target: ExprId, index: ExprId, kind: ExtractKind (Dollar, Bracket1, Bracket2), span: Span }
│   ├── PipeExpr       { left: ExprId, right: ExprId, kind: PipeKind (Native, Magrittr), span: Span }
│   ├── LambdaExpr     { params: Vec<Param>, body: BlockId, span: Span }
│   └── UnknownExpr    { reason: String, span: Span }
├── Argument
│   ├── PositionalArgument { value: ExprId, span: Span }
│   └── NamedArgument      { name: SymbolName, value: ExprId, span: Span }
└── Block              { stmts: Vec<StmtId>, span: Span }
```

### 3.3 Normalization & Desugaring Rules

1. **Context-Sensitive Assignment Normalization:**
   - In statement and block contexts, assignment operators (`<-`, `->`, and top-level `=`) are desugared into an identical `AssignmentStmt(target: LValue, value: ExprId)`.
   - **Crucial Context Sensitivity:** An `=` token inside a function call argument list (`foo(x = b)`) is **NOT** an `AssignmentStmt`; it is parsed and lowered as a `NamedArgument { name: "x", value: "b" }`.
   - An `=` token inside recognized dataframe mutation calls (`mutate(col = expr)`) is parsed as a column transformation specification (`ColumnAssignment { name: "col", expr: expr }`), strictly separating column derivation definitions from variable assignment.
2. **Super-assignment:** `a <<- b` is desugared into `AssignmentStmt` with a non-local lexical scope target flag.
3. **Explicit Pipe Lowering Rules (`PipeLowering`):**
   R-PARSE does **not** universally lower `lhs |> f(b)` or `lhs %>% f(b)` into naive `f(lhs, b)`. Lowering rules are strictly bounded to recognized patterns:
   ```rust
   enum PipeLoweringRule {
       /// lhs |> f(b) -> f(lhs, b)
       FirstPositionalArg,
       /// lhs |> f(x, y = _) -> f(x, y = lhs)
       NativeNamedPlaceholder,
       /// lhs %>% f(a, .) -> f(a, lhs)
       MagrittrDotArg,
       /// lhs |> f -> f(lhs)
       BareFunctionTarget,
       /// Ambiguous, conflicting, or unmodeled pipe constructs; retained as un-lowered
       /// PipeExpr and degrades to UNKNOWN_STATIC dependency rather than being incorrectly rewritten.
       UnsupportedOrAmbiguous,
   }
   ```
   - **Case 1 (Standard First-Argument Call):** In `lhs |> f(b)` or `lhs %>% f(b)` where the RHS is a call expression containing no placeholders, `lhs` is lowered as the first positional argument: `f(lhs, b)`.
   - **Case 2 (Native Pipe Placeholder `_` in R 4.2+):** In `lhs |> f(x, y = _)`, the placeholder `_` must occur in a named argument slot; `lhs` is bound to that designated named argument slot: `f(x, y = lhs)`.
   - **Case 3 (Magrittr Placeholder `.`):** In `lhs %>% f(a, .)`, `.` is replaced with `lhs`. In compound expressions (e.g., `lhs %>% f(names(.))`), standard first-argument injection is suppressed if `.` occurs at the top level of the call arguments.
   - **Case 4 (Bare Function Symbol):** In `lhs |> f` or `lhs %>% f`, lowered to a single-argument call: `f(lhs)`.
   - **Case 5 (Ambiguous / Unsupported Pipe Semantics):** When encountering multiple conflicting placeholders, dynamic expressions, or unrecognized custom pipe operators, R-PARSE does **not** naively rewrite the call. The construct remains an un-lowered `PipeExpr` with an explicit `UNKNOWN_STATIC` dataflow dependency edge.
4. **No Premature Complexity:** Language features that do not impact dataflow (e.g., operator precedence variations, trivia comments) are retained in CST and omitted from Semantic IR.

---

## SECTION 4 — SYMBOL AND SCOPE MODEL

### 4.1 Scope Hierarchy
Scopes form a directed tree where each child points to its enclosing parent:

```text
GlobalScope (Root, ScopeId: 0)
   ├── PackageNamespaceScope (read-only imports)
   └── FunctionScope (Local activation frame)
         ├── NestedClosureScope
         └── BlockScope (Local loop or branch block)
```

Each scope maintains:
- `id: ScopeId`
- `parent: Option<ScopeId>`
- `bindings: Map<SymbolName, Vec<DefinitionId>>`
- `kind: ScopeKind (Global, Function, DataMask)`

### 4.2 Disambiguating Variables from Dataframe Columns (Data-Masking)
In R, verbs like `mutate(total = price * quantity)` introduce a **Data-Masking Context** where identifiers may resolve either to dataframe columns or enclosing lexical variables.

```text
Identifier Reference: `x` inside Call `mutate(df, total = x * 2)`
                      │
                      ▼
       Is `x` present in df's active schema?
         ├── YES ────────► Bound to DataFrame Column `df$x` (Column Dependency)
         ├── NO  ────────► Bound to Enclosing Lexical Variable `x` (Scalar Dependency)
         └── UNCERTAIN ──► Emits UNKNOWN_STATIC Dependency (Conservative Fallback)
```

#### Operational Disambiguation Rules:
1. **Explicit Data Masking Operators:**
   - `.data$col` or `.data[["col"]]`: Statically forced to resolve against the active dataframe schema.
   - `.env$var`: Statically forced to bypass data masking and resolve to lexical environment.
2. **Standard Evaluation:** Outside recognized dataframe verbs, bare symbols resolve strictly via lexical scope inheritance (Local $\to$ Enclosing $\to$ Global $\to$ Packages).
3. **Dynamic Ambiguity:** If `df`'s schema contains `UNKNOWN` columns or is dynamically loaded at runtime, an ambiguous bare identifier resolves to **both** potential lexical binding and `UNKNOWN` column dependency.

---

## SECTION 5 — CFG AND DATAFLOW

### 5.1 CFG Node & Basic Block Definition
A Control Flow Graph $\mathcal{G}_{CFG} = (\mathcal{V}_{CFG}, \mathcal{E}_{CFG})$ consists of Basic Blocks:
- **`BasicBlock`:** A maximal sequence of straight-line Semantic Statements with a single entry and single exit.
- **`CFGNode`:** Individual statement or branch predicate inside a block.
- **`CFGEdge`:** Directed transition between nodes labeled with `EdgeKind`:
  - `Sequential`: Unconditional fall-through.
  - `BranchTrue`: Evaluated when condition is true.
  - `BranchFalse`: Evaluated when condition is false.
  - `LoopBack`: Edge returning to loop header.
  - `Exit`: Normal or early return termination.

### 5.2 Def-Use and Reaching Definitions
1. **`Definition(d)`:** An assignment `target <- expr` creating a new variable or updating an object state.
2. **`Use(u)`:** An identifier read inside an expression.
3. **`Reaching Definitions:`** Computed via bit-vector or map-based forward dataflow iteration:
   $$\text{In}(B) = \bigcup_{P \in \text{Pred}(B)} \text{Out}(P)$$
   $$\text{Out}(B) = \text{Gen}(B) \cup (\text{In}(B) \setminus \text{Kill}(B))$$
4. **`Def-Use Edge:`** Directed edge from definition $d$ to use $u$ if $d$ reaches $u$.

### 5.3 Control Dependencies
For program slicing, control dependencies are computed using the **Post-Dominator Tree (PDT)**:
- A node $Y$ is control dependent on $X$ ($X \xrightarrow{cd} Y$) iff $X$ has multiple control successors and there exists a path from $X$ to $Y$ in the CFG such that $Y$ post-dominates every node on the path except $X$.
- *Example:* In `if (valid) clean <- filter(raw, ...)`: the assignment to `clean` is control dependent on the condition `valid`.

**Boundary Clarification on Filtering:** CFG control dependence (`ControlDepEdge`) applies strictly to language-level control flow structures (`if`/`else`, `for`, `while`, `repeat`, early `return`). Dataframe row filtering (`filter(predicate)`) does **NOT** alter program basic-block control flow and must **NOT** be modeled as a CFG control dependency; it is modeled as a semantic `RowSelectionDependency` (Section 8.5).

---

## SECTION 6 — DATAFRAME STATE MODEL

A dataframe is **not** modeled as a monolithic scalar variable. It is modeled as a **Versioned Semantic State**.

```text
df@v0 ──────► mutate(total = price * quantity) ──────► df@v1 (or df2@v1)
Schema:                                                Schema:
{ price@v0, quantity@v0 }                             { price@v0, quantity@v0, total@v1 }
```

### 6.1 State Abstractions
```rust
struct DataFrameState {
    id: DataFrameId,
    version: u32,
    symbol: SymbolId,
    schema: Map<ColumnName, ColumnVersionId>,
    creation_stmt: StmtId,
}

struct ColumnVersion {
    id: ColumnVersionId,
    dataframe_id: DataFrameId,
    name: ColumnName,
    version: u32,
    provenance_edge: Option<ProvenanceEdgeId>,
    is_unknown: bool,
}
```

### 6.2 State Transition Mechanics
When a statement transforms a dataframe:
1. **Creation:** A new `DataFrameState` ($df@v_{k+1}$) is instantiated.
2. **Retention:** Unmodified columns carry forward references to their existing `ColumnVersionId` ($C@v_i$).
3. **Derivation:** Modified or added columns receive a newly allocated `ColumnVersionId` ($C@v_{k+1}$) with an associated derivation provenance edge.
4. **Deletion:** Removed columns are omitted from $df@v_{k+1}$'s schema map.
5. **Rebinding:** If the assignment rebinds the same symbol (`df <- mutate(df, ...)`), reaching definitions ensures subsequent statements resolve to $df@v_{k+1}$ while preserving $df@v_k$ as an immutable ancestor node in the provenance graph.

---

## SECTION 7 — COLUMN PROVENANCE MODEL

### 7.1 Formal Lineage Edge & ColumnDerivationNode
A Column Provenance Edge $\mathcal{P}$ is a directed derivation event preserved as a first-class graph object:

```rust
struct ColumnDerivationNode {
    id: DerivationId,
    inputs: Vec<ColumnVersionId>,
    output: ColumnVersionId,
    expression: ExprId,
    transformation_type: TransformationKind,
    span: Span,
    certainty: ProvenanceCertainty,
}
```

**Preservation of Derivation Grouping:** A multi-column provenance event such as:
$$\{ \text{price}, \text{quantity} \} \longrightarrow \text{total}$$
must remain representable as a **single, atomic derivation event** (`ColumnDerivationNode`). Even when internal graph query traversal traverses pairwise edges from each input column to the derivation node, the grouping of the derivation is permanently retained.

### 7.2 Schema Presence vs. Column Provenance
The system strictly enforces the distinction between schema shape and derivation lineage:
- **Schema Presence (Prior Art / flowR):** Asserting that schema $S$ contains column name `"total"`. Does not know how `"total"` was produced.
- **Column Provenance (R-PARSE):** Asserting that output column `"total"` derives from input columns `{"price", "quantity"}` through expression `"price * quantity"`.

```text
SCHEMA PRESENCE:              COLUMN DERIVATION:
┌──────────────┐              price ────────┐
│ df contains: │                            ├──► total = price * quantity
│ • price      │              quantity ─────┘
│ • quantity   │
│ • total      │
└──────────────┘
```

---

## SECTION 8 — SUPPORTED DATAFRAME SEMANTICS (MVP TRANSFORMATIONS)

For every supported operation, the transformation rule is mathematically and operationally specified:

---

### 8.1 `mutate` (Sequential Semantics)
- **Pattern:** `df_out <- df_in %>% mutate(col_1 = expr_1, col_2 = expr_2, ...)`
- **Input State:** $\text{Schema}(df\_in) = \{C_1, \dots, C_n\}$
- **Sequential Left-to-Right Processing:**
  Assignments inside a single `mutate` call are processed strictly from left to right.
  - Let $\text{Schema}_0 = \text{Schema}(df\_in)$.
  - For each assignment step $i = 1 \dots m$ (`col_i = expr_i`):
    1. Dependencies $\{C_{dep, i}\}$ are extracted against the intermediate schema $\text{Schema}_{i-1}$.
    2. A new column version $C_{i}@v_{new}$ is created.
    3. An atomic `ColumnDerivationNode` is recorded: $\{C_{dep, i}\} \xrightarrow{expr_i} C_{i}@v_{new}$.
    4. The intermediate schema is updated: $\text{Schema}_i = \text{Schema}_{i-1} \cup \{C_{i}@v_{new}\}$.
  - **Sequential Visibility Guarantee:** An assignment defined earlier in the call is immediately visible to later assignments in the same call.
    *Example:* In `mutate(total = price * quantity, tax = total * 0.18)`, `tax` resolves `total` against $\text{Schema}_1$, producing the derivation chain:
    $$\{ \text{price}, \text{quantity} \} \longrightarrow \text{total} \longrightarrow \text{tax}$$
- **Output State:** $\text{Schema}(df\_out) = \text{Schema}_m$.
- **Unknown Conditions:** If `expr` references dynamic indexing (`.data[[x]]`) or unmodeled calls, the target column derives from `UNKNOWN_STATIC`.

---

### 8.2 `transmute` (Sequential Semantics)
- **Pattern:** `df_out <- df_in %>% transmute(col_1 = expr_1, col_2 = expr_2, ...)`
- **Sequential Semantics:** Follows the identical left-to-right sequential evaluation rules as `mutate`.
- **Output State:** $\text{Schema}(df\_out) = \{C_{created, 1}, \dots, C_{created, m}\}$ (all unmentioned columns from $df\_in$ are dropped).
- **Unknown Conditions:** Same as `mutate`.

---

### 8.3 `select`
- **Pattern:** `df_out <- df_in %>% select(col_a, col_b)`
- **Input State:** $\text{Schema}(df\_in) = \{C_1, \dots, C_n\}$
- **Transformation Rule:** Retains specified columns; drops non-specified columns.
- **Output State:** $\text{Schema}(df\_out) = \{C_{kept}\} \cap \text{Schema}(df\_in)$.
- **Provenance Edges:** Identity forwarding edges ($C_k@v_{in} \to C_k@v_{out}$).
- **Unknown Conditions:** If dynamic select helpers (`all_of(v)`, `any_of(v)`) cannot be statically resolved, output schema contains `UNKNOWN_COLUMNS`.

---

### 8.4 `rename`
- **Pattern:** `df_out <- df_in %>% rename(new_name = old_name)`
- **Input State:** $\text{Schema}(df\_in) = \{C_1, \dots, C_{old}, \dots, C_n\}$
- **Transformation Rule:** Aliases column identity:
  - Create $C_{new}@v_{out}$.
  - Provenance edge: $C_{old}@v_{in} \xrightarrow{\text{alias}} C_{new}@v_{out}$.
- **Output State:** Replace $C_{old}$ with $C_{new}$ in schema.
- **Unknown Conditions:** If `new_name` or `old_name` is non-literal or dynamic.

---

### 8.5 `filter` (RowSelectionDependency)
- **Pattern:** `df_out <- df_in %>% filter(predicate_expr)`
- **Input State:** $\text{Schema}(df\_in) = \{C_1, \dots, C_n\}$
- **Transformation Rule:**
  - Extracts all columns referenced in `predicate_expr`: $\{C_{filter}\} \subseteq \text{Schema}(df\_in)$.
  - Schema columns are carried forward: $\text{Schema}(df\_out) = \text{Schema}(df\_in)$.
  - **RowSelectionDependency:** Filter predicates are **NOT** CFG control dependencies. Filtering establishes a semantic `RowSelectionDependency`:
    $$\mathcal{E}_{row\_sel} = \text{RowSelectionDependency}(df\_out, \{C_{filter}\}, \text{predicate\_expr})$$
  - Each surviving column in $\text{Schema}(df\_out)$ inherits this dependency on $\{C_{filter}\}$.
  - Slicing traversal targeting any output column from $df\_out$ follows the `RowSelectionDependency` backwards to pull in the predicate expression and its input columns.
- **Output State:** Same column names, identical column versions with attached row-selection dependency.
- **Unknown Conditions:** If `predicate_expr` contains unresolvable dynamic expressions.

---

### 8.6 Base R Direct Column Assignment (`df$col <- expr`)
- **Pattern:** `df$total <- df$price * df$quantity`
- **Input State:** Current version $df@v_k$.
- **Transformation Rule:**
  - Extracts dependencies from RHS expression (`price`, `quantity`).
  - Creates new version $df@v_{k+1}$.
  - Adds or replaces `total@v_{k+1}` with derivation edge from RHS dependencies.
- **Output State:** Rebound $df@v_{k+1}$.

---

### 8.7 Base R Subsetting (`df[, c("a", "b")]`)
- **Pattern:** `df_sub <- df[, c("a", "b")]`
- **Semantics:** Modeled identically to `select("a", "b")`.

---

### 8.8 Base R `transform()`
- **Pattern:** `df2 <- transform(df, total = price * quantity)`
- **Semantics:** Modeled identically to `mutate`.

---

## SECTION 9 — EXPRESSION DEPENDENCY EXTRACTION

Dependency extraction walks expression trees recursively under an active scope:

```text
ExtractDependencies(Expr, ActiveSchema, LexicalScope) -> Set<Dependency>
```

### 9.1 Primitive & Compound Rules
1. **Numeric / String / Boolean Literals:** $\emptyset$ (No column or variable dependency).
2. **Binary Arithmetic / Logic (`+`, `-`, `*`, `/`, `&`, `|`):**
   $$\text{Deps}(L \text{ op } R) = \text{Deps}(L) \cup \text{Deps}(R)$$
3. **Identifiers (`col`):**
   - If `col` $\in \text{ActiveSchema}$: Emit `ColumnDependency(col)`.
   - Else if `col` $\in \text{LexicalScope}$: Emit `VariableDependency(col)`.
   - Else: Emit `UnknownDependency(col)`.
4. **Standard Function Calls (`paste(x, y)`):**
   - Accumulate dependencies across all evaluated arguments:
     $$\text{Deps}(f(a_1, a_2)) = \bigcup \text{Deps}(a_i)$$
   - Function symbol $f$ is recorded as a call dependency.
5. **Dynamic Access (`.data[[idx]]` or `df[[var]]`):**
   - If `idx` is a static string literal `"x"`: Resolves cleanly to `ColumnDependency("x")`.
   - If `idx` is an unresolvable variable: Emits `UNKNOWN_DEPENDENCY`.

### 9.2 Conservative Handling of Arbitrary R Functions
- R-PARSE does **not** assume full semantic knowledge of external CRAN/Bioconductor package functions.
- If an unknown function $g(x_1, x_2)$ is called inside a transformation, R-PARSE conservatively assumes that the output depends on all supplied column arguments $\{x_1, x_2\}$. It degrades gracefully without crashing.

---

## SECTION 10 — UNKNOWN MODEL (FIRST-CLASS UNCERTAINTY)

Uncertainty is a formal analytical value in R-PARSE, not an error or an unhandled exception.

### 10.1 Orthogonal StaticKnowledge & RuntimeEvidence Model
Rather than a one-dimensional mathematical lattice that conflates static proof with dynamic observation, R-PARSE separates uncertainty into two orthogonal dimensions:

```rust
struct ProvenanceCertainty {
    static_knowledge: StaticKnowledge,
    runtime_evidence: RuntimeEvidence,
}

enum StaticKnowledge {
    /// Dependency is established by the implemented static semantic model
    /// under its documented assumptions.
    KnownStatic,
    /// Static uncertainty explicitly preserved; dynamic indexing, unmodeled calls,
    /// or unresolvable NSE.
    UnknownStatic,
}

enum RuntimeEvidence {
    /// No runtime observation has been performed (default in STATIC mode).
    None,
    /// Observed during a specific runtime execution context in GNU R.
    Observed {
        context_id: u32,
        observed_columns: Vec<ColumnMetadata>,
        timestamp: String,
    },
}
```

### 10.2 Operational Semantics of the Model
1. **Definition of `KNOWN_STATIC`:**
   > `KNOWN_STATIC` means the dependency is established by the implemented static semantic model under its documented assumptions.  
   *(R-PARSE does not claim universal whole-program soundness across arbitrary runtime metaprogramming).*
2. **Annotation, Not Lattice Promotion:**
   - When the GNU R runtime bridge observes an execution in `ASSISTED` mode, it annotates an `UnknownStatic` dependency with `RuntimeEvidence::Observed`.
   - **Crucial Rule:** Runtime observation does **NOT** promote `UnknownStatic` into `KnownStatic`. It documents empirical ground truth for a specific observation context without claiming universal proof for unobserved inputs.
3. **Trigger Conditions:**
   - Reading external files with dynamic headers (`read.csv(dyn_file)`).
   - Metaprogramming, reflection, `eval()`, `substitute()`.
   - Computed column indexing where constant propagation fails.
   - Non-modeled complex reshape operations.
4. **Propagation Invariant:**
   - If output column $C_{out}$ derives from $C_{in}$ and $C_{in}$ is `UnknownStatic`, then $C_{out}$ inherits `UnknownStatic`.
   - **Never Drop Silently:** An unresolvable dependency must be preserved as an explicit `UNKNOWN_STATIC` node in the dependency graph.
5. **Slicing Impact:**
   - When backward slicing encounters an `UnknownStatic` dependency, it conservatively retains the statement generating the uncertainty and flags the slice result with `contains_unknowns: true`.

---

## SECTION 11 — DEPENDENCY GRAPH / PROGRAM DEPENDENCE GRAPH (PDG)

The Program Dependence Graph $\mathcal{G}_{PDG} = (\mathcal{V}_{PDG}, \mathcal{E}_{PDG})$ unifies scalar dataflow, dataframe state transitions, grouped column derivations, and control dependencies into a single traversable graph.

```text
                               PDG NODE TYPES
 ┌──────────────────────┐  ┌──────────────────────┐  ┌──────────────────────┐
 │   StatementNode      │  │    VariableNode      │  │  ColumnVersionNode   │
 │   (StmtId, Span)     │  │  (Symbol, DefId)     │  │  (ColName, Version)  │
 └──────────────────────┘  └──────────────────────┘  └──────────────────────┘
 ┌──────────────────────────────────────────────────────────────────────────┐
 │                         ColumnDerivationNode                             │
 │    (DerivationId, Inputs: Vec<ColVerId>, Output: ColVerId, ExprId)       │
 └──────────────────────────────────────────────────────────────────────────┘
                                     ▲
                                     │
                              PDG EDGE TYPES
 ┌───────────────────────────────────┴──────────────────────────────────────┐
 │ • DefUseEdge:               VariableNode ──────────► StatementNode       │
 │ • UseDefEdge:               StatementNode ─────────► VariableNode        │
 │ • ColumnDerivationInEdge:   ColumnVersionNode ─────► ColumnDerivationNode│
 │ • ColumnDerivationOutEdge:  ColumnDerivationNode ──► ColumnVersionNode   │
 │ • RowSelectionDependency:   ColumnVersionNode ─────► RowPredicateExpr    │
 │ • ControlDepEdge:           StatementNode(Cond) ───► StatementNode       │
 └──────────────────────────────────────────────────────────────────────────┘
```

### Columns as Fine-Grained Ports
Dataframe state nodes expose individual **Column Ports**. Downstream statements accessing `df$target` connect their dependency edge directly to the port `df@v1.target`, completely bypassing unrelated sibling ports `df@v1.unused`.

---

## SECTION 12 — COLUMN-AWARE SLICING ENGINE

### 12.1 Slicing Criterion Definition
A slicing criterion $\mathcal{C}$ is a pair:

$$\mathcal{C} = \left( \text{StmtTarget}, \ \text{TargetEntity} \right)$$

Where:
- $\text{StmtTarget}$: Source statement or line number.
- $\text{TargetEntity}$:
  - Scalar Variable: `result`
  - Entire Object: `df2`
  - **Fine-Grained Column:** `(df2, "total")` or `df2$total`

### 12.2 Semantic-Node Slicing Granularity (`SliceElement` / `SliceNode`)
Column-aware slicing operates at **semantic-node granularity**, not merely coarse `Set<StmtId>`:

```rust
/// Semantic-node slice element (also termed SliceNode)
enum SliceElement {
    Statement(StmtId),
    Expression(ExprId),
    Definition(DefinitionId),
    ColumnVersion(ColumnVersionId),
    Derivation(DerivationId),
    ControlPredicate(ExprId),
    RowSelectionPredicate(ExprId),
}

pub type SliceNode = SliceElement;

struct SliceResult {
    criterion: SlicingCriterion,
    retained_elements: Set<SliceElement>,
    statement_coverage: Map<StmtId, StatementSliceCoverage>,
    contains_unknowns: bool,
}

enum StatementSliceCoverage {
    FullyRetained,
    PartiallyRetained {
        retained_elements: Vec<SliceElement>,
        excluded_elements: Vec<SliceElement>,
    },
    FullyExcluded,
}
```

**Sub-Statement Inclusion & Exclusion:**
A single source statement is explicitly permitted to contain both retained and excluded semantic regions.
- *Example:* In statement 3:
  ```r
  df2 <- clean %>% mutate(target = price * count, unused = age + 10)
  ```
  Statement 3 is classified as `PartiallyRetained`: the sub-expression and column derivation `target = price * count` is retained, while the sibling sub-expression `unused = age + 10` is excluded.
- **Explicit Non-Promise:** R-PARSE does **NOT** promise automated executable source code reconstruction or source rewriting from partial slices; it provides fine-grained semantic-node inclusion/exclusion attribution and source span mapping.

### 12.3 Backward Traversal Algorithm
```text
Algorithm: ColumnAwareBackwardSlice(Criterion C, Graph PDG) -> SliceResult
──────────────────────────────────────────────────────────────────────────
1. Initialize WorkList = { ResolveInitialElements(C) }
2. Initialize VisitedElements = EmptySet
3. Initialize RetainedElements = EmptySet
4.
5. WHILE WorkList is not empty:
6.    elem = WorkList.pop()
7.    IF elem in VisitedElements: CONTINUE
8.    VisitedElements.add(elem)
9.    RetainedElements.add(elem)
10.
11.   // Follow incoming dependency edges backwards:
12.   FOR EACH edge in PDG.incoming_edges(elem):
13.       MATCH edge:
14.           ColumnDerivationOutEdge(deriv_node) =>
15.               WorkList.push(SliceElement::Derivation(deriv_node.id))
16.               WorkList.push(SliceElement::Expression(deriv_node.expression))
17.               FOR input_col IN deriv_node.inputs:
18.                   WorkList.push(SliceElement::ColumnVersion(input_col))
19.           RowSelectionDependency(pred_expr, pred_cols) =>
20.               WorkList.push(SliceElement::RowSelectionPredicate(pred_expr))
21.               FOR col IN pred_cols:
22.                   WorkList.push(SliceElement::ColumnVersion(col))
23.           DefUseEdge(def_node) =>
24.               WorkList.push(SliceElement::Definition(def_node.id))
25.               WorkList.push(SliceElement::Statement(def_node.stmt_id))
26.           ControlDepEdge(cond_stmt) =>
27.               WorkList.push(SliceElement::ControlPredicate(cond_stmt.cond_expr))
28.               WorkList.push(SliceElement::Statement(cond_stmt.stmt_id))
29.           UnknownEdge(origin_stmt) =>
30.               WorkList.push(SliceElement::Statement(origin_stmt.id))
31.
32. Compute StatementCoverage(RetainedElements, AST)
33. RETURN SliceResult { RetainedElements, StatementCoverage, contains_unknowns }
```

### 12.4 Object-Level vs. Column-Aware Slicing
- **Object-Level Slice:** Slicing criterion `df2`. Traversal reaches all statements and columns that contributed to `df2`, retaining `unused = age + 10`.
- **Column-Aware Slice:** Slicing criterion `df2$target`. Traversal follows `target` $\to$ `Derivation(price * count)` $\to$ `price`, `count` $\to$ `clean` $\to$ `RowSelection(valid)` $\to$ `raw`, completely pruning `unused = age + 10`.

---

## SECTION 13 — RUNTIME BRIDGE (TIER 2 ARCHITECTURE)

```text
┌───────────────────────┐                    ┌────────────────────────┐
│      RUST CORE        │                    │    R BRIDGE PROCESS    │
│  (rparse CLI / Lib)   │                    │    (GNU R / Rscript)   │
└──────────┬────────────┘                    └───────────┬────────────┘
           │                                             │
           │ 1. Spawn worker / Connect IPC               │
           ├────────────────────────────────────────────►│
           │                                             │
           │ 2. Evaluate / Inspect Request (JSON)        │
           ├────────────────────────────────────────────►│
           │                                             │ [Executes in GNU R]
           │                                             │ [Captures colnames]
           │ 3. Response: Facts / Evidence / Error (JSON)│
           │◄────────────────────────────────────────────┤
           │                                             │
           │ 4. Optional Termination / Kill on Timeout   │
           ├────────────────────────────────────────────►│
```

### 13.1 Bridge Responsibilities & Lifecycle
1. **Decoupled Boundary:** The bridge operates across a standard process boundary (standard input/output pipes with length-prefixed JSON-RPC or line-delimited JSON).
2. **On-Demand Launch:** Tier 2 is **never** launched during `STATIC` analysis. It launches only in `ASSISTED` or `EXECUTE` modes upon explicit user invocation.
3. **Execution Environment & Process Isolation:**
   - Evaluations execute within dedicated child process environments (`new.env(parent = globalenv())`) serving strictly as an evaluation environment and process-level isolation mechanism (this is not an OS/container security boundary, and must not be described as a security sandbox).
   - Every request is guarded by a configurable execution timeout: 500ms is a provisional default subject to benchmark validation, not an immutable architectural guarantee.
   - If the timeout expires, the Rust core forcefully terminates the child process and records a `BRIDGE_TIMEOUT`.
   - Runtime execution strictly requires explicit user invocation (`--mode assisted` or `--mode execute`); the default `STATIC` mode remains 100% runtime-free.

### 13.2 Request / Response Wire Schema
```json
// Rust -> R Request
{
  "jsonrpc": "2.0",
  "id": 42,
  "method": "inspect_dataframe",
  "params": {
    "expression": "read.csv('data.csv')",
    "target_symbol": "df",
    "timeout_ms": 500
  }
}

// R -> Rust Response
{
  "jsonrpc": "2.0",
  "id": 42,
  "result": {
    "status": "success",
    "observed_columns": [
      { "name": "price", "type": "numeric" },
      { "name": "quantity", "type": "integer" }
    ],
    "row_count_estimate": 100,
    "execution_time_ms": 14
  }
}
```

---

## SECTION 14 — STATIC / RUNTIME INFORMATION MODEL

R-PARSE defines two orthogonal knowledge classifications:

```text
1. StaticKnowledge:
   • KNOWN_STATIC:
     Dependency is established by the implemented static semantic model under its
     documented assumptions. (Does not claim universal whole-program soundness).
   • UNKNOWN_STATIC:
     Dependency cannot be proven statically due to dynamic constructs or unmodeled calls.

2. RuntimeEvidence:
   • NONE:
     No runtime observation has occurred (default in STATIC mode).
   • OBSERVED:
     Observed during a specific runtime execution context in GNU R.
     Boundary Rule: Describes empirical observations in a particular run;
     does NOT constitute universal proof across unobserved inputs or alternative branches.
```

Runtime observations are stored in the graph as **Evidence Annotations**, never mutating the underlying static AST or erasing the static uncertainty classification.

---

## SECTION 15 — ANALYSIS MODES

| Mode | Runtime Invocation | Safety & Execution Guarantee | Target Use Case |
|---|---|---|---|
| **`STATIC` (Default)** | **Strictly Forbidden** (Tier 2 inactive) | 100% offline, zero side-effects, deterministic | CI/CD, pre-commit hooks, headless linters |
| **`ASSISTED`** | **Selective & Bounded** (On-demand queries) | Bounded expressions executed under timeouts | Interactive developer workflows, REPL |
| **`EXECUTE`** | **Full Script Execution** | Full execution via GNU R to produce oracle trace | Benchmark evaluation, ground-truth diffing |

---

## SECTION 16 — CLI CONTRACT

### 16.1 Command Families
```bash
# 1. Analyze script and emit AST / Diagnostics
rparse analyze <PATH.R> [--mode static|assisted] [--format json|text]

# 2. Extract column provenance graph
rparse provenance <PATH.R> [--column df$target] [--format json|dot|text]

# 3. Compute column-aware backward slice
rparse slice <PATH.R> --criterion <CRITERION> [--mode static|assisted] [--format json|text]
```

### 16.2 Exit Codes
- `0`: Success (Analysis completed cleanly, all criteria resolved).
- `1`: Completed with Uncertainty (Completed, but slice/provenance contains `UNKNOWN` edges).
- `2`: Syntax / Parse Error (Invalid R syntax).
- `3`: Runtime Bridge Failure (Timeout, GNU R crashed or missing).
- `10`: Fatal Engine Error (Internal panic).

---

## SECTION 17 — SERIALIZATION (JSON SCHEMAS)

All machine-readable outputs adhere to versioned JSON schemas (`schema_version: "1.0.0"`).

### 17.1 Provenance Graph Output Schema (`rparse provenance --format json`)
```json
{
  "schema_version": "1.0.0",
  "source_file": "script.R",
  "analysis_mode": "static",
  "dataframes": [
    {
      "dataframe_id": 1,
      "symbol": "df2",
      "version": 1,
      "columns": [
        {
          "name": "total",
          "version": 1,
          "certainty": "KNOWN_STATIC",
          "derived_from": [
            { "column": "price", "dataframe_version": 0 },
            { "column": "quantity", "dataframe_version": 0 }
          ],
          "expression": "price * quantity",
          "span": { "start_line": 2, "start_col": 11, "end_line": 2, "end_col": 27 }
        }
      ]
    }
  ]
}
```

### 17.2 Slice Output Schema (`rparse slice --format json`)
```json
{
  "schema_version": "1.0.0",
  "criterion": "df2$target",
  "coverage_summary": {
    "fully_retained_statements": [1, 2, 5],
    "partially_retained_statements": [3],
    "fully_excluded_statements": []
  },
  "statements": [
    {
      "stmt_id": 1,
      "line": 1,
      "code": "raw <- read.csv(\"data.csv\")",
      "coverage": "fully_retained"
    },
    {
      "stmt_id": 2,
      "line": 2,
      "code": "clean <- raw %>% filter(valid == TRUE)",
      "coverage": "fully_retained",
      "retained_elements": [
        { "type": "row_selection", "predicate": "valid == TRUE", "columns": ["valid"] }
      ]
    },
    {
      "stmt_id": 3,
      "line": 3,
      "code": "df2 <- clean %>% mutate(target = price * count, unused = age + 10)",
      "coverage": "partially_retained",
      "retained_elements": [
        { "type": "derivation", "target_column": "target", "expression": "price * count", "inputs": ["price", "count"] }
      ],
      "excluded_elements": [
        { "type": "derivation", "target_column": "unused", "expression": "age + 10", "inputs": ["age"] }
      ]
    },
    {
      "stmt_id": 5,
      "line": 5,
      "code": "res <- df2$target",
      "coverage": "fully_retained"
    }
  ],
  "contains_unknowns": false
}
```

---

## SECTION 18 — ERROR MODEL

Errors are strictly categorized to avoid masking analysis uncertainty as system crashes:

```text
                               Error Hierarchy
 ┌─────────────────────────┐ ┌─────────────────────────┐ ┌─────────────────────────┐
 │       PARSE_ERROR       │ │    SEMANTIC_UNKNOWN     │ │  UNSUPPORTED_CONSTRUCT  │
 │ Malformed R syntax;     │ │ Analytical uncertainty; │ │ Unmodeled tidyverse/base│
 │ Tree-sitter recovers or │ │ FIRST-CLASS STATE;      │ │ verbs; degrades to      │
 │ reports syntax failure  │ │ NEVER a fatal error     │ │ UNKNOWN gracefully      │
 └─────────────────────────┘ └─────────────────────────┘ └─────────────────────────┘
 ┌─────────────────────────┐ ┌─────────────────────────┐ ┌─────────────────────────┐
 │      RUNTIME_ERROR      │ │     BRIDGE_FAILURE      │ │  INTERNAL_ENGINE_ERROR  │
 │ GNU R evaluated script  │ │ Timeout, IPC disconnect,│ │ Rust bug, invariant    │
 │ raised an error/warning │ │ missing GNU R binary    │ │ violation, memory panic │
 └─────────────────────────┘ └─────────────────────────┘ └─────────────────────────┘
```

---

## SECTION 19 — PROJECT STRUCTURE (MODULAR RUST WORKSPACE)

```text
rparse/
├── Cargo.toml                      # Workspace root configuration
├── crates/
│   ├── rparse-core/                # Shared utilities, Span, NodeId, Diagnostics
│   ├── rparse-parser/              # Tree-sitter-r bindings & CST builder
│   ├── rparse-ir/                  # Normalized Semantic Representation
│   ├── rparse-scope/               # Symbol table & lexical scope resolver
│   ├── rparse-cfg/                 # Control Flow Graph & basic blocks
│   ├── rparse-dataflow/            # Def-use chains & reaching definitions
│   ├── rparse-dataframe/          # Dataframe state versioning & transfer functions
│   ├── rparse-provenance/         # Fine-grained column derivation graph
│   ├── rparse-graph/              # Unified Program Dependence Graph (PDG)
│   ├── rparse-slicing/            # Column-aware backward slicing traversal
│   ├── rparse-runtime-bridge/     # Tier 2 GNU R IPC process client
│   └── rparse-cli/                # Main binary executable (`rparse`)
├── runtime/
│   └── r-bridge/
│       ├── bridge_worker.R         # GNU R daemon script (JSON-RPC over stdio)
│       └── inspect_schema.R        # Reflection helpers for dataframe schemas
└── tests/
    ├── golden/                     # Golden test files (.R and .expected.json)
    ├── unit/                       # Component-level unit tests
    └── integration/                # End-to-end integration tests
```

### Modular Dependency Direction (Zero Circular Dependencies)
```text
rparse-cli
   └── rparse-slicing
          └── rparse-graph
                 ├── rparse-provenance
                 │      └── rparse-dataframe
                 │             └── rparse-dataflow
                 │                    └── rparse-cfg
                 │                           └── rparse-scope
                 │                                  └── rparse-ir
                 │                                         └── rparse-parser
                 │                                                └── rparse-core
                 └── rparse-runtime-bridge
                        └── rparse-core
```

---

## SECTION 20 — IMPLEMENTATION ORDER (PHASED EXECUTION)

Development must proceed in strict linear order to maintain testability at every stage:

1. **Phase 1: Workspace & Core Types (`rparse-core`):** Node IDs, spans, error types, diagnostics.
2. **Phase 2: Parser & CST (`rparse-parser`):** Tree-sitter-r integration, grammar patches (pipe placeholder `_`).
3. **Phase 3: Semantic IR (`rparse-ir`):** Normalizer converting CST to desugared Semantic AST.
4. **Phase 4: Scope & Symbols (`rparse-scope`):** Lexical environments, definitions, references.
5. **Phase 5: CFG & Blocks (`rparse-cfg`):** Basic block decomposition, branch edges, loop edges.
6. **Phase 6: Dataflow (`rparse-dataflow`):** Reaching definitions, def-use and use-def chains.
7. **Phase 7: Dataframe Semantics (`rparse-dataframe`):** Versioned state model, verbs (`select`, `rename`, `mutate`, `filter`).
8. **Phase 8: Column Provenance (`rparse-provenance`):** Input-to-output derivation edges, expression mapping.
9. **Phase 9: Program Dependence Graph (`rparse-graph`):** Unified PDG merging dataflow, control dependencies, and column ports.
10. **Phase 10: Column-Aware Slicer (`rparse-slicing`):** Backward traversal from column criteria to semantic slice elements.
11. **Phase 11: UNKNOWN Semantics:** Propagation and conservative slicing tests for uncertain edges.
12. **Phase 12: Runtime Bridge (`rparse-runtime-bridge`):** GNU R worker process, timeouts, inspection queries.
13. **Phase 13: Runtime Evidence Integration:** Annotating static PDG with `RuntimeEvidence::Observed` evidence.
14. **Phase 14: CLI Interface (`rparse-cli`):** Exposing `analyze`, `provenance`, `slice` commands.
15. **Phase 15: Golden Test Suite & Benchmark Validation:** Executing canonical benchmark suite.

---

## SECTION 21 — TEST STRATEGY & CANONICAL BENCHMARKS

### 21.1 Test Levels
1. **Unit Tests:** Embedded in each crate; tests isolated data structures and transfer functions.
2. **Semantic Golden Tests:** Asserts that input R snippets normalize to identical semantic AST JSON files.
3. **Provenance Golden Tests:** Asserts that dataframe pipelines produce exact column derivation graphs.
4. **Slicing Golden Tests:** Asserts that backward slices extract the exact minimal semantic elements.
5. **Runtime Contract Tests:** Asserts that the GNU R bridge responds within timeouts and handles errors cleanly.

### 21.2 The Seven Frozen Benchmark Scenarios

#### Benchmark Input Schema Discovery Rule
For benchmarks whose input dataframe is external or undefined (e.g., `df` in Benchmarks 1–6):
The analyzer does **NOT** assume magical static knowledge of external schemas. The input schema becomes known via either:
- **Option A (Declared Benchmark Fixture Schema):** A static fixture declaration in the test suite specifying column names and types (e.g., `fixture: df = {price: numeric, quantity: numeric, category: character, ...}`).
- **Option B (Runtime-Assisted Schema Discovery):** In `ASSISTED` mode, the Tier 2 GNU R runtime queries the active session to observe `colnames(df)`.
- *Fallback:* If neither is provided, static analysis treats `df`'s schema as `UNKNOWN_STATIC`, propagating uncertainty gracefully without failing.

---

#### Benchmark 1: Direct Column Derivation
```r
# Input schema for df declared via fixture: {price: numeric, quantity: numeric}
df2 <- df %>% mutate(total = price * quantity)
```
- **Criterion:** `df2$total`
- **Assertion:** Provenance contains atomic derivation event `{price@v0, quantity@v0} -> total@v1` via `price * quantity`.

#### Benchmark 2: Multi-Step Provenance Propagation
```r
# Input schema for df declared via fixture: {price: numeric, quantity: numeric}
df2 <- df %>% mutate(total = price * quantity)
df3 <- df2 %>% mutate(tax = total * 0.18)
```
- **Criterion:** `df3$tax`
- **Assertion:** Provenance chain `price -> total -> tax` and `quantity -> total -> tax` preserved across sequential states.

#### Benchmark 3: Renaming & Projection
```r
# Input schema for df declared via fixture: {price: numeric, customer_id: character, notes: character}
df2 <- df %>% rename(cost = price) %>% select(cost, customer_id)
```
- **Criterion:** `df2$cost`
- **Assertion:** `cost` aliases `price`; unselected column `notes` is dropped from $df2$'s schema.

#### Benchmark 4: Multi-Column Combinations
```r
# Input schema for df declared via fixture: {first_name: character, last_name: character}
df2 <- df %>% mutate(full_name = paste(first_name, last_name))
```
- **Criterion:** `df2$full_name`
- **Assertion:** `full_name` derives from both `first_name` and `last_name` in a single `ColumnDerivationNode`.

#### Benchmark 5: Object vs. Column Isolation
```r
# Input schema for df declared via fixture: {x: numeric, y: numeric}
df2 <- df %>% mutate(a = x + 1, b = y + 1)
```
- **Criterion:** `df2$a`
- **Assertion:** Slice retains `a = x + 1`; statement 1 is partially retained, strictly excluding `b = y + 1`.

#### Benchmark 6: Explicit UNKNOWN Handling
```r
# Input schema for df declared via fixture: {col_a: numeric}
df2 <- df %>% mutate(dynamic_col = .data[[get_col()]])
```
- **Criterion:** `df2$dynamic_col`
- **Assertion:** Provenance records derivation from `UNKNOWN_STATIC`; analysis completes without crashing.

#### Benchmark 7: Filter Predicate + Target Slicing (Corrected & Verified)
```r
raw <- read.csv("data.csv") # schema discovered via runtime assist or mock fixture
clean <- raw %>% filter(valid == TRUE)
df2 <- clean %>% mutate(target = price * count, unused = age + 10)
res <- df2$target
```
- **Criterion:** `res` (or `df2$target`)
- **Required Retained Slice (Semantic Elements):**
  - Statement 1: `raw <- read.csv("data.csv")` (Fully Retained)
  - Statement 2: `clean <- raw %>% filter(valid == TRUE)` (Fully Retained; traverses `RowSelectionDependency` to retain `valid`)
  - Statement 3: `df2 <- clean %>% mutate(...)` (**Partially Retained**: retains `target = price * count`; excludes `unused = age + 10`)
  - Statement 4: `res <- df2$target` (Fully Retained)
- **Required Excluded Elements:**
  - Sub-expression `unused = age + 10` (and upstream statements defining `age` if `age` is otherwise unused).

---

## SECTION 22 — ACCEPTANCE CRITERIA

Each component is accepted only when all objective criteria are satisfied:

1. **Parser (`rparse-parser`):** 100% pass rate on the defined parser conformance corpus, with no panics and graceful recovery for unsupported semantic analysis.
2. **Normalized IR (`rparse-ir`):** Desugars assignment variants to unified AST representation; distinguishes assignments from named arguments.
3. **Dataframe Semantics (`rparse-dataframe`):** Correctly tracks versioned schemas across all 8 supported MVP transformations with sequential mutate evaluation.
4. **Column Provenance (`rparse-provenance`):** Emits atomic `ColumnDerivationNode` records for multi-column expressions.
5. **Slicing Engine (`rparse-slicing`):** Successfully slices targeted columns at semantic-node granularity, supporting partial statement retention.
6. **UNKNOWN Semantics:** Generates explicit `UNKNOWN_STATIC` nodes for dynamic lookups; zero unhandled panics on dynamic R code.
7. **Runtime Bridge (`rparse-runtime-bridge`):** Evaluates expressions in GNU R within provisional timeout (500ms default subject to benchmark validation); isolates child process crashes via evaluation environment isolation; annotates `RuntimeEvidence::Observed` evidence upon explicit user invocation.
8. **End-to-End Pipeline:** All 7 canonical benchmarks pass in automated continuous integration.

---

## SECTION 23 — EXPLICIT NON-GOALS (PRESERVED)

R-PARSE explicitly will **NOT**:
1. Implement a custom R interpreter or virtual machine.
2. Compile R into native binaries or machine code.
3. Replace GNU R or the active R runtime.
4. Guarantee whole-program soundness across arbitrary runtime metaprogramming.
5. Re-implement the 118 generic rules of `lintr`.
6. Function as a workflow DAG execution scheduler (does not replace `targets`).
7. Implement a standalone statistical formula modeling engine (deferred).
8. Implement a standalone ggplot2 visualization engine (deferred).
9. Guarantee automatic generation of a 15-line executable reprex.
10. Implement an LSP server or IDE plugins in the MVP.
11. Treat runtime observation as universal proof: Runtime observations describe specific execution contexts and do not constitute universal proof across unobserved inputs or alternative branches.
12. Provide an OS/container security sandbox: `new.env(parent = globalenv())` is an evaluation environment and process isolation mechanism, not an OS or container security boundary. R-PARSE does not sandbox arbitrary untrusted code execution.

---

## SECTION 24 — OPEN IMPLEMENTATION DETAILS (DEFERRED TO CODE)

The following items are low-level engineering choices that remain intentionally open for the implementer:
1. **Tree-sitter Binding Version:** Choice between `tree-sitter` 0.20.x vs 0.22.x crate.
2. **Graph Data Structure Implementation:** Choice between `petgraph` vs specialized arena-indexed adjacency lists.
3. **JSON Serialization Engine:** Standard `serde` / `serde_json` configuration.
4. **CLI Framework:** Choice between `clap` (derive) vs `argh`.
5. **Exact Process IPC Format:** Length-prefixed JSON vs newline-delimited JSON over stdio for the GNU R bridge worker.

*(All architectural, semantic, and boundary decisions are formally closed).*

---

## SECTION 25 — DESIGN PRINCIPLES (FROZEN)

1. **Broad compatibility, conservative semantics:** Parse broadly, represent broadly, analyze precisely where supported, represent uncertainty explicitly.
2. **Provenance is first-class:** Derivation lineage ($C_{in} \to C_{out}$) is a core primitive, not an afterthought.
3. **Object dependencies and column dependencies are distinct:** Never conflate dataframe-level dataflow with column-level value provenance.
4. **UNKNOWN is explicit:** Uncertainty is represented natively; the system never guesses or drops unresolved edges.
5. **Runtime evidence is observational, not universal proof:** Observed facts describe a specific execution context and never contaminate static invariants.
6. **Rust owns semantic analysis:** All parsing, IR normalization, dataflow, and slicing algorithms live in Tier 1.
7. **GNU R owns actual execution:** Real R code execution is always delegated to official GNU R.
8. **Runtime integration is isolated behind a process boundary:** The core engine functions 100% offline without R.
9. **Unsupported semantics degrade gracefully:** Unmodeled verbs or dynamic constructs degrade to `UNKNOWN` without failing analysis.
10. **Analysis modules must remain independently testable:** Every crate in the workspace must have autonomous unit and golden test coverage.

---

```text
TECHNICAL SPECIFICATION COMPLETE —
R-PARSE OPTION D ARCHITECTURE
RUST SEMANTIC CORE + GNU R RUNTIME BRIDGE
```
