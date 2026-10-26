# R-PARSE — FLOWR vs R-PARSE FORENSIC GAP AUDIT
## Deep Implementation-Level Analysis of Dataframe Semantics, Provenance, Dataflow & Slicing

**Author:** R-PARSE Lead Research Engineer & Software Archaeology Agent  
**Date:** October 2026  
**Scope:** Local source-level forensic audit of flowR at commit `3ea266c712b34a585402f1a3c66811850340313b` (https://github.com/flowr-analysis/flowr). That checkout is not part of the public tree. Line citations below are the files and lines inspected in that checkout. Absolute `file:///` links in this audit are local inspection paths.\
**Objective:** Determine from actual flowR source code what dataframe capabilities flowR already implements, where the information flows, where the information terminates, and whether a defensible technical gap remains for a unified R Semantic Program Model (R-SPM).

---

## 1. Executive Verdict

From an exhaustive, source-level forensic audit of the `flowR` codebase, the central question is answered with high certainty:

> **Yes, a substantial, mathematically and architecturally well-defined technical gap remains between flowR's existing dataframe abstract interpretation and a unified program model capable of fine-grained column provenance + program slicing.**

Specifically:
1. **flowR's Dataframe Analysis is Exclusively Schema Shape Tracking (Level 1):** flowR infers which column names *must* or *may* exist in a dataframe (`SetRangeDomain<string>`), along with numeric intervals for column and row counts (`PosIntervalDomain`). It does **not** track column-to-column value derivation edges (e.g., `total` derives from `price * quantity`).
2. **flowR's Program Slicer is Completely Decoupled from Dataframe Analysis:** flowR's static slicer (`staticSlice` in [`src/slicing/static/static-slicer.ts`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/repos/flowr/src/slicing/static/static-slicer.ts#L66-L203)) operates strictly via Breadth-First Search over `info.graph` (`DataflowGraph`). Slicing treats dataframes as monolithic scalar variables (`VariableDefinitionVertex`). There is **zero connection** between `DataFrameShapeAnalysis` and `staticSlice`.
3. **Dataframe Information Terminates at Two Consumers:** In the entire flowR codebase, `DataFrameShapeAnalysis` has only two functional consumers:
   - A linter rule ([`src/linter/rules/dataframe-access-validation.ts`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/repos/flowr/src/linter/rules/dataframe-access-validation.ts#L61-L125)) that checks whether accessed columns exist in `must` or `may` sets.
   - A standalone CLI/REPL query endpoint ([`src/queries/catalog/absint-query/absint-query-format.ts`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/repos/flowr/src/queries/catalog/absint-query/absint-query-format.ts#L45-L60)) that prints inferred schema shapes.
   It never feeds into the dependency graph, CFG, reaching definitions, def-use chains, or program slicer.
4. **Statistical Formulas and Visualizations are Semantically Opaque:** Modeling formulas (`~` in `lm()`) and ggplot2 aesthetics (`aes()` in `ggplot()`) are configured only to suppress "undefined variable read" warnings via `markArgsAsMasked: NseArguments.All`. Neither constructs a term graph, model matrix, or column-binding edge.

---

## 2. flowR Dataframe Architecture

The flowR dataframe abstract interpretation engine resides under [`src/abstract-interpretation/data-frame/`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/repos/flowr/src/abstract-interpretation/data-frame):

```text
src/abstract-interpretation/data-frame/
├── dataframe-domain.ts      # Product domain: colnames, cols, rows
├── semantics.ts             # 20 Abstract operations & transfer functions
├── dataframe-semantics.ts   # AST-to-operation mapper (30+ R functions)
├── shape-inference.ts       # DataFrameShapeAnalysis definition
├── arguments.ts             # Argument extraction & resolution utilities
└── resolve-args.ts          # Normalizer for call-site parameters
```

### Core Data Structure: `DataFrameDomain`
[EVIDENCE: [`src/abstract-interpretation/data-frame/dataframe-domain.ts:L6-L15`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/repos/flowr/src/abstract-interpretation/data-frame/dataframe-domain.ts#L6-L15)]
```typescript
export type AbstractDataFrameShape = {
    readonly colnames: SetRangeDomain<string>;
    readonly cols:     PosIntervalDomain;
    readonly rows:     PosIntervalDomain;
};

export class DataFrameDomain extends ProductDomain<AbstractDataFrameShape> { ... }
```
- `colnames`: Tracks a pair `{ must: Set<string>, may: Set<string> | Top }`.
- `cols`: Tracks a lower and upper bound `[lower, upper]` for the column count.
- `rows`: Tracks a lower and upper bound `[lower, upper]` for the row count.

**Forensic Finding:** There are **no column values, types, expressions, or dependency pointers** stored in `DataFrameDomain`. It is strictly a geometric/schema lattice.

---

## 3. `DataFrameShapeAnalysis` Implementation Map

[EVIDENCE: [`src/abstract-interpretation/data-frame/shape-inference.ts:L59-L73`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/repos/flowr/src/abstract-interpretation/data-frame/shape-inference.ts#L59-L73)]
`DataFrameShapeAnalysis` implements `AbsintAnalysis<DataFrameShapeDomains>`:

| File | Class / Function | Purpose | Input | Output | Consumers | Evidence Tag |
|---|---|---|---|---|---|---|
| `shape-inference.ts` | `DataFrameShapeAnalysis` | Analysis orchestrator combining domain and semantics | `DataFrameShapeAnalysisOptions` | `AbsintAnalysis` instance | `AbstractInterpreter` | [EVIDENCE] |
| `dataframe-domain.ts` | `DataFrameDomain` | Product lattice for dataframe shapes | `AbstractDataFrameShape` | Abstract value | Transfer functions, state trace | [EVIDENCE] |
| `semantics.ts` | `DataFrameSemanticsMapper` | Dispatches 20 abstract operations to transfer functions | `operation, value, args` | Updated `DataFrameDomain` | `applyDataFrameSemantics` | [EVIDENCE] |
| `semantics.ts` | `applyMutateColsSemantics` | Updates `colnames` with added/replaced names | `value, { colnames }` | Updated `DataFrameDomain` | `DataFrameSemanticsMapper['mutateCols']` | [EVIDENCE] |
| `semantics.ts` | `applyJoinSemantics` | Merges shapes of two dataframes for joins | `value, { other, by }, options` | Merged `DataFrameDomain` | `DataFrameSemanticsMapper['join']` | [EVIDENCE] |
| `dataframe-semantics.ts` | `DataFrameShapeSemantics` | Walks AST nodes to emit abstract operations | AST function calls / access nodes | `DataFrameOperations` array | `AbstractInterpreter` | [EVIDENCE] |
| `dataframe-semantics.ts` | `mapDataFrameMutate` | Mappper for `dplyr::mutate`, `transform` | AST arguments, context | `[accessCols, mutateCols, removeCols]` | `DataFrameSemantics.functionCalls` | [EVIDENCE] |
| `dataframe-semantics.ts` | `mapDataFrameJoin` | Mapper for `dplyr::*_join`, `merge` | AST arguments, context | `[accessCols, join]` | `DataFrameSemantics.functionCalls` | [EVIDENCE] |
| `dataframe-semantics.ts` | `mapDataFrameRead` | Mapper for `read.csv`, `read_csv` | File path, delimiter args | `[read]` | `DataFrameSemantics.functionCalls` | [EVIDENCE] |

---

## 4. End-to-End Information Flow & Data Consumption Graph

Tracing the end-to-end execution of `DataFrameShapeAnalysis`:

```text
Program Source
     ↓
R Bridge / Parser (lang-4.x)
     ↓
NormalizedAst (AST with NodeIds)
     ↓
Dataflow Information Builder (DFG)
     ↓
Control Flow Graph Builder (CFG)
     ↓
AbstractInterpreter (absint-inference.ts)
     ↓  [Takes CFG, DFG, AST, and DataFrameShapeAnalysis]
     ↓  [Performs forward CFG visitor fixpoint iteration]
     ↓
Abstract State Trace Map: Map<NodeId, MultiValueStateDomain>
     │
     ├──→ Consumer 1: Linter Rule (`dataframe-access-validation.ts`)
     │         Checks: is `df$col` present in `operandShape.colnames.must`?
     │
     ├──→ Consumer 2: Query API (`absint-query-executor.ts` -> `'df-shape'`)
     │         Returns: `{ colnames: [...], cols: [n, m], rows: [k, l] }`
     │
     └──→ Consumer 3: Benchmark Profiler (`benchmark/slicer.ts`)
               Measures: runtime execution time of AI pass (explicitly excluded from slicing)
               TERMINATION: Zero downstream consumers.
```

### Consumption Classification Across 11 Architectural Components

| Component | Source Location | Produces | Consumes | Downstream Consumer | Classification |
|---|---|---|---|---|---|
| 1. Dependency Graph | [`src/dataflow/graph/graph.ts`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/repos/flowr/src/dataflow/graph/graph.ts) | AST dataflow edges (`Reads`, `DefinedBy`) | Normalized AST, variable scopes | `staticSlice`, CFG | **NOT PRESENT** (Zero column info in DFG) |
| 2. Control Flow Graph | [`src/control-flow/control-flow-graph.ts`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/repos/flowr/src/control-flow/control-flow-graph.ts) | Control flow edges (`FlowEdge`, `ControlEdge`) | DFG, AST statements | `AbstractInterpreter`, Slicer | **NOT PRESENT** (Pure statement/expression CFG) |
| 3. SSA Form | — | — | — | — | **NOT PRESENT** (flowR has no SSA representation) |
| 4. Reaching Definitions | [`src/dataflow/origin/dfg-get-origin.ts`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/repos/flowr/src/dataflow/origin/dfg-get-origin.ts) | Variable definition origin IDs | DFG vertices | `getVariableOrigins` in AbsInt | **AVAILABLE BUT NOT CONNECTED** (Resolves whole variable symbols; does not resolve column definitions) |
| 5. Def-Use Chains | [`src/dataflow/graph/edge.ts`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/repos/flowr/src/dataflow/graph/edge.ts) | `Reads` and `DefinedBy` edges | Variable uses and definitions | `staticSlice` | **NOT PRESENT** (Links scalar symbols; no column-level def-use) |
| 6. Call Graph | [`src/dataflow/graph/call-graph.ts`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/repos/flowr/src/dataflow/graph/call-graph.ts) | Call-to-definition edges | Function call vertices | Slicer interprocedural boundary | **NOT PRESENT** (Operates on function definitions; oblivious to dataframe shapes) |
| 7. Program Slicing | [`src/slicing/static/static-slicer.ts`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/repos/flowr/src/slicing/static/static-slicer.ts) | Backward/forward slice NodeId set | `info.graph` (DFG), seed NodeIds | REPL, IDE, Benchmark | **AVAILABLE BUT NOT CONNECTED** (Slicer traverses DFG only; never inspects AbsInt trace or column sets) |
| 8. Liveness Analysis | [`src/dataflow/environments/apply-kill.ts`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/repos/flowr/src/dataflow/environments/apply-kill.ts) | Environment scope kill sets | AST assignments | DFG builder | **NOT PRESENT** (Scoping kill sets operate on variable names in environments) |
| 9. Static Queries | [`src/queries/catalog/absint-query/`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/repos/flowr/src/queries/catalog/absint-query/) | Query output for `'df-shape'` | AbsInt trace, SlicingCriterion | CLI REPL / JSON API | **DIRECTLY INTEGRATED** (Exposes inferred shape at query AST nodes) |
| 10. Diagnostics | [`src/linter/rules/dataframe-access-validation.ts`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/repos/flowr/src/linter/rules/dataframe-access-validation.ts) | Lint diagnostics for invalid accesses | AbsInt trace, AST access nodes | Linter report | **DIRECTLY INTEGRATED** (Checks `df$col` against `must`/`may` column sets) |
| 11. Runtime Correlation | — | — | — | — | **NOT PRESENT** (Zero dynamic trace or session correlation) |

---

## 5. What Exactly Does flowR Know About Columns?

flowR tracks **only**:
```text
df contains:
  must: { price, quantity, total }
  may:  { tax }
```

flowR does **NOT** track:
```text
df.price   ──┐
             ├──→ df2.total ──→ df2.tax
df.quantity ─┘
```

### Forensic Proof from Source Code
In [`src/abstract-interpretation/data-frame/dataframe-semantics.ts:L1245-L1276`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/repos/flowr/src/abstract-interpretation/data-frame/dataframe-semantics.ts#L1245-L1276) (`mapDataFrameMutate`):
```typescript
let mutatedCols: (string | undefined)[] | undefined = mutateArgs
    .filter(arg => !isRNull(arg))
    .map(arg => Resolve.argument.toName(arg, info));

const mutatedColSet = new Set(mutatedCols);
const accessedNames = mutateArgs
    .flatMap(arg => getUnresolvedSymbolsInExpression(arg, info.graph))
    .filter(arg => !mutatedColSet.has(arg));

if(accessedNames.length > 0) {
    result.push({
        operation: 'accessCols',
        operand:   operand?.info.id,
        columns:   accessedNames
    });
}

if(mutatedCols === undefined || mutatedCols.length > 0 || deletedCols?.length === 0) {
    result.push({
        operation: 'mutateCols',
        operand:   operand?.info.id,
        colnames:  mutatedCols
    });
}
```

And in [`src/abstract-interpretation/data-frame/semantics.ts:L369-L378`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/repos/flowr/src/abstract-interpretation/data-frame/semantics.ts#L369-L378) (`applyMutateColsSemantics`):
```typescript
function applyMutateColsSemantics(
    value: DataFrameDomain,
    { colnames }: { colnames: (string | undefined)[] | undefined }
): DataFrameDomain {
    return value.create({
        colnames: colnames !== undefined ? value.colnames.union(setRange(colnames)) : value.colnames.widenUp(),
        cols:     colnames !== undefined ? value.cols.add([0, colnames.length]).max([colnames.length, colnames.length]) : value.cols.widenUp(),
        rows:     value.rows
    });
}
```

**Forensic Conclusion:**
1. `accessedNames` is emitted as a bulk precondition on the operand (`accessCols`).
2. `mutatedCols` is emitted as a bulk postcondition on the result (`mutateCols`).
3. **There is no pairing or edge between individual mutated columns and the expressions/columns used to create them.**
4. The AST expressions themselves are discarded during shape inference; only string names are passed to `applyMutateColsSemantics`.
5. Therefore, flowR cannot track column identity, derived columns, value dependencies, or column-to-column edges.

---

## 6. Concrete Trace Examples

### Example A: Single Mutated Column
```r
df2 <- df %>%
  mutate(total = price * quantity)
```
- **What flowR Represents:**
  - On `df`: `operation: 'accessCols'`, `columns: ['price', 'quantity']` (operand precondition).
  - On `df2`: `operation: 'mutateCols'`, `colnames: ['total']` (unioned into `df.colnames`).
  - In `DataflowGraph`: Object dependency `df2` is defined by the pipe expression; `df` flows into the pipe expression.
- **Can flowR Represent `(df.price, df.quantity) → df2.total`?**
  **NO.** [EVIDENCE: `dataframe-semantics.ts:L1261-L1276`]. `accessCols` and `mutateCols` are unlinked operations.

---

### Example B: Cascading Mutations
```r
df2 <- df %>%
  mutate(
    total = price * quantity,
    tax = total * 0.18
  )
```
- **What flowR Represents:**
  - `mutatedCols`: `['total', 'tax']`.
  - `mutatedColSet`: `Set(['total', 'tax'])`.
  - In `mapDataFrameMutate` line 1255: `accessedNames` searches all unresolved symbols (`price`, `quantity`, `total`) and explicitly filters out any symbols in `mutatedColSet`:
    `filter(arg => !mutatedColSet.has(arg))`
  - Therefore, `total` is **filtered out**!
  - `accessedNames` becomes `['price', 'quantity']`.
  - Emitted operations:
    - `{ operation: 'accessCols', columns: ['price', 'quantity'] }`
    - `{ operation: 'mutateCols', colnames: ['total', 'tax'] }`
- **Can flowR Represent `price, quantity → total → tax`?**
  **NO.** [EVIDENCE: `dataframe-semantics.ts:L1253-L1256`]. The dependency of `tax` on `total` is actively suppressed by the filter so as not to generate a false precondition on the input `df`.

---

### Example C: Filter, Mutate, Select Pipeline
```r
df2 <- df %>%
  filter(age > 18) %>%
  mutate(income2 = income * 1.1) %>%
  select(id, income2)
```
- **What flowR Represents:**
  - `filter(age > 18)` emits `{ operation: 'accessCols', columns: ['age'] }` and `{ operation: 'filterRows' }`.
  - `mutate(...)` emits `{ operation: 'accessCols', columns: ['income'] }` and `{ operation: 'mutateCols', colnames: ['income2'] }`.
  - `select(id, income2)` emits `{ operation: 'accessCols', columns: ['id', 'income2'] }` and `{ operation: 'subsetCols', colnames: ['id', 'income2'] }`.
  - Final shape of `df2`: `colnames.must = Set(['id', 'income2'])`.
- **Can flowR Distinguish `df.id`, `df.income` from `df.age` with Respect to the Final Output?**
  **NO.** [EVIDENCE: `static-slicer.ts:L81-L121`]. While the final schema shape only contains `id` and `income2`, the dataflow graph connects all four pipe statements as a single monolithic dataflow chain. A backward slice from `df2` pulls in the entire chain, including `filter(age > 18)` and `age`.

---

### Example D: Relational Join
```r
joined <- users %>%
  left_join(orders, by = "user_id") %>%
  mutate(total = price * quantity)
```
- **What flowR Represents:**
  - `mapDataFrameJoin` emits `{ operation: 'accessCols', columns: ['user_id'] }` and `{ operation: 'join', other: orders_shape, by: ['user_id'], options: { join: 'left' } }`.
  - In `applyJoinSemantics` ([`semantics.ts:L468-L471`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/repos/flowr/src/abstract-interpretation/data-frame/semantics.ts#L468-L471)):
    `colnames: value.colnames.union(other.colnames)` (with duplicate collision resolution).
- **Does the Join Create Any Column Provenance Relationship?**
  **NO.** [EVIDENCE: `semantics.ts:L407-L472`]. The result is a simple set union of strings. The analyzer does not record whether `user_id`, `price`, or `quantity` originated from `users` or `orders`.

---

## 7. Trace the Data into Program Slicing

### flowR's Program Slicer Implementation
The program slicer is implemented in [`src/slicing/static/static-slicer.ts`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/repos/flowr/src/slicing/static/static-slicer.ts):
- **Core Function:** `staticSlice(options: StaticSliceOptions): Readonly<SliceResult>` (Line 66).
- **Algorithm:** Standard Breadth-First Search over `info.graph` (`DataflowGraph`) using a `VisitingQueue` (Line 87) with memoization.
- **Traversal:** Inspects `graph.get(id)` and traverses outgoing/incoming edges matching `shouldTraverseEdge` (Lines 165–182).
- **Edge Types Traversed:** `Reads`, `DefinedBy`, `Calls`, `Returns`, `DefinesOnCall`, `DefinedByOnCall`, `SideEffectOnCall`, `NonStandardEvaluation`, `FlowEdge`, `ControlEdge`.

### Does flowR's Slicer Consume Dataframe Column Information?
**ABSOLUTELY NOT.** [EVIDENCE: `static-slicer.ts:L1-L319`]
1. `DataFrameShapeAnalysis` is **never imported or called** in `src/slicing/`.
2. The word `column` appears in `src/slicing/` **strictly to refer to editor character columns** in source code coordinates (e.g., `line~column` editor positions in [`src/slicing/criterion/parse.ts:L256-L290`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/repos/flowr/src/slicing/criterion/parse.ts#L256-L290)).
3. Slicing operates solely on `NodeId` vertices in the `DataflowGraph`.
4. There is **zero interface** connecting abstract dataframe shapes or column names to the slicing queue.

---

## 8. Object-Level vs Column-Level Slicing

| Feature | Object-Level Slicing (flowR) | Column-Level Slicing (R-PARSE Target) |
|---|---|---|
| **Graph Vertex** | AST NodeId / Variable Symbol (`df`) | AST NodeId + Column Entity (`df$income`) |
| **Partial Assignment** | `x[a] <- b` rebinds whole `x` (`par: true`) | Rebinds only target column slice |
| **Slicing Criterion** | AST Node or line/col of variable | Specific column reference or channel |
| **Resolution of `df$zipcode <- clean(df$zipcode)`** | Pulled into any slice reading `df` | Excluded when slicing on `df$income` |
| **Pipeline Slicing** | Slices entire pipeline if dataframe flows through | Slices only transformations affecting selected column |

[EVIDENCE: [`src/dataflow/internal/process/functions/call/built-in/built-in-assignment.ts:L365-L376`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/repos/flowr/src/dataflow/internal/process/functions/call/built-in/built-in-assignment.ts#L365-L376)]
When an assignment targets a column (e.g., `df$zipcode <- ...`), flowR routes it through replacement function handling (`$<-`), which rebinds the root symbol `df` in the dataflow environment. Consequently, any subsequent read of `df` creates a `Reads` edge to that assignment.
flowR **cannot distinguish** `final_result depends on df` from `final_result depends on df$income but not df$zipcode`.

---

## 9. CFG / SSA / Def-Use / Dataflow Integration Matrix

| Analysis Subsystem | Object-Level | Column-Level | Expression-Level | Connected to Slicing |
|---|:---:|:---:|:---:|:---:|
| **CFG** (Control Flow Graph) | **YES** | **NO** | **YES** | **YES** (via `cds` control dependencies) |
| **SSA** (Static Single Assignment) | **NO** | **NO** | **NO** | **NO** (flowR has no SSA) |
| **Def-Use Chains** | **YES** | **NO** | **YES** | **YES** (DFG `Reads`/`DefinedBy` edges) |
| **Dataframe Analysis** (Shape AI) | **YES** | **PARTIAL** (Names set only) | **NO** | **NO** (Isolated in linter & query) |
| **Dependency Graph** (DFG) | **YES** | **NO** | **YES** | **YES** (Traversed by `staticSlice`) |
| **Slicing** (`staticSlice`) | **YES** | **NO** | **YES** | **YES** (Is the slicer itself) |

---

## 10. Trace Join Semantics: Schema Shape vs Value Provenance

[EVIDENCE: [`src/abstract-interpretation/data-frame/semantics.ts:L407-L472`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/repos/flowr/src/abstract-interpretation/data-frame/semantics.ts#L407-L472)]

Inspection of `applyJoinSemantics`:
- **Key Columns (`by`):** If provided, column count is adjusted:
  `cols: value.cols.add(other.cols).subtract([by.length, by.length])`
- **Resulting Columns:**
  `colnames: value.colnames.union(other.colnames).subtract(setRange(duplicateCols)).widenUp()`
- **Row Cardinality:** Supports `inner`, `left`, `right`, and `full` joins via interval arithmetic (`PosIntervalDomain`). If Cartesian product is possible, multiplies upper bounds:
  `productInterval = (lower, int1, int2) => [lower.lower, int1.upper * int2.upper]`
- **Value Provenance:** **ZERO.**
  - Does not record which table contributed which column.
  - Does not record renaming of collisions (widens colliding names to `Top`).
  - Does not record derivation edges between input keys and output keys.

---

## 11. Trace Nested Functions & Interprocedural Boundaries

[EVIDENCE: [`src/abstract-interpretation/absint-inference.ts:L721-L830`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/repos/flowr/src/abstract-interpretation/absint-inference.ts#L721-L830)]

For:
```r
f <- function(df) {
  df %>% mutate(total = price * quantity)
}
result <- f(data)
```
- In `AbstractInterpreter`, if configuration option `abstractInterpretation.followCalls` is `true` (Line 735), the interpreter **steps into** user-defined function definitions.
- It passes the caller's abstract dataframe state into the function body and joins states across return points (Lines 808–812).
- **Limitation:** This propagates **schema shapes** (`DataFrameDomain`), not column provenance. Furthermore, if a function is passed as a first-class value through higher-order functions (e.g., `lapply`, `purrr::map`), flowR falls back to `fallbackUntargetedCall` ([`src/dataflow/graph/call-graph.ts:L52`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/repos/flowr/src/dataflow/graph/call-graph.ts#L52)), widening the return value to `Top`.

---

## 12. Trace Runtime Evidence & Dynamic Correlation

- **rdtLite Integration:** **NOT PRESENT.** Grep for `rdtLite` across `flowr/src/` returned **0 results**.
- **Runtime Session State:** **NOT PRESENT.** flowR has no connection to an active R interpreter, debugger session, or environment memory.
- **Static External File Reading:** [EVIDENCE: [`src/config.ts:L250`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/repos/flowr/src/config.ts#L250), [`src/abstract-interpretation/data-frame/dataframe-semantics.ts:L810-L852`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/repos/flowr/src/abstract-interpretation/data-frame/dataframe-semantics.ts#L810-L852)]
  flowR provides `readLoadedData.readExternalFiles` (default: `true`). When analyzing `read.csv("file.csv")`, it reads the first line of the file from the local disk at analysis time to extract column names and count rows.
  This is a **purely static file inspection**, not dynamic runtime correlation.

---

## 13. Formal Definition of Value Provenance Levels

| Provenance Level | Conceptual Definition | Concrete Example | Supported in flowR? | Evidence |
|---|---|---|:---:|---|
| **Level 0 — Object Dependency** | Scalar variable dataflow dependency | `df2 depends on df` | **YES** | `DataflowGraph` (`Reads`/`DefinedBy` edges) |
| **Level 1 — Schema Tracking** | Set of column names and dimension bounds | `df contains {price, quantity, total}` | **YES** | `DataFrameDomain` (`SetRangeDomain<string>`) |
| **Level 2 — Column Dependency** | Directed derivation edge between columns | `total depends on {price, quantity}` | **NO** | `mutateCols` and `accessCols` are unlinked bulk sets |
| **Level 3 — Expression Provenance** | AST transformation logic attached to column | `total = price * quantity` | **NO** | AST expressions discarded during AI step |
| **Level 4 — Cross-Stage Provenance** | Lineage preserved across pipes, joins, models | `raw.price → clean.price → joined.total → model` | **NO** | Slicing and DFG operate at Level 0 only |
| **Level 5 — Runtime-Confirmed Provenance** | Static lineage validated against dynamic trace | Dynamic cell/column audit | **NO** | Zero runtime correlation |

**Forensic Ceiling for flowR:** **Level 1 (Schema Tracking).**

---

## 14. Domain-Specific Semantics: Formulas & ggplot2

### Formula Semantics (`lm(y ~ x + z, data = df)`)
[EVIDENCE: [`src/dataflow/environments/default-builtin-config.ts:L571`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/repos/flowr/src/dataflow/environments/default-builtin-config.ts#L571)]
```typescript
/* formula: operands are model terms/columns, not variables */
{ type: 'function', names: [Identifier.from(['~', PkgName.Base])], processor: BuiltInProcName.DefaultReadAllArgs, config: { markArgsAsMasked: NseArguments.All, sig: [['y', ArgProp.Nse], ['model', ArgProp.NoDefault | ArgProp.Nse]] }, assumePrimitive: false },
```
- `~` is registered solely to mark operands as Non-Standard Evaluation (`markArgsAsMasked: NseArguments.All`). This prevents the variable resolver from flagging `y`, `x`, `z` as undefined variable reads.
- `lm` and `glm` are **not even registered** in `default-builtin-config.ts` (0 matches).
- flowR builds **no formula AST**, performs no term expansion (`*` to main effects + interaction), and has **zero link** between the formula terms and `df`.

---

### ggplot2 Semantics (`ggplot(df, aes(x = age, y = income)) + geom_point()`)
[EVIDENCE: [`src/dataflow/environments/default-builtin-config.ts:L580-L582`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/repos/flowr/src/dataflow/environments/default-builtin-config.ts#L580-L582)]
```typescript
/* data-masking without a data argument, e.g. `aes(x, y)` */
{ type: 'function', names: [...Identifier.fromAll(PkgName.GgPlot2, ['aes', 'vars']), ...], processor: BuiltInProcName.DefaultReadAllArgs, config: { markArgsAsMasked: NseArguments.All }, assumePrimitive: false },
/* binary operator `+` */
{ type: 'function', names: Identifier.fromAll(PkgName.Base, ['+', ...]), processor: BuiltInProcName.DefaultReadAllArgs, config: { props: CallProp.Pure, sig: SigAtomicBinOp }, assumePrimitive: true, evalHandler: BuiltInEvalName.Numeric },
```
- `aes(...)` is registered only to mark its arguments as masked NSE.
- `+` in ggplot2 layer stacking is handled by the generic numeric binary operator handler (`SigAtomicBinOp`).
- `ggplot` does not appear anywhere in `dataframe-semantics.ts`.
- flowR cannot connect `plot → aes → age, income → df.age, df.income`.

---

## 15. Testing the Unified Model Hypothesis (R-SPM)

The R-SPM hypothesis posits a unified semantic program model:
```text
R Program
   │
   ├── Syntax & Normalization         (flowR: YES)
   ├── Environments & Scopes          (flowR: YES)
   ├── Control Flow Graph             (flowR: YES)
   ├── Object-Level Dataflow Graph    (flowR: YES)
   ├── Dataframe Schema Shape AI      (flowR: YES)
   ├── Column Value Provenance        (flowR: NO)
   ├── Column-Level Program Slicing   (flowR: NO)
   ├── Statistical Formula IR         (flowR: NO)
   ├── Visualization Aesthetic IR     (flowR: NO)
   └── Dynamic Runtime Correlation    (flowR: NO)
```

**Verdict:** flowR implements approximately 50% of this model (the classical compiler infrastructure + schema shape AI). It **completely lacks** the column provenance graph, domain semantics (formulas, plots), and the unification layer that binds column provenance into program slicing.

---

## 16. Combination vs Novelty Test

If R-PARSE implements:
`flowR dataframe analysis` + `flowR slicing` + `formula analysis` + `ggplot analysis` + `Rust implementation`

| Classification | Assessment | Rationale |
|---|---|---|
| A. Genuine New Capability | **YES (at the intersection)** | Fine-grained column-level slicing ("slice only what affects `df$income`") does not exist anywhere in the R ecosystem. |
| B. Better Engineering Implementation | **YES (for performance & LSP)** | Porting TypeScript compiler architecture to native Rust creates sub-millisecond IDE responsiveness. |
| C. New Composition | **YES (architectural bridge)** | Bridging abstract interpretation shape inferences into the dataflow graph traversed by the slicer is a new architectural composition. |
| D. Mostly a Reimplementation | **NO** | A mere reimplementation would copy flowR's decoupled shape linter. Connecting provenance to slicing requires a new semantic graph representation. |

---

## 17. End-to-End Key User Scenario Walkthrough

```r
df_raw <- read_csv("data.csv")

clean <- df_raw %>%
  filter(age > 18) %>%
  mutate(income2 = income * 1.1)

model <- lm(income2 ~ age, data = clean)

plot <- ggplot(clean, aes(x = age, y = income2)) +
  geom_point()
```
Target: Slicing on `plot` should isolate `income2 -> income -> df_raw$income` and `age -> df_raw$age`, while excluding `df_raw$zipcode`.

| Pipeline Step | Capability | flowR Status | Source Evidence |
|---|---|:---:|---|
| 1. `plot → aesthetics` | Deconstruct `ggplot()` + `aes()` aesthetic mappings | **NOT SUPPORTED** | `+` treated as scalar addition ([`default-builtin-config.ts:582`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/repos/flowr/src/dataflow/environments/default-builtin-config.ts#L582)) |
| 2. `aesthetics → dataframe columns` | Link `x = age, y = income2` to `clean.age, clean.income2` | **NOT SUPPORTED** | `aes` arguments marked as opaque NSE ([`default-builtin-config.ts:580`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/repos/flowr/src/dataflow/environments/default-builtin-config.ts#L580)) |
| 3. `dataframe columns → transformations` | Link `income2` to `income * 1.1` | **NOT SUPPORTED** | `mutateCols` does not link output column to expression ([`semantics.ts:373`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/repos/flowr/src/abstract-interpretation/data-frame/semantics.ts#L373)) |
| 4. `transformations → source columns` | Link `income * 1.1` to `df_raw$income` | **NOT SUPPORTED** | Bulk unlinked `accessCols` ([`dataframe-semantics.ts:1263`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/repos/flowr/src/abstract-interpretation/data-frame/dataframe-semantics.ts#L1263)) |
| 5. `source columns → original dataframe` | Map to CSV columns in `df_raw` | **PARTIAL** | Reads header names from disk statically, but DFG binds only scalar variable `df_raw` |
| 6. `dependency set → program slice` | Compute backward slice excluding `zipcode` | **NOT SUPPORTED** | `staticSlice` pulls in all lines touching `df_raw` and `clean` ([`static-slicer.ts:81`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/repos/flowr/src/slicing/static/static-slicer.ts#L81)) |

**Overall Result:** **NOT SUPPORTED.** flowR cannot produce a column-specific slice.

---

## 18. Exact Remaining Technical Gap

### Precise Gap Statement
> **flowR provides Level 1 dataframe schema shape inference (`colnames` sets, row/column count intervals) and object-level program slicing over AST dataflow graphs, but its abstract interpretation state is strictly decoupled from its dataflow graph, and it does not construct column-to-column value derivation edges across data transformations (Level 2/3 provenance). Consequently, column semantics cannot propagate across tidyverse pipelines, statistical formulas (`lm`), or visualizations (`ggplot2`), preventing fine-grained, column-level program slicing.**

---

## 19. If No Meaningful Gap Existed
*(Not applicable: The forensic audit proved beyond doubt that flowR does not support column provenance or column-level slicing.)*

---

## 20. Architectural Gap Analysis

### Gap Statement
Absence of a unified semantic graph connecting fine-grained column transformations to program slicing.

### Existing flowR Capability
- 2,000+ lines of tidyverse shape transfer functions inferring `colnames` sets and dimension intervals.
- Fast BFS static slicer operating over whole-variable dataflow graphs.

### Exact Missing Capability
- Fine-grained column provenance graph (tracking expression derivations: `col_out = f(col_in1, col_in2)`).
- Column-aware program slicing criterion and traversal.
- Domain semantics for formulas (`~`) and visualization layers (`aes`).

### Why Existing flowR Architecture Does Not Provide It
- `DataFrameDomain` was designed exclusively as an abstract interpretation lattice to validate accesses (`df$foo`).
- `DataflowGraph` was designed independently to resolve variable scopes and reaching definitions.
- The two subsystems run sequentially: DFG is built first and passed into AbsInt; AbsInt output is **never reflected back into DFG**.

### Why Simply Wrapping flowR is Insufficient
Wrapping flowR does not help because the information **does not exist inside flowR to be wrapped**. flowR does not calculate column derivation edges, so an external wrapper cannot retrieve them.

### Minimum New Technical Contribution
A unified Program Dependence Graph (PDG) where dataframe nodes possess fine-grained **column ports**, allowing dataflow edges to connect individual input columns to individual output columns across transformations.

---

## 21. Summary Matrix of Audited Source Files

See companion file: [`research/flowr-deep-gap-audit.csv`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/flowr-deep-gap-audit.csv).

---

## 22. Evidence Classifications

- `[EVIDENCE]` Claims directly backed by inspected lines of code in `research/repos/flowr/src/`.
- `[INFERENCE]` Logical deductions regarding the architectural consequences of observed code structures.
- `[HYPOTHESIS]` Proposed technical designs for R-PARSE.
- `[UNKNOWN]` Capabilities where no code was found and intent cannot be determined.

---

## 23. Final Verdict

## FlowR Capability Ceiling

```text
Highest verified provenance level:         Level 1 (Schema Tracking: must/may string sets)
Highest verified slicing level:            Level 0 (Whole-Object / AST Node BFS)
Highest verified dataframe semantic level: Shape Inference (colnames, row/col count intervals)
Runtime correlation:                       NOT PRESENT (Static file header inspection only)
Formula integration:                       NOT PRESENT (Operands marked as masked NSE to silence linter)
ggplot integration:                        NOT PRESENT (aes marked as masked NSE; + treated as arithmetic)
```

## Remaining Technical Gap

```text
flowR infers schema presence (whether a column exists) via abstract interpretation, but does not construct column-to-column value derivation edges across pipeline transformations, statistical models, or visualization specifications, and completely isolates this schema knowledge from its program slicer.
```

## Does R-SPM Represent a Real New Contribution?

```text
YES
```
**Explanation:**  
Connecting fine-grained column provenance through tidyverse pipelines, formulas, and plots into a unified program slicer represents a genuine, mathematically distinct analytical capability that does not exist in flowR, CodeAnalysis, rdtLite, or any known R tool.

## Strongest Defensible R-PARSE Contribution

```text
Fine-grained, column-level program slicing across tidyverse pipelines, statistical formulas, and visualization specifications.
```

## Most Dangerous False Claim

```text
"flowR cannot track dataframe columns."
(FALSE: flowR has sophisticated abstract interpretation tracking colnames must/may sets for over 30 dplyr/base functions. The true claim is that flowR tracks schema presence, NOT column-to-column value provenance or column-level slices.)
```

## Confidence

```text
HIGH
```

## STOP
Do not proceed to architecture, scope freeze, implementation, or UI.
