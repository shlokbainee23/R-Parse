# R-PARSE — Runtime Bridge & Execution Contract

## Overview
R-PARSE adopts Option D ([ADR-001](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/ADR-001-architecture.md)): a hybrid architecture where Tier 1 (Rust Core) conducts all static analysis, and Tier 2 (GNU R Runtime Bridge) provides optional empirical evidence.

```text
                 R-PARSE
                    │
        ┌───────────┴───────────┐
        ▼                       ▼
    RUST CORE               GNU R RUNTIME
    (Tier 1)                  (Tier 2)
   [Autonomous]              [Optional]
        │                       │
   Parsing / CST             Execution
   Semantic IR               Inspection
   Scope Analysis            Runtime Evidence
   CFG / Dataflow
   Provenance Engine
   Slicing Engine
        │                       │
        └────── RUNTIME BRIDGE ─┘
```

## Fundamental Architectural Invariants
1. **Static Analysis Remains Authoritative:** Static models and program dependence graphs remain 100% self-contained, deterministic, and offline.
2. **Never Execute in Static Mode:** If an analysis command or query is invoked in `AnalysisMode::Static`, no GNU R subprocess may be launched under any circumstance.
3. **Explicit Uncertainty Distinction:**
   - Static analysis records `StaticKnowledge::KnownStatic` or `StaticKnowledge::UnknownStatic`.
   - Runtime observations record `RuntimeEvidence::None` or `RuntimeEvidence::Observed`.
   - Runtime observation never overwrites, overrides, or fabricates static dependencies; it provides empirical run-specific evidence.
4. **No Security Sandbox Claim:** GNU R executes real code. The child process evaluation environment (`new.env(parent = globalenv())`) provides a controlled temporary runtime environment and isolated evaluation process, not an OS-level container or security sandbox.

## Protocol & Wire Format
The runtime bridge uses JSON-RPC 2.0 over standard I/O pipes with robust `Content-Length` framing and newline-delimited fallback:
- **Header:** `Content-Length: <n>\r\n\r\n`
- **Payload:** Structured JSON payload.
- **Operations:**
  - `evaluate`: Evaluates expression within the controlled evaluation environment, capturing stdout and value type.
  - `inspect_object`: Inspects symbol existence, type (`typeof`), class hierarchy (`class`), length, and dataframe status.
  - `inspect_dataframe`: Discovers column names, column storage types, S3/S4 classes, and row counts.
  - `exit`: Gracefully terminates the child process.

## Process Lifecycle & Timeout Management
- The child process runs `Rscript --vanilla <temp_worker.R>`.
- Operations are guarded by configurable timeouts (`timeout_ms`, default 2000 ms).
- On timeout, the child process is forcefully killed, avoiding hung or orphaned processes.
- The `RProcess` struct implements `Drop` to ensure child processes and temporary script files are always cleaned up.

## GNU R Discovery Precedence
1. Explicit path override in `RuntimeConfig`.
2. `RPARSE_R_PATH` environment variable.
3. `R_HOME` environment variable (`bin/Rscript`, `bin/x64/Rscript`).
4. System `PATH` search for `Rscript` / `Rscript.exe`.
5. Platform-specific known directories (e.g. `C:\Program Files\R\R-*\bin\Rscript.exe` on Windows, `/usr/bin/Rscript` on Linux, `/Library/Frameworks/R.framework/Resources/bin/Rscript` on macOS).
6. Clear typed `RuntimeError::ExecutableNotFound` if absent.

## Analysis Modes
| Mode | Runtime Invocation | Behavior |
|---|---|---|
| `STATIC` (Default) | Strictly Forbidden | 100% offline, deterministic, zero side-effects. |
| `ASSISTED` | Selective & Bounded | Bounded queries to GNU R to observe runtime schemas and refine dynamic boundaries. |
| `EXECUTE` | Explicit Full Execution | Full execution via GNU R to produce oracle traces and empirical evidence. |

## Security Boundary Limitations
- Code execution is real GNU R execution.
- While the evaluation environment prevents pollution of `.GlobalEnv`, the R process possesses standard user-level permissions.
- Process hygiene limits risk via non-elevated permissions, controlled working directory, explicit environment variables, bounded timeouts, and reliable process cleanup.
