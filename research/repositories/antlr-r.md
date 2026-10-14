# ANTLR R Grammar — Repository Analysis

## 1. Project Identity

- **Name:** ANTLR grammars-v4 / R Grammar
- **Repository:** https://github.com/antlr/grammars-v4 (path: `r/`)
- **Organization:** antlr (community-maintained)
- **Language:** ANTLR4 grammar (.g4), Java/Python targets
- **License:** BSD
- **Status:** Maintained but minimal — the R grammar is relatively simple
- **Grammar file:** `r/R.g4` (268 lines, 6.4KB)

## 2. Purpose

Provides an ANTLR4 grammar for parsing R code. Part of the larger grammars-v4 collection. Intended for building parsers in ANTLR-supported target languages (Java, Python, C++, etc.).

## 3. Architecture

```text
R Source Code
     ↓
ANTLR4 Lexer (defined in R.g4)
     ↓
ANTLR4 Parser (generated from R.g4)
     ↓
Parse Tree (ANTLR CST)
```

**Note:** Requires a companion `RFilter.g4` to strip whitespace/newlines that are semantically insignificant. This is mentioned in the grammar header but is NOT included in the repository.

**Evidence:** `R.g4` line 35-36: "Requires RFilter.g4 to strip away NL that are really whitespace, not end-of-command."

## 4. Parsing

### Grammar (268 lines)
- **Combined grammar** — lexer and parser in one file
- **Derived from:** `http://svn.r-project.org/R/trunk/src/main/gram.y`
- **Parser algorithm:** ANTLR4 (Adaptive LL(*))

### Supported Syntax
- Expressions with labeled alternatives (ListAccess, ArrayAccess, NamespaceAccess, etc.)
- Function definitions, function calls
- Control flow: if/else, for, while, repeat
- All assignment operators: <-, <<-, ->, ->>, =, :=
- Arithmetic: +, -, *, /, ^
- Comparison: >, >=, <, <=, ==, !=
- Logical: &, &&, |, ||, !
- Formulas: ~ (both prefix and infix)
- Range operator: :
- User-defined operators: %...%
- String literals (double, single, backtick)
- Hex, integer, float, complex literals
- Comments (mapped to NL token)
- Identifiers, NULL, NA, Inf, NaN, TRUE, FALSE

### What's MISSING
- **No raw string support** (`r"(...)"`) — not in the grammar
- **No native pipe** `|>` — not in the grammar
- **No lambda syntax** `\(x) x` — not in the grammar
- **No pipe placeholder** `_` — not in the grammar
- **No walrus operator** `:=` — wait, it IS included in ASSIGN
- **Comments discarded** — mapped to NL tokens and thus lost
- **Newline handling incomplete** — relies on missing RFilter.g4

**Evidence:** R.g4 lines 171, 256-257. The COMMENT rule maps to NL type. No `|>` in any operator rule.

### Operator Precedence
Handled via ANTLR4's alternative ordering in the `expr` rule. Not all operators have correct precedence compared to R's actual grammar — ANTLR4 uses rule ordering for precedence in left-recursive rules.

## 5. AST / IR

- Produces ANTLR4 parse tree with labeled alternatives
- Labels: ListAccess, ArrayAccess, NamespaceAccess, ComponentAccess, Exponent, Sign, Range, UserDefinedOperation, MultOrDiv, AddOrSub, Comparison, Not, And, Or, ModelFormulaePrefix, ModelFormulaeInfix, Assignment, FunctionDefinition, FunctionCall, CompoundStatement, If, IfElse, For, While, Repeat, Help, Next, Break, BracketTerm, Id, String, Hex, Int, Float, Complex, Null, Na, Inf, Nan, True, False, Newline

### Metadata
- Source positions: Yes (ANTLR provides token positions)
- Comments: **Lost** (mapped to NL)
- Parent relationships: Yes (ANTLR parse tree navigation)

## 6. Static Analysis

**None.** The grammar only provides parsing. No analysis tools included.

## 7-9. Statistical/Visualization/Runtime

**None.** Grammar only.

## 10. Testing

- `r/examples/` directory contains example R files
- Java and Python3 target directories with test infrastructure
- No comprehensive grammar test suite visible

## 11. Extensibility

- Grammar can be extended by modifying R.g4
- ANTLR4 supports listeners and visitors for tree walking
- Target language flexibility (Java, Python, C++, Go, etc.)

## 12. Strengths

1. **Simple, readable grammar** (268 lines)
2. **Multi-target:** Generate parsers in many languages
3. **Labeled alternatives** make tree walking easier
4. **BSD license** — permissive

## 13. Limitations

- **Verified:** Missing modern R syntax (|>, \() lambda, raw strings)
- **Verified:** Comments are discarded (COMMENT → NL)
- **Verified:** Requires missing RFilter.g4 for correct newline handling
- **Verified:** Grammar is dated (copyright 2013, derived from old gram.y)
- **Verified:** Only 268 lines — significantly less comprehensive than tree-sitter-r (787 lines)
- **Verified:** `inf` (lowercase) for Inf — incorrect, R uses uppercase `Inf`
- **Likely:** Precedence may not perfectly match R's grammar for all edge cases

## 14. Reusable Ideas for R-PARSE

1. **Labeled alternatives** — clean way to name expression variants
2. **Formula handling** — ModelFormulaePrefix and ModelFormulaeInfix distinction
3. **ANTLR4 as alternative target** — if we need parsers in Java/Python/C++

## 15. Potential Overlap with R-PARSE

**Low.** The ANTLR R grammar is incomplete and outdated. It would need significant updates to be useful for R-PARSE. tree-sitter-r is a far superior parser choice.

If R-PARSE needs an ANTLR parser (e.g., for Java integration), the grammar would need to be substantially rewritten to support modern R.
