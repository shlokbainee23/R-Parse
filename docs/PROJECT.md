# R-PARSE — Project Overview

## Authoritative Definition
**R-PARSE is a broad-compatibility R program analysis system that extends semantic program representation with fine-grained column provenance and uses that information to enable column-aware static program slicing.**

### Primary Technical Contribution
Fine-grained column provenance ($C_{in} \to C_{out}$) integrated with general R program dependency graphs and column-aware static backward program slicing.

### Positioning & Anti-Goals
R-PARSE is strictly **not**:
- A linter CLI replacement (such as `lintr` or `ruff`)
- A full R interpreter or native compiler (does not replace GNU R)
- A build workflow DAG scheduler (such as `targets`)
- A tool guaranteeing universal standalone executable reprex generation

## Phased Delivery Structure
1. **Phase 1:** Foundation + R Parsing + CST (`rparse-core`, `rparse-parser`) — ✅ Complete
2. **Phase 2:** Semantic IR + Normalization + Scope (`rparse-ir`, `rparse-scope`) — ✅ Complete
3. **Phase 3:** Control Flow Graph & Dataflow Analysis (`rparse-cfg`, `rparse-dataflow`) — ✅ Complete
4. **Phase 4:** DataFrame Semantics & Column Provenance Engine (`rparse-dataframe`) — ✅ Complete
5. **Phase 5:** Unified PDG & Column-Aware Program Slicing (`rparse-pdg`, `rparse-slicing`) — ✅ Complete
6. **Phase 6:** GNU R Runtime Bridge (`rparse-runtime`) — ✅ Complete
7. **Phase 7:** CLI Interface, Integration & Production Hardening (`rparse-cli`) — ✅ Complete

## Governing Documents
- Scope authority: [`research/scope-freeze.md`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/scope-freeze.md)
- Architecture decision: [`research/ADR-001-architecture.md`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/ADR-001-architecture.md)
- Technical specification: [`research/technical-specification.md`](file:///c:/Users/thimm/OneDrive/Desktop/Rparse/research/technical-specification.md)
