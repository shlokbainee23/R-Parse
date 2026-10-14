# R-PARSE — Security Architecture & Boundaries

## Threat Model
R scripts in data science repositories frequently execute untrusted code (e.g., executing code submitted in web apps or automated pipelines). R analysis engines must prevent arbitrary code execution during static analysis.

## Core Security Invariants
1. **Zero Execution in Static Mode:**
   - Static analysis passes (parsing, semantic lowering, CFG traversal, slicing) execute strictly within Rust.
   - Tier 1 operations NEVER spawn processes, invoke `system()`, or call GNU R.
2. **Memory Safety & Robust Parsing:**
   - Tree-sitter R parser and CST traversals are memory-safe.
   - Malformed, deeply nested, or adversarial R syntax must terminate gracefully without stack overflow or panic.
3. **Runtime Bridge Containment (Tier 2):**
   - The GNU R runtime bridge is strictly opt-in and controlled.
   - Any runtime execution must run in an isolated subprocess with strict timeouts and explicit user consent.
   - Runtime bridges must never accept unvalidated shell execution strings.
4. **CLI Execution Boundaries (Phase 7):**
   - **No Implicit Execution:** The CLI defaults strictly to `--mode static`. Static mode contains no code paths to spawn child processes or invoke GNU R.
   - **Direct Process Invocation:** When runtime mode is explicitly enabled (`assisted` or `execute`), process spawning uses direct binary execution (`Command::new`) with vector arguments. Shell command string interpolation and cmd/sh shells are strictly prohibited.
   - **No Sandbox Exaggeration:** GNU R execution in runtime modes runs on the host system with calling user privileges. The CLI explicitly does not claim GNU R execution is sandboxed from host filesystem or network access beyond process timeouts and stdio isolation.
   - **Resource Limits:** Input files, diagnostic logs, and subprocess executions are bounded by timeouts and deterministic traversal depths to prevent resource exhaustion.
