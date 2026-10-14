# rdtLite — Repository Analysis

## 1. Project Identity

- **Name:** rdtLite (R Data Tracker Lite)
- **Repository:** https://github.com/End-to-end-provenance/rdtLite
- **Organization:** End-to-end-provenance (Harvard University / Mount Holyoke College / NSF-funded)
- **Language:** R (100%)
- **License:** GPL-3.0
- **Status:** Active research software (latest updates supporting R >= 3.5.0)
- **Size:** 20 R source files in `R/`, ~330 KB total R code
- **Dependencies:** curl, digest, jsonlite, stringr, XML

---

## 2. Purpose & Problem Solved

rdtLite collects fine-grained computational provenance during R script execution or interactive console sessions. It records:
1. The execution steps (procedure nodes).
2. The intermediate data values created, read, or modified (data nodes).
3. The dataflow derivations and control dependencies between steps (edges).
4. File I/O events and environment changes.

The output is written as a W3C PROV-compliant JSON document (`prov.json`), creating an execution-grounded Data Derivation Graph (DDG).

---

## 3. Architecture & Execution Flow

```text
R Script (.R)
      │
      ▼
prov.run() [API_rdtLite.R]
      │
      ├─► parse(file, keep.source=TRUE) [API_core.R line 524]
      │
      ▼
Statement Loop [.ddg.source in API_core.R]
      │
      ├─► Static AST Inspection [.ddg.find.var.uses / .ddg.find.var.sets in DDGStatement.R]
      │     └─► Walks R S-expression calls (`obj[[1]]`, `obj[[2]]`, etc.)
      │
      ├─► Runtime Evaluation & Interception [eval() with environment tracing]
      │     ├─► Captures variable values & digest hashes [DataNodes.R / DDGHash.R]
      │     ├─► Intercepts graphics / file operations [IOTrace.R]
      │     └─► Creates Procedure Nodes & Data Nodes [ProcNodes.R, DataNodes.R]
      │
      └─► Graph Construction & JSON Serialization [Edges.R, OutputJSON.R]
            └─► W3C PROV-JSON (`prov.json`)
```

---

## 4. Parsing Mechanism

rdtLite does **not** implement a custom parser. It relies exclusively on GNU R's native C parser via the R API:
- `parse(file=file, n=-1, keep.source=TRUE)` (`API_core.R` line 524)
- Individual expressions are manipulated as R language objects (`call`, `name`, `expression`).
- Re-parsing of sub-expressions occurs when stringified expressions are inspected (`parse(text=var)`, `RDataTracker_core.R` line 661).

---

## 5. AST & IR Model

rdtLite operates on R's native S-expression (pairlist / language object) hierarchy:
- In `DDGStatement.R`, statement structure is inspected recursively:
  - Base cases: `is.atomic(obj)`, `is.name(obj)` (`line 495, 500`)
  - Function declarations: `.ddg.is.functiondecl(obj)` (`line 516`)
  - Member access: `obj[[1]] == "$"` (`line 521`)
  - Assignments: `.ddg.is.assign(obj)` (`line 534`) handling `<-`, `=`, `<<-`, and indexed assignments `a[1] <- 2` or `a[b] <- 3` (`lines 552-561`)
- Data Derivation Graph (DDG) IR:
  - **Procedure Nodes (`p1`, `p2`, ...):** Represent executed statements, functions, or blocks.
  - **Data Nodes (`d1`, `d2`, ...):** Represent variable states, snapshots, files, or URLs.
  - **Edges:**
    - `Data-Procedure (Data Input)`: Variable read by statement.
    - `Procedure-Data (Data Output)`: Variable modified or created by statement.
    - `Procedure-Procedure (Control Flow)`: Sequential execution order.

---

## 6. Static vs. Dynamic Analysis Capabilities

rdtLite is a **hybrid dynamic provenance collector** rather than a pure static analyzer:
- **Variable use extraction (`DDGStatement.R` lines 488-660):** Recursively traverses statement parse trees to discover referenced variable names before execution.
- **Variable def extraction (`DDGStatement.R` lines 661-750):** Identifies lvalues in assignments.
- **NSE Handling:** Struggles with Non-Standard Evaluation (NSE); handles basic formulas and tidyverse patterns through heuristically skipping unrecognized idioms or recording raw string names.
- **No static path sensitivity or slicing without execution:** If a branch is not executed at runtime, it does not appear in the provenance graph.

---

## 7. Statistical & Visualization Understanding

- **Visualization detection:** `IOTrace.R` hooks into device drivers (`grDevices::pdf`, `png`, `dev.off()`) to detect plot generation and attach graphic files to output nodes.
- **Statistical semantics:** None. rdtLite does not understand formulas (`y ~ x + z`), model families, convergence, or degrees of freedom. It treats them as arbitrary expression objects.

---

## 8. Strengths

1. **Empirical Ground Truth:** Provides exact runtime execution data derivation graphs, showing the actual sequence of values, types, and files created during a run.
2. **W3C PROV-JSON standard compliance:** Compatible with standard graph viewers (ProvToolbox, DDG explorer).
3. **Snapshotting:** Can snapshot data frames and complex objects up to configurable memory limits (`max.snapshot.size`).

---

## 9. Limitations & Failure Modes

1. **Requires Full Execution:** Cannot analyze scripts with missing packages, network calls, missing datasets, or infinite loops.
2. **Runtime Overhead:** Instrumenting every assignment, hashing objects, and capturing snapshots adds substantial performance overhead (often 2x–10x slowdown on heavy scripts).
3. **NSE Blindspots:** In tidyverse operations like `df %>% filter(age > 30)`, `age` is detected syntactically as a variable use in the global scope if not guarded, causing false data-dependency edges to non-existent global variables.
4. **No Type Inference or Abstract Interpretation:** Only records concrete runtime types observed.

---

## 10. Relevance to R-PARSE

1. **Validation Oracle / Ground Truth:** rdtLite provides the ideal empirical benchmark to validate R-PARSE's static dataflow graphs against real-world R executions.
2. **Dynamic-Static Correlation:** Demonstrates how static variable-use extraction correlates (and diverges) with dynamic runtime bindings.
