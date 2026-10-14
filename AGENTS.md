# AGENTS.md — R-PARSE Engineering Guidance for AI Coding Agents

## Mandatory Orientation
Before modifying or generating code in this repository, you MUST read the following foundational documents in `docs/`:
1. [`docs/PROJECT.md`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/docs/PROJECT.md) — Mission, terminology, and core thesis.
2. [`docs/ARCHITECTURE.md`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/docs/ARCHITECTURE.md) — Two-tier hybrid architecture (Rust Core + GNU R Runtime Bridge).
3. [`docs/CODE_STYLE.md`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/docs/CODE_STYLE.md) — Rust idioms, naming, and error handling guidelines.
4. [`docs/SECURITY.md`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/docs/SECURITY.md) — Execution isolation and static analysis boundaries.
5. [`docs/RUNTIME.md`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/docs/RUNTIME.md) — Mandatory when working on runtime bridge or observation code.
6. [`docs/TESTING.md`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/docs/TESTING.md) — Mandatory when writing or maintaining tests.
7. Relevant files in [`research/`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/) before changing semantic architecture.

## Non-Negotiable Rules & Authority
- **Scope Authority:** [`research/scope-freeze.md`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/scope-freeze.md) is authoritative for project scope.
- **Architecture Authority:** [`research/ADR-001-architecture.md`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/ADR-001-architecture.md) is authoritative for system architecture (Option D: Hybrid Rust Core + GNU R Runtime Bridge).
- **Engineering Design Authority:** [`research/technical-specification.md`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/technical-specification.md) is authoritative for engineering design and type contracts.
- **Never silently expand scope:** Do not add linter rules, LSP features, or unsupported data frame operations.
- **Never invent R semantics:** Model real R semantics as validated by research. Ambiguities must not be guessed.
- **UNKNOWN must remain explicit:** Orthogonal uncertainty (`KNOWN_STATIC` vs `UNKNOWN_STATIC`, `NONE` vs `OBSERVED`) must be preserved; never fabricate dependencies or drop uncertain edges.
- **Runtime execution must NEVER occur in STATIC mode:** Tier 1 static analysis is 100% self-contained and offline.
- **Future layers must NOT be implemented:** Core Phases 1–7 are complete. Future interface layers (web UI, LSP, AI/LLM layers, compiler codegen) are strictly out of scope and must not be added to this repository.
