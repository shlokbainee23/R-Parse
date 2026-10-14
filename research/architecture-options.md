# Architecture Options for R-PARSE

This document outlines and evaluates candidate architectural designs for R-PARSE.

> **CRITICAL REMINDER:** DO NOT choose the final architecture yet. This document presents trade-off analyses to inform future design decisions.

---

## 1. Architectural Candidates

```text
┌────────────────────────────────────────────────────────────────────────┐
│                        ARCHITECTURAL CANDIDATES                        │
├──────────┬──────────────────────┬──────────────────────────────────────┤
│ Option   │ Core Implementation  │ Distribution / Interfaces            │
├──────────┼──────────────────────┼──────────────────────────────────────┤
│ Option A │ Pure Rust Standalone │ Single Native Binary, LSP, WASM, FFI │
│ Option B │ TypeScript / Node.js │ npm package, VS Code extension       │
│ Option C │ Pure R + C FFI       │ CRAN Package                         │
│ Option D │ Hybrid Rust + R FFI  │ Native Core + Optional R Runtime IPC │
└──────────┴──────────────────────┴──────────────────────────────────────┘
```

---

## 2. In-Depth Evaluation of Options

### Option A: Pure Rust Standalone Engine (The "Ruff for R" Architecture)
- **Design:**
  - Parser: Extended `tree-sitter-r` embedded in Rust via `tree-sitter` crate.
  - IR Layer: Strongly-typed Rust AST + Control Flow Graph (Petgraph / custom arena).
  - Analysis Passes: Intra/inter-procedural dataflow, reaching definitions, program slicing, rule engine.
  - Interfaces:
    - CLI executable (`r-parse check`, `r-parse slice`).
    - Built-in Language Server Protocol (LSP) daemon.
    - WASM target for browser-based IDEs and documentation sites.
    - Optional C FFI / R package bindings via `extendr`.
- **Strengths:**
  - Blazing performance (<10ms CLI cold start; microsecond incremental reparsing).
  - True standalone zero-dependency distribution.
  - Memory safety with zero garbage collection pauses.
- **Trade-offs / Risks:**
  - High initial development effort to build typed AST and analysis passes in Rust.
  - Cannot evaluate R expressions directly without FFI to `libR`.

### Option B: TypeScript / Node.js Engine (The "Extended flowR" Architecture)
- **Design:**
  - Fork or extend `flowR` directly.
  - Implement missing tidyverse, statistical formula, and visualization passes in TypeScript.
  - Package as an npm module and VS Code language extension.
- **Strengths:**
  - Lowest time-to-prototype by leveraging flowR's existing dataflow and slicing engine.
  - Natural fit for web-based IDEs and VS Code extension architecture.
- **Trade-offs / Risks:**
  - Heavy memory footprint and Node.js runtime dependency.
  - Unpopular for CLI/pre-commit hooks in data science environments where Node.js is not standard.
  - Limited adoption within the native CRAN community.

### Option C: Pure R + C Extension (The "CRAN-First" Architecture)
- **Design:**
  - Compile Tree-sitter C parser into an R package.
  - Write AST normalization, CFG, and analysis algorithms in pure R.
- **Strengths:**
  - Seamless CRAN distribution and 100% natural integration with existing R workflows.
  - Can directly call R functions and inspect loaded namespace environments.
- **Trade-offs / Risks:**
  - Slow analysis passes; R interpreter overhead for graph traversal.
  - Cannot run without an active GNU R installation.
  - Fails the core requirement of providing a fast, standalone language tool.

### Option D: Hybrid Rust Core with Optional R IPC Bridge (The "Two-Tier" Architecture)
- **Design:**
  - **Tier 1 (Rust):** High-speed standalone engine performing 95% of tasks (parsing, CST, AST, CFG, SSA, lints, slicing) with zero R dependencies.
  - **Tier 2 (R IPC):** Optional sidecar daemon that launches when GNU R is available, providing namespace introspection and dynamic type grounding.
- **Strengths:**
  - Combines the absolute speed and portability of Rust with the dynamic fidelity of GNU R when available.
  - Graceful degradation: works everywhere standalone, becomes smarter when R is installed.
- **Trade-offs / Risks:**
  - Requires maintaining an IPC protocol and synchronization between two processes.

---

## 3. Comparative Architecture Matrix

| Dimension | Option A (Pure Rust) | Option B (TypeScript) | Option C (Pure R) | Option D (Hybrid Rust+R) |
| :--- | :---: | :---: | :---: | :---: |
| **Execution Speed** | ★★★★★ | ★★★☆☆ | ★★☆☆☆ | ★★★★★ |
| **Memory Efficiency** | ★★★★★ | ★★☆☆☆ | ★★☆☆☆ | ★★★★☆ |
| **Standalone Portability** | ★★★★★ | ★★☆☆☆ | ★☆☆☆☆ | ★★★★★ |
| **Dynamic R Fidelity** | ★★☆☆☆ | ★★☆☆☆ | ★★★★★ | ★★★★★ |
| **Implementation Effort**| High | Moderate | Moderate | High |
