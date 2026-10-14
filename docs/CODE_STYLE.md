# R-PARSE — Code Style & Idioms

## General Principles
- **Production Rust:** Write robust, safe, idiomatic Rust targeting stable editions.
- **Panic Freedom:** Library crates (`rparse-core`, `rparse-parser`) must **NEVER panic** on invalid, malformed, or hostile inputs. All errors must be expressed via `Result` or `Diagnostic` records.
- **No `unwrap()` in Library Code:** Use `expect()` only when invariant proof is documented, or preferably pattern matching and `?` operator.
- **Tree-sitter Isolation:** Raw Tree-sitter C types (`TSNode`, `TSTree`, `TreeCursor`) must not leak outside `rparse-parser`. Consumers interact with high-level CST wrappers.

## Source Spans & Indexing Conventions
- **Byte Offsets:** `start_byte` and `end_byte` are 0-based byte offsets into UTF-8 source buffers, where `end_byte` is non-inclusive (`[start, end)`).
- **Line & Column Indices:**
  - `start_line` and `end_line` are **1-based** (line 1 is the first line of the file).
  - `start_col` and `end_col` are **0-based** UTF-8 character/byte column offsets matching Tree-sitter's point convention (`Point { row, column }`).
- **Span Struct:** Canonical `Span` in `rparse_core::span::Span`.

## Identifiers & Types
- **NodeId:** `NodeId(u32)` represents a dense, compact node identifier. Do not use raw 64-bit pointers or tree-sitter IDs.
- **Error Types:** Derive `thiserror::Error` for domain-specific errors. Implement standard `std::error::Error`.
- **Formatting & Linting:** Code must pass `cargo fmt --all` and `cargo clippy --workspace --all-targets --all-features`.
