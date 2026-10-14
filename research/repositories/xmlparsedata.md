# xmlparsedata — Repository Analysis

## 1. Project Identity

- **Name:** xmlparsedata
- **Repository:** https://github.com/r-lib/xmlparsedata
- **Organization:** r-lib (Posit / RStudio open-source infrastructure)
- **Language:** R (100%)
- **License:** MIT
- **Status:** Stable, maintained (core dependency of `lintr` and `languageserver`)
- **Size:** 4 R source files, ~231 lines of code in `R/package.R`
- **Dependencies:** `xml2` (downstream consumer)

---

## 2. Purpose & Problem Solved

Converts base R's flat parse table output (`utils::getParseData()`) into a hierarchical XML document string.
Base R provides parse data as a flat data frame with `parent`, `token`, `text`, `line1`, `col1`, `line2`, `col2`. Navigating tree relationships directly on this flat data frame in R requires tedious row indexing and graph operations. By converting the table into an XML document, analyzers can query the AST using standard **XPath 1.0** expressions via the `xml2` package.

---

## 3. Architecture & Execution Flow

```text
R Source Code
      │
      ▼
parse(keep.source=TRUE) [base R C parser]
      │
      ▼
utils::getParseData(x) [returns flat data.frame of parse tokens & parents]
      │
      ▼
xmlparsedata::xml_parse_data() [R/package.R line 60]
      ├─► fix_comments(pd) [fixes parent=-1 on comments to parent=0, line 179]
      ├─► Octal escape workaround [fixes bug #18323 STR_CONST mismatch, lines 90-100]
      ├─► map_token() [translates invalid XML characters e.g. '+' -> OP-PLUS, line 184]
      ├─► Builds XML tags: <expr>, <SYMBOL>, <NUM_CONST>, etc.
      │
      ▼
XML string (<exprlist>...</exprlist>)
      │
      ▼
xml2::read_xml() ──► xml2::xml_find_all(doc, xpath)
```

---

## 4. Parsing Mechanism

`xmlparsedata` does **not parse R code directly**. It delegates 100% of the parsing to R's native C parser via:
```r
pd <- getParseData(x, includeText = includeText) # package.R line 71
```
If `x` was already parsed without `keep.source=TRUE`, it writes `x` to a temporary file via `dput()` and re-parses it (`package.R` lines 73-80).

---

## 5. AST & IR Model

The IR is a serialized XML DOM tree:
- **Root tag:** `<exprlist>`
- **Non-terminal expressions:** `<expr>` with attributes `line1`, `col1`, `line2`, `col2`, `start`, `end`.
- **Terminal tokens:** Tag names derived from R's token types:
  - Valid XML names remain unchanged: `<SYMBOL>`, `<NUM_CONST>`, `<STR_CONST>`, `<SYMBOL_FUNCTION_CALL>`, `<EQ_ASSIGN>`, etc.
  - Operators that violate XML naming are mapped via `xml_parse_token_map` (`package.R` lines 200-221):
    - `'?'` → `OP-QUESTION`
    - `'~'` → `OP-TILDE`
    - `'+'` → `OP-PLUS`, `'-'` → `OP-MINUS`, `'*'` → `OP-STAR`, `'/'` → `OP-SLASH`
    - `':'` → `OP-COLON`, `'^'` → `OP-CARET`, `'$'` → `OP-DOLLAR`, `'@'` → `OP-AT`
    - `'('` → `OP-LEFT-PAREN`, `')'` → `OP-RIGHT-PAREN`
    - `'['` → `OP-LEFT-BRACKET`, `']'` → `OP-RIGHT-BRACKET`
    - `'{'` → `OP-LEFT-BRACE`, `'}'` → `OP-RIGHT-BRACE`
    - `';'` → `OP-SEMICOLON`, `','` → `OP-COMMA`
    - `'\\'` → `OP-LAMBDA` (R 4.1+ shorthand anonymous function)

---

## 6. Analysis Capabilities

- **None directly in the package.** It is purely a data representation converter.
- Downstream packages (`lintr`, `languageserver`) execute XPath queries on the resulting DOM.

---

## 7. Strengths

1. **Leverages 100% Accurate GNU R Grammar:** Because it uses `getParseData()`, syntax coverage is identical to whatever version of R is running.
2. **Enables XPath Power:** Querying AST patterns becomes concise and expressive (e.g., `//SYMBOL_FUNCTION_CALL[text() = 'attach']`).
3. **Lightweight & Stable:** Tiny codebase, zero compiled code, rock-solid reliability.

---

## 8. Limitations & Failure Modes

1. **Hard R Runtime Dependency:** Cannot parse R code in environments lacking GNU R.
2. **Performance Bottleneck:** 
   - Parse code → create flat table in R → allocate XML string in R → parse XML string into C++ DOM in `libxml2`/`xml2`.
   - Generates massive string and DOM overhead for large files (10k+ LOC scripts create tens of thousands of XML nodes).
3. **No Incremental Parsing:** Re-parsing a single modified line requires re-parsing the entire file, re-generating the entire XML string, and re-parsing the XML.
4. **No Semantic Awareness:** Only provides syntactic hierarchy; no symbol resolution, no environment chains, no type information.

---

## 9. Relevance & Reusable Patterns for R-PARSE

- **Token Normalization Map:** The translation table from R punctuation operators to valid identifier names (`OP-PLUS`, `OP-TILDE`, etc.) is directly reusable in any serialized AST representation (XML, JSON, or protocol buffers).
- **Benchmark Baseline:** Represents the de-facto speed and capability baseline for all R-based linting and IDE tools today.
