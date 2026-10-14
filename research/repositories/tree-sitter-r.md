# tree-sitter-r — Repository Analysis

## 1. Project Identity

- **Name:** tree-sitter-r
- **Repository:** https://github.com/r-lib/tree-sitter-r
- **Organization:** r-lib (RStudio/Posit ecosystem)
- **Language:** JavaScript (grammar), C (scanner/parser), Rust/Node bindings
- **License:** MIT
- **Status:** Active, well-maintained
- **Last meaningful activity:** Ongoing (2024-2025+)
- **Version:** Available as CRAN package `treesitter.r`, npm `@davisvaughan/tree-sitter-r`, Rust crate `tree-sitter-r`

## 2. Purpose

Provides an incremental, error-tolerant parser for R source code using the tree-sitter framework. Designed for IDE tooling (syntax highlighting, code folding, structural queries) rather than execution. Used by Positron IDE and VS Code R extensions.

## 3. Architecture

```text
R Source Code
     ↓
External Scanner (scanner.c) — handles newlines, raw strings, brackets, else
     ↓
Tree-sitter GLR Parser (generated from grammar.js)
     ↓
Concrete Syntax Tree (CST)
     ↓
Available via tree-sitter query API (S-expressions, .scm queries)
```

**Evidence:** `grammar.js` (787 lines) defines the full grammar. `src/scanner.c` (906 lines) implements an external scanner for context-sensitive tokens. `src/parser.c` (4MB, auto-generated) is the compiled parser.

## 4. Parsing

### Lexer/Tokenizer
- Uses tree-sitter's built-in lexer plus a custom **external scanner** (`src/scanner.c`)
- External scanner handles: newlines (significant in R), semicolons, raw strings (`r"(...)"`), `else` keyword (dangling else), all bracket types including `[[`/`]]`, and start marker
- **Evidence:** `scanner.c` lines 26-43 enumerate token types: START, NEWLINE, SEMICOLON, RAW_STRING_OPEN/CONTENT/CLOSE, ELSE, OPEN_PAREN, CLOSE_PAREN, OPEN_BRACE, CLOSE_BRACE, OPEN_BRACKET, CLOSE_BRACKET, OPEN_BRACKET2, CLOSE_BRACKET2

### Grammar
- **Format:** tree-sitter JavaScript DSL (`grammar.js`)
- **Grammar size:** 787 lines
- **Parser algorithm:** GLR (tree-sitter's parser generator)
- **Error recovery:** Built-in tree-sitter error recovery (produces ERROR nodes)

### Operator Precedence
Precisely follows R's `gram.y` precedence table. Documented extensively in comments (lines 1-57):
- COMMENT: -1
- BLOCK: 0
- HELP (?): 1
- FUNCTION_OR_LOOP: 2
- IF: 3
- LEFT_ASSIGN (<-, <<-, :=): 4
- EQUALS_ASSIGN (=): 5
- RIGHT_ASSIGN (->, ->>): 6
- TILDE (~): 7
- OR (|, ||): 8
- AND (&, &&): 9
- UNARY_NOT (!): 10
- COMPARISON (>, >=, <, <=, ==, !=): 11
- PLUS_MINUS (+, -): 12
- MULTIPLY_DIVIDE (*, /): 13
- SPECIAL_OR_PIPE (%>%, |>): 14
- COLON (:): 15
- UNARY_PLUS_MINUS: 16
- EXPONENTIATE (^, **): 17
- EXTRACT ($, @): 18
- NAMESPACE (::, :::): 19
- CALL: 20

**Evidence:** `grammar.js` lines 70-164

### Supported Syntax
- Function definitions (including `\` lambda syntax)
- All control flow: if/else, for, while, repeat, break, next
- Braced expressions, parenthesized expressions
- Function calls with named/unnamed arguments, empty arguments
- Subset `[`, subset2 `[[`
- All unary operators: ?, ~, !, +, -
- All binary operators: arithmetic, comparison, logical, assignment (<-, <<-, ->, ->>, =, :=), tilde, pipe (|>), special (%...%), colon
- Extract operators: $, @
- Namespace operators: ::, :::
- All numeric literals: integer, float, complex, hex
- All string types: single-quoted, double-quoted, raw strings (r"(...)")
- Escape sequences
- Identifiers (including backtick-quoted, dot-prefixed, Unicode XID)
- Special identifiers: ..., ..1/..2/etc
- Reserved words: TRUE, FALSE, NULL, Inf, NaN, NA (all variants)
- Comments
- Pipe placeholder `_`

### Known Deviations
- `]]` is treated as a single token (R allows it split across lines)
- **Evidence:** README.md lines 32-59

### Modern R Syntax
- ✓ Native pipe `|>` (precedence 14)
- ✓ Lambda `\(x) x + 1`
- ✓ Raw strings `r"(...)"`
- ✓ Pipe placeholder `_`
- ✓ Walrus operator `:=`

## 5. AST / IR

### Node Types
Defined in `src/node-types.json` (69KB). Major named node types:
- `program` (root)
- `function_definition` (fields: name, parameters, body)
- `parameters`, `parameter`
- `if_statement` (fields: condition, consequence, alternative)
- `for_statement` (fields: variable, sequence, body)
- `while_statement`, `repeat_statement`
- `braced_expression`, `parenthesized_expression`
- `call` (fields: function, arguments)
- `subset`, `subset2`
- `argument` (fields: name, value)
- `unary_operator`, `binary_operator` (fields: operator, lhs, rhs)
- `extract_operator`, `namespace_operator`
- `integer`, `float`, `complex`
- `string` (with string_open, string_content, string_close)
- `identifier`, `dots`, `dot_dot_i`
- `comment`, `comma`
- Keywords: `true`, `false`, `null`, `inf`, `nan`, `na`, `next`, `break`

### Metadata
- **Source positions:** Yes — every node has byte offset + row/col start and end positions
- **Comments:** Preserved as nodes in the tree (listed in `extras`)
- **Parent relationships:** Yes — tree-sitter trees are navigable up/down
- **Source reconstruction:** Possible from the CST (it preserves all tokens)
- **Serialization:** Available via tree-sitter's S-expression format
- **Fields:** Extensively used (function, parameters, body, condition, consequence, alternative, name, value, operator, lhs, rhs, open, close)

### What's NOT in the AST
- No semantic metadata (no type information, no resolved references)
- No package context
- No formula-specific node (formulas are binary_operator with ~ operator)
- No statistical or visualization awareness

## 6. Static Analysis

**None.** tree-sitter-r is purely a parser/grammar. It produces a CST. No static analysis is implemented.

However, it provides the foundation for `highlights.scm` and `tags.scm` query files used by editors.

**Evidence:** `queries/` directory contains highlight and tag queries. No analysis code exists in the repository.

## 7. Statistical Understanding

**None.** No statistical function recognition, no formula understanding, no model awareness. Formulas (`y ~ x`) are parsed as generic binary operators.

## 8. Visualization Understanding

**None.** ggplot2 code is parsed as ordinary function calls. No semantic awareness of aesthetics, layers, or visualization grammar.

## 9. Runtime Integration

**None.** tree-sitter-r is a pure parser. It does not execute R code.

However, flowR uses tree-sitter-r as one of its parser backends (see flowr.md).

## 10. Testing

- **Framework:** tree-sitter's built-in test corpus format
- **Test files:** `test/corpus/expressions.txt` (55KB), `test/corpus/literals.txt` (14KB), `test/corpus/expressions-errors.txt` (10KB), `test/corpus/literals-errors.txt` (1.4KB)
- **Test format:** Input R code → expected S-expression tree
- **Coverage:** Comprehensive — covers all expression types, edge cases, error recovery
- **Highlights tests:** `test/highlight/` directory
- **Tags tests:** `test/tags/` directory

**Evidence:** `test/corpus/` contains 4 test files totaling ~82KB

## 11. Extensibility

- Adding syntax: Modify `grammar.js`, regenerate parser
- Adding AST nodes: Modify grammar rules
- Adding analyses: Not applicable — tree-sitter-r is only a parser
- Adding features: Limited to parsing features; analysis must be external
- Extensibility is moderate — grammar changes require understanding of tree-sitter's DSL and precedence system

## 12. Strengths (Evidence-Based)

1. **Incremental parsing:** Tree-sitter re-parses only changed regions (critical for IDE use)
2. **Error recovery:** Produces partial trees even with syntax errors
3. **Comprehensive R syntax coverage:** All modern R features including |>, \(), raw strings
4. **Precise operator precedence:** Closely mirrors R's gram.y
5. **Rich node fields:** Allows structured queries (e.g., "find all function calls where argument named 'data'")
6. **Multi-language bindings:** R, Rust, Node.js, Python, C
7. **Actively maintained** by the Posit/r-lib ecosystem
8. **Used in production** by Positron IDE and flowR

## 13. Limitations (Evidence-Based)

- **Verified:** No static analysis capability
- **Verified:** No semantic understanding — purely syntactic
- **Verified:** `]]` deviation from R grammar (README)
- **Verified:** Formulas not represented as dedicated nodes
- **Verified:** No S4/R6/R5 class-specific syntax awareness (these are function calls to the parser)
- **Verified:** No package/namespace resolution
- **Likely:** Cannot distinguish metaprogramming/NSE constructs from regular code

## 14. Reusable Ideas for R-PARSE

1. **Operator precedence table** (lines 70-164 of grammar.js) — directly reusable as our precedence reference
2. **External scanner pattern** — handling newline significance, bracket matching, raw strings
3. **Node field naming conventions** — structured field access (function, parameters, body, etc.)
4. **Test corpus format** — input/expected-output pairs for parser validation
5. **Known deviations documentation** — explicit documentation of grammar compromises

## 15. Potential Overlap with R-PARSE

**High overlap in parsing.** tree-sitter-r already provides:
- Complete R grammar
- CST generation with source locations
- Incremental parsing
- Error recovery

R-PARSE should **NOT** reimplement tree-sitter-r's parser. Instead, R-PARSE should either:
1. Use tree-sitter-r as its parsing frontend, or
2. Build on top of the native R parser

The value R-PARSE adds is **above** the parsing layer: semantic analysis, statistical understanding, visualization understanding, workflow reconstruction.
